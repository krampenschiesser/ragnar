use std::any::Any;

use crate::app_component::{AppComponent, AppContext, AppEvent, AppState};
use crate::callback::{
    AppCallbackWrapper, CallbackId, LocalCallbackWrapper, NativeCallbackWrapper,
};
use crate::local_component::LocalHandleResult;
use crate::native_component::NativeEvent;
use crate::node::app_node::UntypedAppNode;
use crate::node::NodeId;
use crate::runtime::diff::operations::{DiffOperation, ParentPosition};
use crate::runtime::diff::{CompleteDiff, DiffError};
use crate::runtime::node_container::NodeContainer;

use super::node::Node;
use crate::runtime::observer::timingcategory::RuntimeTimingCategory;
use crate::runtime::observer::{RuntimeObserver, Timer};
use crate::App;
use std::sync::Arc;

pub(crate) mod diff;
mod node_container;
pub mod observer;

pub struct Runtime<
    C: AppComponent<State = State, Msg = Msg> + Clone,
    State: AppState + Clone,
    Msg: AppEvent,
> {
    root_component: C,
    root: NodeContainer,
    state: State,
    observer: Arc<RuntimeObserver>,
    pub update_function: Arc<Box<dyn Fn(&mut State, &Msg) + Send + Sync + 'static>>,
    native_event_resolvers: Arc<
        Vec<
            Box<
                dyn Fn(&str, &str) -> Result<Option<Box<dyn NativeEvent>>, String>
                    + Send
                    + Sync
                    + 'static,
            >,
        >,
    >,
}

impl<C: AppComponent<State = State, Msg = Msg> + Clone, State: AppState + Clone, Msg: AppEvent>
    Clone for Runtime<C, State, Msg>
{
    fn clone(&self) -> Self {
        let root_component = self.root_component.clone();
        let result = root_component.render(&self.state, AppContext::new());
        let node_container = NodeContainer::from_root(result.into());
        Self {
            root_component,
            root: node_container,
            state: self.state.clone(),
            update_function: self.update_function.clone(),
            native_event_resolvers: self.native_event_resolvers.clone(),
            observer: self.observer.clone(),
        }
    }
}

