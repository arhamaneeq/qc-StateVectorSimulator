use std::ops;

pub struct Complex {
    pub real: f32,
    pub imag: f32,
}

impl Complex {
    pub fn new(real: f32, imag: f32) -> Self {
        Self {
            real: real,
            imag: imag,
        }
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, _rhs: Complex) -> Complex {
        let result = Complex {
            real: self.real + _rhs.real,
            imag: self.imag + _rhs.imag,
        };

        result
    }
}

impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, _rhs: Complex) -> Complex {
        let result = Complex {
            real: self.real - _rhs.real,
            imag: self.imag - _rhs.imag,
        };

        result
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, _rhs: Complex) -> Complex {
        let result = Complex {
            real: self.real * _rhs.real - self.imag * _rhs.imag,
            imag: self.real * _rhs.imag + self.imag * _rhs.real,
        };

        result
    }
}

impl ops::Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        let result = Complex {
            real: -self.real,
            imag: -self.imag,
        };

        result
    }
}

impl ops::Div for Complex {
    type Output = Complex;

    fn div(self, _rhs: Complex) -> Complex {
        let result = Complex {
            real: (self.real * _rhs.real + self.imag * _rhs.imag)
                / (_rhs.real * _rhs.real + _rhs.imag * _rhs.imag),
            imag: (-self.real * _rhs.imag + self.imag * _rhs.real)
                / (_rhs.real * _rhs.real + _rhs.imag * _rhs.imag),
        };

        result
    }
}
