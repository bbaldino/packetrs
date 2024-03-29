use quote::quote;

use crate::{
    match_pat_guard::MatchPatGuard,
    model_types::{
        PacketRsAttributeParam, PacketRsEnum, PacketRsEnumVariant, PacketRsField, PacketRsStruct,
    },
    syn_helpers::{get_attr, parse_fn_args_from_lit_str}, get_param,
};

pub(crate) fn parse_field<'a, 'b>(field: &'a syn::Field, parent_params: &'b Vec<PacketRsAttributeParam>) -> PacketRsField<'a> {
    let mut parameters = parse_packetrs_attrs_from_attributes(&field.attrs);
    if let Some(parent_byte_order) = get_param!(&parent_params, ByteOrder) {
        if let None = get_param!(&parameters, ByteOrder) {
            parameters.push(PacketRsAttributeParam::ByteOrder(parent_byte_order.clone()));
        };
    };
    PacketRsField {
        name: field.ident.clone(),
        ty: &field.ty,
        parameters,
    }
}

pub(crate) fn parse_struct<'a, 'b>(
    name: &'a syn::Ident,
    attrs: &'a [syn::Attribute],
    struct_data: &'a syn::DataStruct,
) -> PacketRsStruct<'b>
where
    'a: 'b,
{
    let parameters = parse_packetrs_attrs_from_attributes(attrs);
    let fields = struct_data
        .fields
        .iter()
        .map(|f| {
            parse_field(f, &parameters)
        })
        .collect::<Vec<PacketRsField>>();

    PacketRsStruct {
        name,
        fields,
        parameters,
    }
}

