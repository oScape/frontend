## lite-lib

### Component
Should provide the default behavior for a component, including:
- new, create the rust struct
- update, to change properties (connection with the store)
- render view

### Store
Should basically store the state of the app which is connected to it:
- set a top level provider to connect and dispatch data on it children
- the child could be connected to a global, multiple specific or a specific state
- the child must explain how to connect with itself (interface like)