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

static _CURRENT_: Allocator = Allocator::_JEMALLOC_;
static DATASIZE: i32 = 100_000_000;
// static PAGE_SIZE : usize = 4096;
// static HUGE_PAGE_SIZE : usize = 16777216;

pub struct BrugStruct {
    mapping: Mutex<BTreeMap<i32, Allocator>>,
    // total_size: u128,
    // ptr:AtomicPtr<u8>,
    mode: AtomicU8,
    records: Mutex<[[Duration; 5];4]>, 
}

static mut BRUG: BrugStruct = BrugStruct {
    //could be the problem here?
    // ptr:AtomicPtr::new(&mut 0),
    mapping: Mutex::new(BTreeMap::new()), //A tree to hold the allocator applied for this particular memory
    mode: AtomicU8::new(0),               //Indicating the Brug current mode
    records: Mutex::new([[Duration::new(0, 0); 5]; 4]), // A tree to hold results for different size allocations
};

unsafe impl Sync for BrugStruct {}

impl BrugStruct {
    unsafe fn input(&mut self, allocator: Allocator) {
        //record the allocator mode
        self.mapping.lock().unwrap();
        let tree = self.mapping.get_mut().unwrap();
        tree.insert(1, allocator); //This insert cause the segamentation fault
    }
    unsafe fn suggest(&mut self, ptr: *mut u8, allocator: Allocator) {
        //change the allocator in next reallocation
        self.mapping.lock().unwrap();
        let tree = self.mapping.get_mut().unwrap();
        //tree.replace
    }
    unsafe fn remove(&mut self, ptr: i32) {
        //remove the entry when deallocate
        self.mapping.lock().unwrap();
        let tree = self.mapping.get_mut().unwrap();
        tree.remove(&ptr);
    }
    unsafe fn record() {

        // recording the speed for 5-level page table 0 -> 4KB -> 2MB -> 1GB -> larger
    } //a function to record the related results
      // unsafe fn optimization(){}   //a function to adjust the allocator according to the data collected
}

#[global_allocator]
static GLOBAL: BrugAllocator = BrugAllocator;

unsafe impl GlobalAlloc for BrugAllocator {
    // #[inline]        //inline seems downgrade the performance
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret: *mut u8;

        match _CURRENT_ {
            Allocator::_SYS_ => ret = System.alloc(layout),
            // Allocator::_TCMALLOC_ => {
            //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8
            // }
            Allocator::_MIMALLOC_ => ret = MiMalloc.alloc(layout),
            Allocator::_JEMALLOC_ => ret = Jemalloc.alloc(layout),
            Allocator::_MMAP_ => {
                //size alignments
                const ADDR: *mut c_void = ptr::null_mut::<c_void>();
                // let size:usize;          //Alignments, current disable
                // match layout.size() >= HUGE_PAGE_SIZE{
                //     true => size = (layout.size()+HUGE_PAGE_SIZE)/HUGE_PAGE_SIZE*HUGE_PAGE_SIZE,
                //     false => size = (layout.size()+PAGE_SIZE)/PAGE_SIZE*PAGE_SIZE,
                // };

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

        // BRUG.input(Allocator::_JEMALLOC_);

        if ret.is_null() {
            panic!("Allocate_error");
        }

        ret
    }

    // #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
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

        //Remove the tree entry
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

        if ret.is_null() {
            panic!("Reallocae_error");
        }

        ret
    }
}

fn measurements() {
    let mut v = Vec::new();

    let start = Instant::now();

    for n in 0..DATASIZE {
        v.push(n);
        // println!("{} get pushed", n);
    }

    let duration = start.elapsed();

    println!("Time measured is: {:?}", duration);
}

fn test_sequential(numbers: i32) {
    for _n in 0..numbers {
        measurements();
    }
}

fn test_multithread(numbers: i32) {
    let threads: Vec<_> = (0..numbers)
        .map(|_i| {
            thread::spawn(move || {
                measurements();
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }
}

pub fn seq_test(repeats: i32) {
    println!(
        "Testing {:?} sequential with {} integer push and {} repetations",
        _CURRENT_, DATASIZE, repeats
    );

    test_sequential(repeats);
}

pub fn multi_test(repeats: i32) {
    println!(
        "Testing {:?} multi-thread with {} integer push and {} repetations",
        _CURRENT_, DATASIZE, repeats
    );

    test_multithread(repeats);
}

pub fn combine_test(repeats: i32) {
    println!(
        "Testing {:?} sequential with {} integer push and {} repetations",
        _CURRENT_, DATASIZE, repeats
    );

    test_sequential(repeats);

    println!(
        "Testing {:?} multi-thread with {} integer push and {} repetations",
        _CURRENT_, DATASIZE, repeats
    );

    test_multithread(repeats);
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
