use std::collections::VecDeque;

pub struct RollingWindow<T> {
    window: VecDeque<T>,
    max_size: usize
}

impl<T> RollingWindow<T> {
    pub fn new(max_size: usize) -> Self {
        RollingWindow {
            window: VecDeque::with_capacity(max_size),
            max_size
        }
    }

    pub fn add(&mut self, value: T) {
        if self.window.len() == self.max_size {
            self.window.pop_front();
        }
        self.window.push_back(value)
    }

   pub fn as_slice(&self) -> &[T] {
        &self.window.as_slices().0
   }

    pub fn get(&self, n: usize) -> Option<&T> {
        self.window.get(n)
    }

}