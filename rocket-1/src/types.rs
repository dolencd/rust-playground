pub mod types {
    use std::collections::HashMap;
    use std::vec;

    use serde::Serialize;
    use strum_macros::Display;

    fn parse_shutdown_type(v: Option<&str>) -> Option<ShutdownType> {
        let shutdown_type_map: HashMap<i32, ShutdownType> = HashMap::from([
            (1, ShutdownType::Reboot),
            (2, ShutdownType::Shutdown),
            (3, ShutdownType::Idle),
        ]);
        Some(shutdown_type_map.get(&(v?).parse().ok()?)?.to_owned())
    }

    pub fn decode_event(event: String) -> Option<DeviceHealthEvent> {
        let mut event_parts = event.split(':');

        match event_parts.next() {
            Some("B") => Some(DeviceHealthEvent::Boot(ParsedBootEvent {
                reasons: parse_boot_reasons(event_parts.next().unwrap_or("")),
            })),
            Some("S") => {
                let mut shutdown_event_parts = event_parts.next().unwrap_or("").split('-');

                Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                    shutdown_type: parse_shutdown_type(shutdown_event_parts.next()),
                    reasons: if let Some(next_part) = shutdown_event_parts.next() {
                        parse_shutdown_reason(next_part)
                    } else {
                        vec![]
                    },
                }))
            }
            Some("fuel_unknown") => Some(DeviceHealthEvent::FuelUnknown),
            Some("fuel_gas") => Some(DeviceHealthEvent::FuelGas),
            Some("fuel_diesel") => Some(DeviceHealthEvent::FuelDiesel),
            Some("odo_gps") => Some(DeviceHealthEvent::OdoGps),
            Some("odo_obd_speed") => Some(DeviceHealthEvent::OdoObdSpeed),
            Some("odo_dashboard") => Some(DeviceHealthEvent::OdoDashboard),
            Some("unknown") => Some(DeviceHealthEvent::Unknown),
            _ => None,
        }
    }

    pub fn read_bitmask_flags<T: Copy>(input_enum: Vec<Option<T>>, input_val: i32) -> Vec<T> {
        let mut output: Vec<T> = vec![];
        let mut i = 0;
        for val in input_enum {
            if val.is_some() && 1 << i & input_val != 0 {
                output.push(val.unwrap())
            }
            i += 1;
        }
        output
    }

    fn parse_shutdown_reason(val: &str) -> Vec<ShutdownReason> {
        if let Ok(input) = val.parse() {
            let mut output: Vec<ShutdownReason> = Vec::new();

            match input {
                3333 => output.push(ShutdownReason::DeviceUnplugged),
                3334 => output.push(ShutdownReason::ResetButtonPressed),
                7777 => output.push(ShutdownReason::TemperatureLimitReached),
                _ => (),
            };

            let div = input / 100;

            if [2, 5, 8].contains(&div) {
                match input / 100 {
                    5 => output.push(ShutdownReason::IdleTransformIntoShutdown),
                    8 => output.push(ShutdownReason::IdleTransformIntoReboot),
                    _ => (),
                };
                for reason in read_bitmask_flags(
                    vec![
                        Some(ShutdownReason::AccelerometerMovementAbsence),
                        Some(ShutdownReason::GpsMovementAbsence),
                        Some(ShutdownReason::IgnitionAbsenceOrPulse),
                        Some(ShutdownReason::DigitalInput1Absence),
                        Some(ShutdownReason::DigitalInput2Absence),
                        Some(ShutdownReason::ExternalBatteryAbsence),
                    ],
                    input % 100,
                ) {
                    output.push(reason);
                }
            }

            output
        } else {
            vec![]
        }
    }

    fn parse_boot_reasons(val: &str) -> Vec<BootReason> {
        if let Ok(parsed) = val.parse() {
            read_bitmask_flags(
                vec![
                    Some(BootReason::PowerOn),
                    Some(BootReason::IgnitionSignal),
                    Some(BootReason::ExternalPowerLowLevel),
                    Some(BootReason::AccelerometerMovement),
                    Some(BootReason::ModemRing),
                    Some(BootReason::DigitalInputs),
                    Some(BootReason::AlarmInput),
                    Some(BootReason::RtcWakeup),
                    Some(BootReason::Watchdog),
                    Some(BootReason::Watchdog),
                    None,
                    None,
                    Some(BootReason::Reboot),
                    None,
                    None,
                    None,
                    Some(BootReason::InternalBatLowLevel),
                    Some(BootReason::ExternalPowerLowLevel),
                ],
                parsed,
            )
        } else {
            vec![]
        }
    }

    #[derive(Debug, Clone, PartialEq, Display, Serialize)]
    pub enum DeviceHealthEvent {
        Shutdown(ParsedShutdownEvent),
        Boot(ParsedBootEvent),
        FuelUnknown,
        FuelGas,
        FuelDiesel,
        OdoGps,
        OdoObdSpeed,
        OdoDashboard,
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]

    pub struct ParsedShutdownEvent {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shutdown_type: Option<ShutdownType>,
        pub reasons: Vec<ShutdownReason>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct ParsedBootEvent {
        pub reasons: Vec<BootReason>,
    }

    #[derive(Debug, Clone, PartialEq, Copy, Display, Serialize)]
    pub enum ShutdownReason {
        DeviceUnplugged,
        ResetButtonPressed,
        TemperatureLimitReached,
        AccelerometerMovementAbsence,
        GpsMovementAbsence,
        IgnitionAbsenceOrPulse,
        DigitalInput1Absence,
        DigitalInput2Absence,
        ExternalBatteryAbsence,
        IdleTransformIntoShutdown,
        IdleTransformIntoReboot,
    }

    #[derive(Debug, Clone, PartialEq, Copy, Display, Serialize)]
    pub enum BootReason {
        PowerOn,
        IgnitionSignal,
        ExternalPowerSupply,
        AccelerometerMovement,
        ModemRing,
        DigitalInputs,
        AlarmInput,
        RtcWakeup,
        Watchdog,
        Reboot,
        InternalBatLowLevel,
        ExternalPowerLowLevel,
    }

    #[derive(Debug, Clone, PartialEq, Copy, Serialize)]
    pub enum ShutdownType {
        Reboot,
        Shutdown,
        Idle,
    }
}

