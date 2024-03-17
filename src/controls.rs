pub mod defaults;

/******************************************************************************/

#[derive(Debug)]
pub enum MidiBits {
    SEVEN,
    EIGHT,
    FOURTEEN,
}

/******************************************************************************/

#[derive(Debug)]
pub enum Assignment1Type {
    Cc { number: u8, nbits: MidiBits },
    Nrpn { msb: u8, lsb: u8, nbits: MidiBits },
    Note(u8),
    ProgChange,
    SongPosn,
}

/******************************************************************************/

#[derive(Debug)]
pub struct Assignment1 {
    ctype: Assignment1Type,
    channel: u8, // 0 (default), 1-16
    start: u8,   // 0-127
    end: u8,     // 0-127
}

/******************************************************************************/

#[derive(Debug)]
pub enum Assignment2Type {
    Cc { number: u8, nbits: MidiBits },
    Nrnp { msb: u8, lsb: u8, nbits: MidiBits },
    ProgChange,
    ChannelPressure,
    PolyAftertouch { note: u8 },
}

/******************************************************************************/

#[derive(Debug)]
pub struct Assignment2 {
    ctype: Assignment2Type,
    channel: u8,
    start: u8,
    end: u8,
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
    assignment: Assignment2,
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
    assignment: Assignment2,
}
