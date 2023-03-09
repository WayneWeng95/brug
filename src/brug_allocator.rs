use jemallocator::Jemalloc;
use mimalloc::MiMalloc;
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU8, Ordering::SeqCst};
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use tcmalloc;
use once_cell::sync::Lazy;
use std::os::raw::c_void;
use std::ptr;

struct BrugAllocator;

#[derive(Debug, Clone, Copy)]
pub enum Allocator {
    _SYS_,        //MODE 0
    _JEMALLOC_,   //MODE 1
    _MIMALLOC_,   //MODE 2
    _MMAP_,       //MODE 3
    _BrugPredef_, //MODE 4
    _BrugCustom_, //MODE 5
                  // _BrugOpt_,  //MODE 6
                  //  _TCMALLOC_,     //MODE 7
}

pub struct BrugTemplate {
    sys: Vec<usize>,
    jemalloc: Vec<usize>,
    mimalloc: Vec<usize>,
    mmap: Vec<usize>,
}

static BRUG_TEMPLATE: Lazy<BrugTemplate> = Lazy::new(|| {
    let m: BrugTemplate = BrugTemplate {
        sys: Vec::from([1, 2, 3, 4, 5, 6]),
        jemalloc: Vec::from([0]),
        mimalloc: Vec::new(),
        mmap: Vec::from([7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
    };

    m
});

static mut CUSTOM_TEMPLATE: BrugTemplate = BrugTemplate {
    sys: Vec::new(),
    jemalloc: Vec::new(),
    mimalloc: Vec::new(),
    mmap: Vec::new(),
};

#[derive(Debug, Clone, Copy)]
struct Allocdata {
    //Data sturcture to hold the characterstic of a reallocation object
    allocator: Allocator,
    counter: i32,
}

static DEFAULT_ALLOCATOR: Allocator = Allocator::_SYS_; //Current Set as the _SYS_ allocator for default
static PTE_PAGE_SIZE: usize = 4096; //4 KiB
                                    // static PMD_PAGE_SIZE: usize = 2097152; //2 MiB
                                    // static PUD_PAGE_SIZE: usize = 1073741824; //1 GiB

pub struct BrugStruct {
    mapping: Mutex<BTreeMap<usize, Allocdata>>,
    mode: AtomicU8,
    records: Mutex<[[Duration; 4]; 21]>,
    current_alloc: Allocator,
    // Burg_Template: BrugTemplate,
}
unsafe impl Sync for BrugStruct {}

static mut BRUG: BrugStruct = BrugStruct {
    mapping: Mutex::new(BTreeMap::new()), //A tree to hold the allocator applied for this particular memory
    mode: AtomicU8::new(0),               //Indicating the Brug current mode
    records: Mutex::new([[Duration::new(0, 0); 4]; 21]), // A 2-d array for holding the records, [size][allocator]
    current_alloc: DEFAULT_ALLOCATOR,
};

#[allow(dead_code)]
impl BrugStruct {
    unsafe fn input(&mut self, address: usize, alloc_data: Allocdata) {
        //record the allocator mode
        let _tree = self.mapping.get_mut().unwrap();
        _tree.insert(address, alloc_data); //This insert cause the segamentation fault
    }
    unsafe fn suggest(&mut self, ptr: *mut u8, alloc_data: Allocdata) {
        //change the allocator with preference in next reallocation
        let _tree = self.mapping.get_mut().unwrap();
        _tree.insert(ptr.clone() as usize, alloc_data); //Insert the value of the PTR
    }

    unsafe fn counter_grow(&mut self, old_address: usize, new_address: usize) {
        //Modify the tree structure when an reallocation is happened
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
                    allocator: BRUG.current_alloc,
                    counter: 1,
                };
                _tree.insert(new_address, _new_data);
            }
        };
    }

    unsafe fn remove(&mut self, ptr: *mut u8) {
        //remove the entry when deallocate
        let _tree = self.mapping.get_mut().unwrap();
        let _ptr = ptr.clone() as usize;
        match _tree.remove(&_ptr) {
            _ => return,
        }
    }

    fn size_match(size: usize) -> usize {
        //Change this one to 2 X per tier
        // size identifier for 5-level page table 0 -> 4KB -> 8KB -> 16KB -> larger
        let _tier_size = (size / PTE_PAGE_SIZE) as usize;
        if _tier_size >= 20 {
            return 20;
        }
        _tier_size
    }

    unsafe fn record(&mut self, size: usize, time: Duration, allocator: Allocator) {
        // A function to record the reallocation speed, according the speed, make the adjustment
        let _size_type = Self::size_match(size);
        let record_table = self.records.get_mut().unwrap();
        match allocator {
            Allocator::_SYS_ => record_table[_size_type][0] = time,
            Allocator::_JEMALLOC_ => record_table[_size_type][1] = time,
            Allocator::_MIMALLOC_ => record_table[_size_type][2] = time,
            Allocator::_MMAP_ => record_table[_size_type][3] = time,
            _ => (),
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
        //a function to adjust the allocator according to the data collected
        //check the number and see which one cloud work better
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
    }

    pub unsafe fn set_mode(mode: Allocator) {
        //Set the mode to change the Allocator
        match mode {
            Allocator::_SYS_ => {
                BRUG.mode.store(0, SeqCst);
                BRUG.current_alloc = Allocator::_SYS_;
            }
            Allocator::_JEMALLOC_ => {
                BRUG.mode.store(1, SeqCst);
                BRUG.current_alloc = Allocator::_JEMALLOC_;
            }
            Allocator::_MIMALLOC_ => {
                BRUG.mode.store(2, SeqCst);
                BRUG.current_alloc = Allocator::_MIMALLOC_;
            }
            Allocator::_MMAP_ => {
                BRUG.mode.store(3, SeqCst);
                BRUG.current_alloc = Allocator::_MMAP_;
            }
            Allocator::_BrugPredef_ => {
                BRUG.mode.store(4, SeqCst);
                BRUG.current_alloc = Allocator::_BrugPredef_;
            }
            Allocator::_BrugCustom_ => {
                BRUG.mode.store(5, SeqCst);
                BRUG.current_alloc = Allocator::_BrugCustom_;
            } // _ => BRUG.mode.store(0, SeqCst), //Default Mode, use the _SYS allocator
        }
    }

    pub unsafe fn end_set() {
        //Set the allocator back for properly realse the metadata
        BRUG.current_alloc = DEFAULT_ALLOCATOR;
    }

    unsafe fn get_allocator(&mut self, ptr: *mut u8) -> Allocator {
        //take a look later
        //Get the current mode
        let _tree = self.mapping.get_mut().unwrap();
        let _ptr = ptr.clone() as usize;

        let _new_data = match _tree.get(&_ptr) {
            Some(allocdata) => {
                return allocdata.allocator;
            }
            None => {
                return Allocator::_JEMALLOC_;
            }
        };
    }

    unsafe fn mem_mmap(layout: Layout) -> *mut u8 {
        const ADDR: *mut c_void = ptr::null_mut::<c_void>();
        let ret: *mut u8;
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
        ret
    }

    unsafe fn brug_template_mode(&mut self, size: usize, mode: Allocator) -> Allocator {
        //predef template
        let _times = size / PTE_PAGE_SIZE;
        let ret: Allocator = match mode {
            Allocator::_BrugPredef_ => match _times {
                _times if BRUG_TEMPLATE.sys.contains(&_times) => Allocator::_SYS_,
                _times if BRUG_TEMPLATE.jemalloc.contains(&_times) => Allocator::_JEMALLOC_,
                _times if BRUG_TEMPLATE.mimalloc.contains(&_times) => Allocator::_MIMALLOC_,
                _times if BRUG_TEMPLATE.mmap.contains(&_times) => Allocator::_MMAP_,
                _ => Allocator::_SYS_,
            },
            Allocator::_BrugCustom_ => match _times {
                _times if CUSTOM_TEMPLATE.sys.contains(&_times) => Allocator::_SYS_,
                _times if CUSTOM_TEMPLATE.jemalloc.contains(&_times) => Allocator::_JEMALLOC_,
                _times if CUSTOM_TEMPLATE.mimalloc.contains(&_times) => Allocator::_MIMALLOC_,
                _times if CUSTOM_TEMPLATE.mmap.contains(&_times) => Allocator::_MMAP_,
                _ => Allocator::_SYS_,
            },
            _ => Allocator::_SYS_,
        };

        ret
    }

    // unsafe fn realloc_mode(&mut self, ptr: usize, new_size: usize) -> (i32, i32) {       //need to tweak this
    //     let _tree = self.mapping.get_mut().unwrap();
    //     let mut _alloc_data: Option<Allocdata>;

    //     let _new_data = match _tree.remove(&ptr) {
    //         Some(allocdata) => {
    //             let _new_data = Allocdata {
    //                 allocator: allocdata.allocator,
    //                 counter: allocdata.counter + 1,
    //             };
    //             _tree.insert(new_address, _new_data);
    //         }
    //         None => {
    //             let _new_data = Allocdata {
    //                 allocator: BRUG.current_alloc,
    //                 counter: 1,
    //             };
    //             _tree.insert(new_address, _new_data);
    //         }
    //     };
    // }
}

