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
