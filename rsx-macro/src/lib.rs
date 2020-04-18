#![feature(proc_macro_diagnostic)]
#[macro_use]
extern crate lazy_static;

mod parser;
mod error;
mod instantiate;

use proc_macro2::{TokenStream, Span};

use ragnar_component_registry_builder::{ComponentCache, ComponentFeatureView, RenderType};
use crate::parser::{ElementParser, Element, ElementOrText};
use syn::spanned::Spanned;
use failure::Fail;
use crate::instantiate::Instantiate;
use quote::{quote_spanned, format_ident};

lazy_static! {
    static ref CACHE: ComponentCache = {
        let cache = match ComponentCache::new() {
            Ok(cache) => {
                cache
            }
            Err(e) => {
                eprintln!("{}", e);
                panic!("{}: {:?}", e, e.backtrace())
            }
        };
        cache
    };
}

enum OutputRenderType {
    Native,
    Local,
    App,
    Nodes,
    Node
}
#[proc_macro]
pub fn local(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse(input.into(), OutputRenderType::Local).into()
}

#[proc_macro]
pub fn native(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse(input.into(), OutputRenderType::Native).into()
}

#[proc_macro]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse(input.into(), OutputRenderType::App).into()
}
#[proc_macro]
pub fn nodes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse(input.into(), OutputRenderType::Nodes).into()
}

#[proc_macro]
pub fn node(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse(input.into(), OutputRenderType::Node).into()
}

fn parse(input: TokenStream, render_type: OutputRenderType) -> TokenStream {
    let parser = ElementParser {};
    let span = input.span();
    let mut iter = input.into_iter();
    let elements = match parser.parse_elements(span, None, &mut iter) {
        Ok(elements) => elements,
        Err(e) => {
            e.span.unwrap().error(e.msg).emit();
            // panic!("{}", e.msg);
            vec![]
        }
    };

    if !check_elements(&elements, &CACHE, None) {
        TokenStream::new()
    } else {
        render_elements(elements, &CACHE, render_type, span)
    }
}

fn render_elements(elements: Vec<ElementOrText>, cache: &ComponentCache, render_type: OutputRenderType, span: Span) -> TokenStream {
    let mut counter = 0usize;
    let mut instantiated_code = Vec::new();
    let mut instantiated_names = Vec::new();
    for element in elements {
        if let ElementOrText::Element(element) = element {
            let (instantiate, c) = Instantiate::new(element, counter, cache);
            counter = c;
            instantiated_code.push(instantiate.code);
            instantiated_names.push(format_ident!("{}",instantiate.name));
        }
    }

    // panic!("{}", quote! {
    //     #(#vec)*
    // });
    match render_type {
        OutputRenderType::Native => {
            quote_spanned! {span=>
                #(# instantiated_code)*
                vec![#(#instantiated_names),*]
            }
        }
        OutputRenderType::App => {
            quote_spanned! {span=>
                #(# instantiated_code)*
                ragnar_lib::AppNode::<Self::Msg>::empty(ctx).with_children(vec![#(#instantiated_names),*])
            }
        }
        OutputRenderType::Local => {
            quote_spanned! {span=>
                #(# instantiated_code)*
                ragnar_lib::LocalNode::new(self,ctx).with_children(vec![#(#instantiated_names),*])
            }
        }
        OutputRenderType::Nodes => {
            quote_spanned! {span=>
                #(# instantiated_code)*
                vec![#(#instantiated_names),*]
            }
        }
        OutputRenderType::Node => {
            quote_spanned! {span=>
                #(# instantiated_code)*
                #(#instantiated_names)*
            }
        }
    }
}

fn check_elements(elements: &[ElementOrText], cache: &ComponentCache, parent: Option<&Element>) -> bool {
    let mut success = true;
    for element in elements {
        match element {
            ElementOrText::Element(element) => {
                if let Some(found) = cache.get_component(&vec![], &element.name) {
                    let sub_components = found.sub_components.iter().filter_map(|s| cache.get_component(&vec![], s)).collect();
                    if !check_element(element, &found, sub_components) {
                        success = false;
                    }
                } else {
                    element.span.unwrap().error("Could not find any component.").emit();
                    success = false;
                }
                if !check_elements(&element.children, cache, Some(element)) {
                    success = false;
                }
            }
            ElementOrText::Text((span, _text)) => {
                if let Some(_parent) = parent {} else {
                    span.unwrap().error("No parent element for text").emit();
                    success = false;
                }
            }
            ElementOrText::CodeChildren(_) => {
                //all good? hopefully
            }
        }
    }
    success
}

fn check_element(element: &Element, component: &ComponentFeatureView, sub_components: Vec<ComponentFeatureView>) -> bool {
    let mut success = true;
    for required_attribute in &component.required_attributes {
        if !element.attributes.iter().any(|a| a.name == required_attribute.name_view) {
            element.span.unwrap().error(format!("Missing required attribute {}", required_attribute.name_view)).emit();
            success = false;
        }
    }
    let contains = |name: &str| {
        let contains = component.contains_attribute(name);
        let contains = contains || sub_components.iter().any(|c| c.contains_attribute(name));
        contains
    };
    element.attributes.iter().filter(|a| !contains(a.name.as_str())).for_each(|a| {
        a.span.unwrap().error("Attribute does not exist in component").emit();
        success = false;
    });
    success
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