#[global_allocator]
static GLOBAL: BrugAllocator = BrugAllocator;

unsafe impl GlobalAlloc for BrugAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret: *mut u8;

        match BRUG.current_alloc {
            Allocator::_SYS_ => ret = System.alloc(layout),
            Allocator::_MIMALLOC_ => ret = MiMalloc.alloc(layout),
            Allocator::_JEMALLOC_ => ret = Jemalloc.alloc(layout),
            Allocator::_MMAP_ => {
                const ADDR: *mut c_void = ptr::null_mut::<c_void>();
                let _ret = libc::mmap(
                    ADDR,
                    layout.size(),
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                    -1,
                    0,
                );

                match libc::madvise(
                    //Tweaking this madvise part
                    _ret,
                    layout.size(),
                    libc::MADV_WILLNEED | libc::MADV_DONTFORK | libc::MADV_HUGEPAGE,
                ) {
                    -1 => panic!("madvise_error"),
                    _ => ret = _ret as *mut u8,
                }
            }
            Allocator::_BrugPredef_ | Allocator::_BrugCustom_ => {
                match BRUG.brug_template_mode(layout.size(), BRUG.current_alloc) {
                    Allocator::_SYS_ => ret = System.alloc(layout),
                    Allocator::_MIMALLOC_ => ret = MiMalloc.alloc(layout),
                    Allocator::_JEMALLOC_ => ret = Jemalloc.alloc(layout),
                    Allocator::_MMAP_ => {
                        const ADDR: *mut c_void = ptr::null_mut::<c_void>();
                        let _ret = libc::mmap(
                            ADDR,
                            layout.size(),
                            libc::PROT_READ | libc::PROT_WRITE,
                            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                            -1,
                            0,
                        );

                        match libc::madvise(
                            //Tweaking this madvise part
                            _ret,
                            layout.size(),
                            libc::MADV_WILLNEED | libc::MADV_DONTFORK | libc::MADV_HUGEPAGE,
                        ) {
                            -1 => panic!("madvise_error"),
                            _ => ret = _ret as *mut u8,
                        }
                    }
                    _ => ret = System.alloc(layout),
                }
            } // Allocator::_TCMALLOC_ => {
              //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8
              // }
        }

        if layout.size() > PTE_PAGE_SIZE {
            let _alloc_data = Allocdata {
                allocator: BRUG.current_alloc,
                counter: 1,
            };
            BRUG.input(ret.clone() as usize, _alloc_data);
        }

        if ret.is_null() {
            panic!("Allocate_error");
        }

        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        BRUG.remove(ptr);

        match BRUG.current_alloc {
            Allocator::_SYS_ => System.dealloc(ptr, layout),
            Allocator::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
            Allocator::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
            Allocator::_MMAP_ => {
                let addr = ptr as *mut c_void;
                libc::munmap(addr, layout.size());
            } // Allocator::_TCMALLOC_ => tcmalloc::tc_free(ptr as *mut c_void),
            Allocator::_BrugPredef_ | Allocator::_BrugCustom_ => {
                match BRUG.brug_template_mode(layout.size(), BRUG.current_alloc) {
                    Allocator::_SYS_ => System.dealloc(ptr, layout),
                    Allocator::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
                    Allocator::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
                    Allocator::_MMAP_ => {
                        let addr = ptr as *mut c_void;
                        libc::munmap(addr, layout.size());
                    }
                    _ => (),
                }
            }
        }
    }

    // unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { ... } //calloc

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ret: *mut u8;
        let _old_addr = ptr.clone() as usize;
        let _start = Instant::now();

        match BRUG.current_alloc {
            Allocator::_SYS_ => ret = System.realloc(ptr, layout, new_size),
            Allocator::_MIMALLOC_ => ret = MiMalloc.realloc(ptr, layout, new_size),
            Allocator::_JEMALLOC_ => ret = Jemalloc.realloc(ptr, layout, new_size),
            Allocator::_MMAP_ => {
                let old_address = ptr as *mut c_void;
                ret = libc::mremap(old_address, layout.size(), new_size, libc::MREMAP_MAYMOVE)
                    as *mut u8
            }
            Allocator::_BrugPredef_ | Allocator::_BrugCustom_ => {
                match BRUG.brug_template_mode(layout.size(), BRUG.current_alloc) {
                    Allocator::_SYS_ => ret = System.realloc(ptr, layout, new_size),
                    Allocator::_MIMALLOC_ => ret = MiMalloc.realloc(ptr, layout, new_size),
                    Allocator::_JEMALLOC_ => ret = Jemalloc.realloc(ptr, layout, new_size),
                    Allocator::_MMAP_ => {
                        let old_address = ptr as *mut c_void;
                        ret =
                            libc::mremap(old_address, layout.size(), new_size, libc::MREMAP_MAYMOVE)
                                as *mut u8
                    }
                    _ => ret = System.realloc(ptr, layout, new_size),
                }
                //Mechanism tweaking                What happens here
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
            BRUG.record(layout.size(), _duration, BRUG.current_alloc);
        }

        if ret.is_null() {
            panic!("Reallocae_error");
        }

        ret
    }
}
