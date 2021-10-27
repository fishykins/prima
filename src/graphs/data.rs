pub trait GraphData<T> {
    fn data(&self) -> Option<&Box<T>>;
    fn data_mut(&mut self) -> Option<&mut Box<T>>;
}