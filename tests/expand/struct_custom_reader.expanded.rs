use packetrs::*;
fn custom_reader(
    _buf: &mut ::packetrs::bitcursor::BitCursor,
    _ctx: (),
) -> ::packetrs::error::PacketRsResult<MyStruct> {
    Ok(MyStruct { foo: 42 })
}
#[packetrs(reader = "custom_reader")]
struct MyStruct {
    foo: u8,
}
impl ::packetrs::packetrs_read::PacketrsRead<()> for MyStruct {
    fn read(
        buf: &mut ::packetrs::bitvec::bit_cursor::BitCursor,
        ctx: (),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        custom_reader(buf, ()).context("custom_reader")
    }
}
