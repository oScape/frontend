import("./pkg/index.js")
    .then( wasm => {
        console.log("WASM loaded")

        const result = wasm.add(1, 2);
        console.log(result);
    }).catch(console.error);
