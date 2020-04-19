# Reality check and plan

Supporting all the features is a big task that will be done over a longer period of time.

The initial plan is as follows:
 
1. Provide a MVP that focuses on the core functionality (including rsx macro) and rendering in web and on Android
1. implement routing and history
1. implement desktop backend
1. implement iOS backend
1. implement common UI

## Services and native integration

The goal is to provide integration with the mobile and web platforms.
Hopefully in a way that the same interface can be shared between all platforms via compiler target switches (with details of each platform).

These service and integrations should stay separated from the main ragnar library so that other rust projects can reuse them.
The future will show if this will be possible.

Future work will try to provide the following:
* http/rest (reqwest???)
* notifications
* actors incl. web workers
* geolocation
* clipboard
* vibration
* app state
* camera

etc.

Maybe even provide full access to native API's on Android and iOS.