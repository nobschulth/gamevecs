mod vecs;
pub use crate::vecs::vec2::Vec2;


#[cfg(test)]
mod tests {
    use super::*;

    //---Vec2s----
    //Operators
    #[test]
    fn v2_add() {
        let vec1 = Vec2::new(5. , 15.5);
        let vec2 = Vec2::new(6. , 15.5);
        let res = Vec2::new(11., 31.);
        assert_eq!(res, vec1 + vec2);
    }
    #[test]
    fn v2_add_assign() {
        let mut vec1 = Vec2::new(5. , 15.5);
        let vec2 = Vec2::new(6. , 15.5);
        vec1 += vec2;
        let res = Vec2::new(11., 31.);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v2_sub() {
        let vec1 = Vec2::new(11., 31.);
        let vec2 = Vec2::new(6. , 15.5);
        let res = Vec2::new(5. , 15.5);
        assert_eq!(res, vec1 - vec2);
    }
    #[test]
    fn v2_sub_assign() {
        let mut vec1 = Vec2::new(11., 31.);
        let vec2 = Vec2::new(6. , 15.5);
        vec1 -= vec2;
        let res = Vec2::new(5. , 15.5);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v2_mul() {
        let vec1 = Vec2::new(5., 6.);
        let vec2 = Vec2::new(5., 10.);
        let res = Vec2::new(25., 60.);
        assert_eq!(res, vec1 * vec2);
    }
    #[test]
    fn v2_mul_assign() {
        let mut vec1 = Vec2::new(5., 6.);
        let vec2 = Vec2::new(5., 10.);
        vec1 *= vec2;
        let res = Vec2::new(25., 60.);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v2_div() {
        let vec1 = Vec2::new(25., 60.);
        let vec2 = Vec2::new(5., 10.);
        let res = Vec2::new(5., 6.);
        assert_eq!(res, vec1 / vec2);
    }
    #[test]
    fn v2_div_assign() {
        let mut vec1 = Vec2::new(25., 60.);
        let vec2 = Vec2::new(5., 10.);
        vec1 /= vec2;
        let res = Vec2::new(5., 6.);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v2_neg() {
        let vec = Vec2::new(69.5, -3.);
        let res = Vec2::new(-69.5, 3.);
        assert_eq!(res, -vec);
    }

    //Other functionality
    #[test]
    fn v2_normalize() {
        let vec = Vec2::new(-2., 0.);
        let res = Vec2::new(-1., 0.);
        assert_eq!(res, vec.normalize());
    }
    #[test]
    fn v2_magnitude() {
        let vec = Vec2::new(3., 4.);
        let res = 5.;
        assert_eq!(res, vec.magnitude());
    }
    #[test]
    fn v2_distance_to() {
        let vec1 = Vec2::new(7., 8.);
        let vec2 = Vec2::new(4., 4.);
        let res = 5.;
        assert_eq!(res, vec1.distance_to(vec2));
    }
    #[test]
    fn v2_distance_to_squared() {
        let vec1 = Vec2::new(7., 8.);
        let vec2 = Vec2::new(4., 4.);
        let res = 25.;
        assert_eq!(res, vec1.distance_to_squared(vec2));
    }
    
}
