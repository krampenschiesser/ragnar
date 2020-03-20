use std::any::Any;

use crate::playground1::app_component::{AppComponent, AppEvent, AppState};
use crate::playground1::callback::{CallbackId, CallbackType};
use crate::playground1::local_component::{LocalComponentWrapper, LocalHandleResult};
use crate::playground1::node::{NodeComponentWrapper, NodeId};
use crate::playground1::runtime::denormalized_node::{DetachedNode, NodeContainer, StrippedNode};
use crate::playground1::runtime::diff::{CompleteDiff, DiffError};
use crate::playground1::runtime::diff::operations::{DiffOperation, ParentPosition};

use super::node::Node;

mod denormalized_node;
mod diff;

pub struct Runtime<C: AppComponent<State=State, Msg=Msg>, State: AppState, Msg: AppEvent> {
    root_component: C,
    root: NodeContainer,
    state: State,
    update_function: Box<dyn Fn(&mut State, &Msg)>,
}

impl<C: AppComponent<State=State, Msg=Msg>, State: AppState, Msg: AppEvent> Runtime<C, State, Msg> {
    pub fn handle_event(&mut self, id: CallbackId, event: Box<dyn Any>) -> Result<Vec<DiffOperation>, DiffError> {
        let mut handling_result = EventHandlingResult::new();
        self.execute_chained_callbacks(id, &event, &mut handling_result)?;
        let EventHandlingResult { state_changes: changes, local_node_updates: node_updates } = handling_result;
        if changes.is_empty() {
            let mut diff_ops = Vec::new();
            let changed_nodes = self.update_local_nodes(node_updates);
            for (new_node_id, old_container, parent) in changed_nodes {
                let diff = CompleteDiff::new(&old_container, &self.root);
                let new_ops = diff.diff_partial(old_container.root_node, new_node_id, parent)?;
                diff_ops.extend(new_ops);
            }
            Ok(diff_ops)
        } else {
            let new_node = self.update(changes);
            let new_container = NodeContainer::from_root(new_node);
            let result = CompleteDiff::new(&self.root, &new_container).diff();
            self.root = new_container;
            result
        }
    }

    fn update_local_nodes(&mut self, updates: Vec<(Box<dyn Any>, NodeId)>) -> Vec<(NodeId, NodeContainer, Option<ParentPosition>)> {
        let mut nodes_that_need_diff = Vec::new();
        for (boxed, id) in updates {
            //TODO remove children if parent gets updated

            let handle_result: LocalHandleResult = if let Some(stripped_node) = self.root.get_node(&id) {
                match &stripped_node.component {
                    NodeComponentWrapper::Local(l) => {
                        l.handle(&boxed)
                    }
                    _ => LocalHandleResult::Keep
                }
            } else {
                LocalHandleResult::Keep
            };

            let node_to_diff = match handle_result {
                LocalHandleResult::Keep => None,
                LocalHandleResult::NewRender(node) => {
                    let new_id = node.id;
                    let option = self.root.replace_node(node, id);
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

    fn update(&mut self, events: Vec<Box<dyn Any>>) -> Node {
        for boxed in events {
            if let Some(event) = boxed.downcast_ref() {
                (self.update_function)(&mut self.state, event);
            }
        }
        self.root_component.render(&self.state).into_node()
    }

    fn execute_chained_callbacks(&self, id: CallbackId, event: &Box<dyn Any>, handling_result: &mut EventHandlingResult) -> Result<(),DiffError> {
        let mut execute_additional = None;
        if let Some(callback_wrapper) = self.root.callbacks.get(&id) {
            let executed = (callback_wrapper.callback)(event);
            let node = self.root.get_node(&callback_wrapper.node_id).ok_or(DiffError::NewNodeNotFound(callback_wrapper.node_id))?;
            // if let Some((converters,event)) = node.converters.and_then(|s|executed.map(|e|(s,e))) {
            //     converters.iter().rev().for_each(|c|{
            //         // (c)(event);
            //     })
            // }
            //fixme chl apply converters to event

            if let Some(event) = executed {
                if !callback_wrapper.chained.is_empty() {
                    execute_additional = Some((event, callback_wrapper.chained.clone()));
                } else {
                    match callback_wrapper.callback_type {
                        CallbackType::Local => handling_result.local_node_updates.push((event, callback_wrapper.node_id)),
                        CallbackType::App => handling_result.state_changes.push(event),
                        CallbackType::Native(_) => (),
                    }
                }
            }
        }
        if let Some((event, additional)) = execute_additional {
            for callback_id in additional {
                self.execute_chained_callbacks(callback_id, &event, handling_result);
            }
        }
        Ok(())
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
}


struct SwappedNode {
    old_node: Node,
    new_node_id: NodeId,
}

enum NodeChanges {
    Nodes(Vec<SwappedNode>),
    StateChange,
}