use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebAssembly, WebAssemblyInstantiatedSource};

// Load and instantiate a Wasm module dynamically
async fn load_plugin_module(url: &str) -> Result<WebAssemblyInstantiatedSource, JsValue> {
    let response = window().fetch_with_str(url).await?;
    let bytes = js_sys::Uint8Array::new(&JsValue::UNDEFINED);
    let wasm = WebAssembly::instantiate_buffer(&response.array_buffer().await?, &bytes)?;
    Ok(wasm)
}

// Load a plugin and retrieve an instance of the ArithmeticPlugin trait
pub async fn load_plugin(url: &str) -> Result<Box<dyn ArithmeticPlugin>, JsValue> {
    let wasm = load_plugin_module(url).await?;
    let exports = wasm.instance.exports();
    let plugin = exports.get_function("instantiate")?
        .call0(&JsValue::NULL)?
        .dyn_into::<ArithmeticPlugin>()?;
    Ok(plugin)
}

// Example usage in your main application
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    // Load and use a plugin
    let plugin_url = "path/to/addition_plugin.wasm";
    let plugin = load_plugin(plugin_url).await?;
    let result = plugin.add(5, 3);
    console::log_1(&format!("Result: {}", result).into());
    Ok(())
}
