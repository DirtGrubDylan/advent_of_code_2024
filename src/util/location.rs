pub trait Location<RHS = Self> {
    type ValueOutput;

    fn manhattan_distance_to(&self, other: &RHS) -> Self::ValueOutput;
}
