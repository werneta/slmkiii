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
