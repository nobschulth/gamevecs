use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};


///3D f32 Vector 
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    ///Short for Vec2::new(0.0, -1.0, 0.0)
    pub const DOWN: Vec3 = Vec3 {x: 0., y: -1., z: 0.};
    ///Short for Vec2::new(0.0, 1.0, 0.0)
    pub const UP: Vec3 = Vec3 {x: 0., y: 1., z: 0. };
    ///Short for Vec2::new(1.0, 0.0, 0.0)
    pub const RIGHT: Vec3 = Vec3 {x: 1., y: 0., z: 0. };
    ///Short for Vec2::new(-1.0, 0.0, 0.0)
    pub const LEFT: Vec3 = Vec3 {x: -1., y: 0., z: 0. };
     ///Short for Vec2::new(0.0, 0.0, 1.0)
     pub const FORWARD: Vec3 = Vec3 {x: 0., y: 0., z: 1. };
     ///Short for Vec2::new(0.0, 0.0, -1.0)
     pub const BACK: Vec3 = Vec3 {x: 0., y: 0., z: -1. };
    ///Short for Vec2::new(1.0, 1.0, 1.0)
    pub const ONE: Vec3 = Vec3 {x: 1., y: 1., z: 1. };
    ///Short for Vec2::new(0.0, 0.0, 0.0)
    pub const ZERO: Vec3 = Vec3 {x: 0., y: 0., z: 0. };


    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    
    //----------Other functionality----------

    ///Sets the values of this vector.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let mut vec = Vec3::new(3.0, 4.0, 6.0);
    /// //set the values
    /// vec.set(4.0, 5.0, 5.0);
    ///
    /// assert_eq!(Vec3::new(4.0, 5.0, 5.0), vec);
    /// ```
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    
    ///Returns the length (distance to (0|0|0)) of this vector. 
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec = Vec3::new(3.0, 4.0, 12.0);
    /// //get the magnitude
    /// let magnitude = vec.magnitude();
    ///
    /// assert_eq!(13.0, magnitude);
    /// ```
    pub fn magnitude(&self) -> f32 {
        Vec3::ZERO.distance_to(*self)
    }

    ///Returns the squared length of this vector.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec = Vec3::new(3.0, 4.0, 12.0);
    /// //get the magnitude
    /// let magnitude = vec.magnitude_squared();
    ///
    /// assert_eq!(169.0, magnitude);
    /// ```
    pub fn magnitude_squared(&self) -> f32 {
        Vec3::ZERO.distance_to_squared(*self)
    }

    ///Returns true if this vector approximately equals another one.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec = Vec3::new(3.000001, 4.000001, 5.000001);
    /// let other_vec = Vec3::new(3.0, 4.0, 5.0);
    /// 
    /// let equals = vec.equals(other_vec, 1e-5);
    ///
    /// assert_eq!(true, equals);
    /// ```
    pub fn equals(&self, other: Vec3, epsilon: f32) -> bool {
        (self.x - other.x).abs() < epsilon && 
        (self.y - other.y).abs() < epsilon && 
        (self.z - other.z).abs() < epsilon 
    }

    ///Returns this vector with a magnitude of 1. 
    /// Used when only the direction of the Vector is important.
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(0.0, 0.0, 5.0);
    /// 
    /// //normalize
    /// let normalized = vec1.normalized();
    /// 
    /// assert_eq!(Vec3::new(0.0, 0.0, 1.0), normalized);
    /// ```
    pub fn normalized(&self) -> Vec3 {
        let magnitude = self.magnitude();
        *self / Vec3::new(magnitude, magnitude, magnitude)
    }
    
    ///Returns the distance from this vector to another.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(10.0, 20.0, 0.0);
    /// let vec2 = Vec3::new(5.0, 10.0, 10.0);
    /// //get the distance
    /// let distance = vec1.distance_to(vec2);
    /// 
    /// assert_eq!(15.0, distance);
    /// ```
    pub fn distance_to(&self, other: Vec3) -> f32 {
        self.distance_to_squared(other).sqrt()
    } 

    ///Returns the squared distance from this vector to another.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(10.0, 20.0, 0.0);
    /// let vec2 = Vec3::new(5.0, 10.0, 10.0);
    /// //get the distance
    /// let distance: f32 = vec1.distance_to_squared(vec2);
    /// 
    /// assert_eq!(225.0, distance);
    /// ```
    pub fn distance_to_squared(&self, other: Vec3) -> f32 {
        let lx = other.x - self.x;
        let ly = other.y - self.y;
        let lz = other.z - self.z;

        lx.powi(2) + ly.powi(2) + lz.powi(2)
    } 

    
    ///Returns the dot product of this vector and another one.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(1.0, 2.0, 3.0);
    /// let vec2 = Vec3::new(4.0, 5.0, 6.0);
    /// //get the dot product
    /// let dot_product = vec1.dot(vec2);
    /// 
    /// assert_eq!(32.0, dot_product);
    /// ```
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    ///Returns the cross product of this vector and another one.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(2.0, 3.0, 4.0);
    /// let vec2 = Vec3::new(5.0, 6.0, 7.0);
    /// //get the cross product
    /// let cross_product = vec1.cross(vec2);
    /// 
    /// assert_eq!(Vec3::new(-3.0, 6.0, -3.0), cross_product);
    /// ```
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x)
    }

    ///Returns the angle between this vector and another one.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(1.0, 1.0, 1.0);
    /// let vec2 = Vec3::new(-1.0, -1.0, -1.0);
    /// //get the dot product
    /// let angle = vec1.angle_between(vec2);
    /// 
    /// assert_eq!(180.0, angle.to_degrees());
    /// ```
    pub fn angle_between(&self, other: Vec3) -> f32 {
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
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(1.0, 2.0, 4.0);
    /// let other = Vec3::new(3.0, 6.0, 8.0);
    /// //lerp
    /// let new_vec = vec1.lerp(other, 0.5);
    /// 
    /// assert_eq!(Vec3::new(2.0, 4.0, 6.0), new_vec);
    /// ```
    pub fn lerp(&self, other: Vec3, t: f32) -> Vec3 {
        Vec3 {
            x: self.x * (1.0 - t) + other.x * t,
            y: self.y * (1.0 - t) + other.y * t,
            z: self.z * (1.0 - t) + other.z * t,
        }
    }


    ///Projects a vector onto another one and returns the result as a new Vec3.
    /// 
    /// # Examples
    /// ```rust
    /// use gamevecs::Vec3;
    /// let vec1 = Vec3::new(3.0, 4.0, 5.0);
    /// let vec2 = Vec3::new(1.0, 0.0, 0.0);
    /// 
    /// //project
    /// let projection = vec1.project(vec2);
    /// 
    /// assert_eq!(Vec3::new(3.0, 0.0, 0.0), projection);
    /// ```
    pub fn project(&self, onto: Vec3) -> Vec3 {
        let onto_normalized = onto.normalized();
        let scalar = self.dot(onto_normalized);
        Vec3::new(onto_normalized.x * scalar, onto_normalized.y * scalar, onto_normalized.z * scalar)
    }

    
}

//----------Operator overloading----------

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

