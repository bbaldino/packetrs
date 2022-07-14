use packetrs::*;

static mut CUSTOM_METHOD_CALLED: bool = false;

fn custom_reader(
    _buf: &mut ::packetrs::bitcursor::BitCursor,
    _ctx: (),
) -> ::packetrs::error::PacketRsResult<MyEnum> {
    unsafe {
        CUSTOM_METHOD_CALLED = true;
    }
    Ok(MyEnum::One)
}

#[derive(PacketrsRead)]
#[packetrs(reader = "custom_reader")]
enum MyEnum {
    One,
    Two,
    Three
}

fn main() {
    let data: Vec<u8> = vec![];
    let mut buf = BitCursor::new(data);

    let _ms = MyEnum::read(&mut buf, ());

    unsafe {
        assert!(CUSTOM_METHOD_CALLED);
    }
}
