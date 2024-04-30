#[derive(Debug, PartialEq, Clone)]
pub enum RotaryType {
    ABSOLUTE,
    RELATIVE,
}

/******************************************************************************/

#[derive(Debug, PartialEq, Clone)]
pub struct RotaryBehavior {
    resolution: u16, // 30 - 3600
    ctype: RotaryType,
    step: u8, // 0-127
}

/******************************************************************************/
