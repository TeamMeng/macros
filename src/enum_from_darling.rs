use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident, Type};

#[derive(FromDeriveInput, Debug)]
struct EnumFromDarling {
    ident: Ident,
    generics: Generics,
    data: Data<EnumVariants, ()>,
}

#[derive(FromVariant, Debug)]
struct EnumVariants {
    ident: Ident,
    fields: Fields<EnumVariantFields>,
}

#[derive(FromField, Debug)]
struct EnumVariantFields {
    ty: Type,
}

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("EnumFromDarling only works on enums");
    };
    println!("ident: {:?}, generics: {:?}", ident, generics);
    println!("{:#?}", data);

    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(v: #ty) -> Self {
                            #ident::#var(v)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
}
