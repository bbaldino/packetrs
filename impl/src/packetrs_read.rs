use bit_cursor::{bit_read::BitRead, bitcursor::BitCursor, ux::*};

use crate::error::PacketRsResult;

/// This trait is what will be derived for a struct, and can be used to implement custom read logic
/// for types
pub trait PacketrsRead<Ctx>: Sized {
    fn read(buf: &mut BitCursor, ctx: Ctx) -> PacketRsResult<Self>;
}

macro_rules! packetrs_read_builtin {
    ($type:ty) => {
        impl PacketrsRead<()> for $type {
            fn read(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
                Ok(<$type as BitRead>::bit_read(buf)?)
            }
        }
    };
}

packetrs_read_builtin!(bool);
packetrs_read_builtin!(u2);
packetrs_read_builtin!(u3);
packetrs_read_builtin!(u4);
packetrs_read_builtin!(u5);
packetrs_read_builtin!(u6);
packetrs_read_builtin!(u7);
packetrs_read_builtin!(u8);
packetrs_read_builtin!(u9);
packetrs_read_builtin!(u10);
packetrs_read_builtin!(u11);
packetrs_read_builtin!(u12);
packetrs_read_builtin!(u13);
packetrs_read_builtin!(u14);
packetrs_read_builtin!(u15);
packetrs_read_builtin!(u16);
packetrs_read_builtin!(u17);
packetrs_read_builtin!(u18);
packetrs_read_builtin!(u19);
packetrs_read_builtin!(u20);
packetrs_read_builtin!(u21);
packetrs_read_builtin!(u22);
packetrs_read_builtin!(u23);
packetrs_read_builtin!(u24);
packetrs_read_builtin!(u25);
packetrs_read_builtin!(u26);
packetrs_read_builtin!(u27);
packetrs_read_builtin!(u28);
packetrs_read_builtin!(u29);
packetrs_read_builtin!(u30);
packetrs_read_builtin!(u31);
packetrs_read_builtin!(u32);
