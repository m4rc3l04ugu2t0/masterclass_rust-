trait Compute {
    type Target;
    fn compute(&self, rhs: Self::Target) -> Self::Target;
}

pub fn execute() {}
