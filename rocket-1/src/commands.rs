use crate::state::state::{
    delete_state as state_delete_state, exists, read_state, save_state, State,
};

pub fn create(args: Vec<&str>) -> Result<Option<State>, String> {
    if let [_command, name] = args.as_slice() {
        if exists(name) {
            return Err("State already exists".to_string());
        }
        let new_state: State = Default::default();
        save_state(name, &new_state).map_err(|err| err.to_string())?;
        Ok(Some(new_state))
    } else {
        Err("Incorrect number of arguments".to_string())
    }
}

pub fn add_num(args: Vec<&str>) -> Result<Option<State>, String> {
    println!("{:?}", args);
    if let [_command, name, to_add_str] = args.as_slice() {
        let num_to_add: i32 = to_add_str
            .parse()
            .map_err(|_| "Failed to parse int".to_string())?;
        let mut state: State = read_state(name).map_err(|err| err.to_string())?;
        state.num += num_to_add;
        save_state(name, &state).map_err(|err| err.to_string())?;
        Ok(Some(state))
    } else {
        Err("Incorrect number of arguments".to_string())
    }
}

pub fn add_to_arr(args: Vec<&str>) -> Result<Option<State>, String> {
    let (name, to_add_str) = if let [_, name, to_add_str] = args.as_slice() {
        (name, to_add_str)
    } else {
        return Err("Incorrect number of arguments".to_string());
    };
    let num_to_add: i32 = to_add_str
        .parse()
        .map_err(|_| "Failed to parse int".to_string())?;
    let mut state: State = read_state(name).map_err(|err| err.to_string())?;
    state.arr.push(num_to_add);
    save_state(name, &state).map_err(|err| err.to_string())?;
    Ok(Some(state))
}

pub fn delete_state(args: Vec<&str>) -> Result<Option<State>, String> {
    let name = if let [_, name] = args.as_slice() {
        name
    } else {
        return Err("Incorrect number of arguments".to_string());
    };
    state_delete_state(name).map_err(|e| e.to_string());
    Ok(None)
}
