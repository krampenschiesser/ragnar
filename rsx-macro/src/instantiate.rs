use proc_macro2::TokenStream;
use crate::parser::{Element, ElementOrText};
use ragnar_component_registry_builder::{ComponentCache, RenderType};
use quote::{quote_spanned, format_ident};

pub struct Instantiate {
    pub name: String,
    pub code: TokenStream,
}

impl Instantiate {
    pub fn new(element: Element, mut counter: usize, cache: &ComponentCache) -> (Instantiate, usize) {
        counter += 1;
        let variable_name = format!("{}{:03}", element.name, counter);
        let component = cache.get_component(&vec![], &element.name).expect("Should be there cause of check before");
        let struct_name = component.qualified_name;

        let mut used = Vec::new();
        let attributes: Vec<_> = component.optional_attributes.iter()
            .filter(|a| a.name_code.as_str() != "children")
            .chain(component.required_attributes.iter())
            .filter(|a| a.sub_component.is_none())
            .map(|a| {
                used.push(*a);

                let ident = format_ident!("{}",a.name_code);
                if let Some(att) = element.attributes.iter().find(|att| att.name == a.name_view) {
                    let val = &att.value;
                    let span = att.span;
                    if a.required {
                        quote_spanned! {span=> #ident: #val.into(), }
                    } else {
                        quote_spanned! {span=> #ident: Some(#val.into()), }
                    }
                } else if a.required {
                    TokenStream::new()
                } else {
                    let span = element.span;
                    quote_spanned! {span=> #ident: std::default::Default::default(),}
                }
            }).collect();
        let sub_components: Vec<_> = component.required_attributes.iter().chain(component.optional_attributes.iter())
            .filter(|a| a.sub_component.is_some())
            .map(|a| {
                let sub_component = cache.get_component(&vec![], a.sub_component.as_ref().unwrap()).unwrap();
                let chain_supplier = || {
                    sub_component.optional_attributes.iter().chain(sub_component.required_attributes.iter())
                };
                let cur_used = used.clone();
                let attributes_of_sub: Vec<_> = element.attributes.iter()
                    .filter(|element_att| !cur_used.iter().any(|u| u.name_view == element_att.name))
                    .filter_map(|element_att| {
                        if let Some(component_att) = chain_supplier().find(|o| o.name_view == element_att.name) {
                            used.push(component_att);
                            let ident = format_ident!("{}", component_att.name_code);
                            let val = &element_att.value;
                            let span = element_att.span;
                            Some(quote_spanned! {span=> #ident: Some(#val.into()), })
                        } else {
                            None
                        }
                    })
                    .collect();
                let ident = format_ident!("{}",a.name_code);

                let struct_def: TokenStream = syn::parse_str(sub_component.qualified_name).unwrap();
                let span = element.span;
                quote_spanned! {span=>
                    #ident: #struct_def { #(#attributes_of_sub)* .. Default::default() },
                }
            })
            .collect();
        let span = element.span;
        let mut child_code = Vec::new();
        let mut child_names = Vec::new();
        for child in element.children.into_iter() {
            let (instantiate, c) = Instantiate::instantiate_child(counter, cache, child);
            counter = c;
            child_code.push(instantiate.code);
            child_names.push(format_ident!("{}",instantiate.name));
        }

        let (child_token, child_instantiation) = if child_names.is_empty() {
            if let Some(_component_att) = component.required_attributes.iter().chain(component.optional_attributes.iter()).find(|a| a.name_code == "children") {
                (
                    quote_spanned! {span=>
                        children: vec![],
                    },
                    TokenStream::new()
                )
            } else {
                (TokenStream::new(), TokenStream::new())
            }
        } else {
            let ident = format_ident!("children_{}",variable_name);
            let child_token = quote_spanned! {span=>
                children: #ident,
            };
            let assignments: Vec<_> = child_names.iter().map(|name| {
                quote_spanned! {span=>
                     ragnar_lib::ExtendNodeChildren::extend_children(#name, &mut #ident);
                }
            }).collect();
            let child_instantiation = quote_spanned! {span=>
                let mut #ident = Vec::new();
                #(#assignments)*
            };
            (child_token,child_instantiation)
        };

        let ident = format_ident!("{}",variable_name);
        let render_statement = match component.render_type {
            RenderType::App => {
                quote_spanned! {span=>
                    let #ident = ragnar_lib::Node::from(ragnar_lib::AppComponent::render(&#ident,state, ragnar_lib::AppContext::new()).into());
                }
            }
            RenderType::Local => {
                quote_spanned! {span=>
                    let #ident: ragnar_lib::Node = ragnar_lib::LocalComponent::render(#ident, ragnar_lib::LocalContext::new()).into();
                }
            }
            RenderType::Native => {
                quote_spanned! {span=>
                    let #ident: ragnar_lib::Node = ragnar_lib::NativeComponent::render(#ident, ragnar_lib::NativeContext::new()).into();
                }
            }
            RenderType::None => {
                quote_spanned! {span=>
                    let #ident: ragnar_lib::Node = #ident.into();
                }
            }
        };
        let struct_def: TokenStream = syn::parse_str(struct_name).unwrap();
        let code = quote_spanned! {span=>
            #(#child_code)*
            #child_instantiation
            let #ident = #struct_def {
                #(#attributes)*
                #(#sub_components)*
                #child_token
            };
            #render_statement
        };
        (Instantiate {
            name: variable_name,
            code,
        }, counter)
    }

    fn instantiate_child(mut counter: usize, cache: &ComponentCache, child: ElementOrText) -> (Instantiate, usize) {
        let instantiate = match child {
            ElementOrText::Element(e) => {
                let (inst, c) = Instantiate::new(e, counter, cache);
                counter = c;
                inst
            }
            ElementOrText::Text((span, string)) => {
                counter += 1;
                let variable_name = format!("text{:03}", counter);
                let ident = format_ident!("{}",variable_name);
                let code = quote_spanned! {span=>
                            let #ident: ragnar_lib::Node = ragnar_lib::TextNode::new(#string).into();
                        };
                Instantiate {
                    name: variable_name,
                    code,
                }
            }
            ElementOrText::CodeChildren((span, tokentree)) => {
                counter += 1;
                let variable_name = format!("code_child{:03}", counter);
                let ident = format_ident!("{}",variable_name);

                let code = quote_spanned! {span=>
                    let #ident = #tokentree;
                };
                Instantiate {
                    name: variable_name,
                    code,
                }
            }
        };
        (instantiate, counter)
    }
}