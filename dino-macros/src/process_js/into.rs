use super::parse_struct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn process_into_js(input: DeriveInput) -> TokenStream {
    let (ident, generics, fields) = parse_struct(input);
    let ident_s = fields.iter().map(|field| {
        let name = field.ident.as_ref().expect("Field must have a name");

        quote! {
            obj.set(stringify!(#name), self.#name)?;
        }
    });

    let mut merged = generics.clone();
    merged.params.push(syn::parse_quote!('js));

    quote! {
        impl #merged rquickjs::IntoJs<'js> for #ident {
            fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
                let obj = rquickjs::Object::new(ctx.clone())?;

                #(#ident_s)*

                Ok(obj.into())
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
    fn process_into_js_should_work() {
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

        // let code = process_into_js(parsed);
        // println!("code: {}", code.to_string());
    }
}
