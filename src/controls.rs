pub mod default;
pub mod key;
pub mod pedal;
pub mod rotary;
pub mod slider;
pub mod wheel;

/******************************************************************************/

use std::iter::repeat;

/******************************************************************************
 * API type definitions
 ******************************************************************************/

#[derive(Debug, PartialEq)]
pub enum CmdType {
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
pub enum MidiBits {
    SEVEN,
    EIGHT,
    FOURTEEN,
}

/******************************************************************************/

#[derive(Debug)]
pub enum MidiCh {
    DEFAULT,
    CHANNEL { ch: u8 },
}

/******************************************************************************
 * Trait implementations
 *****************************************************************************/

impl Into<u8> for CmdType {
    fn into(self: CmdType) -> u8 {
        return match self {
            CmdType::Cc => CMD_TYPE_CC,
            CmdType::Nrpn => CMD_TYPE_NRPN,
            CmdType::Note => CMD_TYPE_NOTE,
            CmdType::ProgChange => CMD_TYPE_PROGRAM_CHANGE,
            CmdType::SongPosn => CMD_TYPE_SONG_POSITION,
            CmdType::ChannelPressure => CMD_TYPE_CHANNEL_PRESSURE,
            CmdType::PolyAftertouch => CMD_TYPE_POLYPHONIC_AFTERTOUCH,
        };
    }
}

/*****************************************************************************/

impl TryFrom<u8> for CmdType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            CMD_TYPE_CC => Ok(CmdType::Cc),
            CMD_TYPE_NRPN => Ok(CmdType::Nrpn),
            CMD_TYPE_NOTE => Ok(CmdType::Note),
            CMD_TYPE_PROGRAM_CHANGE => Ok(CmdType::ProgChange),
            CMD_TYPE_SONG_POSITION => Ok(CmdType::SongPosn),
            CMD_TYPE_CHANNEL_PRESSURE => Ok(CmdType::ChannelPressure),
            CMD_TYPE_POLYPHONIC_AFTERTOUCH => Ok(CmdType::PolyAftertouch),
            _ => Err("Invalid CmdType"),
        };
    }
}

/*****************************************************************************/

impl Into<u8> for MidiBits {
    fn into(self: MidiBits) -> u8 {
        return match self {
            MidiBits::SEVEN => MIDI_BITS_SEVEN,
            MidiBits::EIGHT => MIDI_BITS_EIGHT,
            MidiBits::FOURTEEN => MIDI_BITS_FOURTEEN,
        };
    }
}

/******************************************************************************/

impl TryFrom<u8> for MidiBits {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            MIDI_BITS_SEVEN => Ok(MidiBits::SEVEN),
            MIDI_BITS_EIGHT => Ok(MidiBits::EIGHT),
            MIDI_BITS_FOURTEEN => Ok(MidiBits::FOURTEEN),
            _ => Err("MIDI Bits only supports values 0, 1, and 2"),
        };
    }
}

/******************************************************************************/

impl Into<u8> for MidiCh {
    fn into(self: MidiCh) -> u8 {
        return match self {
            MidiCh::DEFAULT => MIDI_CH_DEFAULT,
            MidiCh::CHANNEL { ch } => ch - 1,
        };
    }
}

/******************************************************************************/

impl TryFrom<u8> for MidiCh {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            MIDI_CH_DEFAULT => Ok(MidiCh::DEFAULT),
            0..=15 => Ok(MidiCh::CHANNEL { ch: value + 1 }),
            _ => Err("MIDI Supports channels 1-16 only"),
        };
    }
}

/******************************************************************************
 * Module function definitions
 *****************************************************************************/

fn zpad(vec: &mut Vec<u8>, len: usize) {
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

const MIDI_CH_DEFAULT: u8 = 127;
const STRUCT_LEN: usize = 44;
const NAME_LEN: usize = 9;

const CMD_TYPE_CC: u8 = 0;
const CMD_TYPE_NRPN: u8 = 1;
const CMD_TYPE_NOTE: u8 = 2;
const CMD_TYPE_PROGRAM_CHANGE: u8 = 3;
const CMD_TYPE_SONG_POSITION: u8 = 4;
const CMD_TYPE_CHANNEL_PRESSURE: u8 = 5;
const CMD_TYPE_POLYPHONIC_AFTERTOUCH: u8 = 6;

const MIDI_BITS_SEVEN: u8 = 0;
const MIDI_BITS_EIGHT: u8 = 1;
const MIDI_BITS_FOURTEEN: u8 = 2;
