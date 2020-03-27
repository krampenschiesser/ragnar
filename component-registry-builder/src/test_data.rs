mod modulea;
#[cfg(feature = "web")]
mod moduleweb;

#[cfg(feature = "web")]
#[derive(Component)]
pub struct ComponentWeb {

}

#[derive(Component)]
pub struct Component {
    my_rop: MyStruct,
    #[default(Vec::with_capacity(0))]
    props: Vec<Bla>,
    #[required]
    required: String,

    #[cfg(feature = "android")]
    attribute_android: String,
}
