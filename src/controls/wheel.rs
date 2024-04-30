use super::{u14_to_bytes, zpad, CmdType, MidiBits, MidiCh, NAME_LEN, STRUCT_LEN};

/******************************************************************************
 * API type definitions
 ******************************************************************************/

#[derive(Debug, PartialEq, Clone)]
pub struct Wheel {
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

impl Into<Vec<u8>> for Wheel {
    fn into(self: Wheel) -> Vec<u8> {
        let mut rv: Vec<u8> = Default::default();
        let name = self.name.as_bytes();

        assert_ne!(self.ctype, CmdType::Note);
        assert_ne!(self.ctype, CmdType::SongPosn);
        assert!(self.name.len() <= NAME_LEN);

        rv.push(self.enabled.into());
        rv.extend(name);
        zpad(&mut rv, NAME_LEN + 1);
        rv.push(self.ctype.into());
        rv.push(0);
        rv.push(self.channel.into());
        rv.extend(u14_to_bytes(self.start));
        rv.extend(u14_to_bytes(self.end));
        rv.push(self.midi_bits.into());
        rv.push(self.cc_nrpn1_note);
        rv.push(self.nrpn2);
        zpad(&mut rv, STRUCT_LEN);

        return rv;
    }
}
