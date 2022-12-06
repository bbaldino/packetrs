use packetrs::*;
struct MyStruct(u8, u16);
impl ::packetrs::packetrs_read::PacketrsRead<()> for MyStruct {
    fn read<T: ::packetrs::b3::byte_order::ByteOrder>(
        buf: &mut ::packetrs::b3::bit_cursor::BitCursor,
        ctx: (),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let field_0 = u8::read::<NetworkOrder>(buf, ()).context("field_0")?;
        let field_1 = u16::read::<NetworkOrder>(buf, ()).context("field_1")?;
        Ok(Self(field_0, field_1))
    }
}
