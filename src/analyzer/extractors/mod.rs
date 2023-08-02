use self::visit::{Visitable, Visitor};

pub mod compound;
pub mod primitive;
pub mod visit;

pub trait Extractor<V, T>: Visitor
where
    V: Visitable,
    T: Visitable,
{
    fn extract(v: V) -> Vec<T>;
}
