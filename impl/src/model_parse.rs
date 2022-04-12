use crate::{
    model_types::{
        PacketRsAttributeParam, PacketRsEnum, PacketRsEnumVariant, PacketRsField, PacketRsStruct,
    },
    syn_helpers::{get_attr, parse_fn_args_from_lit_str},
};

pub(crate) fn parse_field(field: &syn::Field) -> PacketRsField {
    let parameters = parse_packetrs_attrs_from_attributes(&field.attrs);
    PacketRsField {
        name: field.ident.clone(),
        ty: &field.ty,
        parameters,
    }
}

pub(crate) fn parse_struct<'a, 'b>(
    name: &'a syn::Ident,
    attrs: &'a Vec<syn::Attribute>,
    struct_data: &'a syn::DataStruct,
) -> PacketRsStruct<'b>
where
    'a: 'b,
{
    let parameters = parse_packetrs_attrs_from_attributes(attrs);
    let fields = struct_data
        .fields
        .iter()
        .map(|field| parse_field(&field))
        .collect::<Vec<PacketRsField>>();

    PacketRsStruct {
        name,
        fields,
        parameters,
    }
}

pub(crate) fn parse_variant<'a>(variant: &syn::Variant) -> PacketRsEnumVariant {
    let name = &variant.ident;
    let parameters = parse_packetrs_attrs_from_attributes(&variant.attrs);
    let fields = variant
        .fields
        .iter()
        .map(|field| parse_field(&field))
        .collect::<Vec<PacketRsField>>();

    PacketRsEnumVariant {
        name,
        parameters,
        fields,
    }
}

pub(crate) fn parse_enum<'a, 'b>(
    name: &'a syn::Ident,
    attrs: &'a Vec<syn::Attribute>,
    enum_data: &'a syn::DataEnum,
) -> PacketRsEnum<'b>
where
    'a: 'b,
{
    let parameters = parse_packetrs_attrs_from_attributes(attrs);
    let variants = enum_data
        .variants
        .iter()
        .map(|variant| parse_variant(&variant))
        .collect::<Vec<PacketRsEnumVariant>>();

    PacketRsEnum {
        name,
        parameters,
        variants,
    }
}

fn parse_packetrs_namevalue_param(nv: &syn::MetaNameValue) -> Option<PacketRsAttributeParam> {
    let name = nv
        .path
        .get_ident()
        .expect(format!("Couldn't get ident from MetaNameValue: {:#?}", nv).as_ref());
    let value_str = match &nv.lit {
        syn::Lit::Str(ref lit_str) => lit_str,
        _ => panic!(
            "Unexpected attribute value (wasn't a LitStr): {:#?}",
            &nv.lit
        ),
    };

    // TODO: some use the LitStr, and others use String...can they be made consistent?
    match name.to_string().as_ref() {
        "count" => Some(PacketRsAttributeParam::Count(
            value_str
                .parse::<syn::Expr>()
                .expect("Unable to parse count param as an expression"),
        )),
        "ctx" => {
            // value_str represents a comma-separated list of expression we'll pass as arguments
            // to the read method.  Split it, parse each as an Expr, and collect them to a Vec.
            let exprs = value_str
                .value()
                .split(",")
                .map(|e| syn::parse_str::<syn::Expr>(e))
                .collect::<Result<Vec<syn::Expr>, syn::Error>>()
                .expect("Error parsing a context arg as an expression");
            Some(PacketRsAttributeParam::CallerContext(exprs))
        }
        "required_ctx" => {
            let args = parse_fn_args_from_lit_str(&value_str)
                .expect("Error parsing required context args");
            Some(PacketRsAttributeParam::RequiredContext(args))
        }
        "key" => Some(PacketRsAttributeParam::EnumKey(value_str.clone())),
        "id" => Some(PacketRsAttributeParam::EnumId(value_str.clone())),
        "fixed" => Some(PacketRsAttributeParam::Fixed(value_str.clone())),
        "reader" => {
            let reader_ident = syn::parse_str::<syn::Ident>(value_str.value().as_ref())
                .expect("reader param is a valid ident");
            Some(PacketRsAttributeParam::CustomReader(reader_ident))
        }
        _ => {
            // TODO: refactor this to use a spanned compiler error
            panic!("Unrecognized packetrs attribute param name: {:?}", name)
        }
    }
}

fn parse_packetrs_param(meta: &syn::NestedMeta) -> Option<PacketRsAttributeParam> {
    //eprintln!("parsking packetrs param: {:#?}", meta);
    if let syn::NestedMeta::Meta(ref m) = meta {
        if let syn::Meta::NameValue(ref nv) = m {
            return parse_packetrs_namevalue_param(&nv);
        } else {
            panic!("Packetrs attr param that wasn't a NameValue: {:?}", m);
        }
    } else {
        panic!(
            "Packetrs attr param that wasn't a NestedMeta::Meta: {:?}",
            meta
        );
    }
}

/// Given a syn::Attribute that corresponds to a packetrs attribute, parse all
/// the attribute params into PacketRsAttributeParam
///
/// For now, assume all attrs are NameValue and anything else is invalid
fn parse_packetrs_attrs(attr: &syn::Attribute) -> Vec<PacketRsAttributeParam> {
    if let Some(attr_ident) = attr.path.get_ident() {
        if attr_ident != "packetrs" {
            panic!(
                "Non packetrs attribute passed to parse_packetrs_attr: {:?}",
                attr
            );
        }
    } else {
        panic!("Unable to get attribute ident for {:?}", attr);
    }
    //eprintln!("got attribute: {:#?}", attr);
    let mut attr_params: Vec<PacketRsAttributeParam> = vec![];
    if let Ok(node) = attr.parse_meta() {
        if let syn::Meta::List(packetrs_params) = node {
            for param in packetrs_params.nested {
                if let Some(parsed_param) = parse_packetrs_param(&param) {
                    attr_params.push(parsed_param);
                } else {
                    eprintln!("failed to parse packetrs_param {:#?}", param);
                }
            }
        } else {
            eprintln!("attr wasn't a meta::list");
        }
    }
    attr_params
}

fn parse_packetrs_attrs_from_attributes(
    attrs: &Vec<syn::Attribute>,
) -> Vec<PacketRsAttributeParam> {
    if let Some(packetrs_attr) = get_attr("packetrs", &attrs) {
        return parse_packetrs_attrs(packetrs_attr);
    } else {
        return Vec::new();
    }
}
