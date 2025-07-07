const Header = () => {
    return (
      <header className="bg-orange-100 p-4 shadow-md min-w-4xl">
          <div className="max-w-4xl mx-auto flex justify-between items-center">
              <h1 className="text-2xl font-bold text-orange-800">Tiger Chip-8</h1>
              <div className="flex items-center gap-4">
              <a
                  href="https://github.com/emilsharkov/tiger-chip8"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-orange-700 hover:text-orange-900 font-medium"
              >
                  Source Code
              </a>
              <a
                  href="https://en.wikipedia.org/wiki/CHIP-8"
                  target="_blank"
                  rel="noopener noreferrer" 
                  className="text-orange-700 hover:text-orange-900 font-medium"
              >
                  What is Chip-8?
              </a>
              <a
                  href="https://www.auspiciousbeginning.com/page7?product_id=124"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-orange-700 hover:text-orange-900 font-medium"
              >
                  Where to buy tiger?
              </a>
              </div>
          </div>
      </header>
    )
  }
  
  export { Header }