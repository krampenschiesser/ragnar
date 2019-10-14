# Reality check and plan

Supporting all the features is a big task that will be done over a longer period of time.

The initial plan is as follows:
 
1. provide a MVP that focuses on modules, state management, virtual dom and rendering to Android, gtk and web
    1. the rendering on Android and gtk will only be supported by de/serializing the dom/widget modification events and replaying them
    * no access to android API will be provided yet
1. implement rsx(jsx) dialects for the different platforms and a reduced functionality common one for all platforms
1. implement hot reloading of the rust libraries for the platforms
1. implement routing and history
1. work on styling and css
1. work on targeting different resolutions
1. provide tighter integration with Android by native calls and start exposing relevant API's  

But what about iOS:  
It is planned but since I do not own neither a mac nor the developer license 
it has to wait for me to acquire those or another developer bridges that gap.
If that happens the effort will be to do iOS at the same time as Android.

Future:  
Provide common services like 
* http/rest (reqwest???)
* notifications
* actors incl. web workers
* geolocation
* clipboard
* vibration
* app state
* camera

etc.

Provide acces to native API's on Android and iOS