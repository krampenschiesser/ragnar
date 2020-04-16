mod modulea;
#[cfg(feature = "web")]
mod moduleweb;

#[cfg(feature = "web")]
#[derive(Component)]
pub struct ComponentWeb {

}

impl NativeComponent for ComponentWeb {

}

#[derive(Component)]
pub struct Component {
    my_rop: MyStruct,
    #[default(Vec::with_capacity(0))]
    props: Vec<Bla>,
    #[required]
    required: String,
    #[rename("blubb")]
    rename: String,
    #[delegated]
    sub: Sub,

    #[cfg(feature = "android")]
    attribute_android: String,
}

impl AppComponent for ComponentWeb {

}

#[derive(Component)]
pub struct Sub {
    sub_prop: String,
}