use packetrs::*;

#[derive(PacketrsRead)]
struct MyStruct {
    #[packetrs(while = "1 > 2")]
    values: Vec<u32>
}
