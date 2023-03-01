#[cfg(unix)]
mod brug_allocator;

#[cfg(unix)]
pub use crate::brug_allocator::*;

#[cfg(test)]
mod tests {
    use crate::brug_allocator;
    use std::time::{Instant};
    use std::thread;
    static DATASIZE: i32 = 100_000_000;
    static REPEATS: i32 = 10;

    fn measurements(datasize: i32) {
        let mut v = Vec::new();

        let start = Instant::now();

        for n in 0..datasize {
            v.push(n);
            // println!("{} get pushed", n);
        }

        let duration = start.elapsed();

        println!("Time measured is: {:?}", duration);
    }

    fn test_sequential(numbers: i32, datasize: i32) {
        for _n in 0..numbers {
            measurements(datasize);
        }
    }

    fn test_multithread(numbers: i32, datasize: i32) {
        let threads: Vec<_> = (0..numbers)
            .map(|_i| {
                thread::spawn(move || {
                    measurements(datasize);
                })
            })
            .collect();

        for handle in threads {
            handle.join().unwrap();
        }
    }

    fn seq_test(repeats: i32, datasize: i32) {
        println!(
            "Testing sequential with {} integer push and {} repetations",
            datasize, repeats
        );

        test_sequential(repeats, datasize);
    }

    fn multi_test(repeats: i32, datasize: i32) {
        println!(
            "Testing multi-thread with {} integer push and {} repetations",
            datasize, repeats
        );

        test_multithread(repeats, datasize);
    }

    fn combine_test(repeats: i32, datasize: i32) {
        println!(
            "Testing sequential with {} integer push and {} repetations",
            datasize, repeats
        );

        test_sequential(repeats, datasize);

        println!(
            "Testing multi-thread with {} integer push and {} repetations",
            datasize, repeats
        );

        test_multithread(repeats, datasize);
    }

    #[test]
    fn sequential() {
        unsafe {
            brug_allocator::BrugStruct::set_mode(0);
        }
        seq_test(REPEATS, DATASIZE);
    }
    #[test]
    fn multi_thread() {
        unsafe {
            brug_allocator::BrugStruct::set_mode(0);
        }
        multi_test(REPEATS, DATASIZE);
    }
    #[test]
    fn combined() {
        combine_test(REPEATS, DATASIZE);
    }
}

//cargo test -- --nocapture --test sequential   //to run sequential test
//cargo test -- --nocapture    //for all tests
