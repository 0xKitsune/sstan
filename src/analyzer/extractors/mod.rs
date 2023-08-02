use self::visit::Visitable;

pub mod compound;
pub mod primitive;
pub mod visit;

pub trait Extractor<V, T>
where
    V: Visitable,
    T: Visitable,
{
    fn extract(v: V) -> Vec<T>;
}
