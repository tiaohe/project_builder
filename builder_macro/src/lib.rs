extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());

    let fields = if let syn::Data::Struct(s) = input.data {
        s.fields
    } else {
        unimplemented!()
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        pub struct #builder_name {
            #(#field_names: Option<#field_types>,)*
        }

        impl #builder_name {
            #(
                pub fn #field_names(mut self, value: #field_types) -> Self {
                    self.#field_names = Some(value);
                    self
                }
            )*

            pub fn build(self) -> Result<#name, String> {
                Ok(#name {
                    #(
                        #field_names: self
                            .#field_names
                            .ok_or_else(|| format!("Field {} is not set", stringify!(#field_names)))?,
                    )*
                })
            }
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names: None,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
