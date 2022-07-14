use packetrs::*;

#[derive(PacketrsRead)]
struct MyStruct {
    foo: u8,
    bar: u16
}
