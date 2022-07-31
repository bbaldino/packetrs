use packetrs::*;
struct MyStruct {
    #[packetrs(while = "1 > 2")]
    values: Vec<u32>,
}
impl ::packetrs::packetrs_read::PacketrsRead<()> for MyStruct {
    fn read(
        buf: &mut ::packetrs::bitcursor::BitCursor,
        ctx: (),
    ) -> ::packetrs::error::PacketRsResult<Self> {
        let values = (|| {
            let mut values = Vec::<::packetrs::error::PacketRsResult<u32>>::new();
            while 1 > 2 {
                values.push(u32::read(buf, ()).map_err(|e| e.into()));
            }
            values.into_iter().collect::<::packetrs::error::PacketRsResult<Vec<u32>>>()
        })()
            .context("values")?;
        Ok(Self { values })
    }
}
