/******************************************************************************/

pub enum SysexId: [u8] {
    NOVATION_SYSEX_ID = [0x00, 0x20, 0x29],
}

pub struct SysexMsg {}

/******************************************************************************/

const SOX: u8 = 0xF0;
const EOX: u8 = 0xF7;

/******************************************************************************/

pub fn split_sysex_msgs(msgs: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut rv: Vec<Vec<u8>> = vec![vec![]];

    return rv;
}

/******************************************************************************/

pub fn sysex(id: [u8; 3], msg: &Vec<u8>) -> Vec<u8> {
    let mut rv: Vec<u8> = vec![];

    rv.push(SOX);
    rv.extend(id);
    rv.extend(msg);
    rv.push(EOX);

    return rv;
}
