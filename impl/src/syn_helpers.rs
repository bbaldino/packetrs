use quote::quote;

/// Parse a String that looks like this:
/// "arg_one: type_one, arg_two: type_two, ...
/// into a vector of syn::FnArg
pub(crate) fn parse_fn_args_from_lit_str(
    req_ctx: &syn::LitStr,
) -> Result<Vec<syn::FnArg>, syn::Error> {
    req_ctx
        .value()
        .split(',')
        .map(|s| syn::LitStr::new(s, req_ctx.span()))
        .map(|ls| ls.parse::<syn::FnArg>())
        .collect::<Result<Vec<syn::FnArg>, syn::Error>>()
}

/// Find and return the attribute that matches the given name from the given attribute vector, if
/// one is present.
pub(crate) fn get_attr<'a>(
    attr_name: &str,
    attrs: &'a[syn::Attribute],
) -> Option<&'a syn::Attribute> {
    for attr in attrs {
        if let Ok(node) = attr.parse_meta() {
            match node {
                syn::Meta::List(ref l) => {
                    if let Some(ident) = l.path.get_ident() {
                        if ident == attr_name {
                            return Some(attr);
                        }
                    }
                }
                syn::Meta::Path(ref p) => {
                    if let Some(ident) = p.get_ident() {
                        if ident == attr_name {
                            return Some(attr);
                        }
                    }
                }
                syn::Meta::NameValue(ref nv) => {
                    if let Some(ident) = nv.path.get_ident() {
                        if ident == attr_name {
                            return Some(attr);
                        }
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn get_var_type_from_fn_arg(fn_arg: &syn::FnArg) -> Option<&syn::Type> {
    match fn_arg {
        syn::FnArg::Typed(syn::PatType { ty, .. }) => Some(ty),
        _ => None,
    }
}

fn get_type_ident(ty: &syn::Type) -> Option<&syn::Ident> {
    let type_path = match ty {
        syn::Type::Path(p) => p,
        _ => {
            // todo: return result with this error instead
            eprintln!("Unsupported type: {:?}", ty);
            return None;
        }
    };
    type_path.path.get_ident()
}

/// Given an optional vector of FnArgs parsed from an 'expected_context' attribute, extract the
/// types of each field into a single type which could be either:
///   a tuple containing each of the types in order if there are more than 1 types
///   a plain type, if there is only one (tuple can't be used here)
///   the unit type ('()') if there are no args
/// and return it as a syn::Type.  Will return Err if any of the Fn
pub(crate) fn get_ctx_type(
    expected_context: &Option<&Vec<syn::FnArg>>,
) -> syn::parse::Result<syn::Type> {
    if let Some(ctx) = expected_context {
        let types = ctx
            .iter()
            .filter_map(get_var_type_from_fn_arg)
            .collect::<Vec<&syn::Type>>();
        if types.len() == 1 {
            syn::parse::<syn::Type>(
                quote! {
                    #(#types)*
                }
                .into(),
            )
        } else {
            syn::parse::<syn::Type>(
                quote! {
                    (#(#types),*)
                }
                .into(),
            )
        }
    } else {
        syn::parse_str::<syn::Type>("()")
    }
}

pub(crate) fn get_ident_of_inner_type(ty: &syn::Type) -> Option<&syn::Ident> {
    if let syn::Type::Path(ref tp) = ty {
        if tp.path.segments.len() != 1 {
            panic!("Type path has more than one segment: {:#?}", tp);
        }
        let path_segment = &tp.path.segments[0];
        match path_segment.arguments {
            syn::PathArguments::None => return Some(&path_segment.ident),
            syn::PathArguments::AngleBracketed(ref inner_ty) => {
                if inner_ty.args.len() != 1 {
                    panic!("Generic type args has length != 1: {:#?}", inner_ty);
                }
                if let syn::GenericArgument::Type(ref ty) = inner_ty.args[0] {
                    return get_type_ident(ty);
                } else {
                    panic!("Generic argument wasn't a type: {:#?}", inner_ty.args[0]);
                }
            }
            syn::PathArguments::Parenthesized(_) => {
                panic!(
                    "Parenthesized segment arguments not supported: {:#?}",
                    path_segment.arguments
                );
            }
        }
    }
    None
}

/// Return true if the given type is considered to be a "collection".
pub(crate) fn is_collection(ty: &syn::Type) -> bool {
    if let syn::Type::Path(ref tp) = ty {
        // We can't use path.get_ident here, because it doesn't work on a path whose first value
        // has arguments.
        if !tp.path.segments.is_empty() {
            return tp.path.segments[0].ident == "Vec";
        }
    };
    false
}

pub(crate) fn is_option(ty: &syn::Type) -> bool {
    if let syn::Type::Path(ref tp) = ty {
        // We can't use path.get_ident here, because it doesn't work on a path whose first value
        // has arguments.
        if !tp.path.segments.is_empty() {
            return tp.path.segments[0].ident == "Option";
        }
    };
    false
}
