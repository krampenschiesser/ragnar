use proc_macro2::{TokenTree, Span, Ident, Literal, Group};
use crate::error::RsxParseError;


#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub span: Span,
    pub attributes: Vec<ElementAttribute>,
    pub children: Vec<ElementOrText>,
}

#[derive(Debug)]
pub enum ElementOrText {
    Element(Element),
    CodeChildren((Span,TokenTree)),
    Text((Span, String)),
}

#[derive(Debug)]
pub struct ElementAttribute {
    pub span: Span,
    pub name: String,
    pub value: TokenTree,
}

pub struct ElementParser {}

pub struct AttributeParser {}

enum AttributeParseResult {
    StartChildren(Span, Vec<ElementAttribute>),
    EndElement(Span, Vec<ElementAttribute>),
}

impl AttributeParser {
    fn parse_equal_punct(&self, ident: &Ident, iter: &mut impl Iterator<Item=TokenTree>) -> Result<(), RsxParseError> {
        let span = if let Some(o) = iter.next() {
            match o {
                TokenTree::Punct(p) => {
                    if p.as_char() != '=' {
                        Some(p.span())
                    } else {
                        None
                    }
                }
                _ => {
                    Some(ident.span())
                }
            }
        } else {
            Some(ident.span())
        };

        if let Some(span) = span {
            Err(RsxParseError::new(span, "Expected '=' to mark attribute value"))
        } else {
            Ok(())
        }
    }

    fn parse_attribute(ident: Ident, iter: &mut impl Iterator<Item=TokenTree>) -> Result<ElementAttribute, RsxParseError> {
        let parser = AttributeParser {};
        parser.parse_equal_punct(&ident, iter)?;
        if let Some(value) = iter.next() {
            Ok(ElementAttribute { span: ident.span(), name: ident.to_string(), value })
        } else {
            Err(RsxParseError::new(ident.span(), "Expected attribute value"))
        }
    }
}

impl ElementParser {
    fn parse_element(&self, span: Span, parent: Option<&str>, iter: &mut impl Iterator<Item=TokenTree>) -> Result<Option<ElementOrText>, RsxParseError> {
        let token = iter.next();
        if token.is_none() {
            return Ok(None);
        }

        let is_new_element = expect_punct(span, '<', &token, "Expect element start '<'");
        let is_text_element = expect_literal(span, &token, "Expected text start");
        let is_group = expect_group(span,&token,"Expected group");

        let begin_span = if is_new_element.is_err() && is_text_element.is_err() &&is_group.is_err() {
            let msg = format!("Expected begin of element '<', text \"\", or code block {}, but got {:?}", "{}", token);
            return Err(RsxParseError::new(token.map(|t|t.span()).unwrap_or(span), msg));
        } else if let Ok(text) = is_text_element {
            return Ok(Some(ElementOrText::Text((text.span().clone(), text.to_string()))));
        } else if let Ok(group) = is_group {
            return Ok(Some(ElementOrText::CodeChildren((group.span().clone(), token.unwrap()))));
        } else {
            is_new_element.unwrap()
        };

        let token = iter.next();
        let (span, name) = if let Some(parent_name) = parent {
            let res = expect_punct(begin_span, '/', &token, "");
            if res.is_ok() {
                let token = iter.next();
                let (span, _ident) = expect_identifier(span, Some(parent_name), &token, "expected closing tag with parent element name")?;
                let token = iter.next();
                expect_punct(span, '>', &token, "Expected closing tag '>'")?;
                return Ok(None);
            } else {
                expect_identifier(span, None, &token, "expected element name")?
            }
        } else {
            expect_identifier(begin_span, None, &token, "expected element name")?
        };

        let result = self.parse_attributes(span, iter)?;
        match result {
            AttributeParseResult::EndElement(_, attributes) => {
                let element = Element { attributes, span: begin_span, name, children: Vec::with_capacity(0) };
                Ok(Some(ElementOrText::Element(element)))
            }
            AttributeParseResult::StartChildren(span, attributes) => {
                let elements = self.parse_elements(span, Some(name.as_str()), iter)?;
                let element = Element { attributes, span: begin_span, name, children: elements };
                Ok(Some(ElementOrText::Element(element)))
            }
        }
    }

