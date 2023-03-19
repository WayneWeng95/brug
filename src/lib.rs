#[cfg(unix)]
mod brug_allocator;

#[cfg(unix)]
pub use crate::brug_allocator::*;

#[macro_export] //The macro to insert the code block and allocator sign
macro_rules! set_allocator_mode {
    ( $mand_1:expr, $( $x:expr ),* ) => {
        {

            unsafe {
                brug_allocator::BrugStruct::set_mode($mand_1);
            }
            $(
                $x;
            )*
            unsafe {
                brug_allocator::BrugStruct::end_set();
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{brug_allocator, Allocatormode};
    use std::thread;
    use std::time::Instant;

    fn measurements(datasize: i64) {
        //Measurement using the vector push
        let mut v = Vec::new();

        let start = Instant::now();

        for n in 0..datasize {
            v.push(n);
        }

        let duration = start.elapsed();

        println!("Time measured is: {:?}", duration);
    }

    fn test_sequential(numbers: i64, datasize: i64) {
        //Sequential operations
        for _n in 0..numbers {
            measurements(datasize);
        }
    }

    fn test_multithread(numbers: i64, datasize: i64) {
        //Multi-thread operations
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

    fn seq_test(repeats: i64, datasize: i64, allocator: Allocatormode) {
        //Testing caller for sequential
        println!(
            "Testing sequential in plocicy {:?} with {} integer push and {} repetations",
            allocator, datasize, repeats
        );

        test_sequential(repeats, datasize);
    }

    fn multi_test(repeats: i64, datasize: i64, allocator: Allocatormode) {
        //Testing call for multi-thread
        println!(
            "Testing multi-thread in plocicy {:?} with {} integer push and {} repetations",
            allocator, datasize, repeats
        );

        test_multithread(repeats, datasize);
    }

    fn combine_test(repeats: i64, datasize: i64) {
        //Testing call for both
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

    use arrow::{array,record_batch};
    use std::sync;
    fn arrow_functional() {     //A simple arrow test to testify functionality
        println!("Arrow test");
        let col_1 = sync::Arc::new(array::Int32Array::from_iter([1, 2, 3])) as _;
        let col_2 = sync::Arc::new(array::Float32Array::from_iter([1., 6.3, 4.])) as _;

        let batch = record_batch::RecordBatch::try_from_iter([("col1", col_1), ("col_2", col_2)]).unwrap();
        println!("{:?}",batch);
    }

    static DATASIZE: i64 = 100_000_000;
    static REPEATS: i64 = 5;

    #[test]
    fn sequential() {
        // unsafe {
        //     brug_allocator::BRUG_TEMPLATE.mmap = (false, 0, 0);      //Changing of the template variable
        // }
        // let allocator = brug_allocator::Allocatormode::_SYS_;       //Create the flag
        let allocator = brug_allocator::Allocatormode::_BrugAutoOpt_; //Create the flag
                                                                      // let allocator = brug_allocator::Allocatormode::_JEMALLOC_; //Create the flag
        set_allocator_mode!(allocator, { seq_test(REPEATS, DATASIZE, allocator) });
        // set_allocator_mode!(allocator, { arrow_functional() });
        //Use the marco
    }
    #[test]
    fn multi_thread() {
        // let allocator = brug_allocator::Allocator::_JEMALLOC_;
        let allocator = brug_allocator::Allocatormode::_BrugTemplate_;
        set_allocator_mode!(allocator, multi_test(REPEATS, DATASIZE, allocator));
    }
    #[test]
    fn combined() {
        combine_test(REPEATS, DATASIZE);
    }
}

//cargo test -- --nocapture --test sequential   //to run sequential test
//cargo test -- --nocapture    //for all tests
