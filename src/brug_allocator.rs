// extern crate tcmalloc;
use jemallocator::Jemalloc;
use mimalloc::MiMalloc;
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeMap;
use std::sync::atomic::AtomicU8;
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use tcmalloc;
// use std::cell::RefCell;
// use std::cell::Cell;
use std::os::raw::c_void;
use std::ptr;
use std::thread;

struct BrugAllocator;

#[derive(Debug, Clone, Copy)]
enum Allocator {
    _SYS_,
    //  _TCMALLOC_,
    _JEMALLOC_,
    _MIMALLOC_,
    _MMAP_,
}

static _CURRENT_: Allocator = Allocator::_SYS_;
static PTE_PAGE_SIZE: usize = 4096;
static PMD_PAGE_SIZE: usize = 2097152;
static PUD_PAGE_SIZE: usize = 1073741824;

pub struct BrugStruct {
    mapping: Mutex<BTreeMap<usize, Allocator>>,
    // total_size: u128,
    // ptr:AtomicPtr<u8>,
    mode: AtomicU8,
    records: Mutex<[[Duration; 4]; 4]>,
}

unsafe impl Sync for BrugStruct {}

#[allow(dead_code)]
static mut BRUG: BrugStruct = BrugStruct {
    //could be the problem here?
    // ptr:AtomicPtr::new(&mut 0),
    mapping: Mutex::new(BTreeMap::new()), //A tree to hold the allocator applied for this particular memory
    mode: AtomicU8::new(0),               //Indicating the Brug current mode
    records: Mutex::new([[Duration::new(0, 0); 4]; 4]), // A 2-d array for holding the records, [size][allocator]
};

#[allow(dead_code)]
impl BrugStruct {
    unsafe fn input(&mut self, _address: usize, allocator: Allocator) {
        //record the allocator mode
        self.mapping.lock().unwrap(); //change to try_lock()
        let _tree = self.mapping.get_mut().unwrap();
        _tree.insert(_address, allocator); //This insert cause the segamentation fault
    }
    unsafe fn suggest(&mut self, ptr: *mut u8, allocator: Allocator) {
        //change the allocator with preference in next reallocation
        self.mapping.lock().unwrap();
        let _tree = self.mapping.get_mut().unwrap();
        _tree.insert(ptr.clone() as usize, allocator); //Insert the value of the PTR
    }
    unsafe fn remove(&mut self, ptr: *mut u8) {
        //remove the entry when deallocate
        self.mapping.lock().unwrap();
        let _tree = self.mapping.get_mut().unwrap();
        let _ptr = ptr.clone() as usize;
        _tree.remove(&_ptr);
    }

    fn size_match(size: usize) -> usize {
        // size identifier for 5-level page table 0 -> 4KB -> 2MB -> 1GB -> larger
        if PTE_PAGE_SIZE < size || size <= PMD_PAGE_SIZE {
            return 1;
        } else if PMD_PAGE_SIZE < size || size <= PUD_PAGE_SIZE {
            return 2;
        } else if PUD_PAGE_SIZE < size {
            return 3;
        } else {
            return 4;
        };
    }

    unsafe fn record(&mut self, size: usize, time: Duration, allocator: Allocator) {
        let _size_type = Self::size_match(size);
        let _allocator_type: usize = match allocator {
            Allocator::_SYS_ => 1,
            Allocator::_JEMALLOC_ => 2,
            Allocator::_MIMALLOC_ => 3,
            Allocator::_MMAP_ => 4,
        };
        let record_table = self.records.get_mut().unwrap();
        record_table[_size_type][_allocator_type] = time;
    }

    // fn position_max_copy<T: Ord + Copy>(slice: &[T]) -> Option<usize> {
    //     slice.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx)
    // }

    fn position_max<T: Ord>(slice: &[T]) -> Option<usize> {
        slice
            .iter()
            .enumerate()
            .max_by(|(_, value0), (_, value1)| value0.cmp(value1))
            .map(|(idx, _)| idx)
    }

    unsafe fn optimization_mode(&mut self, size: usize) -> Allocator {
        let size_type = Self::size_match(size);
        let record_table = self.records.get_mut().unwrap();
        let allocator_type = Self::position_max(&record_table[size_type]).unwrap();
        let best_allocator = match allocator_type {
            1 => Allocator::_SYS_,
            2 => Allocator::_JEMALLOC_,
            3 => Allocator::_MIMALLOC_,
            4 => Allocator::_MMAP_,
            _ => Allocator::_SYS_, // in case of error, fall back to the system allocator
        };
        best_allocator
    } //a function to adjust the allocator according to the data collected
      //check the number and see which one cloud work better
}

#[global_allocator]
static GLOBAL: BrugAllocator = BrugAllocator;

