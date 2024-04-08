pub const NOVATION_SYSEX_ID: [u8; 3] = [0x00, 0x20, 0x29];

pub fn sysex(id: [u8; 3], msg: Vec<u8>) -> Vec<u8> {
    let mut rv: Vec<u8> = vec![];

    rv.push(0xF0);
    rv.extend(id);
    rv.extend(msg);
    rv.push(0xF7);

    return rv;
}
