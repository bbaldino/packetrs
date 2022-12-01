use packetrs::*;
use packetrs::{bit_cursor::BitCursor, bit_vec::BitVec};

static mut CUSTOM_METHOD_CALLED: bool = false;

fn custom_reader(
    _buf: &mut BitCursor,
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
    let data = BitVec::new();
    let mut buf = BitCursor::new(data);

    let _ms = MyEnum::read(&mut buf, ());

    unsafe {
        assert!(CUSTOM_METHOD_CALLED);
    }
}
