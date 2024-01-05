use std::any::Any;

/// There must be some built in way to do this, right?
pub trait IntoAny {
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
