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
        self.value -= (self.value - self.target) * self.smoothness;
        self.value
    }
    pub fn change(&mut self, value: T) {
        self.target += value * self.speed
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
        self.value -= (self.value - self.target) * self.smoothness;
        self.value
    }
    pub fn change(&mut self, value: T) {
        self.target += value * self.speed;
        if self.target > self.max { self.target = self.max }
        if self.target < self.min { self.target = self.min }
    }
}