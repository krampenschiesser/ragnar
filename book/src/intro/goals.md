# Features

**Ragnar** provides a cross platform UI development framework written in rust that focuses on large scale applications
that can be developed independently by multiple teams.

* Cross platform
    * Web
    * Android
    * iOS
    * Desktop (Windows, Mac, Linux)
* Modularity
    * Lazy loading of modules
    * Each module can have local state or inherit state from it's parent module
    * Developers can independently work on parts of the application
    * Hot reloading during development
* Isomorphic applications possible due to single language for backend and fronted
* Redux style state management
* State and events are serializable (CBOR or MessagePack)
    * Share the same state between all platforms during development
    * Go back and forth in state history
    * Have *mocked* parts of inherited state if you develop a module
* Unified CSS like markup but with variables, themes and color derivation
    * Default Material theme
* Routing and history
    * Bring the routing and history we know from the web to all platforms
    