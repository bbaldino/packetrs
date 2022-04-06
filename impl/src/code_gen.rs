use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use crate::{
    model_types::{
        are_fields_named, GetParameterValue, PacketRsEnum, PacketRsEnumVariant, PacketRsField,
        PacketRsStruct,
    },
    syn_helpers::{get_ctx_type, get_ident_of_inner_type},
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

/// Given a field, generate the code that will be used to read this field from a BitCursor.
/// It will generate the call to read the value from the buffer, taking into account any ctx
/// variables that need to be passed.
/// field_type              resulting tokenstream
/// ---------------------------------------------
/// u2                      buf.read_u2()
/// Vec<u2>                 buf.read_u2()
/// MyStruct                MyStruct::read(buf, ())
/// #[packetrs(ctx = "length")]
/// MyOtherStruct           MyOtherStruct::read(buf, length)
/// #[packetrs(ctx = "length", reader = "read_my_other_struct")]
/// MyOtherStruct           read_my_other_struct(&mut buf, length)
fn generate_read_call(field: &PacketRsField) -> proc_macro2::TokenStream {
    let read_context = if let Some(caller_context) = field.get_caller_context_param_value() {
        // TODO: we have to do the clone here so we can return an empty vec in the else case,
        // otherwise we can't return a reference to a temporary vector.  is there a better way?
        caller_context.clone()
    } else {
        Vec::new()
    };
    // TODO: find some cleaner way to test if a type is a bitcursor 'built in' type
    let bitcursor_read_built_in_types: Vec<&str> = vec![
        "bool", "u2", "u3", "u4", "u5", "u6", "u7", "u8", "u14", "u16", "u24", "u32", "u128",
    ];
    // If we have a custom reader, use that
    if let Some(ref custom_reader) = field.get_custom_reader() {
        return quote! {
            #custom_reader(buf, (#(#read_context),*))
        };
    }
    let inner_type = get_ident_of_inner_type(&field.ty)
        .expect(format!("Unable to get ident of inner type from: {:#?}", &field.ty).as_ref());
    let built_in_type = bitcursor_read_built_in_types.contains(&inner_type.to_string().as_ref());
    if built_in_type {
        let read_field = format_ident!("read_{}", inner_type);
        quote! {
            buf.#read_field()
        }
    } else {
        quote! {
            #inner_type::read(buf, (#(#read_context),*))
        }
    }
}

/// Given a PacketRsField, return a TokenStream that will take care of reading the value from the
/// buffer into a variable correctly.  Takes into account fields with a 'count' parameter that need
/// to be read into a Vec.
fn generate_field_read(field: &PacketRsField) -> TokenStream {
    let read_call = generate_read_call(field);
    let field_name = &field.name;
    let field_ty = &field.ty;
    let crate_name = get_crate_name();
    let context = field_name
        .as_ref()
        .expect(format!("Unable to get name of field for read {:#?}", field).as_ref())
        .to_string();
    if let Some(ref count_param) = field.get_count_param_value() {
        // When we have a count param, we get a Vec<Result<T>> that we need to collect as
        // Result<Vec<T>>.
        // TODO: anyhow or something would clean this up a bit
        quote! {
            let #field_name = (0..#count_param)
                .map(|_| #read_call.context(#context))
                .map(|r| r.map_err(|e| e.into()))
                .collect::<::#crate_name::error::PacketRsResult<#field_ty>>()?;
        }
    } else {
        quote! {
            let #field_name = #read_call.context(#context)?;
        }
    }
}

/// Return a proc_macro2::TokenStream that includes local assignments for the read value of each of
/// the given fields.
fn generate_field_reads(fields: &Vec<PacketRsField>) -> TokenStream {
    let field_reads = fields
        .iter()
        .map(|f| {
            let read = generate_field_read(&f);
            if let Some(fixed_value) = f.get_fixed_value() {
                let field_name = &f.name;
                let field_name_str = &f.name.as_ref().unwrap().to_string();
                let fixed_value = syn::parse_str::<syn::Expr>(fixed_value.value().as_ref()).unwrap();
                quote! {
                    #read
                    if #field_name != #fixed_value {
                        bail!("{} value didn't match: expected {}, got {}", #field_name_str, #fixed_value, #field_name);
                    }
                }
            } else {
                quote! {
                    #read
                }
            }
        })
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
fn generate_context_assignments(context: &Vec<syn::FnArg>) -> TokenStream {
    // If there's only a single context argument, then it won't be stored in a type so we'll assign
    // it directly
    if context.len() == 1 {
        let fn_arg = &context[0];
        quote! {
            let #fn_arg = ctx;
        }
    } else {
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
}

/// Generate the body of the packetrs::PacketRsRead::read method for a struct with named fields.
fn generate_struct_read_body_named_fields(rs_struct: &PacketRsStruct) -> proc_macro2::TokenStream {
    let context_assignments =
        if let Some(required_ctx) = rs_struct.get_required_context_param_value() {
            generate_context_assignments(&required_ctx)
        } else {
            proc_macro2::TokenStream::new()
        };

    let reads = generate_field_reads(&rs_struct.fields);
    let field_names = rs_struct
        .fields
        .iter()
        .map(|f| f.name.as_ref().expect("Unable to get name of named field"));
    quote! {
        #context_assignments
        #reads
        Ok(Self { #(#field_names),* })
    }
}

fn generate_struct_read_body_unnamed_fields(
    rs_struct: &PacketRsStruct,
) -> proc_macro2::TokenStream {
    let reads = rs_struct
        .fields
        .iter()
        // Here, we copy the parameters from the parent struct and 'pass them down' to the
        // unnamed field, since it's convenient to be able to annotate an unnamed field this
        // way rather than having to use a named field just to pass parameters
        .map(|f| PacketRsField {
            name: f.name,
            ty: f.ty,
            parameters: rs_struct.parameters.clone(),
        })
        .map(|f| generate_read_call(&f))
        .enumerate()
        // Since the unnamed fields version used generate_read_call directly, which doesn't add the
        // trailing '?', we have to add it here
        .map(|(i, f)| {
            let context = format!("Reading unnamed field {} of struct {}", i, &rs_struct.name);
            quote! {
                #f.context(#context)?
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        Ok(Self(#(#reads),*))
    }
}

pub(crate) fn generate_struct(packetrs_struct: &PacketRsStruct) -> TokenStream {
    let crate_name = get_crate_name();
    let expected_context = packetrs_struct.get_required_context_param_value();
    let ctx_type = get_ctx_type(&expected_context).expect("Error getting ctx type");
    let struct_name = &packetrs_struct.name;
    let read_body = if are_fields_named(&packetrs_struct.fields) {
        generate_struct_read_body_named_fields(packetrs_struct)
    } else {
        generate_struct_read_body_unnamed_fields(packetrs_struct)
    };
    quote! {
        impl ::#crate_name::packetrs_read::PacketRsRead<#ctx_type> for #struct_name {
            fn read(buf: &mut ::#crate_name::bitcursor::BitCursor, ctx: #ctx_type) -> ::#crate_name::error::PacketRsResult<Self> {
                #read_body
            }
        }
    }.into()
}

fn generate_match_arm(enum_name: &syn::Ident, variant: &PacketRsEnumVariant) -> TokenStream {
    let variant_name = variant.name;
    let key = variant
        .get_enum_id()
        .expect(format!("Enum variant {} is missing 'id' attribute", variant_name).as_ref())
        .value();
    // TODO: this won't cover everything (like a guard on a match arm), but it's probably
    // good enough?  See https://docs.rs/syn/latest/syn/struct.Arm.html
    let key = syn::parse_str::<syn::Pat>(&key).expect("Unable to parse match pattern");

    if are_fields_named(&variant.fields) {
        let reads = generate_field_reads(&variant.fields);
        let field_names = variant.fields.iter().map(|f| {
            f.name
                .as_ref()
                .expect(format!("Found unnamed fields amongst named fields: {:#?}", f).as_ref())
        });

        quote! {
            #key => {
                #reads

                Ok(#enum_name::#variant_name { #(#field_names),* })
            }
        }
    } else {
        let reads = variant
            .fields
            .iter()
            // Here, we copy the parameters from the parent variant and 'pass them down' to the
            // unnamed field, since it's convenient to be able to annotate an unnamed field this
            // way rather than having to use a named field just to pass parameters
            .map(|f| PacketRsField {
                name: f.name,
                ty: f.ty,
                parameters: variant.parameters.clone(),
            })
            .map(|f| generate_read_call(&f))
            .enumerate()
            // Since the unnamed fields version used generate_read_call directly, which doesn't add the
            // trailing '?', we have to add it here
            .map(|(i, f)| {
                let context = format!(
                    "Reading unnamed field {} of variant {} in enum {}",
                    i, &variant_name, &enum_name
                );
                quote! {
                    #f.context(#context)?
                }
            })
            .collect::<Vec<proc_macro2::TokenStream>>();

        quote! {
            #key => Ok(#enum_name::#variant_name(#(#reads),*))
        }
    }
}

pub(crate) fn generate_enum(packetrs_enum: &PacketRsEnum) -> TokenStream {
    let crate_name = get_crate_name();
    let expected_context = packetrs_enum.get_required_context_param_value();
    let context_assignments = if let Some(required_ctx) = expected_context {
        generate_context_assignments(&required_ctx)
    } else {
        TokenStream::new()
    };
    let ctx_type = get_ctx_type(&expected_context).expect("Error getting ctx type");
    let enum_name = &packetrs_enum.name;
    let enum_variant_key = packetrs_enum
        .get_enum_key()
        .expect(format!("Enum {} is missing 'key' attribute", enum_name).as_ref())
        .value();

    // TODO: without this, we get quotes around the variant key in the match statement below.  is
    // there a better way?
    let enum_variant_key = syn::parse_str::<syn::Expr>(&enum_variant_key).expect(
        format!(
            "Unable to parse enum key as an expression: {}",
            enum_variant_key
        )
        .as_ref(),
    );

    let match_arms = packetrs_enum
        .variants
        .iter()
        .map(|v| generate_match_arm(&enum_name, &v))
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        impl ::#crate_name::packetrs_read::PacketRsRead<#ctx_type> for #enum_name {
            fn read(buf: &mut ::#crate_name::bitcursor::BitCursor, ctx: #ctx_type) -> ::#crate_name::error::PacketRsResult<Self> {
                #context_assignments
                match #enum_variant_key {
                    #(#match_arms),*,
                    v @ _ => {
                        todo!("Value of {} is not implemented", v);
                    }
                }
            }
        }
    }.into()
}
