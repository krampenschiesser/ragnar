pub struct AppFirstRequired {
    a: String,
    b: Option<String>,
}

#[derive(Default)]
struct AppFirstRequiredComponentBuilder {}

struct AppFirstRequiredComponentBuilderStepA {
    a: String,
}

struct AppFirstRequiredComponentBuilderStepB {
    a: String,
    b: Option<String>,
}

impl AppFirstRequiredComponentBuilder {
    pub fn a(self, val: String) -> AppFirstRequiredComponentBuilderStepA {
        AppFirstRequiredComponentBuilderStepA {
            a: val
        }
    }
}

impl AppFirstRequiredComponentBuilderStepA {
    pub fn b(self, val: String) -> AppFirstRequiredComponentBuilderStepB {
        AppFirstRequiredComponentBuilderStepB {
            a: self.a,
            b: Some(val),
        }
    }
    pub fn into_component(self) {}
}

impl AppFirstRequiredComponentBuilderStepB {
    pub fn into_component(self) {}
}

fn construct_it_without_option() {
    let missing_a = AppFirstRequiredComponentBuilder::default();
    let complete = missing_a.a(String::new());
    complete.into_component();
}

fn construct_it_with_option() {
    let missing_a = AppFirstRequiredComponentBuilder::default();
    let complete = missing_a.a(String::new());
    let complete = complete.b(String::new());
    complete.into_component();
}

fn construct_without_required() {
    let missing_a = AppFirstRequiredComponentBuilder::default();
    // let missing_a = missing_a.b(String::new());
    // missing_a.into_component();
}