use packetrs::*;
fn custom_reader(
    _buf: &mut ::packetrs::bitcursor::BitCursor,
    _ctx: (u8, u16),
) -> ::packetrs::error::PacketRsResult<MyStruct> {
    Ok(MyStruct { foo: 42 })
}
#[packetrs(required_ctx = "size: u8, ty: u16", reader = "custom_reader")]
struct MyStruct {
    foo: u8,
}
impl ::packetrs::packetrs_read::PacketrsRead<(u8, u16)> for MyStruct {
    fn read(
        buf: &mut ::packetrs::bitcursor::BitCursor,
        ctx: (u8, u16),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let size: u8 = ctx.0;
        let ty: u16 = ctx.1;
        custom_reader(buf, (size, ty)).context("custom_reader")
    }
}
