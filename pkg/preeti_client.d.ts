/* tslint:disable */
/* eslint-disable */
/**
*/
export function init(): void;
/**
* @param {string} input
* @returns {string}
*/
export function preeti_to_unicode(input: string): string;
/**
* @param {Uint8Array} input
* @returns {Uint8Array}
*/
export function preeti_to_unicode_docx(input: Uint8Array): Uint8Array;
/**
* @param {string} input
* @returns {string}
*/
export function unicode_to_preeti(input: string): string;
/**
* @param {Uint8Array} input
* @returns {Uint8Array}
*/
export function unicode_to_preeti_docx(input: Uint8Array): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init: () => void;
  readonly preeti_to_unicode: (a: number, b: number, c: number) => void;
  readonly preeti_to_unicode_docx: (a: number, b: number, c: number) => void;
  readonly unicode_to_preeti: (a: number, b: number, c: number) => void;
  readonly unicode_to_preeti_docx: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
