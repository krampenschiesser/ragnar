use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;

pub struct Node {
    attributes: HashMap<String, String>,
    name: Cow<'static, str>,
    children: Vec<Box<Node>>,
}

pub struct NativeCallback {
    function: Box<dyn Fn(dyn Any) -> String>,
    id: u64,
}

#[derive(Clone)]
pub struct ClickEvent {
    button: u8
}

pub struct NativeButton<Out> {
    on_click: Box<dyn Fn(&ClickEvent) -> Out>,
    text: Cow<'static, str>,
}

pub struct FancyButton<Out> {
    on_click: Box<dyn Fn() -> Out>,

}

pub struct App {}

pub struct AppState {
    count: u8
}

pub enum AppMsg {
    Increment,
    Decrement,
}

pub trait Component {
    type State;
    type Msg;

    fn render(self, state: &Self::State) -> Node;
}

impl<Out> Component for NativeButton<Out> {
    type State = ();
    type Msg = ();

    fn render(self, state: &Self::State) -> Node {
        let mut map = HashMap::new();
        map.insert("name".into(), self.text.into_owned());
        let x = self.on_click;
        let cow = "Button".into();
        let callbacks = vec![x];
        Node {
            children: Vec::new(),
            attributes: map,
            // callbacks,
            name: cow,
        }
    }
}

impl Component for FancyButton<AppMsg> {
    type State = AppState;
    type Msg = AppMsg;

    fn render(self, state: &Self::State) -> Node {
        let button = NativeButton {
            text: Cow::Owned(format!("Clicked {}", state.count)),
            on_click: Box::new(|e: &ClickEvent| {
                (self.on_click)()
            }),
        };
        let node = button.render(&());
        Node {
            name: "fancyButton".into(),
            attributes: HashMap::new(),
            children: vec![Box::new(node)],
        }
    }
}

impl Component for App {
    type State = AppState;
    type Msg = AppMsg;

    fn render(self, state: &Self::State) -> Node {
        let fancy_button = FancyButton {
            on_click: Box::new(|| {
                AppMsg::Increment
            })
        };
        let node = fancy_button.render(state);
        Node {
            name: "app".into(),
            attributes: HashMap::new(),
            children: vec![Box::new(node)],
        }
    }
}

pub trait NativeEvent: Any  +Clone {}

pub struct NativeEventWrapper<T: NativeEvent> {
    id: u64,
    event: T,
}

struct Renderer<State> {
    state: State,
    root_node: Node,
}

impl<State> Renderer<State> {
    fn render_all(&mut self, root: impl Component<State=State>) {
        let node = root.render(&self.state);
        self.root_node = node;
    }

    fn on_event<T: NativeEvent>(&mut self, event: NativeEventWrapper<T>) {
        let (node, callback) = Self::find_node_for_event(&self.root_node, event.id);
        (callback.function)(event);
    }

    fn find_node_for_event(root: &Node, id: u64) -> (&Node, &Callback) {
        unimplemented!()
    }
}