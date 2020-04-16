use proc_macro2::Span;

#[derive(Debug)]
pub struct RsxParseError {
    pub span: Span,
    pub msg: String,
}

impl RsxParseError {
    pub fn new(span: Span, msg: impl Into<String>) -> Self {
        Self {
            span,
            msg: msg.into(),
        }
    }
}