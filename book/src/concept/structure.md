# Project structure

If you start a new application you will need the following:

* App-Library that contains your application
    * You might want to split this apart into modules to safe on compile time of unchanged/unrelated parts of the application
* Dev-server that imports your app library. You launch the dev server while developing
* single backend project per target platform (wasm/html project for web, android studio project for android etc...)
