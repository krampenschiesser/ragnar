# General concepts

We are driven by 5 key concepts:

1. We want to be able to have some kind of warm/hot reloading
1. We want to run on many platforms
1. We want to use rust and cargo with its normal workflows
1. Measure the performance of our application
1. Provide tools for easy testing of our application

## development

During development we have a not changing application running on android (emulator or device), 
web browser, IOS or desktop.  
This application connects to a specific development server (1 per platform) that the developer can launch and debug via
normal rust tools.  
State is managed like in elm/redux via events/actions and handlers/reducers.  
All changes from the UI are serialized and send as events to the dev-server.  
All changes in the UI are send from the dev-server to the UI.  
Events can trigger background service that do calls or interact with the device.  

In development mode services (eg http requests, location services, cameras) will be used from the host/development machine.
We need to provide some possibility to deploy services to the device to make sure everything runs on the device too.
 
Platform specific libraries (like support for platform widgets or mobile functionality) need to be added to the platforms application.

event handling, redrawing and service calls are measured automatically and published (which backend? prometheus or integrated? )

## release candidate

For release candidates we use the native device.
That means that we deploy a build library to the native device.
However this library still has a connection to the dev-server for visualisation of logging, metrics, state+history.


## production

For production we build a native library that is deployed to the device.
We still use event based communication between the device OS and the application but without any serialization (Wasm,JNI,cbindgen)
so that in fact we just call methods and get structs/objects back.
This removes network and serialization latency. 