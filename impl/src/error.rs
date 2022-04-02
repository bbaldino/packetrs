use bitcursor::bitcursor::BitCursorError;

#[derive(Debug)]
pub enum PacketRsError {
    BitCursorError(BitCursorError),
}

pub type PacketRsResult<T> = Result<T, PacketRsError>;

impl From<BitCursorError> for PacketRsError {
    fn from(err: BitCursorError) -> Self {
        PacketRsError::BitCursorError(err)
    }
}
