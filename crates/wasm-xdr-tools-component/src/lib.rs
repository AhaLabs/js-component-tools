use soroban_env_host::xdr::{ReadXdr, ScObject, ScVal, WriteXdr, ScVec};
use std::sync::Once;

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
        let spec = strval::Spec::from_wasm(&wasm).map_err(|e| e.to_string())?;
        let func = spec.find_function(&func_name).map_err(|e| e.to_string())?;
        let value = serde_json::from_str::<serde_json::Value>(&json_args)
            .map_err(|_| "failed to parse json".to_string())?;
        let args = value.as_object().unwrap();
        let mut encoded_args = func
            .inputs
            .iter()
            .map(|input| {
                let arg = args.get(&input.name.to_string_lossy()).unwrap();
                spec.from_json(arg, &input.type_).map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut res = vec![
            ScVal::Object(Some(ScObject::Bytes(
                contract_id.as_bytes().try_into().unwrap(),
            ))),
            ScVal::Symbol(func_name.try_into().unwrap()),
        ];
        res.append(&mut encoded_args);
            let sc_vec: ScVec = res.try_into().unwrap();
        sc_vec.to_xdr().map_err(|e|e.to_string())
    }
    fn decode_args(wasm: Vec<u8>, func_name: String, xdr_args: Vec<u8>) -> Result<String, String> {
        init();
        let spec = strval::Spec::from_wasm(&wasm).map_err(|e| e.to_string())?;
        let func = spec.find_function(&func_name).unwrap();
        let args = ScVal::from_xdr(&xdr_args).unwrap();
        spec.xdr_to_json(&args, &func.outputs[0])
            .map(|v| v.to_string())
            .map_err(|e| e.to_string())
    }
}
