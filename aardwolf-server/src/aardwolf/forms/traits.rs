use failure::Fail;

pub trait Validate<T, E>
where
    E: Fail,
{
    fn validate(self) -> Result<T, E>;
}
