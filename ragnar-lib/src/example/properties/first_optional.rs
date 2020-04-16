// #[derive(ComponentBuilder)]
pub struct AppFirstOptional {
    a: Option<String>,
    b: String,
}


#[derive(Default, Eq, PartialEq)]
struct AppFirstOptionalComponentBuilder {
    a: Option<String>,
}

struct AppFirstOptionalComponentBuilderStepB {
    a: Option<String>,
    b: String,
}

impl AppFirstOptionalComponentBuilder {
    pub fn a(self, val: String) -> AppFirstOptionalComponentBuilder {
        AppFirstOptionalComponentBuilder {
            a: Some(val)
        }
    }
    pub fn b(self, val: String) -> AppFirstOptionalComponentBuilderStepB {
        AppFirstOptionalComponentBuilderStepB {
            a: None,
            b: val,
        }
    }
}

impl AppFirstOptionalComponentBuilderStepB {
    pub fn into_component(self) -> () {
        ()
    }
}

fn construct_it_without_option() {
    let builder_missing_b = AppFirstOptionalComponentBuilder::default();
    let builder = builder_missing_b.b(String::new());
    builder.into_component();
}

fn construct_it_with_option() {
    let builder_missing_b = AppFirstOptionalComponentBuilder::default();
    let builder_missing_b = builder_missing_b.a(String::new());
    let complete = builder_missing_b.b(String::new());
    complete.into_component();
}

fn construct_without_required() {
    let missing_b = AppFirstOptionalComponentBuilder::default();
    let missing_b = missing_b.a(String::new());
    // missing_b.into_component();
}