# This is a very early version undergoing heavy changes

That being said what is ragnar:

Ragnar is a library to enable Rust UI development on most platforms.

## Planned platform supports

1. Web 
2. Android
3. Desktop once decided which way to go for rendering widgets:
    * Platform independent toolkits: Gtk, Fltk, Rust rendered natives(conrod, iced)
    * Web in webview
    * JavaFx bundled via GraalVm -> WATCH OUT, ORACLE!
    * Native toolkit per platform  
  -> Current preference is double backend for Gtk and Web in webview, let's see how support intensive that is
4. IOS put at the end because I don't own a Mac

## Features

* Development mode, aka warm reloading
    * Main interaction with UI is via events and dom-changes, both will be serialized in development mode
    * Therefore you can run an android,web,ios,desktop app that connects to dev server and receives dom-changes and sends events
    * Production mode will not serialize but directly call/execute shared library/wasm
    * warm reloading because we can recompile the dev server and restart it which will cause the frontends to update
        * we have to recompile/restart so we can also debug the application
        * if debugging is not needed we can just reload the shared libraries
* Native code on every platform with no runtime -> Rust
* RSX macro similar to JSX but with compile time checking 
* Redux style state management
* Elm style component architecture (with slight deviations)
* CommonUI, develop a common UI lib that will enable developers to not have to use platform specific widgets
    * Create something similar to ReactNative/Nativescript/Flutter widget libs

## License

MIT and Apache