#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    deq1: VecDeque<T>,
    deq2: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        MinQueue {
            deq1: VecDeque::new(),
            deq2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        while !self.deq2.is_empty() {
            if *self.deq2.back().unwrap() > val {
                self.deq2.pop_back();
            } else {
                break;
            }
        }

        self.deq1.push_back(val.clone());
        self.deq2.push_back(val.clone());
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.deq1.is_empty() {
            return None;
        }
        let front = self.deq1.pop_front().unwrap();
        if front == *self.deq2.front().unwrap() {
            self.deq2.pop_front();
        }
        Some(front)
    }

    pub fn front(&self) -> Option<&T> {
        if self.deq1.is_empty() {
            return None;
        }
        self.deq1.front()
    }

    pub fn min(&self) -> Option<&T> {
        if self.deq2.is_empty() {
            return None;
        }
        self.deq2.front()
    }

    pub fn len(&self) -> usize {
        self.deq1.len()
    }

    pub fn is_empty(&self) -> bool {
        self.deq1.is_empty()
    }
}
