use proc_macro::TokenStream;
use quote::{quote,};
use syn::__private::TokenStream2;
use syn::{ItemStruct, };

#[proc_macro_derive(MySerde)]
pub fn serde_derive(input: TokenStream) -> TokenStream {
    general(input.into()).into()
}

fn general(code: TokenStream2) -> TokenStream2 {
    let item: ItemStruct = syn::parse2(code).unwrap();
    let name = item.ident;
    let mut println_codes = Vec::with_capacity(item.fields.len());
    for field in item.fields.iter() {
        if let Some(c) = field.ident.as_ref() {
            let field_name = c.to_string();
            let println_code = quote!(
                println!("{}: {}", #field_name, self.#c);
            );
            dbg!(println_code.to_string());
            println_codes.push(println_code);
        }
    }
    let end = quote!(
        impl #name {
            pub fn serialize(&self) {
                #(#println_codes)*
            }
        }
    );
    dbg!(end.to_string());
    end
}

#[test]
fn test() {
    let code = quote!(
        struct A {
            a: i32,
            b: usize,
        }
    );
    general(code);
}

