// extern crate tcmalloc;
use jemallocator::Jemalloc;
use mimalloc::MiMalloc;
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU8, Ordering::SeqCst};
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use tcmalloc;
use std::os::raw::c_void;
use std::ptr;

struct BrugAllocator;

#[derive(Debug, Clone, Copy)]
pub enum Allocator {
    _SYS_,      //MODE 0
    _JEMALLOC_, //MODE 1
    _MIMALLOC_, //MODE 2
    _MMAP_,     //MODE 3
                // _BRUG_,     //MODE 4
                //  _TCMALLOC_     //MODE 5
}

#[derive(Debug, Clone, Copy)]
struct Allocdata {
    allocator: Allocator,
    counter: i32,
}

static mut _CURRENT_: Allocator = Allocator::_SYS_;
static PTE_PAGE_SIZE: usize = 4096;
static PMD_PAGE_SIZE: usize = 2097152;
static PUD_PAGE_SIZE: usize = 1073741824;

pub struct BrugStruct {
    mapping: Mutex<BTreeMap<usize, Allocdata>>,
    mode: AtomicU8,
    records: Mutex<[[Duration; 4]; 4]>,
}
unsafe impl Sync for BrugStruct {}

static mut BRUG: BrugStruct = BrugStruct {
    mapping: Mutex::new(BTreeMap::new()), //A tree to hold the allocator applied for this particular memory
    mode: AtomicU8::new(0),               //Indicating the Brug current mode
    records: Mutex::new([[Duration::new(0, 0); 4]; 4]), // A 2-d array for holding the records, [size][allocator]
};

#[allow(dead_code)]
impl BrugStruct {
    unsafe fn input(&mut self, address: usize, alloc_data: Allocdata) {
        //record the allocator mode
        // self.mapping.lock().unwrap(); //change to try_lock()
        let _tree = self.mapping.get_mut().unwrap();
        _tree.insert(address, alloc_data); //This insert cause the segamentation fault
    }
    unsafe fn suggest(&mut self, ptr: *mut u8, alloc_data: Allocdata) {
        //change the allocator with preference in next reallocation
        // self.mapping.lock().unwrap();
        let _tree = self.mapping.get_mut().unwrap();
        _tree.insert(ptr.clone() as usize, alloc_data); //Insert the value of the PTR
    }

    unsafe fn counter_grow(&mut self, old_address: usize, new_address: usize) {
        // self.mapping.lock().unwrap();
        let _tree = self.mapping.get_mut().unwrap();
        let mut _alloc_data: Option<Allocdata>;

        let _new_data = match _tree.remove(&old_address) {
            Some(allocdata) => {
                let _new_data = Allocdata {
                    allocator: allocdata.allocator,
                    counter: allocdata.counter + 1,
                };
                _tree.insert(new_address, _new_data);
            }
            None => {
                let _new_data = Allocdata {
                    allocator: Allocator::_SYS_,
                    counter: 1,
                };
                _tree.insert(new_address, _new_data);
            }
        };
    }

    unsafe fn remove(&mut self, ptr: *mut u8) {
        //remove the entry when deallocate
        // self.mapping.lock().unwrap();
        let _tree = self.mapping.get_mut().unwrap();
        let _ptr = ptr.clone() as usize;
        match _tree.remove(&_ptr){
            _ => return,
        }
    }

    fn size_match(size: usize) -> usize {
        // size identifier for 5-level page table 0 -> 4KB -> 2MB -> 1GB -> larger
        if PTE_PAGE_SIZE < size && size <= PMD_PAGE_SIZE {
            return 0;
        } else if PMD_PAGE_SIZE < size && size <= PUD_PAGE_SIZE {
            return 1;
        } else if PUD_PAGE_SIZE < size {
            return 2;
        } else {
            return 3;
        };
    }

    unsafe fn record(&mut self, size: usize, time: Duration, allocator: Allocator) {
        // self.records.lock().unwrap();
        let _size_type = Self::size_match(size);
        let record_table = self.records.get_mut().unwrap();
        match allocator {
            Allocator::_SYS_ => record_table[_size_type][0] = time,
            Allocator::_JEMALLOC_ => record_table[_size_type][1] = time,
            Allocator::_MIMALLOC_ => record_table[_size_type][2] = time,
            Allocator::_MMAP_ => record_table[_size_type][3] = time,
        };
    }

