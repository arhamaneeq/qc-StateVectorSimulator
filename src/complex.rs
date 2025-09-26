pub struct Complex {
    r : f32,
    i : f32,
}

impl Complex {
    pub fn new(real : f32, imag: f32) -> Self {
        Self {real, imag}
    }
}
