pub mod abelian;
pub mod symmetric;

pub trait Group<T: ?Sized> {
    fn op(&self, a: T, b: T) -> T;
    fn inv(&self, g: T) -> T;
}