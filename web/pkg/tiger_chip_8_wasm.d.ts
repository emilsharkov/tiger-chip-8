/* tslint:disable */
/* eslint-disable */
export function get_canvas_context(width: number, height: number, scale: number): CanvasRenderingContext2D;
export function get_audio(): HTMLAudioElement;
export function get_width(): number;
export function get_height(): number;
export class WasmEmulator {
  free(): void;
  constructor(ctx: CanvasRenderingContext2D, audio: HTMLAudioElement);
  emulate_instruction(): void;
  tick_timers(): void;
  load_rom(rom_bytes: Uint8Array): void;
  load_font_set(): void;
  to_keycode(control: string): number | undefined;
  handle_key_press(key: number, pressed: boolean): void;
  draw_screen(width: number, scale: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_canvas_context: (a: number, b: number, c: number) => any;
  readonly get_audio: () => any;
  readonly get_width: () => number;
  readonly get_height: () => number;
  readonly __wbg_wasmemulator_free: (a: number, b: number) => void;
  readonly wasmemulator_new: (a: any, b: any) => number;
  readonly wasmemulator_emulate_instruction: (a: number) => void;
  readonly wasmemulator_tick_timers: (a: number) => void;
  readonly wasmemulator_load_rom: (a: number, b: number, c: number) => void;
  readonly wasmemulator_load_font_set: (a: number) => void;
  readonly wasmemulator_to_keycode: (a: number, b: number, c: number) => number;
  readonly wasmemulator_handle_key_press: (a: number, b: number, c: number) => void;
  readonly wasmemulator_draw_screen: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
