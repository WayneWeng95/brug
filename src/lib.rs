#[cfg(unix)]
mod brug_allocator;

#[cfg(unix)]
pub use crate::brug_allocator::*;

#[cfg(test)]
mod tests {
    use crate::brug_allocator;
    static DATASIZE: i32 = 100_000_000;
    static REPEATS: i32 = 10;

    #[test]
    fn sequential() {
        brug_allocator::seq_test(REPEATS, DATASIZE);
    }
    #[test]
    fn multi_thread() {
        brug_allocator::multi_test(REPEATS, DATASIZE);
    }
    #[test]
    fn combined() {
        brug_allocator::combine_test(REPEATS, DATASIZE);
    }
}

//cargo test -- --nocapture --test sequential   //to run sequential test
//cargo test -- --nocapture    //for all tests