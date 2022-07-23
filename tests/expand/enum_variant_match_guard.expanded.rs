use packetrs::*;
#[packetrs(required_ctx = "value: u32", key = "value")]
enum MyEnum {
    #[packetrs(id = "x if x > 10")]
    One,
}
impl ::packetrs::packetrs_read::PacketrsRead<(u32,)> for MyEnum {
    fn read(
        buf: &mut ::packetrs::bitcursor::BitCursor,
        ctx: (u32,),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let value: u32 = ctx.0;
        match value {
            x if x > 10 => (|| Ok(MyEnum::One))().context("One"),
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
