'use client';

import { useState } from 'react';
import { useChip8 } from '@/hooks/useChip8';

export default function Home() {
  const [romBytes, setRomBytes] = useState<Uint8Array | null>(null);
  useChip8(romBytes);

  const handleRomUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;
    const buffer = await file.arrayBuffer();
    setRomBytes(new Uint8Array(buffer));
  };

  return (
    <main className="p-4">
      <h1 className="text-2xl font-bold mb-4">CHIP-8 Emulator</h1>
      <canvas className='border border-black' id="canvas"/>
      <audio id="audio"/>
      <input type="file" onChange={handleRomUpload} />
    </main>
  );
}

