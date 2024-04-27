use super::{append_u16, AssignmentType, MidiBits, NAME_LEN, STRUCT_LEN};

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
