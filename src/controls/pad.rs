/******************************************************************************/

#[derive(Debug)]
pub enum ToggleType {
    OnPush,
    OnRelease,
}

/******************************************************************************/

#[derive(Debug)]
pub enum LimType {
    None,
    Limit,
    Scale,
}
