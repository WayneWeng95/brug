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
    use std::time::Instant;

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

    use arrow::ipc::Bool;
    use arrow::{array, record_batch};
    use std::sync;
    fn arrow_functional() {
        //A simple arrow test to testify functionality
        println!("Arrow test");
        let col_1 = sync::Arc::new(array::Int32Array::from_iter([0; 100])) as _;
        let col_2 = sync::Arc::new(array::Int32Array::from_iter([0; 100])) as _;

        let batch =
            record_batch::RecordBatch::try_from_iter([("col1", col_1), ("col_2", col_2)]).unwrap();
        println!("{:?}", batch);
    }

    struct Product {
        name: String,
        id: i32,
        stock: i8,
        description: String,
    }

    #[derive(Clone, Copy)]
    struct Person<'a> {
        name:&'a String,
        telephone:&'a String,
        age: i8,
        married: bool,
        account_number: i32,
        address:&'a String,
        whishlist:&'a Vec<Product>,
    }

    use std::{collections, thread, vec};

    fn measurements(datasize: i64) {
        //Measurement using the vector push
        // let mut vec = vec::Vec::new();
        // let mut vec_deque = collections::VecDeque::new();
        // let mut list= collections::LinkedList::new();
        // let mut hasmap = collections::HashMap::new();
        // let mut btreemap = collections::BTreeMap::new();
        // let mut hashset = collections::HashSet::new();
        // let mut btreeset = collections::BTreeSet::new();
        let mut heap = collections::BinaryHeap::new();

        let product1 = Product {
            name: String::from("Prodcut1"),
            id: 1,
            stock: 50,
            description: String::from("Product1"),
        };

        let product2 = Product {
            name: String::from("Prodcut2"),
            id: 2,
            stock: 20,
            description: String::from("Product2"),
        };

        let product3 = Product {
            name: String::from("Prodcut2"),
            id: 3,
            stock: 100,
            description: String::from("Product2"),
        };

        let p1 = Person {
            name: &String::from("abc"),
            telephone: &String::from("1234567654321"),
            age: 35,
            married: true,
            account_number: 1,
            address: &String::from("123 One Street"),
            whishlist: &Vec::from([product1, product2]),
        };

        let p2 = Person {
            name: &String::from("efg"),
            telephone: &String::from("7654321123456"),
            age: 85,
            married: false,
            account_number: 2,
            address: &String::from("456 Four Street"),
            whishlist: &Vec::from([product3]),
        };

        let start = Instant::now();

        for n in 0..datasize {
            // if n % 2 == 0 {
            //     vec.push(p1);
            // } else {
            //     vec.push(p2);
            // }
           
            // vec.push("This is a test string 123321123321");

            // if n % 2 == 0 {
            //     vec_deque.push_back(p1);
            // } else {
            //     vec_deque.push_back(p2);
            // }

            // vec_deque.push_back(n);

            // if n % 2 == 0 {
            //     list.push_back(p1);
            // } else {
            //     list.push_back(p2);
            // }

            // list.push_back(n);
            // list.push_front(n);

                        // if n % 2 == 0 {
            //     hasmap.insert(n, p1);
            // } else {
            //     hasmap.insert(n, p2);
            // }
            // hasmap.insert(n + 1, n.to_string());


            // if n % 2 == 0 {
            //     btreemap.insert(n, p1);
            // } else {
            //     btreemap.insert(n, p2);
            // }

            // btreemap.insert(n + 1, n.to_string());

            // hashset.insert(n);
            
            // btreeset.insert(n);

            heap.push(n);
        }

        let duration = start.elapsed();

        // println!("Time measured is: {:?}", duration);
        println!("{:?}", duration);
    }

    static DATASIZE: i64 = 100;
    static REPEATS: i64 = 10;

    #[test]
    fn sequential() {
        // unsafe {
        //     brug_allocator::BRUG_TEMPLATE.mmap = (false, 0, 0);      //Changing of the template variable
        // }
        // let allocator = brug_allocator::Allocatormode::_SYS_;       //Create the flag

        // unsafe {
        //     brug_allocator::BrugStruct::enable_monitor(); //This cause stack overflow
        // }

        // let allocator = brug_allocator::Allocatormode::_SYS_; //Create the flag
        // let allocator = brug_allocator::Allocatormode::_JEMALLOC_; //Create the flag
        // let allocator = brug_allocator::Allocatormode::_SYS_;
        let allocator = brug_allocator::Allocatormode::_JEMALLOC_;
        // let allocator = brug_allocator::Allocatormode::_MIMALLOC_;
        // let allocator = brug_allocator::Allocatormode::_MMAP_;
        // let allocator = brug_allocator::Allocatormode::_BrugTemplate_;
        // let allocator = brug_allocator::Allocatormode::_BrugAutoOpt_;

        set_allocator_mode!(allocator, { seq_test(REPEATS, DATASIZE, allocator) });

        // unsafe {
        //     brug_allocator::BrugStruct::monitor_print();
        //     brug_allocator::BrugStruct::disable_monitor();
        // }

        // set_allocator_mode!(allocator, { arrow_functional() });
        //Use the marco
    }

    #[test]
    fn multi_thread() {
        // let allocator = brug_allocator::Allocator::_JEMALLOC_;
        // unsafe {
        //     brug_allocator::BrugStruct::enable_monitor(); //This cause stack overflow
        //     brug_allocator::BrugStruct::change_monitor_limiter(4096);
        // }
        // let allocator = brug_allocator::Allocatormode::_SYS_;
        let allocator = brug_allocator::Allocatormode::_JEMALLOC_;
        // let allocator = brug_allocator::Allocatormode::_MIMALLOC_;
        // let allocator = brug_allocator::Allocatormode::_MMAP_;
        // let allocator = brug_allocator::Allocatormode::_BrugTemplate_;
        // let allocator = brug_allocator::Allocatormode::_BrugAutoOpt_;

        set_allocator_mode!(allocator, multi_test(REPEATS, DATASIZE, allocator));

        // unsafe {
        //     brug_allocator::BrugStruct::monitor_print();
        //     brug_allocator::BrugStruct::disable_monitor();
        // }
    }
    #[test]
    fn combined() {
        combine_test(REPEATS, DATASIZE);
    }

    #[test]
    fn arrow() {
        // let allocator = brug_allocator::Allocatormode::_SYS_;
        // set_allocator_mode!(allocator, { arrow_functional() });
        arrow_functional();
    }

    #[test]
    fn glass_bench() {}
}

//cargo test -- --nocapture --test sequential   //to run sequential test
//cargo test -- --nocapture    //for all tests
