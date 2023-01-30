use soroban_env_host::{
    storage::Storage,
    xdr::{HostFunction, WriteXdr},
    Host,
};
use soroban_ledger_snapshot::LedgerSnapshot;
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
        let spec = Spec::from_wasm(&wasm).map_err(|e| e.to_string())?;
        let res = spec
            .encode_args(&contract_id, &func_name, &json_args)
            .map_err(|e| e.to_string())?;
        res.to_xdr().map_err(|e| e.to_string())
    }

    fn decode_ret(wasm: Vec<u8>, func_name: String, xdr_ret: Vec<u8>) -> Result<String, String> {
        init();
        Spec::from_wasm(&wasm)
            .and_then(|spec| spec.decode_args(&func_name, &xdr_ret))
            .map_err(|e| e.to_string())
    }

    fn run(
        wasm: Vec<u8>,
        contract_id: String,
        func_name: String,
        json_args: String,
    ) -> Result<String, String> {
        init();

        Spec::from_wasm(&wasm)
            .and_then(|spec| {
                let state = LedgerSnapshot::default();
                let storage = Storage::default();
                let h = Host::with_storage_and_budget(storage, Default::default());

                let mut ledger_info = state.ledger_info();
                ledger_info.sequence_number += 1;
                ledger_info.timestamp += 5;
                h.set_ledger_info(ledger_info);
                let args = spec.encode_args(&contract_id, &func_name, &json_args)?;
                let xdr_return = h
                    .invoke_function(HostFunction::InvokeContract(args))
                    .unwrap()
                    .to_xdr()
                    .unwrap();
                spec.decode_args(&func_name, &xdr_return)
            })
            .map_err(|e| e.to_string())
    }
}
