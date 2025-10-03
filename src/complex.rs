#[derive(Clone, Copy, Debug)]

pub struct Complex {
    pub real: f32,
    pub imag: f32,
}

#[allow(dead_code)]
impl Complex {
    pub fn new(real: f32, imag: f32) -> Self {
        Self {
            real: real,
            imag: imag,
        }
    }

    pub fn polar(r: f32, theta: f32) -> Self {
        Self {
            real: r * f32::cos(theta),
            imag: r * f32::sin(theta),
        }
    }

    pub fn norm(self) -> f32 {
        let norm_sq: f32 = self.real.powi(2) + self.imag.powi(2);
        let sqrt_norm_sq: f32 = norm_sq.sqrt();

        sqrt_norm_sq
    }

    pub fn normalize(self) -> Complex {
        let norm: f32 = self.norm();
        let normalized_complex: Complex = Complex::new(self.real / norm, self.imag / norm);

        normalized_complex
    }

    pub fn conj(self) -> Complex {
        Complex::new(self.real, -self.imag)
    }
}

impl std::cmp::PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl std::ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, _rhs: Complex) -> Complex {
        let result: Complex = Complex {
            real: self.real + _rhs.real,
            imag: self.imag + _rhs.imag,
        };

        result
    }
}

impl std::ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, _rhs: Complex) -> Complex {
        let result: Complex = Complex {
            real: self.real - _rhs.real,
            imag: self.imag - _rhs.imag,
        };

        result
    }
}

impl std::ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, _rhs: Complex) -> Complex {
        let result: Complex = Complex {
            real: self.real * _rhs.real - self.imag * _rhs.imag,
            imag: self.real * _rhs.imag + self.imag * _rhs.real,
        };

        result
    }
}

impl std::ops::Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        let result: Complex = Complex {
            real: -self.real,
            imag: -self.imag,
        };

        result
    }
}

impl std::ops::Div for Complex {
    type Output = Complex;

    fn div(self, _rhs: Complex) -> Complex {
        let result: Complex = Complex {
            real: (self.real * _rhs.real + self.imag * _rhs.imag)
                / (_rhs.real * _rhs.real + _rhs.imag * _rhs.imag),
            imag: (-self.real * _rhs.imag + self.imag * _rhs.real)
                / (_rhs.real * _rhs.real + _rhs.imag * _rhs.imag),
        };

        result
    }
}

impl std::ops::Div<f32> for Complex {
    type Output = Complex;

    fn div(self, _rhs: f32) -> Complex {
        let result: Complex = Complex::new( 
            self.real / _rhs, 
            self.imag / _rhs 
        );

        result
    }
}