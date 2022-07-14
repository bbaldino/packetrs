use packetrs::*;

fn custom_reader(
    _buf: &mut ::packetrs::bitcursor::BitCursor,
    _ctx: (),
) -> ::packetrs::error::PacketRsResult<MyStruct> {
    Ok(MyStruct {
        foo: 42
    })
}

#[derive(PacketrsRead)]
#[packetrs(reader = "custom_reader")]
struct MyStruct {
    foo: u8
}
