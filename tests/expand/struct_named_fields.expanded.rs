use packetrs::*;
struct MyStruct {
    foo: u8,
    bar: u16,
}
impl ::packetrs::packetrs_read::PacketrsRead<()> for MyStruct {
    fn read(
        buf: &mut ::packetrs::bitvec::bit_cursor::BitCursor,
        ctx: (),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let foo = u8::read(buf, ()).context("foo")?;
        let bar = u16::read(buf, ()).context("bar")?;
        Ok(Self { foo, bar })
    }
}
