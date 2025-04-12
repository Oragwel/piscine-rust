#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        // Swapping logic: If first and second correspond to the same field, no swap happens
        if first == self.r {
            if second == self.g {
                std::mem::swap(&mut self.r, &mut self.g);
            } else if second == self.b {
                std::mem::swap(&mut self.r, &mut self.b);
            } else if second == self.a {
                std::mem::swap(&mut self.r, &mut self.a);
            }
        } else if first == self.g {
            if second == self.r {
                std::mem::swap(&mut self.g, &mut self.r);
            } else if second == self.b {
                std::mem::swap(&mut self.g, &mut self.b);
            } else if second == self.a {
                std::mem::swap(&mut self.g, &mut self.a);
            }
        } else if first == self.b {
            if second == self.r {
                std::mem::swap(&mut self.b, &mut self.r);
            } else if second == self.g {
                std::mem::swap(&mut self.b, &mut self.g);
            } else if second == self.a {
                std::mem::swap(&mut self.b, &mut self.a);
            }
        } else if first == self.a {
            if second == self.r {
                std::mem::swap(&mut self.a, &mut self.r);
            } else if second == self.g {
                std::mem::swap(&mut self.a, &mut self.g);
            } else if second == self.b {
                std::mem::swap(&mut self.a, &mut self.b);
            }
        }
        self
    }
}
