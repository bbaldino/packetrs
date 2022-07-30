use packetrs::*;

#[derive(PacketrsRead)]
#[packetrs(key = "1")]
enum MyEnum {
    One = 1,
    Two = 2,
    Three = 3,
}

