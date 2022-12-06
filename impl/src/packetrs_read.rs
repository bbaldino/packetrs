use b3::{bit_cursor::BitCursor, ux::*, bit_read_exts::BitReadExts, byte_order::ByteOrder};

use crate::error::PacketRsResult;

/// This trait is what will be derived for a struct, and can be used to implement custom read logic
/// for types
pub trait PacketrsRead<Ctx>: Sized {
    fn read<T: ByteOrder>(buf: &mut BitCursor, ctx: Ctx) -> PacketRsResult<Self>;
}

macro_rules! packetrs_read_builtin {
    ($type:ty) => {
        impl PacketrsRead<()> for $type {
            fn read<T: ByteOrder>(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
                ::paste::paste! {
                    Ok(buf.[<read_ $type>]()?)
                }
            }
        }
    };
}

macro_rules! packetrs_read_builtin_bo {
    ($type:ty) => {
        impl PacketrsRead<()> for $type {
            fn read<T: ByteOrder>(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
                ::paste::paste! {
                    Ok(buf.[<read_ $type>]::<T>()?)
                }
            }
        }
    };
}

impl PacketrsRead<()> for bool {
    fn read<T: ByteOrder>(buf: &mut BitCursor, _: ()) -> PacketRsResult<Self> {
        Ok(buf.read_bool()?)
    }
}

packetrs_read_builtin!(u1);
packetrs_read_builtin!(u2);
packetrs_read_builtin!(u3);
packetrs_read_builtin!(u4);
packetrs_read_builtin!(u5);
packetrs_read_builtin!(u6);
packetrs_read_builtin!(u7);
packetrs_read_builtin!(u8);
packetrs_read_builtin_bo!(u9);
packetrs_read_builtin_bo!(u10);
packetrs_read_builtin_bo!(u11);
packetrs_read_builtin_bo!(u12);
packetrs_read_builtin_bo!(u13);
packetrs_read_builtin_bo!(u14);
packetrs_read_builtin_bo!(u15);
packetrs_read_builtin_bo!(u16);
packetrs_read_builtin_bo!(u17);
packetrs_read_builtin_bo!(u18);
packetrs_read_builtin_bo!(u19);
packetrs_read_builtin_bo!(u20);
packetrs_read_builtin_bo!(u21);
packetrs_read_builtin_bo!(u22);
packetrs_read_builtin_bo!(u23);
packetrs_read_builtin_bo!(u24);
packetrs_read_builtin_bo!(u25);
packetrs_read_builtin_bo!(u26);
packetrs_read_builtin_bo!(u27);
packetrs_read_builtin_bo!(u28);
packetrs_read_builtin_bo!(u29);
packetrs_read_builtin_bo!(u30);
packetrs_read_builtin_bo!(u31);
packetrs_read_builtin_bo!(u32);
