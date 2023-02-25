#[cfg(unix)]
mod brug_allocator;

#[cfg(unix)]
pub use crate::brug_allocator::*;

#[cfg(test)]
mod tests {
    use crate::brug_allocator;

    #[test]
    fn sequential() {
        let repeats = 10;
        brug_allocator::seq_test(repeats);
    }
    #[test]
    fn multi_thread() {
        let repeats = 10;
        brug_allocator::multi_test(repeats);
    }
    #[test]
    fn combined() {
        use crate::brug_allocator;
        let repeats = 10;
        brug_allocator::combine_test(repeats);
    }
}
