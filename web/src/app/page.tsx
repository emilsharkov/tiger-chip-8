'use client';

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Slider } from "@/components/ui/slider";
import { useChip8 } from '@/hooks/useChip8';
import { Rom, ROMS } from '@/lib/roms';
import { Pause, Play, RotateCcw } from "lucide-react";
import Image from "next/image";
import { useState } from 'react';

export default function Home() {
  const [isPlaying, setIsPlaying] = useState(true)
  const [speed, setSpeed] = useState([12])
  const [romBytes, setRomBytes] = useState<Uint8Array | null>(null);
  const [showTiger, setShowTiger] = useState(true);
  const { reset } = useChip8({ romBytes, speed, isPlaying });

  const selectRom = async (value: string) => {
    setShowTiger(false);
    const response = await fetch(`/roms/${value}.rom`);
    if (!response.ok) {
      console.error('Failed to fetch ROM:', response.statusText);
      return;
    }
    const buffer = await response.arrayBuffer();
    setIsPlaying(true);
    setRomBytes(new Uint8Array(buffer));
  };

  const handleReset = () => {
    setIsPlaying(true);
    reset();
  }

  const handleSpeedChange = (value: number[]) => {
    setIsPlaying(true);
    setSpeed(value);
  }

  return (
    <main>
      {/* Header with GitHub Link */}
      <header className="bg-orange-100 p-4 shadow-md">
        <div className="max-w-4xl mx-auto flex justify-between items-center">
          <h1 className="text-2xl font-bold text-orange-800">Tiger Chip-8</h1>
          <div className="flex items-center gap-4">
            <a
              href="https://github.com/emilsharkov/tiger-chip8"
              target="_blank"
              rel="noopener noreferrer"
              className="text-orange-700 hover:text-orange-900"
            >
              Source Code
            </a>
            <a
              href="https://en.wikipedia.org/wiki/CHIP-8"
              target="_blank"
              rel="noopener noreferrer" 
              className="text-orange-700 hover:text-orange-900"
            >
              What is Chip-8?
            </a>
            <a
              href="https://www.auspiciousbeginning.com/page7?product_id=124"
              target="_blank"
              rel="noopener noreferrer"
              className="text-orange-700 hover:text-orange-900"
            >
              Where to buy tiger?
            </a>
          </div>
        </div>
      </header>

      <div className="min-h-screen bg-gradient-to-br from-orange-50 to-amber-50 p-4">
        <div className="max-w-4xl mx-auto space-y-8">
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
                <Slider value={speed} onValueChange={handleSpeedChange} max={20} min={1} step={1} className="flex-1" />
                <span className="text-orange-700 font-medium w-8">{speed[0]}</span>
              </div>

              <div className="flex gap-2">
                <Button onClick={() => setIsPlaying(!isPlaying)} className="bg-orange-500 hover:bg-orange-600 text-white">
                  {isPlaying ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
                  {isPlaying ? "Pause" : "Resume"}
                </Button>
                <Button onClick={handleReset} variant="outline" className="border-orange-400 text-orange-700 hover:bg-orange-50 bg-transparent">
                  <RotateCcw className="w-4 h-4" />
                  Reset
                </Button>
              </div>
            </div>
          </Card>

          {/* Main Display */}
          <Card className="p-8 bg-gradient-to-br from-amber-50 to-orange-50 border-2 border-orange-200 shadow-lg relative">
            <div className="aspect-video bg-gradient-to-br from-orange-100 to-amber-100 border-2 border-orange-300 rounded-lg flex items-center justify-center relative">
              {showTiger && (
                <div className="absolute inset-0 flex flex-col items-center justify-center z-10">
                  <Image
                    src="/images/tiger.png"
                    alt="Tiger Display Ready"
                    width={200}
                    height={200}
                    className="mx-auto rounded-full shadow-xl"
                  />
                  <p className="text-orange-600 font-medium">Tiger Display Ready</p>
                  <p className="text-orange-500 text-sm">Select a game to start playing!</p>
                </div>
              )}
              <canvas id="canvas" className="z-0" />
              <audio id="audio" />
            </div>
          </Card>

          {/* About */}
          <Card className="py-6 px-16 bg-gradient-to-r from-orange-100 to-amber-100 border-2 border-orange-200 shadow-lg flex flex-col items-center justify-between gap-2">
            <h2 className="text-3xl font-bold text-orange-800 text-center">About</h2>
            <div className="flex flex-row items-center justify-center"> 
              <p className="text-orange-700 text-xl mr-4 leading-relaxed">
                Tiger Chip-8 is my implementation of the Chip-8 interpreter using Rust and WebAssembly. 
                Chip-8 was designed to be easy to program for and to use less memory than other programming languages.
                As a result, it was used to create many fan favorite retro games like Pong, Tetris, and Space Invaders. 
              </p>
              <Image  
                src="/images/tiger-isometric.png"
                alt="Tiger Logo"
                width={150}
                height={150}
                className="shadow-lg rounded-lg"
              />
            </div>
          </Card>

          {/* Controls Section */}
          <Card className="p-6 bg-gradient-to-br from-orange-100 to-amber-100 border-2 border-orange-200 shadow-lg gap-2">
            <h2 className="text-3xl font-bold text-orange-800 text-center">Controls</h2>
            <p className="text-center text-xl text-orange-700 max-w-2xl mx-auto leading-relaxed">
              This emulator maps the keys from the original COSMAC VIP keypad to the modern QWERTY keyboard
            </p>

            <div className="flex flex-col lg:flex-row justify-center items-start gap-8 lg:gap-16">
              {/* COSMAC VIP Keypad */}
              <div className="text-center">
                <h3 className="text-xl font-semibold text-orange-800 mb-4">COSMAC VIP Keypad</h3>
                <div className="grid grid-cols-4 gap-2 w-fit mx-auto">
                  {["1", "2", "3", "C", "4", "5", "6", "D", "7", "8", "9", "E", "A", "0", "B", "F"].map((key) => (
                    <div
                      key={key}
                      className="w-12 h-12 bg-gradient-to-br from-orange-200 to-amber-200 border-2 border-orange-300 rounded-lg flex items-center justify-center text-orange-800 font-bold text-lg shadow-md hover:shadow-lg transition-shadow"
                    >
                      {key}
                    </div>
                  ))}
                </div>
              </div>

              {/* Arrow */}
              <div className="flex items-center justify-center">
                <div className="text-4xl text-orange-400">→</div>
              </div>

              {/* QWERTY Equivalents */}
              <div className="text-center">
                <h3 className="text-xl font-semibold text-orange-800 mb-4">QWERTY Equivalents</h3>
                <div className="grid grid-cols-4 gap-2 w-fit mx-auto">
                  {["1", "2", "3", "4", "Q", "W", "E", "R", "A", "S", "D", "F", "Z", "X", "C", "V"].map((key) => (
                    <div
                      key={key}
                      className="w-12 h-12 bg-gradient-to-br from-amber-200 to-orange-200 border-2 border-amber-300 rounded-lg flex items-center justify-center text-orange-800 font-bold text-lg shadow-md hover:shadow-lg transition-shadow"
                    >
                      {key}
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </Card> 
        </div>
      </div>
    </main>
  );
}

