import {exports} from "../xdr/wasm-xdr-tools.js";
import  * as fs from "node:fs/promises";

console.log(exports)

let [wasm, contract_id, fn, args]  = process.argv.slice(2);
console.log(wasm, contract_id, fn, args);
console.log(exports.run(await fs.readFile(wasm), contract_id, fn, args));