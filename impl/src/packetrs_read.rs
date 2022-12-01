use bitvec::{bit_cursor::BitCursor, bit_read_exts::BitReadExts, ux::*, byte_order::NetworkOrder};

use crate::error::PacketRsResult;

/// This trait is what will be derived for a struct, and can be used to implement custom read logic
/// for types
pub trait PacketrsRead<Ctx>: Sized {
    fn read(buf: &mut BitCursor, ctx: Ctx) -> PacketRsResult<Self>;
}

impl PacketrsRead<()> for u1 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u1())
    }
}

impl PacketrsRead<()> for u2 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u2())
    }
}

impl PacketrsRead<()> for u3 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u3())
    }
}

impl PacketrsRead<()> for u4 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u4())
    }
}

impl PacketrsRead<()> for u5 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u5())
    }
}

impl PacketrsRead<()> for u6 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u6())
    }
}

impl PacketrsRead<()> for u7 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u7())
    }
}

impl PacketrsRead<()> for u8 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u8())
    }
}

impl PacketrsRead<()> for u9 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u9::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u10 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u10::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u11 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u11::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u12 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u12::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u13 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u13::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u14 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u14::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u15 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u15::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u16 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u16::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u17 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u17::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u18 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u18::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u19 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u19::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u20 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u20::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u21 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u21::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u22 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u22::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u23 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u23::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u24 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u24::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u25 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u25::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u26 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u26::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u27 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u27::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u28 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u28::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u29 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u29::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u30 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u30::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u31 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u31::<NetworkOrder>())
    }
}

impl PacketrsRead<()> for u32 {
    fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_u32::<NetworkOrder>())
    }
}
