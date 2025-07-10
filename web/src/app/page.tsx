'use client';

import { Controls } from "@/components/sections/Controls";
import { Display } from "@/components/sections/Display";
import { GameInfo } from "@/components/sections/GameInfo";
import { Header } from "@/components/sections/Header";
import { Settings } from "@/components/sections/Settings";
import { Rom } from '@/lib/roms';
import { useState } from 'react';

export default function Home() {
  const [showTiger, setShowTiger] = useState<boolean>(true);
  const [selectedRom, setSelectedRom] = useState<Rom | null>(null);

  return (
    <main>
      <Header />
      <div className="min-h-screen min-w-4xl bg-gradient-to-br from-orange-50 to-amber-50 p-4">
        <div className="max-w-4xl mx-auto space-y-8">
          <Settings setShowTiger={setShowTiger} setSelectedRom={setSelectedRom} />
          <Display showTiger={showTiger} />
          {selectedRom && <GameInfo selectedRom={selectedRom} />}
          <Controls />
        </div>
      </div>
    </main>
  );
}

