use packetrs::prelude::*;

#[derive(PacketrsRead)]
#[packetrs(byte_order = "blah")]
struct Foo {
}

fn main() {}
