use crate::controls::{defaults, Button, Fader, Footswitch, Key, Pad, Pedal, Rotary, Wheel};

/******************************************************************************/

#[derive(Debug)]
pub struct Template {
    pub name: String,
    pub rotary_controls: [Rotary; 16],
    pub faders: [Fader; 8],
    pub buttons: [Button; 16],
    pub pads: [Pad; 16],
    pub wheel: Wheel,
    pub sustain: Pedal,
    pub footswitch: Footswitch,
    pub expression: Pedal,
    pub keys: Key,
}

/******************************************************************************/

pub fn default() -> Template {
    // Is there a cleaner way to convert to an array?  And getting array lengths to put into the
    // sequence limits?
    let rotary_controls = (0..16).map(|id| defaults::rotary(id)).next_chunk().unwrap();
    let faders = (0..8).map(|id| defaults::fader(id)).next_chunk().unwrap();
    let buttons = (0..16).map(|id| defaults::button(id)).next_chunk().unwrap();
    let pads = (0..16).map(|id| defaults::pad(id)).next_chunk().unwrap();

    return Template {
        name: String::from("New Template"),
        rotary_controls,
        faders,
        buttons,
        pads,
        wheel: defaults::wheel(),
        sustain: defaults::pedal(true),
        footswitch: defaults::footswitch(),
        expression: defaults::pedal(false),
        keys: defaults::key(),
    };
}
