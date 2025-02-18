use super::parse_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn process_from_js(input: DeriveInput) -> TokenStream {
    let (ident, generics, fields) = parse_struct(input);

    let code = fields.iter().map(|field| {
        let name = field.ident.as_ref().expect("Field must have a name");
        let ty = &field.ty;

        quote! {
            let #name: #ty = obj.get(stringify!(#name))?;
        }
    });

    let ident_s = fields.iter().map(|field| {
        let name = field.ident.as_ref().expect("Field must have a name");
        quote! {
            #name,
        }
    });

    let mut merged = generics.clone();
    merged.params.push(syn::parse_quote!('js));

    quote! {
        impl #merged rquickjs::FromJs<'js> for #ident {
            fn from_js(ctx: &rquickjs::Ctx<'js>, value: rquickjs::Value<'js>) -> rquickjs::Result<Self> {
            let obj = rquickjs::Object::from_js(ctx, value)?;

                #(#code)*

                Ok(#ident {
                    #(#ident_s)*
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process_js::StructData;
    use darling::FromDeriveInput;

    #[test]
    fn process_from_js_should_work() {
        let input = r#"
        #[derive(Debug)]
        pub struct Response {
            pub headers: HashMap<String, String>,
            pub status: u16,
            pub body: Option<String>,
        }
        "#;

        let parsed: DeriveInput = syn::parse_str(input).unwrap();
        let into = StructData::from_derive_input(&parsed).unwrap();

        assert_eq!(into.ident.to_string(), "Response");

        // let code = process_from_js(parsed);
        // println!("code: {}", code.to_string());
    }
}
