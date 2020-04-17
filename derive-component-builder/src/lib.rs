#![feature(proc_macro_diagnostic)]

use proc_macro2::{Ident, TokenStream};
use syn;
use syn::{Data, Field, Fields, FieldsNamed};
use syn::spanned::Spanned;

use quote::{format_ident, quote};

#[proc_macro_derive(Component, attributes(required, default,delegated,rename))]
pub fn component_derive(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

#[proc_macro_derive(ComponentBuilder, attributes(required, default))]
pub fn component_builder_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed: syn::DeriveInput = syn::parse(input).unwrap();
    let ident = &parsed.ident;


    let token_stream = match &parsed.data {
        Data::Struct(my_struct) => {
            match &my_struct.fields {
                Fields::Named(named_fields) => parse_struct(named_fields, ident),
                _ => {
                    parsed.span().unwrap().error("Only named fields are supported").emit();
                    panic!("Only named fields are supported");
                }
            }
        }
        _ => {
            parsed.span().unwrap().error("Only structs are supported").emit();
            panic!("Only structs are supported");
        }
    };
    token_stream.into()
}

fn parse_struct(fields: &FieldsNamed, root_ident: &Ident) -> TokenStream {
    let mut required_fields: Vec<&Field> = Vec::new();
    let mut optional_fields: Vec<&Field> = Vec::with_capacity(fields.named.len());

    for field in &fields.named {
        let is_required = field.attrs.iter().find(|a| a.path.is_ident("required")).is_some();
        if is_required {
            required_fields.push(field);
        } else {
            optional_fields.push(field);
        }
    }
    let struct_default = generate_default_struct(&optional_fields, root_ident);
    let struct_required = generate_required_struct(&optional_fields, &required_fields, root_ident);
    quote! {
        #struct_default

        #struct_required
    }
}

fn generate_default_struct(fields: &Vec<&Field>, root_ident: &Ident) -> TokenStream {
    let new_name = format_ident!("{}ComponentBuilderDefaults",root_ident);
    let fields: Vec<TokenStream> = fields.iter().map(|f| {
        quote! {#f,}
    }).collect();
    quote! {
        #[derive(Default)]
        pub struct #new_name {
            #(#fields)*
        }
    }
}

fn generate_required_struct(optional_fields: &Vec<&Field>, required_fields: &Vec<&Field>, root_ident: &Ident) -> TokenStream {
    let opt_name = format_ident!("{}ComponentBuilderDefaults",root_ident);
    let new_name = format_ident!("{}ComponentBuilder",root_ident);
    let fields: Vec<TokenStream> = required_fields.iter().map(|f| {
        let f = Field {
            attrs: Vec::new(),
            ident: f.ident.clone(),
            ty: f.ty.clone(),
            colon_token: f.colon_token.clone(),
            vis: f.vis.clone(),
        };

        quote! {#f,}
    }).collect();

    let into_req_fields: Vec<TokenStream> = required_fields.iter().map(|f| {
        let field_name = f.ident.clone().expect("Named field should have identifier");
        quote! {
            #field_name: self.#field_name,
        }
    }).collect();
    let into_opt_fields: Vec<TokenStream> = optional_fields.iter().map(|f| {
        let field_name = f.ident.clone().expect("Named field should have identifier");
        quote! {
            #field_name: self.defaults.#field_name,
        }
    }).collect();

    let new_for_default = if required_fields.is_empty() {
        quote! {
        pub fn new() -> Self {
            Self {
                defaults: ::core::default::Default::default(),
            }
        }
    }
    } else {
        quote!()
    };
    let default_for_main_type = if required_fields.is_empty() {
        quote! {
            impl ::core::default::Default for #root_ident {
                fn default() -> Self {
                    #new_name::new().into_component()
                }
            }

    }
    } else {
        quote!()
    };
    quote! {
        pub struct #new_name {
            defaults: #opt_name,
            #(#fields)*
        }

        impl #new_name {
            pub fn into_component(self) -> #root_ident {
                #root_ident {
                    #(#into_req_fields)*
                    #(#into_opt_fields)*

                }
            }

            #new_for_default
        }

        #default_for_main_type
    }
}