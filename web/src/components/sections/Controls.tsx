import { Card } from "../ui/card"

const Controls = () => {
    return (
        <Card className="p-6 bg-gradient-to-br from-orange-100 to-amber-100 border-2 border-orange-200 shadow-lg gap-2">
            <h2 className="text-3xl font-bold text-orange-800 text-center">Controls</h2>
            <p className="text-center text-xl text-orange-700 max-w-2xl mx-auto leading-relaxed">
                This emulator maps the keys from the original COSMAC VIP keypad to the modern QWERTY keyboard
            </p>

            <div className="flex flex-col lg:flex-row justify-center items-center gap-8 lg:gap-16">
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
    )
}

export { Controls }