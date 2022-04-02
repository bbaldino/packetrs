use bitcursor::bitcursor::BitCursor;

use crate::error::PacketRsResult;

/// This trait is what will be derived for a struct, and can be used to implement custom read logic
/// for types
pub trait PacketRsRead<Ctx>: Sized {
    fn read(buf: &mut BitCursor, ctx: Ctx) -> PacketRsResult<Self>;
}
