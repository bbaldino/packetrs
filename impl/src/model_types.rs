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

impl HasParameters for PacketRsEnum<'_> {
    fn get_parameters(&self) -> &Vec<PacketRsAttributeParam> {
        &self.parameters
    }
}

pub trait GetParameterValue {
    fn get_count_param_value(&self) -> Option<&syn::Expr>;
    fn get_caller_context_param_value(&self) -> Option<&syn::LitStr>;
    fn get_required_context_param_value(&self) -> Option<&Vec<syn::FnArg>>;
    fn get_enum_id(&self) -> Option<&syn::LitStr>;
    fn get_enum_key(&self) -> Option<&syn::LitStr>;
    fn get_fixed_value(&self) -> Option<&syn::LitStr>;
    fn get_assert(&self) -> Option<&syn::Expr>;
    fn get_when(&self) -> Option<&syn::Expr>;
    fn get_read_value(&self) -> Option<&syn::Expr>;
    fn get_custom_reader(&self) -> Option<&syn::Ident>;
    fn get_ctx_delim(&self) -> Option<&syn::LitStr>;
}

impl<T> GetParameterValue for T
where
    T: HasParameters,
{
    fn get_count_param_value(&self) -> Option<&syn::Expr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::Count(ref s) => Some(s),
            _ => None,
        })
    }

    fn get_caller_context_param_value(&self) -> Option<&syn::LitStr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::CallerContext(ref s) => Some(s),
            _ => None,
        })
    }

    fn get_required_context_param_value(&self) -> Option<&Vec<syn::FnArg>> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::RequiredContext(ref v) => Some(v),
            _ => None,
        })
    }

    fn get_enum_id(&self) -> Option<&syn::LitStr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::EnumId(ref id) => Some(id),
            _ => None,
        })
    }

    fn get_enum_key(&self) -> Option<&syn::LitStr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::EnumKey(ref key) => Some(key),
            _ => None,
        })
    }

    fn get_fixed_value(&self) -> Option<&syn::LitStr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::Fixed(ref val) => Some(val),
            _ => None,
        })
    }

    fn get_assert(&self) -> Option<&syn::Expr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::Assert(ref val) => Some(val),
            _ => None,
        })
    }

    fn get_when(&self) -> Option<&syn::Expr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::When(ref val) => Some(val),
            _ => None,
        })
    }

    fn get_read_value(&self) -> Option<&syn::Expr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::ReadValue(ref val) => Some(val),
            _ => None,
        })
    }

    fn get_custom_reader(&self) -> Option<&syn::Ident> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::CustomReader(ref val) => Some(val),
            _ => None,
        })
    }

    fn get_ctx_delim(&self) -> Option<&syn::LitStr> {
        self.get_parameters().iter().find_map(|p| match p {
            PacketRsAttributeParam::CtxDelim(ref val) => Some(val),
            _ => None,
        })
    }
}

pub(crate) fn are_fields_named(fields: &[PacketRsField<'_>]) -> bool {
    fields.iter().any(|f| f.name.is_some())
}
