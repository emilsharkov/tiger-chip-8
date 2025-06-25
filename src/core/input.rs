pub struct Input {
    pub keys: [bool; 16],
}

impl Input {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }
}