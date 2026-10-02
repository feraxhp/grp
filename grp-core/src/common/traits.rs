

pub trait Convert {
    type Local;
    fn convert(&self) -> Self::Local;
}
