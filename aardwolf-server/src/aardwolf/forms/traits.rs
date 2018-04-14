pub trait Validate<T> {
    fn validate(&self) -> Result<(), T>;
}
