use std::fmt::{Debug, Formatter, Result};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, -self.im)
        }
    }
}
