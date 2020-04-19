# Modules

A module defines its own state, event, components.
Modules can be integrated into each other. So a parent module's state contains the child modules state.
When rendering the entry-component of a child-module it will just select the correct state property.
When rendering a child-module the parent module needs to define a converter that converts child-module events
to events the parent can handle.
The child-modules reducers must work together with the parent moduleslreducer.

This separation is necessary to enable loose coupling and independent development of teams.