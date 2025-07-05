import { useEffect, useState } from "react";

const scale = 12;

interface UseChip8Props {
  romBytes: Uint8Array | null;
  speed: number[];
}

interface UseChip8Return {
  reset: () => void;
}

const useChip8 = (props: UseChip8Props): UseChip8Return => {
  const { romBytes, speed } = props;
  const [reset, setReset] = useState(false);

  useEffect(() => {
    if (!romBytes || speed.length === 0) return;

    let animationFrameId: number;

    (async () => {
      const { 
        get_audio, 
        get_canvas_context, 
        get_height, 
        get_width, 
        WasmEmulator 
      } = await import("../../public/wasm/tiger_chip_8_wasm");

      const width = get_width();
      const height = get_height();
      const ctx = get_canvas_context(width, height, scale);
      const audio = get_audio();
      const opPerFrame = speed[0];

      const emulator = new WasmEmulator(ctx, audio);
      emulator.load_font_set();
      emulator.load_rom(romBytes);

      const keydownHandler = (event: KeyboardEvent) => {
        const keycode = emulator.to_keycode(event.key);
        if (keycode !== undefined) {
          emulator.handle_key_press(keycode, true);
        }
      };
      const keyupHandler = (event: KeyboardEvent) => {
        const keycode = emulator.to_keycode(event.key);
        if (keycode !== undefined) {
          emulator.handle_key_press(keycode, false);
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

      if (reset) {
        setReset(false);
      }

      renderFrame();

      return () => {
        document.removeEventListener("keydown", keydownHandler);
        document.removeEventListener("keyup", keyupHandler);
        cancelAnimationFrame(animationFrameId);
      };
    })();
  }, [romBytes, speed, reset]);

  return { reset: () => setReset(true) };
};

export { useChip8 };
