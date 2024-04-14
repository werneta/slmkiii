use crate::controls::{defaults, Button, Fader, Footswitch, Key, Pad, Pedal, Rotary, Wheel};
use crate::midi::SysEx;
use std::iter::repeat;

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

/******************************************************************************/

pub fn serialize(template: &Template) -> Vec<u8> {
    let mut rv: Vec<u8> = vec![];
    let ser_len = 4214; // Bytes

    /*
    // Bogus filler logic, capturing the 16 messages
    let mut msg: Vec<u8> = vec![];
    msg.extend(repeat(0).take(4214 - 2 - 3));
    let msg0 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg1 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg2 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg3 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg4 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg5 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg6 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg7 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg8 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg9 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg10 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg11 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg12 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg13 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg14 = sysex(NOVATION_SYSEX_ID, &msg);
    let msg15 = sysex(NOVATION_SYSEX_ID, &msg);

    rv.extend(msg0);
    rv.extend(msg1);
    rv.extend(msg2);
    rv.extend(msg3);
    rv.extend(msg4);
    rv.extend(msg5);
    rv.extend(msg6);
    rv.extend(msg7);
    rv.extend(msg8);
    rv.extend(msg9);
    rv.extend(msg10);
    rv.extend(msg11);
    rv.extend(msg12);
    rv.extend(msg13);
    rv.extend(msg14);
    rv.extend(msg15);
    */

    return rv;
}