    // fn position_max_copy<T: Ord + Copy>(slice: &[T]) -> Option<usize> {
    //     slice.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx)
    // }

    fn position_min<T: Ord>(slice: &[T]) -> Option<usize> {
        slice
            .iter()
            .enumerate()
            .min_by(|(_, value0), (_, value1)| value0.cmp(value1))
            .map(|(idx, _)| idx)
    }

    unsafe fn optimization_mode(&mut self, size: usize) -> Allocator {
        let size_type = Self::size_match(size);
        let record_table = self.records.get_mut().unwrap();
        let allocator_type = Self::position_min(&record_table[size_type]).unwrap();
        let best_allocator = match allocator_type {
            0 => Allocator::_SYS_,
            1 => Allocator::_JEMALLOC_,
            2 => Allocator::_MIMALLOC_,
            3 => Allocator::_MMAP_,
            _ => Allocator::_SYS_, // in case of error, fall back to the system allocator
        };
        best_allocator
    } //a function to adjust the allocator according to the data collected
      //check the number and see which one cloud work better

    pub unsafe fn set_mode(mode: i32) {
        match mode {
            0 => {
                //Default Mode, use the _SYS allocator
                BRUG.mode.store(0, SeqCst);
                _CURRENT_ = Allocator::_SYS_;
            }
            1 => {
                BRUG.mode.store(1, SeqCst);
                _CURRENT_ = Allocator::_JEMALLOC_;
            }
            2 => {
                BRUG.mode.store(2, SeqCst);
                _CURRENT_ = Allocator::_MIMALLOC_;
            }
            3 => {
                BRUG.mode.store(3, SeqCst);
                _CURRENT_ = Allocator::_MMAP_;
            }
            4 => {
                BRUG.mode.store(4, SeqCst);
                // _CURRENT_ = Allocator::_SYS_;       //Set Mimalloc as the deafult allocator
            }
            _ => BRUG.mode.store(0, SeqCst),
        }
    }

    unsafe fn get_allocator() -> u8 {
        BRUG.mode.load(SeqCst)
    }
}

#[global_allocator]
static GLOBAL: BrugAllocator = BrugAllocator;

unsafe impl GlobalAlloc for BrugAllocator {
    // #[inline]        //inline seems downgrade the performance
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret: *mut u8;
        // let _start = Instant::now();

        match _CURRENT_ {
            Allocator::_SYS_ => ret = System.alloc(layout),
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
            } // Allocator::_TCMALLOC_ => {
              //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8
              // }
        }

        if layout.size() > PTE_PAGE_SIZE {
            //We record this object usage
            // let _duration = _start.elapsed();
            let _alloc_data = Allocdata {
                allocator: _CURRENT_,
                counter: 1,
            };
            BRUG.input(ret.clone() as usize, _alloc_data);
            // BRUG.record(layout.size(), _duration, _alloc_data);
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
            Allocator::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
            Allocator::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
            Allocator::_MMAP_ => {
                let addr = ptr as *mut c_void;
                libc::munmap(addr, layout.size());
            } // Allocator::_TCMALLOC_ => tcmalloc::tc_free(ptr as *mut c_void),
        }
    }

    // unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { ... } //calloc
    // #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ret: *mut u8;
        let _old_addr = ptr.clone() as usize;
        let _start = Instant::now();

        match _CURRENT_ {
            Allocator::_SYS_ => ret = System.realloc(ptr, layout, new_size),
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
            } // Allocator::_TCMALLOC_ => {
              //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8;
              //     std::ptr::copy_nonoverlapping(ptr, ret, layout.size());
              // }
        }

        if new_size < PTE_PAGE_SIZE {
            if layout.size() > PTE_PAGE_SIZE {
                BRUG.remove(ptr);
            }
        } else {
            let _ret = ret.clone() as usize;
            let _duration = _start.elapsed();
            BRUG.counter_grow(_old_addr, _ret);
            BRUG.record(layout.size(), _duration, _CURRENT_);
        }

        // println!("Realloc:{}",layout.size());

        if ret.is_null() {
            panic!("Reallocae_error");
        }

        ret
    }
}