pub fn sysex(id: [u8; 3], msg: Vec<u8>) -> Vec<u8> {
    let mut rv: Vec<u8> = vec![];

    rv.push(0xF0);
    rv.extend(id);
    rv.extend(msg);
    rv.push(0xF7);

    return rv;
}
