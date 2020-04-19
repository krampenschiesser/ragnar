# Warm reloading

We are talking about warm reloading instead of hot reloading.
This is caused by a few things:

* Rust compiling is not that fast (I still want to point out the amazing efforts of the compiler team, the improvements they did are incredible)
* We want to be able to debug our application, so we need to have it run as a normal _main_ application

What we do:

* We have one dev-server per platform (because platforms are switched by compiler flags)
* We have a basic *backend* running on each target platform (eg. android activity, wasm+JS page, gtk application).
  * This *backend* sends serialized events to the dev-server.
  * The dev-server renders a new UI does the state diff and returns dom-change-events ot the UI-backend.
  * UI-backend will redraw 
* we serialize state before shutdown and store it, then startup and deserialize the state again to fill our application with where we left off
    * This allows us to also fork the state during eg. QA testing. 
      So QA can fork the state when a bug happened, send the state to the developer and continue testing on an earlier or fresh version of the state.
* Via **cargo-watch** or maybe integrated into the dev-server we will watch file changes and recompile the app-library and dev-server and then restart the dev-server

