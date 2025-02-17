mod from;
mod into;

pub use from::process_from_js;
pub use into::process_into_js;

use darling::{
    ast::{Data, Style},
    FromDeriveInput, FromField,
};
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
pub(crate) struct StructData {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), StructFields>,
}

#[derive(Debug, FromField)]
pub(crate) struct StructFields {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

fn parse_struct(input: DeriveInput) -> (syn::Ident, syn::Generics, Vec<StructFields>) {
    let StructData {
        ident,
        generics,
        data: Data::Struct(fields),
    } = StructData::from_derive_input(&input).expect("Failed to parse input")
    else {
        panic!("Only struct is supported");
    };

    let fields: Vec<crate::process_js::StructFields> = match fields.style {
        Style::Struct => fields.fields,
        _ => panic!("Only struct is supported"),
    };

    (ident, generics, fields)
}
