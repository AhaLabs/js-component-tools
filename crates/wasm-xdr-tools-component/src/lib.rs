use std::sync::Once;

use strval::Spec;

wit_bindgen_guest_rust::generate!("wasm-tools");

struct WasmToolsJs;

export_wasm_tools_js!(WasmToolsJs);

fn init() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            console::error(&info.to_string());
            prev_hook(info);
        }));
    });
}

impl exports::Exports for WasmToolsJs {
    fn encode_args(
        wasm: Vec<u8>,
        contract_id: String,
        func_name: String,
        json_args: String,
    ) -> Result<Vec<u8>, String> {
        init();
        Spec::from_wasm(&wasm)
            .and_then(|spec| spec.encode_args(&contract_id, &func_name, &json_args))
            .map_err(|e| e.to_string())
    }
    fn decode_args(wasm: Vec<u8>, func_name: String, xdr_args: Vec<u8>) -> Result<String, String> {
        init();
        Spec::from_wasm(&wasm)
            .and_then(|spec| spec.decode_args(&func_name, &xdr_args))
            .map_err(|e| e.to_string())
    }
}
