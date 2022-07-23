/// Model the different attributes
#[derive(Debug, Clone)]
pub(crate) enum PacketRsAttributeParam {
    // The 'count' attr, which is associated with a field that contains how many of the annotated
    // field should be parsed.
    Count(syn::Expr),
    // A String containing the list of context arguments that will be passed to the read method of
    // the annotated field.  Value will be split by a comma by default.  See CtxDelim to override
    // the delimiter.
    CallerContext(syn::LitStr),
    // A vector containing the list of function arguments in the form of syn::FnArg that is
    // required to be passed to the read method of the annotated struct.
    RequiredContext(Vec<syn::FnArg>),
    // A value containing the key to which enum variants should be mapped.  Tagged on the enum.
    EnumKey(syn::LitStr),
    // An ID of a specific enum variant that will be retrieved via the EnumKey.  Tagged on an enum
    // variant.
    EnumId(syn::LitStr),
    // A value that a given field must equal. (use Expr?)
    Fixed(syn::LitStr),
    // An expression that the field's value must pass
    Assert(syn::Expr),
    // An expression that defines when an optional field is present
    When(syn::Expr),
    // An expression that should be used to assign to the field instead of reading it from the
    // buffer.
    ReadValue(syn::Expr),
    // The name of a custom reader function to be used to read this type
    CustomReader(syn::Ident),
    // Sometimes values including a comma need to be passed in a CallerContext argument.  If so,
    // the default delimiter can be overriden via this parameter.
    CtxDelim(syn::LitStr),
}

#[derive(Debug, Clone)]
pub(crate) struct PacketRsField<'a> {
    pub name: Option<syn::Ident>,
    pub ty: &'a syn::Type,
    pub parameters: Vec<PacketRsAttributeParam>,
}

#[derive(Debug)]
pub(crate) struct PacketRsStruct<'a> {
    pub name: &'a syn::Ident,
    pub fields: Vec<PacketRsField<'a>>,
    pub parameters: Vec<PacketRsAttributeParam>,
}

#[derive(Debug)]
pub(crate) struct PacketRsEnumVariant<'a> {
    pub name: &'a syn::Ident,
    pub parameters: Vec<PacketRsAttributeParam>,
    pub fields: Vec<PacketRsField<'a>>,
}

#[derive(Debug)]
pub(crate) struct PacketRsEnum<'a> {
    pub name: &'a syn::Ident,
    pub parameters: Vec<PacketRsAttributeParam>,
    pub variants: Vec<PacketRsEnumVariant<'a>>,
}

pub(crate) trait HasParameters {
    fn get_parameters(&self) -> &Vec<PacketRsAttributeParam>;
}

impl HasParameters for PacketRsField<'_> {
    fn get_parameters(&self) -> &Vec<PacketRsAttributeParam> {
        &self.parameters
    }
}

impl HasParameters for PacketRsStruct<'_> {
    fn get_parameters(&self) -> &Vec<PacketRsAttributeParam> {
        &self.parameters
    }
}

impl HasParameters for PacketRsEnumVariant<'_> {
    fn get_parameters(&self) -> &Vec<PacketRsAttributeParam> {
        &self.parameters
    }
}

/// Find the first element in $params that matches the given variant.  Will return
/// an Option of the type inside the variant.  Only works with a variant with a single
/// unnamed field.
#[macro_export]
macro_rules! get_param {
    ($params:expr, $variant:tt) => {
        $params.iter().find_map(|p| match p {
            PacketRsAttributeParam::$variant(ref v) => Some(v),
            _ => None,
        })
    };
}

/// Find all elements in $params that matches the given variant.  Will return
/// a vector of the found inner values.  Only works with a variant with a single
/// unnamed field.
macro_rules! get_params {
    ($params:ident, $variant:tt) => {
        $params
            .iter()
            .filter_map(|p| match p {
                PacketRsAttributeParam::$variant(ref v) => Some(v),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
}

pub(crate) fn are_fields_named(fields: &[PacketRsField<'_>]) -> bool {
    fields.iter().any(|f| f.name.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let params = vec![
            PacketRsAttributeParam::EnumKey(syn::parse_str::<syn::LitStr>("\"hello\"").unwrap()),
            PacketRsAttributeParam::Fixed(syn::parse_str::<syn::LitStr>("\"world\"").unwrap()),
            PacketRsAttributeParam::EnumKey(syn::parse_str::<syn::LitStr>("\"foo\"").unwrap()),
        ];
        let result = get_params!(params, EnumKey);
        println!("found param: {:?}", result);
    }
}
