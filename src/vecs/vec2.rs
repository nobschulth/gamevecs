
use core::f32;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};


///2D f32 Vector 
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    ///Short for Vec2::new(0.0, -1.0)
    pub const DOWN: Vec2 = Vec2 {x: 0., y: -1. };
    ///Short for Vec2::new(0.0, 1.0)
    pub const UP: Vec2 = Vec2 {x: 0., y: 1. };
    ///Short for Vec2::new(1.0, 0.0)
    pub const RIGHT: Vec2 = Vec2 {x: 1., y: 0. };
    ///Short for Vec2::new(-1.0, 0.0)
    pub const LEFT: Vec2 = Vec2 {x: -1., y: 0. };
    ///Short for Vec2::new(1.0, 1.0)
    pub const ONE: Vec2 = Vec2 {x: 1., y: 1. };
    ///Short for Vec2::new(0.0, 0.0)
    pub const ZERO: Vec2 = Vec2 {x: 0., y: 0. };


    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }


    //----------Other functionality----------

    ///Returns the length of this vector.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec = Vec2::new(3.0, 4.0);
    /// //get the magnitude
    /// let magnitude = vec.magnitude();
    ///
    /// assert_eq!(5.0, magnitude);
    /// ```
    pub fn magnitude(&self) -> f32 {
        Vec2::ZERO.distance_to(*self)
    }

    ///Returns this vector with a magnitude of 1. 
    /// Used when only the direction of the Vector is important.
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(7.0, 8.0);
    /// let vec2 = Vec2::new(4.0, 4.0);
    /// //get the distance
    /// let distance = vec1.distance_to(vec2);
    /// 
    /// assert_eq!(5.0, distance);
    /// ```
    pub fn normalize(&self) -> Vec2 {
        let magnitude = self.magnitude();
        *self / Vec2::new(magnitude, magnitude)
    }
    
    ///Returns the distance from this vector to another.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(7.0, 8.0);
    /// let vec2 = Vec2::new(4.0, 4.0);
    /// //get the distance
    /// let distance = vec1.distance_to(vec2);
    /// 
    /// assert_eq!(5.0, distance);
    /// ```
    pub fn distance_to(&self, other: Vec2) -> f32 {
        let lx = other.x - self.x;
        let ly = other.y - self.y;

        (lx.powi(2) + ly.powi(2)).sqrt()
    } 

    ///Returns the squared distance from this vector to another.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(7.0, 8.0);
    /// let vec2 = Vec2::new(4.0, 4.0);
    /// //get the squared distance
    /// let distance = vec1.distance_to_squared(vec2);
    /// 
    /// assert_eq!(25.0, distance);
    /// ```
    pub fn distance_to_squared(&self, other: Vec2) -> f32 {
        let lx = other.x - self.x;
        let ly = other.y - self.y;

        lx.powi(2) + ly.powi(2)
    } 

    
}

//----------Operator overloading----------
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self {x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}


