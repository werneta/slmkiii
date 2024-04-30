use super::{
    Assignment1, Assignment1Type, Assignment2, Assignment2Type, Button, ButtonBehavior, Fader,
    Footswitch, HitAssignment, HitType, Key, LimType, MidiBits, Pad, Pedal, Rotary, RotaryBehavior,
    RotaryType, Wheel,
};

/******************************************************************************/

pub fn button(id: usize) -> Button {
    let cc_lut = [
        51, 52, 53, 54, 55, 56, 57, 58, 71, 72, 73, 74, 75, 76, 77, 73,
    ];
    let cc = cc_lut[id];

    let name = format!("CC {}", cc);

    return Button {
        name,
        enabled: true,
        behavior: ButtonBehavior::Momentary,
        assignment: Assignment1 {
            ctype: Assignment1Type::Cc {
                number: cc,
                nbits: MidiBits::SEVEN,
            },
            channel: 0,
            start_down: 127,
            end_up: 0,
        },
    };
}

/******************************************************************************/

pub fn fader(id: usize) -> Fader {
    let cc_lut = [41, 42, 43, 44, 45, 46, 47, 48];
    let cc = cc_lut[id];

    let name = format!("CC {}", cc);
    return Fader {
        name,
        enabled: true,
        assignment: Assignment2 {
            ctype: Assignment2Type::Cc {
                number: cc,
                nbits: MidiBits::SEVEN,
            },
            channel: 0,
            start: 0,
            end: 127,
        },
    };
}

/******************************************************************************/

pub fn footswitch() -> Footswitch {
    return Footswitch {
        name: String::from("Footsw."),
        enabled: true,
        behavior: ButtonBehavior::Momentary,
        assignment: Assignment1 {
            ctype: Assignment1Type::Cc {
                number: 68,
                nbits: MidiBits::SEVEN,
            },
            channel: 0,
            start_down: 127,
            end_up: 0,
        },
    };
}

/******************************************************************************/

pub fn key() -> Key {
    return Key {
        name: String::from("Aftertouch"),
        enabled: true,
        assignment: Assignment2 {
            ctype: Assignment2Type::ChannelPressure,
            channel: 0,
            start: 0,
            end: 127,
        },
    };
}

/******************************************************************************/

pub fn rotary(id: usize) -> Rotary {
    let cc_lut = [
        21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38,
    ];
    let cc = cc_lut[id];

    let name = format!("CC {}", cc);

    return Rotary {
        name,
        enabled: true,
        behavior: RotaryBehavior {
            resolution: 360,
            ctype: RotaryType::ABSOLUTE,
            step: 1,
        },
        assignment: Assignment1 {
            ctype: Assignment1Type::Cc {
                number: cc,
                nbits: MidiBits::SEVEN,
            },
            channel: 0,
            start_down: 0,
            end_up: 127,
        },
        pivot: 0,
    };
}

/******************************************************************************/

pub fn pad(id: usize) -> Pad {
    let cc_lut = [
        44, 45, 46, 47, 48, 49, 50, 51, 36, 37, 38, 39, 40, 41, 42, 43,
    ];
    let cc = cc_lut[id];

    let name = format!("CC {}", cc);

    return Pad {
        name,
        behavior: ButtonBehavior::Momentary,
        pad_hit: HitAssignment {
            htype: HitType::Note {
                note: cc,
                min_vel: 1,
                max_vel: 127,
                lim: LimType::None,
            },
            channel: 0,
            down: 127,
            up: 0,
        },
        hit_en: true,
        assignment: Assignment2 {
            ctype: Assignment2Type::PolyAftertouch { note: cc },
            channel: 0,
            start: 0,
            end: 127,
        },
        assign_en: true,
    };
}

/******************************************************************************/

pub fn pedal(sustain: bool) -> Pedal {
    let (name, cc) = if sustain {
        (String::from("Sustain"), 11)
    } else {
        (String::from("Expr."), 11)
    };

    return Pedal {
        name,
        enabled: true,
        assignment: Assignment2 {
            ctype: Assignment2Type::Cc {
                number: cc,
                nbits: MidiBits::SEVEN,
            },
            channel: 0,
            start: 0,
            end: 127,
        },
    };
}

/******************************************************************************/

pub fn wheel() -> Wheel {
    let cc = 1;
    return Wheel {
        name: format!("CC {}", cc),
        enabled: true,
        assignment: Assignment2 {
            ctype: Assignment2Type::Cc {
                number: cc,
                nbits: MidiBits::SEVEN,
            },
            channel: 0,
            start: 0,
            end: 127,
        },
    };
}

/******************************************************************************/
