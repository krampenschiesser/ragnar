pub struct AppMultipleOptional {
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
    d: String,
    e: String,
}


struct AppMultipleOptionalComponentBuilder {
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
}
struct AppMultipleOptionalComponentBuilderD {
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
    d: String,
}
struct AppMultipleOptionalComponentBuilderE {
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
    d: String,
    e: String,
}

impl AppMultipleOptionalComponentBuilder {
    pub fn a(self, val: String) ->AppMultipleOptionalComponentBuilder{
        Self {
            a: Some(val),
            ..self
        }
    }
    pub fn b(self, val: String) ->AppMultipleOptionalComponentBuilder{
        Self {
            b: Some(val),
            ..self
        }
    }
    pub fn c(self, val: String) ->AppMultipleOptionalComponentBuilder{
        Self {
            c: Some(val),
            ..self
        }
    }
    pub fn d(self, val: String) ->AppMultipleOptionalComponentBuilderD {
        AppMultipleOptionalComponentBuilderD {
            a: self.a,
            b: self.b,
            c: self.c,
            d: val,
        }
    }
}
impl AppMultipleOptionalComponentBuilderD {

    pub fn e(self, val: String) ->AppMultipleOptionalComponentBuilderE {
        AppMultipleOptionalComponentBuilderE {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: val,
        }
    }
}