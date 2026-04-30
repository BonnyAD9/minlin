/// Represents types that can represent the value of negative infinity.
pub trait NegInfinity {
    /// Negative infinity of the given type.
    const NEG_INFINITY: Self;
}

impl NegInfinity for f32 {
    const NEG_INFINITY: Self = f32::NEG_INFINITY;
}

impl NegInfinity for f64 {
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
}