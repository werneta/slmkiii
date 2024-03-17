use super::{Assignment1, Assignment1Type, MidiBits, Rotary, RotaryBehavior, RotaryType};

pub fn rotary(id: u8) -> Rotary {
    let idx = id as usize;
    let cc_lut: [u8; 16] = [
        21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38,
    ];

    let name = format!("CC {}", cc_lut[idx]);

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
                number: id,
                nbits: MidiBits::SEVEN,
            },
            channel: 0,
            start: 0,
            end: 127,
        },
        pivot: 0,
    };
}
