use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use crate::{
    get_param,
    model_types::{
        are_fields_named, PacketRsAttributeParam, PacketRsEnum, PacketRsEnumVariant, PacketRsField,
        PacketRsStruct,
    },
    syn_helpers::{
        get_ctx_type, get_ident_of_inner_type, get_var_name_from_fn_arg, is_collection, is_option,
    },
};

pub(crate) fn get_crate_name() -> syn::Ident {
    let found_crate =
        proc_macro_crate::crate_name("packetrs").expect("packetrs is present in Cargo.toml");

    let crate_name = match found_crate {
        proc_macro_crate::FoundCrate::Itself => "packetrs".to_string(),
        proc_macro_crate::FoundCrate::Name(name) => name,
    };

    syn::Ident::new(&crate_name, Span::call_site())
}

/// Based on whether the 'inner' type of the given field (i.e. the type that will actually be read
/// from the buffer) is 'built-in' or not (from BitCursor's perspective), generate and return the
/// call to read the value from a buffer.
fn generate_read_call(field: &PacketRsField, read_context: &Vec<syn::Expr>) -> TokenStream {
    let inner_type = get_ident_of_inner_type(field.ty)
        .unwrap_or_else(|| panic!("Unable to get ident of inner type from: {:#?}", &field.ty));

    match get_param!(&field.parameters, ByteOrder).map_or("network_order".to_owned(), |f| f.value()).as_str() {
        "big_endian" | "network_order" => {
            quote! {
                #inner_type::read::<NetworkOrder>(buf, (#(#read_context,)*))
            }
        },
        "little_endian" => {
            quote! {
                #inner_type::read::<LittleEndian>(buf, (#(#read_context,)*))
            }
        },
        p @ _ => panic!("Invalid byte order param: {}", p),
    }
}

