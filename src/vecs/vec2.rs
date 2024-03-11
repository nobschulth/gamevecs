
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

    ///Sets the values of this vector.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let mut vec = Vec2::new(3.0, 4.0);
    /// //set the values
    /// vec.set(4.0, 5.0);
    ///
    /// assert_eq!(Vec2::new(4.0, 5.0), vec);
    /// ```
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

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

    ///Returns the squared length of this vector.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec = Vec2::new(3.0, 4.0);
    /// //get the magnitude
    /// let magnitude = vec.magnitude_squared();
    ///
    /// assert_eq!(25.0, magnitude);
    /// ```
    pub fn magnitude_squared(&self) -> f32 {
        Vec2::ZERO.distance_to_squared(*self)
    }

    ///Returns true if this vector approximately equals another one.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec = Vec2::new(3.000001, 4.000001);
    /// let other_vec = Vec2::new(3.0, 4.0);
    /// 
    /// let equal = vec.equals(other_vec, 1e-5);
    ///
    /// assert_eq!(true, equal);
    /// ```
    pub fn equals(&self, other: Vec2, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon && (self.y - other.y).abs() < epsilon
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


    ///Returns the dot product of this vector and another one.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(3.0, 4.0);
    /// let vec2 = Vec2::new(-2.0, 5.0);
    /// //get the dot product
    /// let dot_product = vec1.dot(vec2);
    /// 
    /// assert_eq!(14.0, dot_product);
    /// ```
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    ///Returns the angle between this vector and another one.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(1.0, 1.0);
    /// let vec2 = Vec2::new(-1.0, -1.0);
    /// //get the dot product
    /// let angle = vec1.angle_between(vec2);
    /// 
    /// assert_eq!(180.0, angle.to_degrees());
    /// ```
    pub fn angle_between(&self, other: Vec2) -> f32 {
        let dot = self.dot(other);
        let magnitude_product = self.magnitude() * other.magnitude();

        //avoid division by 0
        if magnitude_product == 0.0 {
            return 0.0;
        }
        
        let cos_theta = dot / magnitude_product;
        
        cos_theta.min(1.0).max(-1.0).acos()

    }
    
    ///Returns the linear interpolation by t between this and another vector. T should be between 0 and 1.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(1.0, 2.0);
    /// let other = Vec2::new(3.0, 4.0);
    /// //lerp
    /// let new_vec = vec1.lerp(other, 0.5);
    /// 
    /// assert_eq!(Vec2::new(2.0, 3.0), new_vec);
    /// ```
    pub fn lerp(&self, other: Vec2, t: f32) -> Vec2 {
        Vec2 {
            x: self.x * (1.0 - t) + other.x * t,
            y: self.y * (1.0 - t) + other.y * t,
        }
    }

    ///Adds the length in direction of the angle in radians to the vector and returns the result as a new Vec2.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(3.0, 4.0);
    /// 
    /// //add length
    /// let new_vec = vec1.add_length(53.13f32.to_radians(), 5.0);
    /// 
    /// //using the approximate equals function due to floating point inaccuracity
    /// assert!(new_vec.equals(Vec2::new(6.0, 8.0), 1e-4));
    /// ```
    pub fn add_length(&self, angle: f32, length: f32) -> Vec2 {
        Vec2 {
            x: self.x + angle.cos() * length,
            y: self.y + angle.sin() * length,
        }
    }

    ///Projects a vector onto another and returns the result as a new Vec2.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(3.0, 4.0);
    /// let vec2 = Vec2::new(1.0, 0.0);
    /// 
    /// //project
    /// let projection = vec1.project(vec2);
    /// 
    /// assert_eq!(Vec2::new(3.0, 0.0), projection);
    /// ```
    pub fn project(&self, onto: Vec2) -> Vec2 {
        let onto_normalized = onto.normalize();
        let scalar = self.dot(onto_normalized);
        Vec2 { x: onto_normalized.x * scalar, y: onto_normalized.y * scalar }
    }

    ///Rotates a vector counterclockwise and returns the result as a new Vec2.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec2;
    /// let vec1 = Vec2::new(1.0, 0.0);
    /// 
    /// //rotate
    /// let rotated_vec = vec1.rotate(90.0f32.to_radians());
    /// 
    /// //using the approximate equals function due to floating point inaccuracity
    /// assert!(rotated_vec.equals(Vec2::new(0.0, 1.0), 1e-4));
    /// ```
    pub fn rotate(&self, angle: f32) -> Vec2 {
        let mut new_vec = Vec2::new(0.0, 0.0);
        new_vec.x = self.x * angle.cos() - self.y * angle.sin();
        new_vec.y = self.x * angle.sin() + self.y * angle.cos();

        new_vec
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


