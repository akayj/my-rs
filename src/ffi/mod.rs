use std::fmt;

// this extern block links to the *libm* library
#[link(name = "m")]
extern "C" {
    // this is a foreign function
    // that computes the square root of a single precision complext number
    pub fn csqrtf(z: Complex) -> Complex;

    pub fn ccosf(z: Complex) -> Complex;
}

// Since calling foreign functions is considered unsafe,
// it's common to write safe wrappers around them.
pub fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, -self.im)
        }
    }
}
