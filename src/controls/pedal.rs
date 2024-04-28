use super::{zpad, CmdType, MidiBits, MidiCh, NAME_LEN, STRUCT_LEN};

/*
#[derive(Debug)]
pub struct Footswitch {
    name: String,
    enabled: bool,
    ctype: CmdType,
    cc_nrpn1_note: u8
    behavior: ButtonBehavior,
    assignment: Assignment1,
}
*/

/******************************************************************************
 * API type definitions
 ******************************************************************************/

#[derive(Debug)]
pub struct Pedal {
    name: String,
    enabled: bool,
    ctype: CmdType,
    cc_nrpn1_note: u8,
    nrpn2: u8,
    midi_bits: MidiBits,
    channel: MidiCh,
    start: u16,
    end: u16,
}

/******************************************************************************
 * Trait implementations
 ******************************************************************************/

impl Into<Vec<u8>> for Pedal {
    fn into(self: Pedal) -> Vec<u8> {
        let mut rv: Vec<u8> = Default::default();
        let name = self.name.as_bytes();

        assert_ne!(self.ctype, CmdType::Note);
        assert_ne!(self.ctype, CmdType::SongPosn);
        assert!(name.len() <= 9);

        rv.push(self.enabled.into());
        rv.extend(name);
        zpad(&mut rv, NAME_LEN + 1);
        rv.push(self.ctype.into());
        rv.push(0);
        rv.push(self.channel.into());
        rv.extend(self.start.to_be_bytes());
        rv.extend(self.end.to_be_bytes());
        rv.push(self.midi_bits.into());
        rv.push(self.cc_nrpn1_note);
        rv.push(self.nrpn2);
        zpad(&mut rv, STRUCT_LEN);

        return rv;
    }
}

/******************************************************************************/

impl TryFrom<&Vec<u8>> for Pedal {
    type Error = &'static str;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        return Err("Not Implemented");
    }
}
