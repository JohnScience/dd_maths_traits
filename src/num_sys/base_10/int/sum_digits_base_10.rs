pub use min_max_traits::{Min, Max};

trait SumDigitsBase10 {
    fn sum_digits_base_10<Acc,Output>(&self) -> Output;
}
