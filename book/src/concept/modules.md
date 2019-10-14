# Modules

A module defines its own state, event, components and routes.
It can inherit state from another module which is different then its own state.

This separation is necessary to enable loose coupling and independent development of teams 
which a redux style single state for the application would destroy.