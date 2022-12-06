use packetrs::*;
struct MyStruct {
    foo: u8,
    bar: u16,
}
impl ::packetrs::packetrs_read::PacketrsRead<()> for MyStruct {
    fn read<T: ::packetrs::b3::byte_order::ByteOrder>(
        buf: &mut ::packetrs::b3::bit_cursor::BitCursor,
        ctx: (),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let foo = u8::read::<NetworkOrder>(buf, ()).context("foo")?;
        let bar = u16::read::<NetworkOrder>(buf, ()).context("bar")?;
        Ok(Self { foo, bar })
    }
}
