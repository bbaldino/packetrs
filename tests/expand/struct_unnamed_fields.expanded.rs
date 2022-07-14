use packetrs::*;
struct MyStruct(u8, u16);
impl ::packetrs::packetrs_read::PacketrsRead<()> for MyStruct {
    fn read(
        buf: &mut ::packetrs::bitcursor::BitCursor,
        ctx: (),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let field_0 = u8::read(buf, ()).context("field_0")?;
        let field_1 = u16::read(buf, ()).context("field_1")?;
        Ok(Self(field_0, field_1))
    }
}
