use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::{DataStruct, Field};

#[proc_macro_derive(GetDocs)]
pub fn derive_get_docs(tokens: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    TokenStream::from(impl_get_docs_macro(&ast))
}

fn impl_get_docs_macro(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(DataStruct { fields, .. }) => {
            let docs = fields
                .iter()
                .map(|field| {
                    let ident = field.ident.as_ref().expect("Field has to ident");
                    let doc_strings = parse_docs(field);

                    quote! { get_docs::StructDoc::new(stringify!(#ident).to_string(), vec![  #( #doc_strings[1..].to_string()),* ], vec![]) }
                })
                .collect::<Vec<proc_macro2::TokenStream>>();

            let q = quote! {
                impl get_docs::GetDocs for #name {
                    fn get_struct_docs() -> Vec<get_docs::StructDoc> {
                        vec![ #(#docs),* ]
                    }
                }
            };

            // For debugging:
            // panic!("{}", q);
            q
        }
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }
}

fn parse_docs(field: &Field) -> Vec<proc_macro2::TokenStream> {
    field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .map(|attr| {
            let mut docs = proc_macro2::TokenStream::new();
            for tt in attr.tokens.clone().into_iter() {
                if let TokenTree::Literal(lit) = tt {
                    lit.to_tokens(&mut docs);
                }
            }
            docs
        })
        .collect::<Vec<proc_macro2::TokenStream>>()
}
