use crate::controls::{Button, Fader, Footswitch, Key, Pad, Pedal, Rotary, Wheel};

pub struct Template {
    name: String,
    rotary_controls: [Rotary; 16],
    faders: [Fader; 8],
    buttons: [Button; 16],
    pads: [Pad; 16],
    wheel: Wheel,
    sustain: Pedal,
    footswitch: Footswitch,
    expression: Pedal,
    keys: Key,
}
