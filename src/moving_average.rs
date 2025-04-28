use core::ops::Add;

#[derive(Debug)]
pub struct MovingAverage;

impl MovingAverage {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn average<T: Sized + Add<T, Output = T> + PartialEq>(&self, input: T) -> T {
        input
    }
}

impl Default for MovingAverage {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::MovingAverage;

    #[test]
    fn given_new_moving_average_when_average_value_then_return_same_value() {
        let sut = MovingAverage::new();
        let expected: u32 = 44;
        assert_eq!(expected, sut.average(expected));
    }
}
