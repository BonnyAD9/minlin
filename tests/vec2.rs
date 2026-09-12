use minlin::Vec2;

#[test]
pub fn test_convert() {
    let v = Vec2::new(1_u32, 1_u32);
    assert_eq!(v.cast_to::<f32>(), Vec2::new(1.0_f32, 1.));
    assert_eq!(v.convert_to::<f64>(), Vec2::new(1.0_f64, 1.));
}
