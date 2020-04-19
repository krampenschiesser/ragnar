# State and events

## Mutability

The state is only mutable by handlers(reducers) for events(actions).

## Serializability

State and events are serializable via Serde.
With this limitation we can store the state and events before reloading/restarting the dev-server.
After restart we will feed the state and events back into the dev-server and rerender for all clients.

The need for feeding events back in helps so that while developing the UI and reloading its module we can keep the event history
and do A<->B comparisons between the different states of the UI.

## Event history and tool

During development we want to be able to go back and forth in the history of all events modifying the state.
For hat we store the initial or oldest state if event list gets too long, eg. max 1000 events.
We also store the current state as the application needs it.
Going back in the event history means replaying all the events against a copy of the initial state which becomes the current state.

To help with development we will provide a tool that allows us to see the state and events as json serialized data
similar to the redux dev tools.
The initial application will probably be a desktop application (since that is our development platform) 
but hopefully later be ported to the universal application markup and run on every platform.

## State inheritance

Modules naturally build a tree structure and a child module can inherit state from its parent module.
On all platforms but the web this will be actually the same state (pointer).
On the web due to the usage of WASM we will try to share the memory between modules if that is possible.
Otherwise we will have to copy it with every update down to the child.