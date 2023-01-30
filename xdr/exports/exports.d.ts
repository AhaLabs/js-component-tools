export namespace Exports {
  export function encodeArgs(wasm: Uint8Array | ArrayBuffer, contractId: string, funcName: string, jsonArgs: string): Uint8Array;
  export function decodeArgs(wasm: Uint8Array | ArrayBuffer, funcName: string, xdrArgs: Uint8Array | ArrayBuffer): string;
}
