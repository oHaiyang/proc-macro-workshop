use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Iter, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_name = Ident::new(&(name.to_string() + "Builder"), name.span());
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote!(#name: ::core::option::Option<#ty>)
            }),
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let none_fields = proc_named_values(input, |f: syn::Field| {
        let name = &f.ident;
        quote!(#name: ::core::option::Option::None)
    });

    // TODO: should not repeat this
    let methods = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! {
                    pub fn #name(&mut self, value: #ty) -> &mut Self {
                        self.#name = ::core::option::Option::Some(value);
                        self
                    }
                }
            }),
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        pub struct #builder_name {
            #( #fields ),*
        }
        impl #builder_name {
            #( #methods )*
        }
        // The generated impl.
        impl #impl_generics #name #ty_generics #where_clause {
            fn builder() -> #builder_name {
                #builder_name {
                    #( #none_fields ),*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn proc_named_values<F>(
    input: DeriveInput,
    f: F,
) -> core::iter::Map<Iter<'_, syn::Field>, impl Fn(&syn::Field) -> TokenStream>
where
    F: Fn(&syn::Field) -> TokenStream,
{
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let aha = fields.named.iter();
                fields.named.iter().map(f)
            }
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
