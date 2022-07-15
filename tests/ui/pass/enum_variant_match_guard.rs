use packetrs::*;

#[derive(PacketrsRead)]
#[packetrs(required_ctx = "value: u32", key = "value")]
enum MyEnum {
    #[packetrs(id = "x if x > 10")]
    One
}

fn main() {}
