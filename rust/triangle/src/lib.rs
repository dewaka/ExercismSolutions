use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + Add<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let a = sides[0];
        let b = sides[1];
        let c = sides[2];

        if (a + b > c) && (a + c > b) && (b + c > a) {
            Some(Triangle { a, b, c })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        (self.a == self.b) && (self.b == self.c) && (self.c == self.a)
    }

    pub fn is_scalene(&self) -> bool {
        !(self.is_equilateral() || self.is_isosceles())
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.c == self.a
    }
}
