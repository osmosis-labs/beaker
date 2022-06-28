use core::panic;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::{quote, ToTokens};
use syn::{Attribute, DataEnum, DataStruct, Field, Variant};

#[proc_macro_derive(GetDataDocs)]
pub fn derive_get_data_doc(tokens: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    TokenStream::from(impl_get_data_docs(&ast))
}

macro_rules! summarize_get_data_docs {
    (# $data_type:ident, # $docs:ident) => {
        quote! {
            impl data_doc::GetDataDocs for #$data_type {
                fn get_data_docs() -> Vec<data_doc::DataDoc> {
                    vec![ #(#$docs),* ]
                }
            }
        }
    };
}

fn impl_get_data_docs(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let data_type = &ast.ident;
    match &ast.data {
        syn::Data::Struct(DataStruct { fields, .. }) => {
            let docs = fields
                .iter()
                .map(field_to_data_doc)
                .collect::<Vec<proc_macro2::TokenStream>>();

            summarize_get_data_docs!(#data_type, #docs)
        }
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let docs = variants
                .iter()
                .map(|variant| variant_to_data_doc(data_type, variant))
                .collect::<Vec<proc_macro2::TokenStream>>();

            summarize_get_data_docs!(#data_type, #docs)
        }
        syn::Data::Union(_) => panic!("union type is currently unsupported"),
    }
}

fn parse_docs(attrs: &[Attribute]) -> Vec<proc_macro2::TokenStream> {
    attrs
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

fn field_to_data_doc(field: &Field) -> proc_macro2::TokenStream {
    let ident = field.ident.as_ref().expect("Field has no ident");
    let doc_strings = parse_docs(&field.attrs);

    let ty = &field.ty;

    quote! {
        data_doc::DataDoc::new(
            stringify!(#ident).to_string(),
            stringify!(#ty).to_string(),
            vec![  #( #doc_strings[1..].to_string()),* ],
            <#ty>::get_data_docs()
        )
    }
}

fn variant_to_data_doc(data_type: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
    let ident = &variant.ident;
    let doc_strings = parse_docs(&variant.attrs);

    let docs = variant
        .fields
        .iter()
        .map(field_to_data_doc)
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        data_doc::DataDoc::new(
            stringify!(#ident).to_string(),
            format!("{}::{}", stringify!(#data_type), stringify!(#ident)),
            vec![  #( #doc_strings[1..].to_string()),* ],
            vec![
                #(#docs),*
            ]
        )
    }
}