    pub fn parse_elements(&self, span: Span, parent: Option<&str>, iter: &mut impl Iterator<Item=TokenTree>) -> Result<Vec<ElementOrText>, RsxParseError> {
        let mut elements = Vec::new();
        loop {
            let element = self.parse_element(span, parent, iter)?;
            if let Some(element) = element {
                elements.push(element);
            } else {
                break;
            }
        }
        Ok(elements)
    }

    fn parse_attributes(&self, span: Span, iter: &mut impl Iterator<Item=TokenTree>) -> Result<AttributeParseResult, RsxParseError> {
        let mut attributes = Vec::new();
        loop {
            let token = iter.next();
            if let Ok(span) = expect_punct(span.clone(), '>', &token, "Expected closing tag '>") {
                return Ok(AttributeParseResult::StartChildren(span, attributes));
            } else if let Ok(span) = expect_punct(span.clone(), '/', &token, "Expected closing tag '/") {
                let token = iter.next();
                expect_punct(span.clone(), '>', &token, "Expected closing tag '/>")?;
                return Ok(AttributeParseResult::EndElement(span, attributes));
            } else if let Some(token) = token {
                match token {
                    TokenTree::Ident(i) => {
                        let attribute = AttributeParser::parse_attribute(i, iter)?;
                        attributes.push(attribute);
                    }
                    o => {
                        Err(RsxParseError::new(o.span(), "Expected attribute or closing tag"))?
                    }
                }
            } else {
                Err(RsxParseError::new(span, "Unexpected end, expected attribute or closing tag"))?
            }
        }
    }
}

fn expect_identifier(span: Span, expected_content: Option<&str>, token: &Option<TokenTree>, msg: &'static str) -> Result<(Span, String), RsxParseError> {
    if let Some(token) = token {
        match token {
            TokenTree::Ident(i) => {
                let string = i.to_string();
                if let Some(expected) = expected_content {
                    if string == expected {
                        Ok((i.span(), string))
                    } else {
                        Err(RsxParseError::new(i.span(), msg))?
                    }
                } else {
                    Ok((i.span(), string))
                }
            }
            o => {
                Err(RsxParseError::new(o.span(), msg))?
            }
        }
    } else {
        Err(RsxParseError::new(span, msg))?
    }
}

fn expect_punct(span: Span, expected_char: char, token: &Option<TokenTree>, msg: &'static str) -> Result<Span, RsxParseError> {
    if let Some(token) = token {
        match token {
            TokenTree::Punct(p) => {
                if p.as_char() == expected_char {
                    Ok(p.span())
                } else {
                    Err(RsxParseError::new(p.span(), msg))
                }
            }
            o => {
                Err(RsxParseError::new(o.span(), msg))
            }
        }
    } else {
        Err(RsxParseError::new(span, msg))
    }
}

fn expect_group<'a>(span: Span, token: &'a Option<TokenTree>, msg: &'static str) -> Result<&'a Group, RsxParseError> {
    if let Some(token) = token {
        match token {
            TokenTree::Group(l) => {
                Ok(l)
            }
            o => {
                Err(RsxParseError::new(o.span(), msg))
            }
        }
    } else {
        Err(RsxParseError::new(span, msg))
    }
}
fn expect_literal<'a>(span: Span, token: &'a Option<TokenTree>, msg: &'static str) -> Result<&'a Literal, RsxParseError> {
    if let Some(token) = token {
        match token {
            TokenTree::Literal(l) => {
                Ok(l)
            }
            o => {
                Err(RsxParseError::new(o.span(), msg))
            }
        }
    } else {
        Err(RsxParseError::new(span, msg))
    }
}