unsafe impl GlobalAlloc for BrugAllocator {
    // #[inline]        //inline seems downgrade the performance
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret: *mut u8;
        let _start = Instant::now();

        match _CURRENT_ {
            Allocator::_SYS_ => ret = System.alloc(layout),
            // Allocator::_TCMALLOC_ => {
            //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8
            // }
            Allocator::_MIMALLOC_ => ret = MiMalloc.alloc(layout),
            Allocator::_JEMALLOC_ => ret = Jemalloc.alloc(layout),
            Allocator::_MMAP_ => {
                //size alignments
                // let size:usize;          //Alignments, current disable
                // match layout.size() >= HUGE_PAGE_SIZE{
                //     true => size = (layout.size()+HUGE_PAGE_SIZE)/HUGE_PAGE_SIZE*HUGE_PAGE_SIZE,
                //     false => size = (layout.size()+PAGE_SIZE)/PAGE_SIZE*PAGE_SIZE,
                // };

                const ADDR: *mut c_void = ptr::null_mut::<c_void>();
                ret = libc::mmap(
                    ADDR,
                    layout.size(),
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                    -1,
                    0,
                ) as *mut u8;

                match libc::madvise(
                    //Tweaking this madvise part
                    ret as *mut c_void,
                    layout.size(),
                    libc::MADV_WILLNEED | libc::MADV_DONTFORK | libc::MADV_HUGEPAGE,
                ) {
                    -1 => panic!("madvise_error"),
                    _ => (),
                }
            }
        }

        if layout.size() > PTE_PAGE_SIZE {
            //We record this object usage
            let _duration = _start.elapsed();
            BRUG.input(ret.clone() as usize, _CURRENT_);
            BRUG.record(layout.size(), _duration, _CURRENT_);
        }

        if ret.is_null() {
            panic!("Allocate_error");
        }

        ret
    }

    // #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        BRUG.remove(ptr);
        match _CURRENT_ {
            Allocator::_SYS_ => System.dealloc(ptr, layout),
            // Allocator::_TCMALLOC_ => tcmalloc::tc_free(ptr as *mut c_void),
            Allocator::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
            Allocator::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
            Allocator::_MMAP_ => {
                let addr = ptr as *mut c_void;
                libc::munmap(addr, layout.size());
            }
        }
    }

    // unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { ... } //calloc
    // #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ret: *mut u8;
        match _CURRENT_ {
            Allocator::_SYS_ => ret = System.realloc(ptr, layout, new_size),
            // Allocator::_TCMALLOC_ => {
            //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8;
            //     std::ptr::copy_nonoverlapping(ptr, ret, layout.size());
            // }
            Allocator::_MIMALLOC_ => ret = MiMalloc.realloc(ptr, layout, new_size),
            Allocator::_JEMALLOC_ => ret = Jemalloc.realloc(ptr, layout, new_size),
            Allocator::_MMAP_ => {
                let old_address = ptr as *mut c_void;

                // let size;
                // match layout.size() >= HUGE_PAGE_SIZE{
                //     true => size = (layout.size()+HUGE_PAGE_SIZE)/HUGE_PAGE_SIZE*HUGE_PAGE_SIZE,
                //     false => size = (layout.size()+PAGE_SIZE)/PAGE_SIZE*PAGE_SIZE,
                // };

                ret = libc::mremap(old_address, layout.size(), new_size, libc::MREMAP_MAYMOVE)
                    as *mut u8
            }
        }

        // println!("Realloc:{}",layout.size());

        if ret.is_null() {
            panic!("Reallocae_error");
        }

        ret
    }
}

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

pub fn seq_test(repeats: i32, datasize: i32) {
    println!(
        "Testing {:?} sequential with {} integer push and {} repetations",
        _CURRENT_, datasize, repeats
    );

    test_sequential(repeats, datasize);
}

pub fn multi_test(repeats: i32, datasize: i32) {
    println!(
        "Testing {:?} multi-thread with {} integer push and {} repetations",
        _CURRENT_, datasize, repeats
    );

    test_multithread(repeats, datasize);
}

pub fn combine_test(repeats: i32, datasize: i32) {
    println!(
        "Testing {:?} sequential with {} integer push and {} repetations",
        _CURRENT_, datasize, repeats
    );

    test_sequential(repeats, datasize);

    println!(
        "Testing {:?} multi-thread with {} integer push and {} repetations",
        _CURRENT_, datasize, repeats
    );

    test_multithread(repeats, datasize);
}

// fn main() {
//     let repeats = 10;

//     println!(
//         "Testing {:?} sequential with {} integer push and {} repetations",
//         _CURRENT_, DATASIZE, repeats
//     );

//     test_sequential(repeats);

//     // println!("Testing {:?} multi-thread with {} integer push and {} repetations",_CURRENT_,DATASIZE,repeats);

//     // test_multithread(repeats);
// }
