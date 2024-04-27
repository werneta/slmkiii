pub mod default;
pub mod key;
pub mod pedal;
pub mod rotary;
pub mod wheel;

/******************************************************************************/

use std::iter::repeat;

/******************************************************************************/

#[derive(Debug)]
pub enum MidiCh {
    DEFAULT,
    CHANNEL { ch: u8 },
}

/******************************************************************************/

#[derive(Debug)]
pub enum MidiBits {
    SEVEN,
    EIGHT,
    FOURTEEN,
}

/******************************************************************************/

#[derive(Debug, PartialEq)]
pub enum AssignmentType {
    Cc,
    Nrpn,
    Note,
    ProgChange,
    SongPosn,
    ChannelPressure,
    PolyAftertouch,
}

/******************************************************************************/

#[derive(Debug)]
pub struct Assignment1 {
    ctype: AssignmentType,
    channel: u8,    // 0 (default), 1-16
    start_down: u8, // 0-127
    end_up: u8,     // 0-127
}

/******************************************************************************/

#[derive(Debug)]
pub struct Assignment2 {
    ctype: AssignmentType,
    channel: u8,
    start: u16,
    end: u16,
}

/******************************************************************************/

impl From<AssignmentType> for u8 {
    fn from(atype: AssignmentType) -> u8 {
        return match atype {
            AssignmentType::Cc => 0,
            AssignmentType::Nrpn => 1,
            AssignmentType::Note => 2,
            AssignmentType::ProgChange => 3,
            AssignmentType::SongPosn => 4,
            AssignmentType::ChannelPressure => 5,
            AssignmentType::PolyAftertouch => 6,
        };
    }
}

/******************************************************************************
 * Trait definitions
 ******************************************************************************/

impl Into<u8> for MidiBits {
    fn into(self: MidiBits) -> u8 {
        return match self {
            MidiBits::SEVEN => SEVEN_BIT_MIDI_VAL,
            MidiBits::EIGHT => EIGHT_BIT_MIDI_VAL,
            MidiBits::FOURTEEN => FOURTEEN_BIT_MIDI_VAL,
        };
    }
}

/******************************************************************************/

impl Into<u8> for MidiCh {
    fn into(self: MidiCh) -> u8 {
        return match self {
            MidiCh::DEFAULT => DEFAULT_CH,
            MidiCh::CHANNEL { ch } => ch - 1,
        };
    }
}

/******************************************************************************/

impl TryFrom<u8> for MidiCh {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            DEFAULT_CH => Ok(MidiCh::DEFAULT),
            0..=15 => Ok(MidiCh::CHANNEL { ch: value + 1 }),
            _ => Err("MIDI Supports channels 1-16 only"),
        };
    }
}

/******************************************************************************
 * Module function definitions
 *****************************************************************************/

fn append_u16(mut vec: Vec<u8>, val: u16) {
    vec.push((val >> 8).try_into().unwrap());
    vec.push((val & 0xFF).try_into().unwrap());
}

/******************************************************************************/

fn zpad(mut vec: Vec<u8>, len: usize) {
    if len > vec.len() {
        vec.extend(zeros(len - vec.len()))
    }
}

/******************************************************************************/

fn zeros(num: usize) -> Vec<u8> {
    return repeat(0).take(num).collect();
}

/******************************************************************************
 * Module constant definitions
 *****************************************************************************/

const DEFAULT_CH: u8 = 127;
const STRUCT_LEN: usize = 44;
const NAME_LEN: usize = 9;

const SEVEN_BIT_MIDI_VAL: u8 = 0;
const EIGHT_BIT_MIDI_VAL: u8 = 1;
const FOURTEEN_BIT_MIDI_VAL: u8 = 2;
