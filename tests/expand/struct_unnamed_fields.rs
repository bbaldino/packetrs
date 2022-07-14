use packetrs::*;

#[derive(PacketrsRead)]
struct MyStruct(u8, u16);
