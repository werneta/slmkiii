pub mod defaults;

/******************************************************************************/

use std::iter::repeat;

/******************************************************************************/

const DEFAULT_CH: u8 = 127;
const STRUCT_LEN: usize = 44;
const NAME_LEN: usize = 9;

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

#[derive(Debug)]
pub enum ToggleType {
    OnPush,
    OnRelease,
}

/******************************************************************************/

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

#[derive(Debug)]
pub struct Fader {
    name: String,
    enabled: bool,
    assignment: Assignment2,
}

/******************************************************************************/

#[derive(Debug)]
pub struct Key {
    name: String,
    enabled: bool,
    assignment: AssignmentType,
    cc_nrpn1_note: u8,
    nrpn2: u8,
    midi_bits: MidiBits,
    channel: u8,
    start: u16,
    end: u16,
}

/******************************************************************************/

#[derive(Debug)]
pub enum LimType {
    None,
    Limit,
    Scale,
}

/******************************************************************************/

#[derive(Debug)]
pub enum HitType {
    Cc {
        number: u8,
        nbits: MidiBits,
    },
    Nrpn {
        msb: u8,
        lsb: u8,
        nbits: MidiBits,
    },
    Note {
        note: u8,
        min_vel: u8,
        max_vel: u8,
        lim: LimType,
    },
    ProgChange,
    SongPosn,
}

/******************************************************************************/

#[derive(Debug)]
pub struct HitAssignment {
    htype: HitType,
    channel: u8,
    down: u8,
    up: u8,
}

/******************************************************************************/

#[derive(Debug)]
pub struct Pad {
    name: String,
    behavior: ButtonBehavior,
    pad_hit: HitAssignment,
    hit_en: bool,
    assignment: Assignment2,
    assign_en: bool,
}

/******************************************************************************/

#[derive(Debug)]
pub struct Footswitch {
    name: String,
    enabled: bool,
    behavior: ButtonBehavior,
    assignment: Assignment1,
}

/******************************************************************************/

#[derive(Debug)]
pub struct Pedal {
    name: String,
    enabled: bool,
    assignment: Assignment2,
}

/******************************************************************************/

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

#[derive(Debug)]
pub struct Wheel {
    name: String,
    enabled: bool,
    assignment: AssignmentType,
    cc_nrpn1_note: u8,
    nrpn2: u8,
    midi_bits: MidiBits,
    channel: u8,
    start: u16,
    end: u16,
}

/******************************************************************************/

fn append_u16(mut vec: Vec<u8>, val: u16) {
    vec.push((val >> 8).try_into().unwrap());
    vec.push((val & 0xFF).try_into().unwrap());
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

/******************************************************************************/

impl From<MidiBits> for u8 {
    fn from(nbits: MidiBits) -> u8 {
        return match nbits {
            MidiBits::SEVEN => 0,
            MidiBits::EIGHT => 1,
            MidiBits::FOURTEEN => 2,
        };
    }
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

/******************************************************************************/

impl Into<Vec<u8>> for Key {
    fn into(self: Key) -> Vec<u8> {
        assert_ne!(self.assignment, AssignmentType::Note);
        assert_ne!(self.assignment, AssignmentType::SongPosn);
        assert!(self.name.len() <= 9);
        assert!(self.channel <= 16);

        let mut rv: Vec<u8> = Default::default();
        let name = self.name.as_bytes();

        let chan = self.channel;
        let chan = if chan == 0 { DEFAULT_CH } else { chan - 1 };

        rv.push(self.enabled as u8);
        rv.extend(name);
        zpad(rv, NAME_LEN + 1);
        rv.push(u8::from(self.assignment));
        rv.push(0);
        rv.push(chan);
        append_u16(rv, self.start);
        append_u16(rv, self.end);
        rv.push(u8::from(self.midi_bits));
        rv.push(self.cc_nrpn1_note);
        rv.push(self.nrpn2);
        zpad(rv, STRUCT_LEN);

        return rv;
    }
}

/******************************************************************************/

impl Into<Vec<u8>> for Wheel {
    fn into(self: Wheel) -> Vec<u8> {
        assert_ne!(self.assignment, AssignmentType::Note);
        assert_ne!(self.assignment, AssignmentType::SongPosn);
        assert!(self.name.len() <= 9);
        assert!(self.channel <= 16);

        let mut rv: Vec<u8> = Default::default();
        let name = self.name.as_bytes();

        let chan = self.channel;
        let chan = if chan == 0 { DEFAULT_CH } else { chan - 1 };

        rv.push(self.enabled as u8);
        rv.extend(name);
        zpad(rv, NAME_LEN + 1);
        rv.push(u8::from(self.assignment));
        rv.push(0);
        rv.push(chan);
        append_u16(rv, self.start);
        append_u16(rv, self.end);
        rv.push(u8::from(self.midi_bits));
        rv.push(self.cc_nrpn1_note);
        rv.push(self.nrpn2);
        zpad(rv, STRUCT_LEN);

        return rv;
    }
}
