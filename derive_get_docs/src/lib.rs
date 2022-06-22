use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{DataStruct, Field};

// Generate a compile error to output struct name
#[proc_macro_derive(GetDocs)]
pub fn derive_get_docs(tokens: TokenStream) -> TokenStream {
    // convert the input tokens into an ast, specially from a derive
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

                    quote! { (stringify!(#ident).to_string(), vec![ #(#doc_strings.to_string()),* ]) }
                })
                .collect::<Vec<proc_macro2::TokenStream>>();

            let q = quote! {
                impl get_docs::GetDocs for #name {
                    fn get_docs() -> Vec<(String, Vec<String>)> {
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

fn parse_docs(field: &Field) -> Vec<String> {
    field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .map(|attr| {
            let mut docs = String::new();
            for tt in attr.tokens.clone().into_iter() {
                if let TokenTree::Literal(lit) = tt {
                    docs.push_str(
                        lit.to_string()
                            .replace("\" ", "")
                            .replace('"', "")
                            .replace("\\'", "'")
                            .as_str(),
                    );
                }
            }
            docs
        })
        .collect::<Vec<String>>()
}
