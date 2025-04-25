mod has_kconfig;
pub use has_kconfig::HasKconfig;

pub trait ConditionError: std::error::Error {
    /// Whether it's okay to ignore this error if evaluating an Or.
    fn is_ignorable(&self) -> bool;
}

pub type DynConditionError = Box<dyn ConditionError + Send + Sync + 'static>;

pub trait Condition {
    type Err: ConditionError + Send + Sync + 'static;
    fn check(&self) -> Result<bool, Self::Err>;
}

pub trait DynCondition {
    fn check(&self) -> Result<bool, DynConditionError>;
}

impl<T> DynCondition for T
where
    T: Condition,
    T::Err: ConditionError + Send + Sync + 'static,
{
    fn check(&self) -> Result<bool, DynConditionError> {
        let result: Result<bool, T::Err> = self.check();
        result.map_err(|e| -> DynConditionError { Box::new(e) })
    }
}