pub(crate) fn parse_variant<'a, 'b>(variant: &'a syn::Variant, parent_params: &'b Vec<PacketRsAttributeParam>) -> PacketRsEnumVariant<'a> {
    let name = &variant.ident;
    let mut parameters = parse_packetrs_attrs_from_attributes(&variant.attrs);
    // Add an inherited 'ByteOrder' param if there is one and the variant hasn't overridden it
    if let Some(parent_byte_order) = get_param!(&parent_params, ByteOrder) {
        if let None = get_param!(&parameters, ByteOrder) {
            parameters.push(PacketRsAttributeParam::ByteOrder(parent_byte_order.clone()));
        };
    };
    let fields = variant
        .fields
        .iter()
        .map(|f| {
            parse_field(f, &parameters)
        })
        .collect::<Vec<PacketRsField>>();

    // If the variant has a discriminant value, use that as the id
    if let Some((_, discriminant)) = &variant.discriminant {
        let pat = syn::parse2::<syn::Pat>(quote! { #discriminant })
            .unwrap_or_else(|e| panic!("Unable to parse discriminant as syn::Pat: {}", e));
        parameters.push(PacketRsAttributeParam::EnumId(MatchPatGuard {
            pat,
            guard: None,
        }));
    }

    PacketRsEnumVariant {
        name,
        parameters,
        fields,
    }
}

pub(crate) fn parse_enum<'a, 'b>(
    name: &'a syn::Ident,
    attrs: &'a [syn::Attribute],
    enum_data: &'a syn::DataEnum,
) -> PacketRsEnum<'b>
where
    'a: 'b,
{
    let parameters = parse_packetrs_attrs_from_attributes(attrs);
    let variants = enum_data
        .variants
        .iter()
        .map(|f| {
            parse_variant(f, &parameters)
        })
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
        .unwrap_or_else(|| panic!("Couldn't get ident from MetaNameValue: {:#?}", nv));
    let value_str = match &nv.lit {
        syn::Lit::Str(ref lit_str) => lit_str,
        _ => panic!(
            "Unexpected attribute value (wasn't a LitStr): {:#?}",
            &nv.lit
        ),
    };

    // TODO: some use the LitStr, and others use String...can they be made consistent?
    match name.to_string().as_ref() {
        "count" => {
            let expr = value_str
                .parse::<syn::Expr>()
                .unwrap_or_else(|e| panic!("Unable to parse 'count' param as expression: {}", e));
            Some(PacketRsAttributeParam::Count(expr))
        }
        "while" => {
            let expr = value_str
                .parse::<syn::Expr>()
                .unwrap_or_else(|e| panic!("Unable to parse 'while' param as expression: {}", e));
            Some(PacketRsAttributeParam::While(expr))
        }
        "ctx" => {
            // We just grab the context value as one string here, because a custom delimiter on
            // which to split may have been passed, so we delay splitting until later when all
            // attributes have been parsed.
            Some(PacketRsAttributeParam::CallerContext(value_str.clone()))
        }
        "required_ctx" => {
            let args = parse_fn_args_from_lit_str(value_str)
                .unwrap_or_else(|e| panic!("Error parsing 'required_ctx' value as fn args: {}", e));
            Some(PacketRsAttributeParam::RequiredContext(args))
        }
        "key" => {
            let expr = syn::parse_str::<syn::Expr>(&value_str.value())
                .unwrap_or_else(|e| panic!("Error parsing 'enum_key' value as expression: {}", e));
            Some(PacketRsAttributeParam::EnumKey(expr))
        }
        "id" => {
            let id = syn::parse_str::<MatchPatGuard>(&value_str.value())
                .unwrap_or_else(|e| panic!("Error parsing 'id' value as MatchPatGuard: {}", e));
            Some(PacketRsAttributeParam::EnumId(id))
        }
        "fixed" => Some(PacketRsAttributeParam::Fixed(value_str.clone())),
        "assert" => {
            let expr = syn::parse_str::<syn::Expr>(&value_str.value())
                .unwrap_or_else(|e| panic!("Error parsing 'assert' value as expression: {}", e));
            Some(PacketRsAttributeParam::Assert(expr))
        }
        "byte_order" => {
            match value_str.value().as_str() {
                "big_endian" | "little_endian" | "network_order" => {
                    Some(PacketRsAttributeParam::ByteOrder(value_str.clone()))
                }
                _ => panic!("Error: 'byte_order' parameter is invalid")
            }
        }
        "when" => {
            let expr = syn::parse_str::<syn::Expr>(&value_str.value())
                .unwrap_or_else(|e| panic!("Error parsing 'when' value as expression: {}", e));
            Some(PacketRsAttributeParam::When(expr))
        }
        "read_value" => {
            let expr = syn::parse_str::<syn::Expr>(&value_str.value()).unwrap_or_else(|e| {
                panic!("Error parsing 'read_value' value as expression: {}", e)
            });
            Some(PacketRsAttributeParam::ReadValue(expr))
        }
        "reader" => {
            let reader_ident = syn::parse_str::<syn::Ident>(value_str.value().as_ref())
                .unwrap_or_else(|e| panic!("Error parsing 'reader' param as a valid Ident: {}", e));
            Some(PacketRsAttributeParam::CustomReader(reader_ident))
        }
        "ctx_delim" => Some(PacketRsAttributeParam::CtxDelim(value_str.clone())),
        _ => {
            // TODO: refactor this to use a spanned compiler error
            panic!("Unrecognized packetrs attribute param name: {:?}", name)
        }
    }
}

fn parse_packetrs_param(meta: &syn::NestedMeta) -> Option<PacketRsAttributeParam> {
    //eprintln!("parsing packetrs param: {:#?}", meta);
    if let syn::NestedMeta::Meta(ref m) = meta {
        if let syn::Meta::NameValue(ref nv) = m {
            parse_packetrs_namevalue_param(nv)
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

fn parse_packetrs_attrs_from_attributes(attrs: &[syn::Attribute]) -> Vec<PacketRsAttributeParam> {
    if let Some(packetrs_attr) = get_attr("packetrs", attrs) {
        parse_packetrs_attrs(packetrs_attr)
    } else {
        Vec::new()
    }
}