#[cfg(test)]
mod tests {
    use crate::types::types::{BootReason, ParsedBootEvent};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::types::{
        decode_event, read_bitmask_flags, DeviceHealthEvent, ParsedShutdownEvent, ShutdownReason,
        ShutdownType,
    };

    #[test]
    fn test_bitmask() {
        assert_eq!(read_bitmask_flags(vec![] as Vec<Option<char>>, 63), vec![]);
        assert_eq!(read_bitmask_flags(vec![Some('a'), Some('b')], 1), vec!['a']);
        assert_eq!(read_bitmask_flags(vec![Some('a'), Some('b')], 2), vec!['b']);
        assert_eq!(
            read_bitmask_flags(vec![Some('a'), Some('b')], 3),
            vec!['a', 'b']
        );
        assert_eq!(read_bitmask_flags(vec![Some('a'), Some('b')], 4), vec![]);
        assert_eq!(
            read_bitmask_flags(vec![None, Some('a'), Some('b')], 1),
            vec![]
        );
        assert_eq!(
            read_bitmask_flags(
                vec![
                    Some('a'),
                    Some('b'),
                    Some('c'),
                    Some('d'),
                    Some('e'),
                    Some('f'),
                    Some('g'),
                    Some('h')
                ],
                63
            ),
            vec!['a', 'b', 'c', 'd', 'e', 'f']
        );
        assert_eq!(
            read_bitmask_flags(
                vec![
                    None,
                    Some('b'),
                    Some('c'),
                    None,
                    None,
                    Some('f'),
                    Some('g'),
                    Some('h')
                ],
                63
            ),
            vec!['b', 'c', 'f']
        );
    }

