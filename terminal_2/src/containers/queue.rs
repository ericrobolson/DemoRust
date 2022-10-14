/// A queue container.
#[derive(Debug, Clone, PartialEq)]
pub struct Queue<T> {
    items: Vec<T>,
}
impl<T> Queue<T> {
    /// Creates a new queue
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    /// Pushes an item off of the queue.
    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    /// Dequeues an item off the queue.
    pub fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}
