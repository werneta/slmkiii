use std::fmt;

/******************************************************************************/

const SYSTEM_EXCLUSIVE: u8 = 0xF0;
const EOX: u8 = 0xF7;
const MFGR_ID_NOVATION: [u8; 3] = [0x00, 0x20, 0x29];
const DEV_ID_SLMKIII: [u8; 3] = [0x02, 0x0A, 0x03];

const PACKETTYPE_START: u8 = 0x01;
const PACKETTYPE_CONTINUE: u8 = 0x02;
const PACKETTYPE_END: u8 = 0x03;

/******************************************************************************/

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DevId {
    SLMKIII,
}

/******************************************************************************/

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MfgrId {
    NOVATION,
}

/******************************************************************************/

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PacketType {
    START,
    CONTINUE,
    END,
}

/******************************************************************************/

impl Into<Vec<u8>> for DevId {
    fn into(self) -> Vec<u8> {
        return match self {
            DevId::SLMKIII => Vec::from(DEV_ID_SLMKIII),
        };
    }
}

/******************************************************************************/

impl Into<Vec<u8>> for MfgrId {
    fn into(self) -> Vec<u8> {
        return match self {
            MfgrId::NOVATION => Vec::from(MFGR_ID_NOVATION),
        };
    }
}

/******************************************************************************/

impl Into<Vec<u8>> for PacketType {
    fn into(self) -> Vec<u8> {
        return match self {
            PacketType::START => vec![PACKETTYPE_START],
            PacketType::CONTINUE => vec![PACKETTYPE_CONTINUE],
            PacketType::END => vec![PACKETTYPE_END],
        };
    }
}

/******************************************************************************/

fn get_dev_id(data: &[u8]) -> DevId {
    return if data.starts_with(&DEV_ID_SLMKIII) {
        DevId::SLMKIII
    } else {
        panic!("No matching device id")
    };
}

/******************************************************************************/

fn get_mfgr_id(data: &[u8]) -> (MfgrId, usize) {
    return if data.starts_with(&MFGR_ID_NOVATION) {
        (MfgrId::NOVATION, 3)
    } else {
        panic!("No matching mfgr id")
    };
}

/******************************************************************************/

fn get_packet_type(pkt_type: u8) -> PacketType {
    return match pkt_type {
        PACKETTYPE_START => PacketType::START,
        PACKETTYPE_CONTINUE => PacketType::CONTINUE,
        PACKETTYPE_END => PacketType::END,
        _ => panic!("No matching packet type"),
    };
}

/******************************************************************************/

#[derive(Debug, PartialEq, Clone)]
pub struct SysEx {
    mfgr: MfgrId,
    device: DevId,
    pkt_type: PacketType,
    pkt_id: u64,
    magic: u16,
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
            let msg = &msg[(1 + id_len)..];

            let device = get_dev_id(&msg);
            let msg = &msg[3..];

            let pkt_type = get_packet_type(msg[0]);
            let msg = &msg[1..];

            let pkt_id = msg[..8]
                .iter()
                .cloned()
                .fold(0, |acc: u64, v| (acc << 8) + (v as u64));
            let msg = &msg[8..];

            let magic = ((msg[0] as u16) << 8) | (msg[1] as u16);
            let msg = &msg[2..];

            let data = Vec::from(&msg[..(msg.len() - 1)]);
            msgs.push(SysEx {
                mfgr,
                device,
                pkt_type,
                pkt_id,
                magic,
                data,
            });
        }
    }

    return msgs;
}

/******************************************************************************/

impl Into<Vec<u8>> for SysEx {
    fn into(self) -> Vec<u8> {
        let mut rv: Vec<u8> = Default::default();

        rv.push(SYSTEM_EXCLUSIVE);
        rv.extend::<Vec<u8>>(self.mfgr.into());
        rv.extend::<Vec<u8>>(self.device.into());
        rv.extend::<Vec<u8>>(self.pkt_type.into());
        // TODO: pkt ID, magic
        rv.extend(&self.data);
        rv.push(EOX);

        return rv;
    }
}

/******************************************************************************/

impl fmt::Display for SysEx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data = Vec::new();

        if self.pkt_type == PacketType::CONTINUE {
            for (n, &dval) in self.data.iter().enumerate() {
                if (n % 8) != 0 {
                    data.push(dval);
                } else {
                    assert!(dval == 0x00);
                }
            }
        } else {
            data.extend(&self.data);
        }

        let hex = String::from_iter(data.iter().map(|v| format!("{:02x}", v)));
        let mut ascii: String = String::new();

        for &ch in data.iter() {
            let ch = ch as char;
            ascii.push(' ');
            ascii.push(match ch {
                ' ' | 'a'..='z' | 'A'..='Z' | '0'..='9' => ch,
                _ => '.',
            })
        }

        writeln!(f, "Mfgr:  {:?}", self.mfgr)?;
        writeln!(f, "Dev:   {:?}", self.device)?;
        writeln!(f, "Type:  {:?}", self.pkt_type)?;
        writeln!(f, "Pkt:   {:?}", self.pkt_id)?;
        writeln!(f, "Magic: {:?}", self.magic)?;
        writeln!(f, "Data:  {}", hex)?;
        writeln!(f, "Data:  {}", ascii)?;

        return Ok(());
    }
}
