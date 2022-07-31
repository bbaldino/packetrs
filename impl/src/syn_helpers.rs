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
    attrs: &'a [syn::Attribute],
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

pub(crate) fn get_var_name_from_fn_arg(fn_arg: &syn::FnArg) -> Option<&syn::Ident> {
    match fn_arg {
        syn::FnArg::Typed(syn::PatType { pat, .. }) => {
            if let syn::Pat::Ident(ref pat_ident) = **pat {
                Some(&pat_ident.ident)
            } else {
                None
            }
        }
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
/// types of each field into a single tuple type.
pub(crate) fn get_ctx_type(
    expected_context: &Option<&Vec<syn::FnArg>>,
) -> syn::parse::Result<syn::Type> {
    expected_context.map_or(syn::parse2::<syn::Type>(quote! { () }), |fn_args| {
        let type_vec = fn_args
            .iter()
            .map(get_var_type_from_fn_arg)
            .collect::<Option<Vec<&syn::Type>>>()
            // TODO: instead of unwrap, should map None to an syn::parse::Error and return it
            .unwrap();
        syn::parse2::<syn::Type>(quote! {
            (#(#type_vec,)*)
        })
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ctx_type_single_type() {
        let fn_arg = syn::parse_str::<syn::FnArg>("foo: u32").unwrap();
        let result = get_ctx_type(&Some(&vec![fn_arg]));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            syn::parse_str::<syn::Type>("(u32,)").unwrap()
        );
    }

    #[test]
    fn test_get_ctx_type_multiple_types() {
        let fn_arg = syn::parse_str::<syn::FnArg>("foo: u32").unwrap();
        let fn_arg2 = syn::parse_str::<syn::FnArg>("bar: u8").unwrap();
        let result = get_ctx_type(&Some(&vec![fn_arg, fn_arg2]));

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            syn::parse_str::<syn::Type>("(u32,u8,)").unwrap()
        );
    }

    #[test]
    fn test_get_ctx_type_empty_types() {
        let result = get_ctx_type(&Some(&vec![]));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), syn::parse_str::<syn::Type>("()").unwrap());
    }
}
