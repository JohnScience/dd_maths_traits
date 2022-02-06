pub use min_max_traits::{Max, Min};

trait SumDigitsBase10 {
    fn sum_digits_base_10<Acc, Output>(&self) -> Output;
}
