#[derive(Debug)]
pub enum ButtonBehavior {
    Momentary,
    Toggle(ToggleType),
    IncDec {
        ttype: ToggleType,
        step: i16,
        wrap: bool,
        pair: bool,
    }, // -8191-8192
    Trigger(ToggleType),
}

/******************************************************************************/

#[derive(Debug)]
pub struct Button {
    name: String,
    enabled: bool,
    behavior: ButtonBehavior,
    assignment: Assignment1,
}

/******************************************************************************/
