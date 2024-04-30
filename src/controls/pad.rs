/******************************************************************************/

#[derive(Debug, PartialEq, Clone)]
pub enum ToggleType {
    OnPush,
    OnRelease,
}

/******************************************************************************/

#[derive(Debug, PartialEq, Clone)]
pub enum LimType {
    None,
    Limit,
    Scale,
}
