use std::fmt;

/******************************************************************************/

const SYSTEM_EXCLUSIVE: u8 = 0xF0;
const EOX: u8 = 0xF7;

/******************************************************************************/

#[derive(Debug)]
pub enum MfgrId {
    NOVATION,
}

/******************************************************************************/

impl MfgrId {
    fn bytes(&self) -> Vec<u8> {
        return match self {
            MfgrId::NOVATION => vec![0x00, 0x20, 0x29],
        };
    }
}

/******************************************************************************/

fn get_mfgr_id(data: &[u8]) -> (MfgrId, usize) {
    let first = data[0];

    return match first {
        0x00 => match data[1..3] {
            [0x20, 0x29] => (MfgrId::NOVATION, 3),
            _ => panic!("No matching mfgr id"),
        },
        _ => panic!("No matching mfgr id"),
    };
}

/******************************************************************************/

#[derive(Debug)]
pub struct SysEx {
    mfgr: MfgrId,
    data: Vec<u8>,
}

/******************************************************************************/

pub fn split_sysex_msgs(bytes: &Vec<u8>) -> Vec<SysEx> {
    let mut msgs: Vec<SysEx> = Default::default();
    let msg_bytes = bytes.split_inclusive(|byte| byte == &EOX);

    for msg in msg_bytes {
        if !msg.is_empty() {
            assert!(msg.starts_with(&[SYSTEM_EXCLUSIVE]));
            assert!(msg.ends_with(&[EOX]));

            let (mfgr, id_len) = get_mfgr_id(&msg[1..]);
            let data = Vec::from(&msg[(1 + id_len)..]);
            msgs.push(SysEx { mfgr, data });
        }
    }

    return msgs;
}

/******************************************************************************/

impl SysEx {
    pub fn bytes(&self) -> Vec<u8> {
        let mut rv: Vec<u8> = vec![];

        rv.push(SYSTEM_EXCLUSIVE);
        rv.extend(self.mfgr.bytes());
        rv.extend(&self.data);
        rv.push(EOX);

        return rv;
    }
}

/******************************************************************************/

impl fmt::Display for SysEx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.mfgr).unwrap();
        writeln!(
            f,
            "{}",
            self.data.iter().map(|&val| format!("{:X}", val)).collect()
        )
    }
}