fn generate_field_read(field: &PacketRsField) -> TokenStream {
    let crate_name = get_crate_name();
    let field_name = &field.name;
    let field_ty = &field.ty;
    let error_context = field_name
        .as_ref()
        .unwrap_or_else(|| panic!("Unable to get name of field for error_context {:#?}", field))
        .to_string();

    // Generate the context assignments, if there are any.
    let read_context = if let Some(read_context) = get_param!(&field.parameters, CallerContext) {
        let delimiter = get_param!(&field.parameters, CtxDelim)
            .map(|ls| ls.value())
            .unwrap_or(",".to_owned());
        read_context
            .value()
            .split::<&str>(delimiter.as_ref())
            .map(syn::parse_str::<syn::Expr>)
            .collect::<Result<Vec<syn::Expr>, syn::Error>>()
            .unwrap_or_else(|e| {
                panic!(
                    "Error parsing 'ctx' value as Vec of expressions using delimiter {}: {}, {:?}",
                    delimiter, e, read_context
                )
            })
    } else {
        Vec::new()
    };

    if let Some(ref read_value) = get_param!(&field.parameters, ReadValue) {
        return quote! {
            let #field_name = #read_value;
        };
    }

    let read_call = if let Some(ref custom_reader_value) =
        get_param!(&field.parameters, CustomReader)
    {
        quote! {
            #custom_reader_value(buf, (#(#read_context,)*))
        }
    } else {
        let field_read_call = generate_read_call(field, &read_context);
        if is_collection(field_ty) {
            // Must have a 'count' or 'while' param
            if let Some(ref count_param_value) = get_param!(&field.parameters, Count) {
                quote! {
                    (0u32..#count_param_value.into())
                        .map(|_| #field_read_call)
                        .map(|r| r.map_err(|e| e.into()))
                        .collect::<::#crate_name::error::PacketRsResult<#field_ty>>()
                }
            } else if let Some(ref while_param_value) = get_param!(&field.parameters, While) {
                // Note that, unlike the 'count' branch where we can collect directly into
                // Result<Vec<inner_ty>>, here (because of the while loop) we need to create
                // a Vec<Result<inner_ty>> and then convert it.
                // TODO: Is there a way to do that directly with a 'while'-style condition?
                let inner_type = get_ident_of_inner_type(field_ty).unwrap_or_else(|| {
                    panic!(
                        "Unable to get inner type of collection type: {:?}",
                        field_ty
                    )
                });
                quote! {
                    (|| {
                        let mut values = Vec::<::#crate_name::error::PacketRsResult<#inner_type>>::new();
                        while #while_param_value {
                            values.push(#field_read_call.map_err(|e| e.into()));
                        }
                        values.into_iter().collect::<::#crate_name::error::PacketRsResult<#field_ty>>()
                    })()
                }
            } else {
                panic!(
                    "Field {:?} is a collection: either a 'custom_reader', 'count', or 'while' param is required",
                    field_name
                );
            }
        } else if is_option(field_ty) {
            // Must have a 'when' param
            if let Some(ref when_param_value) = get_param!(&field.parameters, When) {
                quote! {
                    if #when_param_value {
                        Ok(Some(#field_read_call?))
                    } else {
                        Ok(None)
                    }
                }
            } else {
                panic!("Field {:?} is an Option, either a 'custom_reader' or a 'when' param is required", field_name);
            }
        } else {
            quote! {
                #field_read_call
            }
        }
    };

    // If there is a fixed value param, generate the assertion
    let fixed_value_assertion = if let Some(fixed_value) = get_param!(&field.parameters, Fixed) {
        let field_name_str = field_name.as_ref().unwrap().to_string();
        let fixed_value = syn::parse_str::<syn::Expr>(fixed_value.value().as_ref()).unwrap();
        quote! {
            if #field_name != #fixed_value {
                bail!("{} value didn't match: expected {}, got {}", #field_name_str, #fixed_value, #field_name);
            }
        }
    } else {
        TokenStream::new()
    };
    // If there is an assert expression, generate the assertion
    let assertion = if let Some(assertion) = get_param!(&field.parameters, Assert) {
        let field_name_str = field_name.as_ref().unwrap().to_string();
        let assertion_str = quote! { #assertion }.to_string();
        quote! {
            let assert_func = #assertion;
            if !assert_func(#field_name) {
                bail!("value of field '{}' ({}) didn't pass assertion: {}", #field_name_str, #field_name, #assertion_str);
            }
        }
    } else {
        TokenStream::new()
    };

    quote! {
        let #field_name = #read_call.context(#error_context)?;
        #fixed_value_assertion
        #assertion
    }
}

/// Return a proc_macro2::TokenStream that includes local assignments for the read value of each of
/// the given fields.
fn generate_field_reads(fields: &[PacketRsField]) -> TokenStream {
    let field_reads = fields
        .iter()
        .map(generate_field_read)
        .collect::<Vec<TokenStream>>();

    quote! {
        #(#field_reads)*
    }
}

/// Given a Vec of FnArgs, generate the context variable assignments, e.g.:
/// let foo = ctx.0;
/// let bar = ctx.1;
/// NOTE: I tried to return a Vec<syn::Local> here by doing:
///  syn::parse::<syn::Local>(
///      quote! {
///          let #fn_arg = ctx.#idx;
///      }
///      .into(),
///  )
/// But for some reason parse isn't implemented for syn::Local, so for now just returning a
/// TokenStream instead
fn generate_context_assignments(context: &[syn::FnArg]) -> TokenStream {
    let lines = context
        .iter()
        .enumerate()
        .map(|(idx, fn_arg)| {
            let idx: syn::Index = idx.into();
            quote! {
                let #fn_arg = ctx.#idx;
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        #(#lines)*
    }
}

/// Generate the PacketrsRead method for the given struct.
pub(crate) fn generate_struct(packetrs_struct: &PacketRsStruct) -> TokenStream {
    let crate_name = get_crate_name();
    let expected_context = get_param!(&packetrs_struct.parameters, RequiredContext);
    let ctx_type = get_ctx_type(&expected_context).expect("Error getting ctx type");
    let struct_name = &packetrs_struct.name;

    let context_assignments = if let Some(required_ctx) = expected_context {
        generate_context_assignments(required_ctx)
    } else {
        proc_macro2::TokenStream::new()
    };

    let read_body = if let Some(ref custom_reader_value) =
        get_param!(&packetrs_struct.parameters, CustomReader)
    {
        // TODO: move to helper, re-use in generate_enum
        // When using a custom reader, we'll pass all the required context variables
        // to the custom reader
        let ctx_args = if let Some(ctx) = expected_context {
            ctx.iter()
                .map(get_var_name_from_fn_arg)
                .collect::<Option<Vec<&syn::Ident>>>()
                .map_or(quote! { () }, |idents| {
                    quote! {
                        (#(#idents,)*)
                    }
                })
        } else {
            quote! { () }
        };

        let error_context = format!("{}", custom_reader_value);
        quote! {
            #custom_reader_value(buf, #ctx_args).context(#error_context)
        }
    } else {
        // If the struct has named fields, then take them directly. If not, then generate synthetic
        // field names for each of the unnamed fields, and copy the attributes from the struct itself
        // to make it more convenient.
        // TODO: way to avoid the clone here?
        let fields = if are_fields_named(&packetrs_struct.fields) {
            packetrs_struct.fields.clone()
        } else {
            packetrs_struct
                .fields
                .iter()
                .enumerate()
                .map(|(idx, f)| PacketRsField {
                    name: Some(format_ident!("field_{}", idx)),
                    ty: f.ty,
                    parameters: packetrs_struct.parameters.clone(),
                })
                .collect()
        };
        let reads = generate_field_reads(&fields);
        let field_names = fields
            .iter()
            .map(|f| f.name.as_ref().expect("Unable to get name of named field"));
        let creation = if are_fields_named(&packetrs_struct.fields) {
            quote! {
                Ok(Self { #(#field_names),* })
            }
        } else {
            quote! {
                Ok(Self(#(#field_names),*))
            }
        };
        quote! {
            #reads
            #creation
        }
    };

    quote! {
        impl ::#crate_name::packetrs_read::PacketrsRead<#ctx_type> for #struct_name {
            fn read<T: ::#crate_name::b3::byte_order::ByteOrder>(buf: &mut ::#crate_name::b3::bit_cursor::BitCursor, ctx: #ctx_type) -> ::#crate_name::error::PacketRsResult<Self> {
                #context_assignments
                #read_body
            }
        }
    }
}

fn generate_match_arm(enum_name: &syn::Ident, variant: &PacketRsEnumVariant) -> TokenStream {
    let variant_name = variant.name;
    let variant_name_str = variant_name.to_string();
    let key = get_param!(&variant.parameters, EnumId)
        .unwrap_or_else(|| panic!("Enum variant {} is missing 'id' attribute", variant_name));

    let fields = if are_fields_named(&variant.fields) {
        variant.fields.clone()
    } else {
        variant
            .fields
            .iter()
            .enumerate()
            .map(|(idx, f)| PacketRsField {
                name: Some(format_ident!("field_{}", idx)),
                ty: f.ty,
                parameters: variant.parameters.clone(),
            })
            .collect()
    };

    let reads = generate_field_reads(&fields);
    let field_names = fields.iter().map(|f| {
        f.name
            .as_ref()
            .unwrap_or_else(|| panic!("Found unnamed fields amongst named fields: {:#?}", f))
    });
    if variant.fields.is_empty() {
        quote! {
            #key => {
                (|| {
                    Ok(#enum_name::#variant_name)
                })().context(#variant_name_str)
            }
        }
    } else if are_fields_named(&variant.fields) {
        quote! {
            #key => {
                (|| {
                    #reads
                    Ok(#enum_name::#variant_name { #(#field_names),* })
                })().context(#variant_name_str)
            }
        }
    } else {
        quote! {
            #key => {
                (|| {
                    #reads
                    Ok(#enum_name::#variant_name(#(#field_names),*))
                })().context(#variant_name_str)
            }
        }
    }
}

pub(crate) fn generate_enum(packetrs_enum: &PacketRsEnum) -> TokenStream {
    let crate_name = get_crate_name();
    let expected_context = get_param!(&packetrs_enum.parameters, RequiredContext);
    let context_assignments = if let Some(required_ctx) = expected_context {
        generate_context_assignments(required_ctx)
    } else {
        TokenStream::new()
    };
    let ctx_type = get_ctx_type(&expected_context).expect("Error getting ctx type");
    let enum_name = &packetrs_enum.name;

    // If there is a custom reader, then the function body will just be a passthrough call to
    // that custom reader function.  Otherwise it will be a match expression.
    let body = if let Some(ref custom_reader_value) =
        get_param!(&packetrs_enum.parameters, CustomReader)
    {
        // When using a custom reader, we'll pass all the required context variables
        // to the custom reader
        let ctx_args = if let Some(ctx) = expected_context {
            ctx.iter()
                .map(get_var_name_from_fn_arg)
                .collect::<Option<Vec<&syn::Ident>>>()
                .map_or(quote! { () }, |idents| {
                    quote! {
                        (#(#idents,)*)
                    }
                })
        } else {
            quote! { () }
        };

        let error_context = format!("{}", custom_reader_value);
        quote! {
            #custom_reader_value(buf, #ctx_args).context(#error_context)
        }
    } else {
        let enum_variant_key = get_param!(&packetrs_enum.parameters, EnumKey)
            .unwrap_or_else(|| panic!("Enum {} is missing 'key' attribute", enum_name));
        
        let match_arms = packetrs_enum
            .variants
            .iter()
            .map(|v| generate_match_arm(enum_name, v))
            .collect::<Vec<proc_macro2::TokenStream>>();

        quote! {
            match #enum_variant_key {
                #(#match_arms),*,
                v @ _ => {
                    todo!("Value of {:?} is not implemented", v);
                }
            }
        }
    };

    quote! {
        impl ::#crate_name::packetrs_read::PacketrsRead<#ctx_type> for #enum_name {
            fn read<T: ::#crate_name::b3::byte_order::ByteOrder>(buf: &mut ::#crate_name::b3::bit_cursor::BitCursor, ctx: #ctx_type) -> ::#crate_name::error::PacketRsResult<Self> {
                #context_assignments
                #body
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_context_assignments() {
        let fn_arg = syn::parse_str::<syn::FnArg>("foo: u32").unwrap();
        let result = generate_context_assignments(&vec![fn_arg]);
        assert_eq!(
            result.to_string(),
            quote! {
                let foo: u32 = ctx.0;
            }
            .to_string()
        );
    }

    #[test]
    fn test_generate_context_assignments_multiple() {
        let fn_arg = syn::parse_str::<syn::FnArg>("foo: u32").unwrap();
        let fn_arg2 = syn::parse_str::<syn::FnArg>("bar: u8").unwrap();
        let result = generate_context_assignments(&vec![fn_arg, fn_arg2]);
        assert_eq!(
            result.to_string(),
            quote! {
                let foo: u32 = ctx.0;
                let bar: u8 = ctx.1;
            }
            .to_string()
        );
    }
}
