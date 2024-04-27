#[derive(Debug)]
pub enum RotaryType {
    ABSOLUTE,
    RELATIVE,
}

/******************************************************************************/

#[derive(Debug)]
pub struct RotaryBehavior {
    resolution: u16, // 30 - 3600
    ctype: RotaryType,
    step: u8, // 0-127
}

/******************************************************************************/

#[derive(Debug)]
pub struct Rotary {
    name: String,
    enabled: bool,
    behavior: RotaryBehavior,
    assignment: Assignment1,
    pivot: u8, // 0-127
}

/******************************************************************************/

impl Rotary {
    pub fn serialize(self: &Rotary) -> Vec<u8> {
        let mut rv: Vec<u8> = vec![0];

        rv.push(if self.enabled { 1 } else { 0 });
        rv.extend(self.name.as_bytes());
        rv.extend(zeros(9 - self.name.len()));

        return rv;
    }
}
