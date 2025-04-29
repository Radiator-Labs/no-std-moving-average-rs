use core::{
    cmp::PartialOrd,
    fmt::Debug,
    mem::size_of,
    ops::{Add, Div, Mul, Sub},
};
use heapless::HistoryBuf;

pub struct MovingAverage<T, TCALC, const N: usize>
where
    T: Sized + PartialEq + TryFrom<TCALC, Error: Debug> + Clone + Copy,
    TCALC: Sized
        + Add<TCALC, Output = TCALC>
        + Sub<TCALC, Output = TCALC>
        + Div<Output = TCALC>
        + Mul<Output = TCALC>
        + PartialEq
        + PartialOrd
        + From<T>
        + TryFrom<usize, Error: Debug>
        + Clone
        + Copy,
{
    num: TCALC,
    sum: Option<TCALC>,
    buffer: HistoryBuf<T, N>,
}

/// # Panics
/// Panics if TCALC not larger than T, allocation-time assert.
/// Panics if unable to convert from usize to TCALC.
/// This panic should never occur due to allocation-time assert checks.
#[expect(clippy::unwrap_used, reason = "Made safe by compile-time asserts")]
impl<T, TCALC, const N: usize> Default for MovingAverage<T, TCALC, N>
where
    T: Sized + PartialEq + TryFrom<TCALC, Error: Debug> + Clone + Copy,
    TCALC: Sized
        + Add<TCALC, Output = TCALC>
        + Sub<TCALC, Output = TCALC>
        + Div<Output = TCALC>
        + Mul<Output = TCALC>
        + PartialEq
        + PartialOrd
        + From<T>
        + TryFrom<usize, Error: Debug>
        + Clone
        + Copy,
{
    fn default() -> Self {
        const {
            assert!(
                size_of::<TCALC>() > size_of::<T>(),
                "TCALC must be larger than T"
            );
            assert!(N > 0, "N must be non-zero");
        }
        Self {
            num: TCALC::try_from(N).unwrap(),
            sum: None,
            buffer: HistoryBuf::new(),
        }
    }
}

impl<T, TCALC, const N: usize> MovingAverage<T, TCALC, N>
where
    T: Sized + PartialEq + TryFrom<TCALC, Error: Debug> + Clone + Copy,
    TCALC: Sized
        + Add<TCALC, Output = TCALC>
        + Sub<TCALC, Output = TCALC>
        + Div<Output = TCALC>
        + Mul<Output = TCALC>
        + PartialEq
        + PartialOrd
        + From<T>
        + TryFrom<usize, Error: Debug>
        + Clone
        + Copy,
{
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Panics
    /// Panics if unable to convert from TCALC to T.
    /// This panic should never occur due to compile-time assert checks.
    #[must_use]
    pub fn average(&mut self, input: T) -> T {
        let new_value = TCALC::from(input);
        let prev_sum = self.get_or_init_and_get_sum(input);
        let remove = self.insert_new_value_pop_oldest_value(input);
        self.create_average(new_value, prev_sum, remove)
    }

    fn get_or_init_and_get_sum(&mut self, input: T) -> TCALC {
        let new_value = TCALC::from(input);
        if let Some(sum) = self.sum {
            sum
        } else {
            for _ in 0..N {
                self.buffer.write(input);
            }
            self.num * new_value
        }
    }

    fn insert_new_value_pop_oldest_value(&mut self, input: T) -> TCALC {
        let remove = self.get_remove_value();
        self.buffer.write(input);
        remove
    }

    #[expect(clippy::expect_used, reason = "Made safe by compile-time asserts")]
    fn create_average(&mut self, new_value: TCALC, prev_sum: TCALC, remove: TCALC) -> T {
        #[cfg(test)]
        assert!(prev_sum >= remove, "Remove must not be bigger than sum.");

        let new_sum = prev_sum + new_value - remove;
        self.sum = Some(new_sum);
        let average_as_tcalc = new_sum / self.num;
        T::try_from(average_as_tcalc).expect("Converting from TCALC to T should be safe")
    }

    #[expect(clippy::expect_used, reason = "Made safe by compile-time asserts")]
    fn get_remove_value(&self) -> TCALC {
        #[cfg(test)]
        assert!(
            self.buffer.len() == N,
            "Buffer len {} different than capacity {N}.",
            self.buffer.len()
        );

        TCALC::from(*self.buffer.oldest().expect("Buffer should be full"))
    }
}

#[expect(clippy::let_underscore_must_use, reason = "Desirable in tests")]
#[expect(clippy::let_underscore_untyped, reason = "Desirable in tests")]
#[cfg(test)]
mod tests {
    use super::MovingAverage;

    #[test]
    fn given_new_moving_average_when_average_value_then_return_same_value() {
        let mut sut = MovingAverage::<u32, u64, 1>::new();
        let expected: u32 = 44;
        assert_eq!(expected, sut.average(expected));
    }

    #[test]
    fn given_two_item_moving_average_when_average_twice_value_then_return_average_of_those_values()
    {
        let mut sut = MovingAverage::<u32, u64, 2>::new();
        let first: u32 = 22;
        let second: u32 = 44;
        let expected = (first + second) / 2;
        let _ = sut.average(first);
        assert_eq!(expected, sut.average(second));
    }

    #[test]
    fn given_two_item_moving_average_when_average_called_thrice_then_return_average_of_the_last_two_values()
     {
        let mut sut = MovingAverage::<u32, u64, 2>::new();
        let first: u32 = 22;
        let second: u32 = 44;
        let third: u32 = 66;
        let expected = (second + third) / 2;
        let _ = sut.average(first);
        let _ = sut.average(second);
        assert_eq!(expected, sut.average(third));
    }

    // checked at compile time
    // #[test]
    // #[should_panic(expected = "TCALC must be larger than T")]
    // fn confirm_tcalc_must_be_larger_than_t() {
    //     let _sut = MovingAverage::<u32, u32, 1>::new();
    // }

    // checked at compile time
    // #[test]
    // #[should_panic(expected = "N must be non-zero")]
    // fn confirm_n_must_be_non_zero() {
    //     let _sut = MovingAverage::<u32, u64, 0>::new();
    // }
}
