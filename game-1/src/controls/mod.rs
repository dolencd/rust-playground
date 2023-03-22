pub enum Controls {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    InventorySlot(u8),
    Esc,
}

impl TryFrom<[u8; 4]> for Controls {
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        println!("{:?}", value);
        Ok(match value {
            [119, 0, 0, 0] => Controls::MoveUp,
            [97, 0, 0, 0] => Controls::MoveLeft,
            [115, 0, 0, 0] => Controls::MoveDown,
            [100, 0, 0, 0] => Controls::MoveRight,
            [e @ 49..=56, 0, 0, 0] => Controls::InventorySlot(e),
            [27, 0, 0, 0] => Controls::Esc,
            _ => return Err(()),
        })
    }
    type Error = ();
}