impl<C: AppComponent<State = State, Msg = Msg> + Clone, State: AppState + Clone, Msg: AppEvent>
    Runtime<C, State, Msg>
{
    pub fn new(app: &App<C, State, Msg>, observer: Arc<RuntimeObserver>) -> Self {
        let state = app.initial_state.clone();
        let root = app.root_component.render(&state, AppContext::new());
        Self {
            update_function: app.update_function.clone(),
            state,
            root_component: app.root_component.clone(),
            root: NodeContainer::from_root(root.into()),
            native_event_resolvers: app.native_event_resolvers.clone(),
            observer,
        }
    }

    pub fn initial_diff(&self) -> Vec<DiffOperation> {
        let init = Vec::new();
        let native_view = self.root.native_view(None);
        CompleteDiff::new(&init, &native_view).diff()
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn resolve_native_event(
        &self,
        event_type: &str,
        payload: &str,
    ) -> Result<Box<dyn NativeEvent>, String> {
        for resolver in self.native_event_resolvers.iter() {
            if let Some(event) = (resolver)(event_type, payload)? {
                return Ok(event);
            }
        }
        Err(format!("Could not find a handler for event_type='{}'. {} resolvers registered in App#native_event_resolvers",event_type,self.native_event_resolvers.len()))
    }
    pub fn handle_event(
        &mut self,
        id: CallbackId,
        event: Box<dyn NativeEvent>,
    ) -> Result<Vec<DiffOperation>, DiffError> {
        let mut handling_result = EventHandlingResult::new();
        let callback = self
            .root
            .native_callbacks
            .get(&id)
            .ok_or(DiffError::NewCallbackNotFound(id))?;
        {
            let timer = Timer::new("test");
            self.execute_native_callback(&callback, event, &mut handling_result)?;
            self.observer
                .observe_time(RuntimeTimingCategory::ExecuteNativeCallback, timer)
        }

        let EventHandlingResult {
            state_changes,
            local_node_updates: node_updates,
        } = handling_result;
        if state_changes.is_empty() {
            let mut diff_ops = Vec::new();
            let changed_nodes = self.update_local_nodes(node_updates);
            for (new_node_id, old_container, _parent) in changed_nodes {
                let old_native_view = old_container.native_view(None);
                let new_native_view = self.root.native_view(Some(new_node_id));
                let diff = CompleteDiff::new(&old_native_view, &new_native_view);
                let new_ops = diff.diff();
                diff_ops.extend(new_ops);
            }
            Ok(diff_ops)
        } else {
            let new_node = self.update(state_changes);
            let new_container = NodeContainer::from_root(new_node);
            let result = CompleteDiff::new(
                &self.root.native_view(None),
                &new_container.native_view(None),
            )
            .diff();
            self.root = new_container;
            Ok(result)
        }
    }

    fn update_local_nodes(
        &mut self,
        updates: Vec<(Box<dyn Any>, NodeId)>,
    ) -> Vec<(NodeId, NodeContainer, Option<ParentPosition>)> {
        let mut nodes_that_need_diff = Vec::new();
        for (boxed, id) in updates {
            //TODO remove children if parent gets updated

            let handle_result: LocalHandleResult =
                if let Some(stripped_node) = self.root.local_nodes.get(&id) {
                    stripped_node.component.handle(&boxed)
                } else {
                    LocalHandleResult::Keep
                };

            let node_to_diff = match handle_result {
                LocalHandleResult::Keep => None,
                LocalHandleResult::NewRender(node) => {
                    let new_id = node.id;
                    let option = self.root.replace_local_node(node, id);
                    option.map(|o| (new_id, o.1, o.0))
                }
                LocalHandleResult::NewState(state) => {
                    self.root.swap_node_component(&id, state);
                    None
                }
            };
            if let Some(n) = node_to_diff {
                nodes_that_need_diff.push(n);
            }
        }
        nodes_that_need_diff
    }

    fn update(&mut self, events: Vec<Box<dyn Any>>) -> UntypedAppNode {
        for boxed in events {
            if let Some(event) = boxed.downcast_ref() {
                (self.update_function)(&mut self.state, event);
            }
        }
        self.root_component
            .render(&self.state, AppContext::new())
            .into()
    }

    fn execute_callback(
        &self,
        id: CallbackId,
        event: &Box<dyn Any>,
        handling_result: &mut EventHandlingResult,
    ) -> Result<(), DiffError> {
        if let Some(callback) = self.root.local_callbacks.get(&id) {
            self.execute_local_callback(callback, event, handling_result)
        } else if let Some(callback) = self.root.app_callbacks.get(&id) {
            self.execute_app_callback(callback, event, handling_result)
        } else {
            Err(DiffError::NewCallbackNotFound(id))
        }
    }
    fn execute_native_callback(
        &self,
        callback: &NativeCallbackWrapper,
        event: Box<dyn NativeEvent>,
        handling_result: &mut EventHandlingResult,
    ) -> Result<(), DiffError> {
        if let Some(output) = (callback.callback)(event) {
            if callback.chained.is_empty() {
                warn!("Native callback is not chained, its output will be lost. Callback={:?}, node={:?}", callback.id, callback.node_id);
            } else {
                for other in &callback.chained {
                    self.execute_callback(*other, &output, handling_result)?;
                }
            }
        }
        Ok(())
    }
    fn execute_local_callback(
        &self,
        callback: &LocalCallbackWrapper,
        event: &Box<dyn Any>,
        handling_result: &mut EventHandlingResult,
    ) -> Result<(), DiffError> {
        if let Some(output) = (callback.callback)(event) {
            if callback.chained.is_empty() {
                handling_result.add_local(output, callback.node_id);
            } else {
                for other in &callback.chained {
                    self.execute_callback(*other, &output, handling_result)?;
                }
            }
        }
        Ok(())
    }
    fn execute_app_callback(
        &self,
        callback: &AppCallbackWrapper,
        event: &Box<dyn Any>,
        handling_result: &mut EventHandlingResult,
    ) -> Result<(), DiffError> {
        let handler = callback.callback.replace(None);
        if let Some(handler) = handler {
            if let Some(output) = (handler)(event) {
                if callback.chained.is_empty() {
                    handling_result.add_app(output);
                } else {
                    for other in &callback.chained {
                        self.execute_callback(*other, &output, handling_result)?;
                    }
                }
            }
            Ok(())
        } else {
            Err(DiffError::HandlerAlreadyUsed(callback.id, callback.node_id))?
        }
    }
}

pub struct EventHandlingResult {
    state_changes: Vec<Box<dyn Any>>,
    local_node_updates: Vec<(Box<dyn Any>, NodeId)>,
}

impl EventHandlingResult {
    pub fn new() -> Self {
        Self {
            local_node_updates: Vec::new(),
            state_changes: Vec::new(),
        }
    }

    pub fn add_local(&mut self, event: Box<dyn Any>, node: NodeId) {
        self.local_node_updates.push((event, node));
    }
    pub fn add_app(&mut self, event: Box<dyn Any>) {
        self.state_changes.push(event);
    }
}

struct SwappedNode {
    old_node: Node,
    new_node_id: NodeId,
}

enum NodeChanges {
    Nodes(Vec<SwappedNode>),
    StateChange,
}
