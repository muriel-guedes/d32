use std::ops::{SubAssign, AddAssign, Sub, Mul};

#[derive(Copy, Clone)]
pub struct SmoothValue<T> {
    value: T,
    target: T,
    smoothness: T,
    speed: T
}
impl<T: Copy + SubAssign + AddAssign + Sub<Output = T> + Mul<Output = T>> SmoothValue<T> {
    pub fn new(value: T, speed: T, smoothness: T) -> Self {
        Self {
            value, speed, smoothness,
            target: value
        }
    }
    pub fn get(&mut self) -> T {
        self.target -= (self.target - self.value) * self.smoothness;
        self.target
    }
    pub fn change(&mut self, value: T) {
        self.value += value * self.speed
    }
}

pub struct SmoothValueBounded<T> {
    value: T,
    target: T,
    speed: T,
    smoothness: T,
    min: T,
    max: T
}
impl<T: Copy + SubAssign + AddAssign + PartialOrd + Sub<Output = T> + Mul<Output = T>> SmoothValueBounded<T> {
    pub fn new(value: T, speed: T, smoothness: T, min: T, max: T) -> Self {
        Self {
            value, speed, min, max, smoothness,
            target: value
        }
    }
    pub fn get(&mut self) -> T {
        self.target -= (self.target - self.value) * self.smoothness;
        self.target
    }
    pub fn change(&mut self, value: T) {
        self.value += value * self.speed;
        if self.value > self.max { self.value = self.max }
        if self.value < self.min { self.value = self.min }
    }
}