use core::{
    cmp::PartialOrd,
    fmt::Debug,
    mem::size_of,
    ops::{Add, Div, Mul, Sub},
};
use heapless::HistoryBuffer;

/// # Intent
/// Creates a Moving Average filter for integer values,
/// in a nostd context. The filter uses a minimal calculation
/// approach, and does not sum the entire buffer when finding
/// the average.
///
/// # Instantiating `MovingAverage`
///
/// The `MovingAverage` type is generic over three values:
///
/// * T - the data type being averaged
/// * TCALC - a larger data type for calculating the average
///   * Must fit the value `N * T::MAX`
/// * N - the depth of the average
///   * Must be non-zero
///
/// # Example
///
/// ```rust
/// use no_std_moving_average::MovingAverage;
///
/// let mut sut = MovingAverage::<u32, u64, 2>::new();
/// let first: u32 = 22;
/// let second: u32 = 44;
/// let third: u32 = 66;
/// let expected = (second + third) / 2;
/// let _ = sut.average(first);
/// let _ = sut.average(second);
/// let result = sut.average(third);
///
/// assert_eq!(expected, result);
/// ```
///
/// # Static and Allocation Asserts
///
/// A combination of compile-time and allocation time
/// assertions are used to ensure `MovingAverage` is
/// instantiated correctly. Once instantiated, there
/// are no known Panics when operating `MovingAverage`.
///
/// ## T and TCALC must be Integer/Unsigned types
///
/// ```compile_fail
/// use no_std_moving_average::MovingAverage;
/// let _sut = MovingAverage::<f32, u64, 2>::new();
/// ```
///
/// ```compile_fail
/// use no_std_moving_average::MovingAverage;
/// let _sut = MovingAverage::<u32, f64, 2>::new();
/// ```
///
/// ```compile_fail
/// use no_std_moving_average::MovingAverage;
/// let _sut = MovingAverage::<f32, f64, 2>::new();
/// ```
///
/// ## TCALC must be larger than T
///
/// ```compile_fail
/// use no_std_moving_average::MovingAverage;
/// let _sut = MovingAverage::<u32, u32, 1>::new();
/// ```
///
/// ## N must be non-zero
///
/// ```compile_fail
/// use no_std_moving_average::MovingAverage;
/// let _sut = MovingAverage::<u32, u64, 0>::new();
/// ```
///
/// ## N * `T::MAX` must fit in TCALC
///
/// ```should_panic
/// use no_std_moving_average::MovingAverage;
/// let _sut = MovingAverage::<u8, u16, 512>::new();
/// ```
///
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
    buffer: HistoryBuffer<T, N>,
}

/// # Panics
/// Panics if TCALC not larger than T, compile-time assert.
/// Panics if N is zero, compile-time assert.
/// : These panics should never occur due to compile-time assert checks.
/// Panics if unable to convert from usize to TCALC.
/// Panics if N * `T::MAX` won't fit in TCALC.
/// : These panics happen at allocation time, so should be found predictably.
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "no size_of return bigger than u32"
    )]
    fn default() -> Self {
        const {
            assert!(
                size_of::<TCALC>() > size_of::<T>(),
                "TCALC must be larger than T"
            );
            assert!(N > 0, "N must be non-zero");
        }
        assert!(
            (2_u128.pow((size_of::<T>() as u32) * 8) * u128::try_from(N).unwrap())
                <= 2_u128.pow((size_of::<TCALC>() as u32) * 8),
            "N * T.max() must fit in TCALC"
        );
        Self {
            num: TCALC::try_from(N).unwrap(),
            sum: None,
            buffer: HistoryBuffer::new(),
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

        TCALC::from(*self.buffer.first().expect("Buffer should be full"))
    }
}

#[expect(clippy::let_underscore_must_use, reason = "Desirable in tests")]
#[expect(clippy::let_underscore_untyped, reason = "Desirable in tests")]
#[expect(clippy::cast_possible_truncation, reason = "Desirable in tests")]
#[expect(clippy::cast_possible_wrap, reason = "Desirable in tests")]
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

    #[test]
    fn given_two_signed_item_moving_average_when_average_called_thrice_then_return_average_of_the_last_two_values()
     {
        let mut sut = MovingAverage::<i32, i64, 2>::new();
        let first: i32 = -22;
        let second: i32 = 44;
        let third: i32 = -66;
        let expected = (second + third) / 2_i32;
        let _ = sut.average(first);
        let _ = sut.average(second);
        assert_eq!(expected, sut.average(third));
    }

    #[test]
    fn given_large_item_moving_average_when_average_called_thrice_then_return_average_of_the_last_two_values()
     {
        const DEPTH: usize = 128;
        let mut sut = MovingAverage::<i32, i64, DEPTH>::new();
        let first: i32 = -22;
        let second: i32 = 44;
        let third: i32 = -66;
        let expected = (first + second + third + (((DEPTH as i32) - 3_i32) * first)) / DEPTH as i32;
        let _ = sut.average(first);
        let _ = sut.average(second);
        assert_eq!(expected, sut.average(third));
    }

    #[test]
    #[should_panic(expected = "N * T.max() must fit in TCALC")]
    fn confirm_n_times_t_max_fits_in_tcalc() {
        let _sut = MovingAverage::<u8, u16, 512>::new();
    }

    // fails at compile time, due to missing conversions
    // #[test]
    // #[should_panic(expected = "T must be an integer type")]
    // fn confirm_t_is_an_integer_type() {
    //     let _sut = MovingAverage::<f32, u64, 2>::new();
    // }

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
