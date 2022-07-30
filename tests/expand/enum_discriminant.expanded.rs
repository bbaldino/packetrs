use packetrs::*;
#[packetrs(key = "1")]
enum MyEnum {
    One = 1,
    Two = 2,
    Three = 3,
}
impl ::packetrs::packetrs_read::PacketrsRead<()> for MyEnum {
    fn read(
        buf: &mut ::packetrs::bitcursor::BitCursor,
        ctx: (),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        match 1 {
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
