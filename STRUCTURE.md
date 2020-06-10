# lite-lib

## Rust-lang
What is this dirty moonshine would you speak?

## WASM - Web Assembly
Since the first release in December 2019 it's a W3C conventional language. It must now be supported natively by all the most recent web browsers which follow the convention. It's a native target for rust-lang, like ios and android.

The bindings until rust and different browsers are made by `wasm-bindgen`. It made for us the high-level interactions between wasm modules and JavaScript. The Javascript promise for example, are implemented with the asynchronous futures in rust:
`use wasm_bindgen_futures::JsFuture;`

We found some interpolation with the web system too:
`use web_sys::{Request, RequestInit, RequestMode, Response};`

And Javascript system:
`use js_sys::Function;`

`wasm-pack` will compile our rust code to a wasm binary.

## WASI - Web Assembly System Interface
Wasi is an API designed by the Wasmtime project that provides access to several operating-system-like. It's designed to be independent of browsers, so it doesn't depend on Web APIs or JS, and isn't limited by the need to be compatible with JS.

To rephrase the official definition, it will provide us a way to use WebAssembly everywhere, without browser or JS engine. ::drop the mike::

Wasi is to WebAssembly what Node is to Javascript, with a few more subtleties, but it's close enough.

The core WebAssembly language is independent of its surrounding environment, and WebAssembly interacts with the outside world exclusively through APIs. On the Web, it naturally uses the existing Web APIs provided by browsers.

Wasi is an initiative with a clean set of APIs which can be implemented on multiple platforms by multiple engines, and which don't depend on browser functionality but it still can run in browsers.

## Actual architecture
See the code

## New architecture
### Target
- Create a frontend framework (like react)
- Usable on every hardware where wasm/wasi are runnable
- Memory resource consumption as light as possible
- Embedded stateful manager (like redux)
- Agnostic of renderer langage (not only HTML/JS)

### Rust structure
- Rust component creator:
    - Create struct
    - Set properties
    - Set behavior
    - Set properties and behavior in BTreeMap
    - Render element
    - Destroy rust component
- Rust component updater:
    - Get properties and behavior from BTreeMap
    - Create struct
    - Found element
    - Replace it
    - Destroy rust component
- Store
    - State, provide the public API of a rust component, properties / behavior
    - Concatenation of States as BTreeMap (source of thruth):
        - Composed by:
            - UID (used as unique identifier for the element)
            - Properties (string / u32 / ...)
            - Behavior (callback)
        - Get / Set properties and behavior
    - Reducer, function for update properties and behavior of an atom of the BTreeMap
    - Dispatcher, launch rust component updater for the atom
    
### Expected
I expect to build the entire skeleton of the first page before it displayed, and done the rest of the app with lazy loading. For doing it I need to know before all the exact properties/behavior of the app and it must have defaults properties / behavior for each rust components.

After the first renderer, the entire memory will be released, only the runtime will run. It's the first real difference with javascript where all the components are waiting in memory to be used.
To be clear, it is only because of the paradigm difference between them. 

The app will wait a user interaction, it might update the states, or not. If the action update the state of the app, it will engage the modification of the Store::BTreeMap. After it, a delta is done between the old and the new BTreeMap, and changes are done. They are done with the creation of the new rust components which will be instantiate with the new properties/behavior and update the old component dom element (found by the UID).
