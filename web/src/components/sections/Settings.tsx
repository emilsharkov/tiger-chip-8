import { useChip8 } from "@/hooks/useChip8"
import { Rom, ROMS } from "@/lib/roms"
import { Pause, Play, RotateCcw } from "lucide-react"
import { useRef, useState } from "react"
import { Button } from "../ui/button"
import { Card } from "../ui/card"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../ui/select"
import { Slider } from "../ui/slider"


interface SettingsProps {
    setShowTiger: (show: boolean) => void;
    setSelectedRom: (rom: Rom | null) => void;
}

const Settings = (props: SettingsProps) => {
    const { setShowTiger, setSelectedRom } = props;
    const [isPlaying, setIsPlaying] = useState(true)
    const [speed, setSpeed] = useState([6])
    const [romBytes, setRomBytes] = useState<Uint8Array | null>(null);
    const triggerRef = useRef<HTMLButtonElement>(null);
    const { reset } = useChip8({ romBytes, speed, isPlaying });

    const selectRom = async (value: string) => {
        setShowTiger(false);
        const response = await fetch(`/roms/${value}.rom`);
        if (!response.ok) {
          console.error('Failed to fetch ROM:', response.statusText);
          return;
        }
        const buffer = await response.arrayBuffer();
        const rom = ROMS.find(rom => rom.filename === value) || null;
        setIsPlaying(true);
        setRomBytes(new Uint8Array(buffer));
        setSelectedRom(rom);
        setSpeed([rom?.speed || 6]);
        triggerRef.current?.blur();
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
        <Card className="p-6 bg-gradient-to-r from-orange-100 to-amber-100 border-2 border-orange-200 shadow-lg">
            <div className="flex flex-wrap items-center gap-4">
                <div className="flex items-center gap-2">
                <span className="text-orange-700 font-medium">Game:</span>
                <Select onValueChange={selectRom}>
                    <SelectTrigger ref={triggerRef} className="w-32 bg-white border-orange-300 text-orange-700">
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
                <span className="text-orange-700 font-medium">Speed:</span>
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
    )
}

export { Settings }
