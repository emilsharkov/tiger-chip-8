'use client';

import { useState } from 'react';
import { useChip8 } from '@/hooks/useChip8';
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Slider } from "@/components/ui/slider"
import { Upload, Play, Pause, RotateCcw } from "lucide-react"
import Image from "next/image"
import { Rom, ROMS } from '@/lib/roms';

export default function Home() {
  const [isPlaying, setIsPlaying] = useState(true)
  const [speed, setSpeed] = useState([12])
  const [romBytes, setRomBytes] = useState<Uint8Array | null>(null);
  const [showTiger, setShowTiger] = useState(true);
  const { reset } = useChip8({ romBytes, isPlaying, speed });

  const selectRom = async (value: string) => {
    setShowTiger(false);
    const response = await fetch(`/roms/${value}.rom`);
    if (!response.ok) {
      console.error('Failed to fetch ROM:', response.statusText);
      return;
    }
    const buffer = await response.arrayBuffer();
    setRomBytes(new Uint8Array(buffer));
  };

  return (
    <main>
      <div className="min-h-screen bg-gradient-to-br from-orange-50 to-amber-50 p-4">
        <div className="max-w-4xl mx-auto space-y-8">
          {/* Header */}
          <div className="text-center space-y-4">
            <h1 className="text-5xl font-bold text-orange-800 tracking-wide">Tiger Chip-8</h1>
            <p className="text-orange-600 text-lg font-medium">By Emil Sharkov</p>
          </div>

          {/* Controls */}
          <Card className="p-6 bg-gradient-to-r from-orange-100 to-amber-100 border-2 border-orange-200 shadow-lg">
            <div className="flex flex-wrap items-center gap-4">
              <div className="flex items-center gap-2">
                <span className="text-orange-700 font-medium">Game:</span>
                <Select onValueChange={selectRom}>
                  <SelectTrigger className="w-32 bg-white border-orange-300 text-orange-700">
                    <SelectValue className="text-orange-700" />
                  </SelectTrigger>
                  <SelectContent className="text-orange-700">
                    {ROMS.map((rom: Rom) => (
                      <SelectItem key={rom.filename} value={rom.filename}>{rom.label}</SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div className="flex items-center gap-4 flex-1">
                <span className="text-orange-700 font-medium">Steps/tick:</span>
                <Slider value={speed} onValueChange={setSpeed} max={20} min={1} step={1} className="flex-1" />
                <span className="text-orange-700 font-medium w-8">{speed[0]}</span>
              </div>

              <div className="flex gap-2">
                <Button onClick={() => setIsPlaying(!isPlaying)} className="bg-orange-500 hover:bg-orange-600 text-white">
                  {isPlaying ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
                  {isPlaying ? "Pause" : "Resume"}
                </Button>
                <Button onClick={reset} variant="outline" className="border-orange-400 text-orange-700 hover:bg-orange-50 bg-transparent">
                  <RotateCcw className="w-4 h-4" />
                  Reset
                </Button>
              </div>
            </div>
          </Card>

          {/* Main Display */}
          <Card className="p-8 bg-gradient-to-br from-amber-50 to-orange-50 border-2 border-orange-200 shadow-lg">
            <div className="aspect-video bg-gradient-to-br from-orange-100 to-amber-100 border-2 border-orange-300 rounded-lg flex items-center justify-center">
              <div className="text-center space-y-4">
                {showTiger ? (
                  <>
                    <Image
                      src="/images/tiger.png"
                      alt="Tiger Display Ready"
                      width={200}
                      height={200}
                      className="mx-auto rounded-full shadow-xl"
                    />
                    <p className="text-orange-600 font-medium">Tiger Display Ready</p>
                    <p className="text-orange-500 text-sm">Select a game to start playing!</p>
                  </>
                ) : (
                  <>
                    <canvas id="canvas" />
                    <audio id="audio" />
                  </>
                )}
              </div>
            </div>
          </Card>
        </div>
      </div>
    </main>
  );
}

