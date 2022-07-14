use packetrs::*;

fn custom_reader(
    _buf: &mut ::packetrs::bitcursor::BitCursor,
    _ctx: (u8, u16),
) -> ::packetrs::error::PacketRsResult<MyStruct> {
    Ok(MyStruct {
        foo: 42
    })
}


#[derive(PacketrsRead)]
#[packetrs(required_ctx = "size: u8, ty: u16", reader = "custom_reader")]
struct MyStruct {
    foo: u8
}

