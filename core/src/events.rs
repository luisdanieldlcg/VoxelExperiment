pub struct Events<T> {
    pub events: Vec<T>,
}

impl<T> Events<T> {
    pub fn send(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.events.pop()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}
