use packetrs::*;

#[derive(PacketrsRead)]
#[packetrs(required_ctx = "left: u32, right: u32", key = "left + right")]
enum MyEnum {
    #[packetrs(id = "1")]
    One,
    #[packetrs(id = "2")]
    Two,
    #[packetrs(id = "3")]
    Three
}
