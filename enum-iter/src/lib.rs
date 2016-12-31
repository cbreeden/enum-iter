#![feature(proc_macro, proc_macro_lib)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{ Ident, Body, Variant, VariantData };
use proc_macro::TokenStream;

#[proc_macro_derive(EnumIterator)]
pub fn enum_iterator(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();

    let name = &ast.ident;
    let gen = match ast.body {
        Body::Enum(ref variants) => impl_enum_iter(name, variants),
        Body::Struct(_) =>
            quote! {
                impl EnumIteratorOnlyWorksForEnumsNotStructsSorryNotSorry for #name { }
            },
    };
    gen.parse().unwrap()
}

fn impl_enum_iter(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    let interface = quote::Ident::from(format!("_EnumIterator{}", name));
    let match_usize = match_usize(&name, variants);
    let size = variants.len();

    quote! {
        #[derive(Debug, Default)]
        pub struct #interface {
            count: usize,
        }

        impl #name {
            fn enum_iter() -> #interface {
                #interface::default()
            }
        }

        impl #interface {
            fn from_usize(n: usize) -> #name {
                match n {
                    #(#match_usize)*
                    _ => unreachable!(), // I think
                }
            }
        }

        impl ::std::iter::Iterator for #interface {
            type Item = #name;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count >= #size { return None }
                let result = #interface::from_usize(self.count);
                self.count += 1;
                Some(result)
            }
        }
    }
}

fn match_usize(name: &Ident, variants: &[Variant]) -> Vec<quote::Tokens> {
    let mut result = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        let id = &variant.ident;
        let new = match variant.data {
            VariantData::Unit => quote! {
                    #idx => #name::#id,
                },

            VariantData::Tuple(ref fields) => {
                let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
                quote! {
                    #idx => #name::#id( #(#types::default(),)* ),
                }
            },

            VariantData::Struct(ref fields) => {
                let items: Vec<_> = fields.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;

                    quote! {
                        #ident: #ty::default()
                    }
                }).collect();

                quote! {
                    #idx => #name::#id { #(#items,)*  },
                }
            }
        };
        result.push(new);
    }

    result
}