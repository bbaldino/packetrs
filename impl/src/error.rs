#[derive(Debug)]
pub struct PacketRsError {}

pub type PacketRsResult<T> = Result<T, PacketRsError>;
