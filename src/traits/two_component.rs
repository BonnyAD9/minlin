pub trait TwoComponent {
    type Val;

    /// Creates two component value from its components.
    fn from_components(c1: Self::Val, c2: Self::Val) -> Self;
    fn to_components(self) -> (Self::Val, Self::Val);

    /// Gets the first of the two components.
    fn comp1(&self) -> &Self::Val;
    /// Gets the second of the two components.
    fn comp2(&self) -> &Self::Val;
}
