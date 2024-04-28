#rustcalc
Not really getting wasm... wasm_bindgen allows you to build wasm modules and run them into browsers, wasmer should be a sort of runtime (?) to execute wasm modules.. Can I use wasmer to run a wasm compiled standalone application?

Oh if I build my app as wasm32-wasi and I run it with wasmtime it works as I run it as a rust application. Oh! Once I built in wasm32-wasi, I can run it with wasmer too with expected result!

So, what's the difference between wasm32-unknown-unknown, my previous target choiche, and wasm32-wasi?

Can I build wasm32-wasi and run that module on browser? Well, it seems possible, see [here](https://github.com/happybeing/svelte-wasi-with-rust).

Let's try add a first module...

Having hard times about finding docs about plugins.. Maybe [this](https://stackoverflow.com/questions/77784495/build-a-wasm-module-from-rust-to-be-used-as-a-plugin-for-a-rust-process-reading) helps.