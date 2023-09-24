pub trait Pointer {
    type Ptr;
    fn as_ptr(&self) -> Self::Ptr;
}