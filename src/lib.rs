use proc_macro::TokenStream;
use quote::quote;

// empty file
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}", input);
    // get ident
    let ident = input.ident;
    // get generics
    let generics = input.generics;

    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
    .into()
}
