import { useEffect } from "react";

const useChip8 = (romBytes: Uint8Array | null) => {
  useEffect(() => {
    if (!romBytes) return;

    let animationFrameId: number;

    (async () => {
      const { get_audio, get_canvas_context, get_height, get_width, WasmEmulator } = await import("../../public/wasm/tiger_chip_8_wasm");

      const width = get_width();
      const height = get_height();
      const scale = 10;
      const opPerFrame = 10;

      const ctx = get_canvas_context(width, height, scale);
      const audio = get_audio();

      const emulator = new WasmEmulator(ctx, audio);
      emulator.load_font_set();
      emulator.load_rom(romBytes);

      const keydownHandler = (event: KeyboardEvent) => {
        const keycode = emulator?.to_keycode(event.key);
        if (keycode !== undefined) {
          emulator?.handle_key_press(keycode, true);
        }
      };
      const keyupHandler = (event: KeyboardEvent) => {
        const keycode = emulator?.to_keycode(event.key);
        if (keycode !== undefined) {
          emulator?.handle_key_press(keycode, false);
        }
      };

      document.addEventListener("keydown", keydownHandler);
      document.addEventListener("keyup", keyupHandler);

      const renderFrame = () => {
        for (let i = 0; i < opPerFrame; i++) {
          emulator.emulate_instruction();
        }
        emulator.tick_timers();
        emulator.draw_screen(width, scale);
        animationFrameId = requestAnimationFrame(renderFrame);
      };

      renderFrame();

      return () => {
        document.removeEventListener("keydown", keydownHandler);
        document.removeEventListener("keyup", keyupHandler);
        cancelAnimationFrame(animationFrameId);
      };
    })();
  }, [romBytes]);
};

export { useChip8 };