    #[test]
    fn test_decode_event() {
        assert_eq!(decode_event("No field".to_string()), None);
        assert_eq!(
            decode_event("odo_gps".to_string()),
            Some(DeviceHealthEvent::OdoGps)
        );
        assert_eq!(
            decode_event("S:2-7777".to_string()),
            Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                shutdown_type: Some(ShutdownType::Shutdown),
                reasons: vec![ShutdownReason::TemperatureLimitReached],
            }))
        );
        assert_eq!(
            decode_event("S:1-23".to_string()),
            Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                shutdown_type: Some(ShutdownType::Reboot),
                reasons: vec![],
            }))
        );
        assert_eq!(
            decode_event("S:3-205".to_string()),
            Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                shutdown_type: Some(ShutdownType::Idle),
                reasons: vec![
                    ShutdownReason::AccelerometerMovementAbsence,
                    ShutdownReason::IgnitionAbsenceOrPulse
                ],
            }))
        );
        assert_eq!(
            decode_event("S:1-805".to_string()),
            Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                shutdown_type: Some(ShutdownType::Reboot),
                reasons: vec![
                    ShutdownReason::IdleTransformIntoReboot,
                    ShutdownReason::AccelerometerMovementAbsence,
                    ShutdownReason::IgnitionAbsenceOrPulse
                ],
            }))
        );
        assert_eq!(
            decode_event("S:2-501".to_string()),
            Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                shutdown_type: Some(ShutdownType::Shutdown),
                reasons: vec![
                    ShutdownReason::IdleTransformIntoShutdown,
                    ShutdownReason::AccelerometerMovementAbsence
                ],
            }))
        );
        assert_eq!(
            decode_event("B:123".to_string()),
            Some(DeviceHealthEvent::Boot(ParsedBootEvent {
                reasons: vec![
                    BootReason::PowerOn,
                    BootReason::IgnitionSignal,
                    BootReason::AccelerometerMovement,
                    BootReason::ModemRing,
                    BootReason::DigitalInputs,
                    BootReason::AlarmInput
                ],
            }))
        );
        assert_eq!(
            decode_event("B:48".to_string()),
            Some(DeviceHealthEvent::Boot(ParsedBootEvent {
                reasons: vec![BootReason::ModemRing, BootReason::DigitalInputs,],
            }))
        );
        assert_eq!(decode_event("fgfsgf".to_string()), None);
        assert_eq!(
            decode_event("B:asd".to_string()),
            Some(DeviceHealthEvent::Boot(ParsedBootEvent { reasons: vec![] }))
        );
        assert_eq!(
            decode_event("B:b-asd".to_string()),
            Some(DeviceHealthEvent::Boot(ParsedBootEvent { reasons: vec![] }))
        );
        assert_eq!(
            decode_event("S:2".to_string()),
            Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                shutdown_type: Some(ShutdownType::Shutdown),
                reasons: vec![],
            }))
        );
        assert_eq!(
            decode_event("S:b".to_string()),
            Some(DeviceHealthEvent::Shutdown(ParsedShutdownEvent {
                shutdown_type: None,
                reasons: vec![],
            }))
        );
    }
} /*
  [
    {
      input: 2,
      input_flags: [null, 'a', 'b'],
      output: ['a'],
    },
    {
      input: 1,
      input_flags: [null, 'a', 'b'],
      output: [],
    },
    {
      input: 2,
      input_flags: ['a', 'b'],
      output: ['b'],
    },
    {
      input: 3,
      input_flags: ['a', 'b'],
      output: ['a', 'b'],
    },
    {
      input: 63,
      input_flags: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
      output: ['a', 'b', 'c', 'd', 'e', 'f'],
    },
    {
      input: 21,
      input_flags: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
      output: ['a', 'c', 'e'],
    },
    {
      input: 64,
      input_flags: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
      output: ['g'],
    },
  ]
   */
