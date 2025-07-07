import { Gamepad2 } from "lucide-react";
import { Card } from "../ui/card";
import { Rom } from "@/lib/roms";

interface GameInfoProps {
    selectedRom: Rom;
}

const GameInfo = (props: GameInfoProps) => {
    const { selectedRom } = props;
    return (
        <Card className="p-6 bg-gradient-to-br from-orange-100 to-amber-100 border-2 border-orange-200 shadow-lg gap-2">
            <div>
              <div className="flex items-center justify-center mb-6">
                <h2 className="text-3xl font-bold text-orange-800 flex items-center gap-2">
                  <Gamepad2 className="w-8 h-8" />
                  {selectedRom.label} - How to Play
                </h2>
              </div>

              <div className="grid lg:grid-cols-2 gap-8">
                {/* Left Column - Game Info */}
                <div className="space-y-6">
                  <div>
                    <h3 className="text-xl font-semibold text-orange-800 mb-3">About this game:</h3>
                    <p className="text-gray-700 leading-relaxed">{selectedRom.description}</p>
                  </div>

                  {selectedRom.objective && (
                    <div>
                      <h3 className="text-xl font-semibold text-orange-800 mb-3">Objective:</h3>
                      <div className="bg-amber-50 p-4 rounded-lg border-2 border-amber-200">
                        <p className="text-gray-700 leading-relaxed">{selectedRom.objective}</p>
                      </div>
                    </div>
                  )}
                  {selectedRom.speed && (
                    <div>
                      <h3 className="text-xl font-semibold text-orange-800 mb-3">Speed:</h3>
                      <div className="bg-amber-50 p-4 rounded-lg border-2 border-amber-200">
                        <p className="text-gray-700 leading-relaxed">
                          The recommended speed for this game is {selectedRom.speed}
                        </p>
                      </div>
                    </div>
                  )}
                </div>

                {/* Right Column - Controls & Tips */}
                <div className="space-y-6">
                  <div>
                    <h3 className="text-xl font-semibold text-orange-800 mb-4">Controls:</h3>
                    <div className="space-y-3">
                      {selectedRom.controls.map((control, index) => (
                        <div
                          key={index}
                          className="flex items-center gap-4 p-3 bg-white rounded-lg border border-orange-200 shadow-sm"
                        >
                          <div className="w-12 h-12 bg-gradient-to-br from-orange-200 to-amber-200 border-2 border-orange-300 rounded-lg flex items-center justify-center">
                            <span className="text-orange-800 font-bold text-lg font-mono">{control.key}</span>
                          </div>
                          <span className="text-gray-700 font-medium">{control.action}</span>
                        </div>
                      ))}
                    </div>
                  </div>

                  {selectedRom.tips && selectedRom.tips.length > 0 && (
                    <div>
                      <h3 className="text-xl font-semibold text-orange-800 mb-4">Tips:</h3>
                      <div className="space-y-3">
                        {selectedRom.tips.map((tip, index) => (
                          <div
                            key={index}
                            className="flex items-start gap-3 p-3 bg-orange-50 rounded-lg border border-orange-200"
                          >
                            <div className="w-6 h-6 bg-orange-200 rounded-full flex items-center justify-center flex-shrink-0 mt-0.5">
                              <span className="text-orange-700 font-bold text-sm">{index + 1}</span>
                            </div>
                            <span className="text-gray-700">{tip}</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              </div>
            </div>
        </Card>
    )
}

export { GameInfo };