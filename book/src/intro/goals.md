# Features

**Ragnar** provides a cross platform UI development framework written in rust that focuses on large scale applications
that can be developed independently by multiple teams.

* Cross platform
    * Web
    * Android
    * iOS
    * Desktop (Windows, Mac, Linux)
* Modularity
    * Provide features on how to couple different applications together
    * Developers can independently work on parts of the application
    * Warm reloading during development
* Isomorphic applications possible due to single language for backend and fronted
* Redux style state management
* State and events are serializable ([serde](https://serde.rs/))
    * Share the same state between all platforms during development
    * Go back and forth in state history
    * Have *mocked* parts of inherited state if you develop a module and don't want to depend on other modules
* Unified component markup langauge that lets developers write a UI once and run it everywhere
* Routing and history
    * Bring the routing and history we know from the web to all platforms