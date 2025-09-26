use std::ops;

pub struct Complex {
    real : f32,
    imag : f32,    
}

impl Complex {
    pub fn new(real : f32, imag: f32) -> Self {
        Self {real: real, imag: imag}
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, _rhs: Complex) -> Complex {
        let result = Complex {real: self.real + _rhs.real, i: self.imag  + _rhs.imag};

        result
    }
}


