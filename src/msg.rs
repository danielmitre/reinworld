const UNKNOWN: u8 = 0xFF;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum MsgType {
    GetMap = 0x10,
    Move = 0x20,
    Unknown = UNKNOWN,
}

impl From<u8> for MsgType {
    fn from(value: u8) -> Self {
        const GET_MAP: u8 = MsgType::GetMap as u8;
        const MOVE_: u8 = MsgType::Move as u8;
        match value {
            GET_MAP => MsgType::GetMap,
            MOVE_ => MsgType::Move,
            _ => MsgType::Unknown,
        }
    }
}

impl Default for MsgType {
    fn default() -> Self {
        MsgType::Unknown
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up = 0x01,
    Down = 0x02,
    Left = 0x03,
    Right = 0x04,
}

impl TryFrom<u8> for Direction {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        const UP: u8 = Direction::Up as u8;
        const DOWN: u8 = Direction::Down as u8;
        const LEFT: u8 = Direction::Left as u8;
        const RIGHT: u8 = Direction::Right as u8;
        match value {
            UP => Ok(Direction::Up),
            DOWN => Ok(Direction::Down),
            LEFT => Ok(Direction::Left),
            RIGHT => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

pub enum Msg {
    GetMap,
    Move(Direction),
    Unknown,
}

impl Default for Msg {
    fn default() -> Self {
        Msg::Unknown
    }
}

impl From<Vec<u8>> for Msg {
    fn from(bytes: Vec<u8>) -> Msg  {
        let mut it = bytes.into_iter();
        let type_ = it.next().unwrap_or_default();
        match MsgType::from(type_) {
            MsgType::GetMap => Msg::GetMap,
            MsgType::Move => {
                let _dir = it.next().unwrap_or_default();
                if let Ok(dir) = Direction::try_from(_dir) {
                    return Msg::Move(dir)
                }
                Msg::Unknown
            }
            _ => Msg::Unknown,
        }
    }
}
