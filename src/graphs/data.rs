/// A Graph item that can hold generic data will impliment this.
pub trait GraphData<T> {
    /// Returns a borrowed refference to the data.
    fn data(&self) -> Option<&Box<T>>;
    /// Returns a borrowed mutable refference to the data.
    fn data_mut(&mut self) -> Option<&mut Box<T>>;
}