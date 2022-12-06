use packetrs::*;
#[packetrs(required_ctx = "left: u32, right: u32", key = "left + right")]
enum MyEnum {
    #[packetrs(id = "1")]
    One,
    #[packetrs(id = "2")]
    Two,
    #[packetrs(id = "3")]
    Three,
}
impl ::packetrs::packetrs_read::PacketrsRead<(u32, u32)> for MyEnum {
    fn read<T: ::packetrs::b3::byte_order::ByteOrder>(
        buf: &mut ::packetrs::b3::bit_cursor::BitCursor,
        ctx: (u32, u32),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let left: u32 = ctx.0;
        let right: u32 = ctx.1;
        match left + right {
            1 => (|| Ok(MyEnum::One))().context("One"),
            2 => (|| Ok(MyEnum::Two))().context("Two"),
            3 => (|| Ok(MyEnum::Three))().context("Three"),
            v @ _ => {
                ::core::panicking::panic_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &["not yet implemented: "],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &::core::fmt::Arguments::new_v1(
                                    &["Value of ", " is not implemented"],
                                    &[::core::fmt::ArgumentV1::new_debug(&v)],
                                ),
                            ),
                        ],
                    ),
                );
            }
        }
    }
}
