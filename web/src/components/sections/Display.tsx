import { Card } from "../ui/card";
import Image from "next/image";

interface DisplayProps {
    showTiger: boolean;
}

const Display = (props: DisplayProps) => {
    const { showTiger } = props;
    return (
        <Card className="p-8 bg-gradient-to-br from-amber-50 to-orange-50 border-2 border-orange-200 shadow-lg relative">
            <div className="aspect-video bg-gradient-to-br from-orange-100 to-amber-100 border-2 border-orange-300 rounded-lg flex items-center justify-center relative">
              {showTiger && (
                <div className="absolute inset-0 flex flex-col items-center justify-center z-10">
                  <a
                    href="https://www.auspiciousbeginning.com/page7?product_id=124"
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <Image
                      src="/images/tiger.png"
                      alt="Tiger Display Ready"
                      width={200}
                      height={200}
                      className="mx-auto rounded-full shadow-xl"
                    />
                  </a>
                  <p className="text-orange-600 font-medium">Tiger Display Ready</p>
                  <p className="text-orange-500 text-sm">Select a game to start playing!</p>
                </div>
              )}
              <canvas id="canvas" className="z-0" />
              <audio id="audio" />
            </div>
        </Card>
    )
}

export { Display };