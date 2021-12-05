extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn attribute(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item: TokenStream = item.into();

    let r = quote! {
        #[repr(C)]
        #[derive(Clone, Copy, limelight::Attribute, limelight::bytemuck::Pod, limelight::bytemuck::Zeroable)]
        #item
    };

    r.into()
}

#[proc_macro_derive(Attribute)]
pub fn vertex_attribute_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_vertex_attribute_derive(input.into()).into()
}

fn bind(field: &syn::Field) -> TokenStream {
    let name = field.ident.as_ref().unwrap().to_string();
    let kind = &field.ty;

    quote! {
        limelight::AttributeBinding {
            variable_name: (#name).to_string(),
            kind: <#kind as limelight::types::AsSizedDataType>::as_sized_data_type(),
        }
    }
}

fn impl_vertex_attribute_derive(input: TokenStream) -> TokenStream {
    let ast: ItemStruct = syn::parse2(input).expect("Should decorate a struct.");

    let name = &ast.ident;

    let bindings: Vec<TokenStream> = match &ast.fields {
        syn::Fields::Named(fields) => fields.named.iter().map(bind).collect(),
        _ => panic!("Only structs with named fields can derive StateMachine currently."),
    };

    quote! {
        impl limelight::Attribute for #name {
            fn describe() -> Vec<limelight::AttributeBinding> {
                vec![
                    #(#bindings),*
                ]
            }
        }
    }
}
