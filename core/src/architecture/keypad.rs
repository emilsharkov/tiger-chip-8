pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }
}
