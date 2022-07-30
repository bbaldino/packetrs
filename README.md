# Packetrs

Packetrs is a Rust macro which auto generates deserialzation and serialization (eventually) code for struct packets on macro attributes.  Its API is heavily inspired by/ripped off from [Deku](https://github.com/sharksforarms/deku).  This was implemented mainly for my own fun/educational purposes.

##### Examples

Say you wanted to parse a STUN header, which has the following layout:
```
       0                   1                   2                   3
       0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |0 0|     STUN Message Type     |         Message Length        |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |                         Magic Cookie                          |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
      |                                                               |
      |                     Transaction ID (96 bits)                  |
      |                                                               |
      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```
In Packetrs, you'd do:
```rust
#[derive(Debug, PacketrsRead)]
pub struct StunHeader {
    #[packetrs(fixed = "0")]
    pub reserved: u2,
    pub msg_type: u14,
    pub msg_length: u16,
    #[packetrs(fixed = "0x2112A442")]
    pub cookie: u32,
    #[packetrs(count = "12")]
    pub transaction_id: Vec<u8>,
}
```

##### PacketrsRead Trait
Deriving `PacketrsRead` on a struct or enum generates an implementation of the `PacketrsRead` trait for that type:
```rust
pub trait PacketRsRead<Ctx>: Sized {
    fn read(buf: &mut BitCursor, ctx: Ctx) -> PacketRsResult<Self>;
}
```

TODO: link to BitCursor repo once it's up


#### PacketrsRead Attributes
##### Context & Required Context
Structs, fields, enums and enum variants all may need additional context in order to be read.  The `required_ctx` attribute allows a Struct, field, enum or enum variant to define a required value or values that must be passed to its `PacketrsRead::read` method.  The other side of this is the `ctx` attribute, which defines what will be passed to the read method of whatever is annotated.

In this example, the `Address` enum requires a `u8` named `address_family` to be passed, and then uses that value as the `key` to distinguish enum variants.
```rust
#[derive(Debug, PacketrsRead)]
#[packetrs(required_ctx = "address_family: u8", key = "address_family")]
pub enum Address {
    #[packetrs(id = "0x01")]
    IpV4(u32),
    #[packetrs(id = "0x02")]
    IpV6(u128),
}
```
Here, another enum passes an `address_family` field as context to an `Address` field 
```rust
#[derive(Debug, PacketrsRead)]
#[packetrs(required_ctx = "message_type: u16, length: u16", key = "message_type")]
pub enum StunAttribute {
    #[packetrs(id = "0x0001")]
    MappedAddress {
        reserved: u8,
        address_family: u8,
        port: u16,
        #[packetrs(ctx = "address_family")]
        address: Address,
    },
    #[packetrs(id = "0x0020")]
    XorMappedAddress {
        reserved: u8,
        address_family: u8,
        x_port: u16,
        #[packetrs(ctx = "address_family")]
        x_address: Address,
    },
    #[packetrs(id = "0x0006", count = "length")]
    Username(Vec<u8>),
    #[packetrs(id = "0x0008", count = "length")]
    MessageIntegrity(Vec<u8>),
}
```
##### Generic Field attributes
These attributes can be applied to fields of a struct or enum variant
###### Count
The `count` attribute can be used on collection fields (Vec<T>) to describe how many of the inner type (`T`) should be read into the collection.  The count can be an expression, and can refer to any field or method that will be in scope.
```rust
#[derive(PacketrsRead)]
struct MyStruct {
    pub length: u8
    #[packetrs(count = "length")]
    pub values: Vec<u8>
}
```
```rust
fn get_length(length_val: u8) -> u8 { ... }
#[derive(PacketrsRead)]
struct MyOtherStruct {
    pub length: u8
    #[packetrs(count = "get_length(length)")]
    pub values: Vec<u8>
}
```

###### Fixed
The `fixed` attribute allows defining a value which a read field _must_ have.  After reading the field, if the read value doesn't match the value defined in the `fixed` attribute, then an error is returned.

```rust
#[derive(PacketrsRead)]
struct Foo {
    #[packetrs(fixed = "0b0000")]
    reserved: u4,
    other_field: u8
}
```

###### Assert
The `assert` attribute allows defining an assertion that a read field _must_ pass.  The expression must support taking a single argument of the type of the field's value and return a boolean.  After reading the field, the value will be passed to the assert expression; if the expression returns false an error is returned.

```rust
#[derive(PacketrsRead)]
struct Foo {
    #[packetrs(assert = "|v| v > 2 && v < 4")
    version: u8,
}
```

###### When
The `when` attribute must be used on an `Option` field, and provides an expression to denote whether or not the optional field is present/should be read.  The expression must return a bool.

```rust
#[derive(PacketrsRead)]
struct Foo {
    #[packetrs(when = "buf.bytes_remaining() >= 4")]
    optional_field: Option<u8>,
}
```

###### Reader
The `reader` attribute allows using a custom-defined reader method instead of auto-generating one.  The method must return a `PacketRsResult<T>` where `T` matches the type of the annotated field.

```rust
fn parse_stun_attributes(buf: &mut BitCursor, _ctx: ()) -> PacketRsResult<Vec<StunAttribute>> { ... }

#[derive(Debug, PacketrsRead)]
pub struct StunPacket {
    pub header: StunHeader,
    #[packetrs(reader = "parse_stun_attributes")]
    pub attributes: Vec<StunAttribute>,
}
```
##### Enum/enum variant attributes
These attributes are valid on either enums or enum variants
###### Key & Id
The `key` attribute _must_ be present on an enum, and is an expression that's used in a match statement to differentiate between the variants.  Its counterpart is the `id` attribute, which _must_ be present on enum variants (except for variants with discriminant values, for those the discriminant value will be used as the ID).  It should correspond to a result of the `key` that distinguishes its annotated variant.
```rust
#[derive(Debug, PacketrsRead)]
#[packetrs(required_ctx = "address_family: u8", key = "address_family")]
pub enum Address {
    #[packetrs(id = "0x01")]
    IpV4(u32),
    #[packetrs(id = "0x02")]
    IpV6(u128),
}
```

#### Struct or enum variants with unnamed fields
Unnamed fields can't be annotated, but they're common enough that there's special support to "pass down" annotations from the struct or enum variant itself onto the unnamed fields.  Any annotation on the struct or enum variant will be treated as though it exists on all of the unnamed fields. TODO: example

##### TODO
[ ] More/better documentation
[ ] Better compile-time error messages
[ ] Unit tests - still need to research how best to do those for proc macros
[ ] Implement PacketrsWrite
[ ] More features: read until/while for collection fields. 
[ ] Support params spread across multiple packetrs attributes
