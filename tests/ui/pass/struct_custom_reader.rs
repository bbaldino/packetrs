use packetrs::*;

static mut CUSTOM_METHOD_CALLED: bool = false;

fn custom_reader(
    _buf: &mut ::packetrs::bitcursor::BitCursor,
    _ctx: (),
) -> ::packetrs::error::PacketRsResult<MyStruct> {
    unsafe {
        CUSTOM_METHOD_CALLED = true;
    }
    Ok(MyStruct {
        foo: 42
    })
}

#[derive(PacketrsRead)]
#[packetrs(reader = "custom_reader")]
struct MyStruct {
    foo: u8
}

fn main() {
    let data: Vec<u8> = vec![];
    let mut buf = BitCursor::new(data);

    let _ms = MyStruct::read(&mut buf, ());

    unsafe {
        assert!(CUSTOM_METHOD_CALLED);
    }
}

