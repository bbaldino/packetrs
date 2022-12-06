use packetrs::prelude::*;

static mut CUSTOM_METHOD_CALLED: bool = false;

fn custom_reader(
    _buf: &mut BitCursor,
    _ctx: (),
) -> PacketRsResult<MyStruct> {
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
    let mut buf = BitCursor::from_vec(data);

    let _ms = MyStruct::read::<NetworkOrder>(&mut buf, ());

    unsafe {
        assert!(CUSTOM_METHOD_CALLED);
    }
}

