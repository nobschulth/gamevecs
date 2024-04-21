mod vecs;
pub use crate::vecs::vec2::Vec2;
pub use crate::vecs::vec3::Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    //---Vec2----
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


    //---Vec3----
    #[test]
    fn v3_add() {
        let vec1 = Vec3::new(5. , 15.5, 6.);
        let vec2 = Vec3::new(6. , 15.5, 6.);
        let res = Vec3::new(11., 31., 12.);
        assert_eq!(res, vec1 + vec2);
    }
    #[test]
    fn v3_add_assign() {
        let mut vec1 = Vec3::new(5. , 15.5, 6.);
        let vec2 = Vec3::new(6. , 15.5, 6.);
        vec1 += vec2;
        let res = Vec3::new(11., 31., 12.);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v3_sub() {
        let vec1 = Vec3::new(11., 31., 10.);
        let vec2 = Vec3::new(6. , 15.5, 5.);
        let res = Vec3::new(5. , 15.5, 5.);
        assert_eq!(res, vec1 - vec2);
    }
    #[test]
    fn v3_sub_assign() {
        let mut vec1 = Vec3::new(11., 31., 10.);
        let vec2 = Vec3::new(6. , 15.5, 5.);
        vec1 -= vec2;
        let res = Vec3::new(5. , 15.5, 5.);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v3_mul() {
        let vec1 = Vec3::new(5., 6., 2.);
        let vec2 = Vec3::new(5., 10., 2.);
        let res = Vec3::new(25., 60., 4.);
        assert_eq!(res, vec1 * vec2);
    }
    #[test]
    fn v3_mul_assign() {
        let mut vec1 = Vec3::new(5., 6., 2.);
        let vec2 = Vec3::new(5., 10., 2.);
        vec1 *= vec2;
        let res = Vec3::new(25., 60., 4.);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v3_div() {
        let vec1 = Vec3::new(25., 60., 4.);
        let vec2 = Vec3::new(5., 10., 2.);
        let res = Vec3::new(5., 6., 2.);
        assert_eq!(res, vec1 / vec2);
    }
    #[test]
    fn v3_div_assign() {
        let mut vec1 = Vec3::new(25., 60., 4.);
        let vec2 = Vec3::new(5., 10., 2.);
        vec1 /= vec2;
        let res = Vec3::new(5., 6., 2.);
        assert_eq!(res, vec1);
    }
    #[test]
    fn v3_neg() {
        let vec = Vec3::new(69.5, -3., -4.);
        let res = Vec3::new(-69.5, 3., 4.);
        assert_eq!(res, -vec);
    }
    
}
