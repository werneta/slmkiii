use super::{u14_to_bytes, zpad, BehaviorType, CmdType, MidiBits, MidiCh, NAME_LEN, STRUCT_LEN};

/******************************************************************************
 * API type definitions
 ******************************************************************************/

#[derive(Debug)]
pub struct Button {
    name: String,
    enabled: bool,
    btype: BehaviorType,
    ctype: CmdType,
    channel: MidiCh,
    midi_bits: MidiBits,
    cc_nrpn1: u8,
    down_on_to_trigger: u16,
    up_off_from: u16,
    nrpn2: u8,
    note: u8,
    on_release: bool,
    step: i16,
    wrap: bool,
    pair: bool,
}

/******************************************************************************
 * Trait implementations
 *****************************************************************************/

impl Into<Vec<u8>> for Button {
    fn into(self) -> Vec<u8> {
        let mut rv: Vec<u8> = Default::default();
        let name = self.name.as_bytes();
        let step: u16 = (0x2000 + self.step).try_into().unwrap();

        assert_ne!(self.ctype, CmdType::ChannelPressure);
        assert_ne!(self.ctype, CmdType::PolyAftertouch);
        assert!(name.len() <= NAME_LEN);

        rv.push(self.enabled.into());
        rv.extend(name);
        zpad(&mut rv, NAME_LEN + 1);
        rv.push(self.ctype.into());
        rv.push(0);

        rv.push(self.btype.into());
        rv.push(self.on_release.into());
        rv.extend(u14_to_bytes(self.down_on_to_trigger));
        rv.extend(u14_to_bytes(self.up_off_from));
        rv.extend(u14_to_bytes(step));

        rv.push(self.wrap.into());
        rv.push(self.pair.into());

        if self.ctype == CmdType::Note {
            rv.push(self.note);
        } else {
            rv.push(self.midi_bits.into());
        }
        rv.push(self.cc_nrpn1);
        rv.push(self.nrpn2);
        zpad(&mut rv, STRUCT_LEN);

        return rv;
    }
}
