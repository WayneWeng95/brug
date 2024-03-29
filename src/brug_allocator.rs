use jemallocator::Jemalloc;
use mimalloc::MiMalloc;
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::BTreeMap;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use tcmalloc;            //The rust implementation of tcmalloc is not stable so we will not use it
use byte_unit::{GIBIBYTE, KIBIBYTE}; //MEBIBYTE
use std::os::raw::c_void;
use std::ptr;

pub struct BrugAllocator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Allocatormode {
    _SYS_,          //MODE 0
    _JEMALLOC_,     //MODE 1
    _MIMALLOC_,     //MODE 2
    _MMAP_,         //MODE 3
    _BrugTemplate_, //MODE 4
    _BrugAutoOpt_,  //MODE 5
                    //  _TCMALLOC_,    //MODE 6
}

// Make the default allocator to Jemalloc for better allocation performance ?

pub struct BrugTemplate {
    //This is the data structure for using the Brug mode. Each allocator is called when it match the size.
    //Each enum includes a bool for indicating its activation, a lower bound u128 and a upper bound u128 variable
    pub sys: (bool, u128, u128),
    pub jemalloc: (bool, u128, u128),
    pub mimalloc: (bool, u128, u128),
    pub mmap: (bool, u128, u128),
}

pub static mut BRUG_TEMPLATE: BrugTemplate = BrugTemplate {
    //The default need bit tweaking
    //This is the default template. It is set mutable so that user can make changes from outside.
    //The cargo make sure the user need to understand the unsage{} before using this
    //Becareful with the tweaking, the size not cover will be set as system allocator, this could bring extra copy
    sys: (true, 4 * KIBIBYTE, 64 * KIBIBYTE),
    jemalloc: (true, 0, 4 * KIBIBYTE),
    mimalloc: (false, 0, 0),
    mmap: (true, 64 * KIBIBYTE, 128 * GIBIBYTE),
};

#[derive(Debug)]
struct Allocdata {
    //Data sturcture to hold the characterstic of a reallocation object
    allocator: Allocatormode,
    counter: i32,
}

#[derive(Debug)]
struct Monitordata {
    //Data sturcture to hold the monitor datas
    realloc_counter: i32,
    addr_counter: i32,
    total_size: usize,
    total_duration: Duration,
}

static DEFAULT_ALLOCATOR: Allocatormode = Allocatormode::_JEMALLOC_; //Current Set as the _SYS_ allocator for default
static PTE_PAGE_SIZE: usize = 4096; //4 KiB
                                    // static PMD_PAGE_SIZE: usize = 2097152; //2 MiB
                                    // static PUD_PAGE_SIZE: usize = 1073741824; //1 GiB

pub struct BrugStruct {
    //Adding the monitor mode
    mapping: Mutex<BTreeMap<usize, Allocdata>>,
    records: Mutex<[[Duration; 4]; 21]>,
    current_alloc: Allocatormode,
    monitor_flag: AtomicBool,
    monitor_map: Mutex<BTreeMap<usize, Monitordata>>,
    monitor_size_limiter: AtomicI32, //This is the limiter for the monitor mode, these ensure the tree is not populated with
                                     //small objects
}
unsafe impl Sync for BrugStruct {}

static mut BRUG: BrugStruct = BrugStruct {
    mapping: Mutex::new(BTreeMap::new()), //A tree to hold the allocator applied for this particular memory
    records: Mutex::new([[Duration::new(0, 0); 4]; 21]), // A 2-d array for holding the records, [size][allocator]
    current_alloc: DEFAULT_ALLOCATOR, //Indicating the Brug current mode, can be change to another ？
    monitor_flag: AtomicBool::new(false),
    monitor_map: Mutex::new(BTreeMap::new()),
    monitor_size_limiter: AtomicI32::new(1024), //Currently assign to 1024, this is the default value
};

#[allow(dead_code)]
impl BrugStruct {
    pub unsafe fn set_mode(mode: Allocatormode) {
        //Set the mode to change the Allocator
        match mode {
            Allocatormode::_SYS_ => {
                BRUG.current_alloc = Allocatormode::_SYS_;
            }
            Allocatormode::_JEMALLOC_ => {
                BRUG.current_alloc = Allocatormode::_JEMALLOC_;
            }
            Allocatormode::_MIMALLOC_ => {
                BRUG.current_alloc = Allocatormode::_MIMALLOC_;
            }
            Allocatormode::_MMAP_ => {
                BRUG.current_alloc = Allocatormode::_MMAP_;
            }
            Allocatormode::_BrugTemplate_ => {
                BRUG.current_alloc = Allocatormode::_BrugTemplate_;
            }
            Allocatormode::_BrugAutoOpt_ => {
                BRUG.current_alloc = Allocatormode::_BrugAutoOpt_;
            } // _ => BRUG.mode.store(0, SeqCst), //Default Mode, use the _SYS allocator
        }
    }

    pub unsafe fn end_set() {
        //Set the allocator back for properly realse the metadata
        BRUG.current_alloc = DEFAULT_ALLOCATOR;
    }

    unsafe fn brug_template_mode(&mut self, size: usize) -> Allocatormode {
        //predef template  using the size inforamtion to choose the allocator
        let ret: Allocatormode;
        let _size = size as u128;
        if BRUG_TEMPLATE.jemalloc.0
            && BRUG_TEMPLATE.jemalloc.1 < _size
            && BRUG_TEMPLATE.jemalloc.2 >= _size
        {
            ret = Allocatormode::_JEMALLOC_;
        } else if BRUG_TEMPLATE.mimalloc.0
            && BRUG_TEMPLATE.mimalloc.1 < _size
            && BRUG_TEMPLATE.mimalloc.2 >= _size
        {
            ret = Allocatormode::_MIMALLOC_;
        } else if BRUG_TEMPLATE.sys.0
            && BRUG_TEMPLATE.mimalloc.1 < _size
            && BRUG_TEMPLATE.mimalloc.2 >= _size
        {
            ret = Allocatormode::_SYS_;
        } else if BRUG_TEMPLATE.mmap.0
            && BRUG_TEMPLATE.mmap.1 < _size
            && BRUG_TEMPLATE.mmap.2 >= _size
        {
            ret = Allocatormode::_MMAP_;
        } else {
            ret = DEFAULT_ALLOCATOR; // Used be SYS
        }

        ret
    }

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

    unsafe fn counter_grow(
        &mut self,
        old_address: usize,
        new_address: usize,
        new_allocator: Allocatormode,
    ) {
        //Modify the tree structure when an reallocation is happened
        // counter will be increase, and data structure will be update
        let _tree = self.mapping.get_mut().unwrap();
        let _new_counter;

        match _tree.remove(&old_address) {
            Some(allocdata) => {
                _new_counter = allocdata.counter + 1;
            }
            None => {
                _new_counter = 1;
            }
        };
        let _new_data = Allocdata {
            allocator: new_allocator,
            counter: _new_counter,
        };
        _tree.insert(new_address, _new_data);
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

    unsafe fn record(&mut self, size: usize, time: Duration, allocator: Allocatormode) {
        // A function to record the reallocation speed, according the speed, make the adjustment
        // New record will combine with old records, after certain amout of running, the best one will be used
        let _size_type = Self::size_match(size);
        let record_table = self.records.get_mut().unwrap();
        match allocator {
            Allocatormode::_SYS_ => {
                record_table[_size_type][0] = (time + record_table[_size_type][0]) / 2
            }
            Allocatormode::_JEMALLOC_ => {
                record_table[_size_type][1] = (time + record_table[_size_type][0]) / 2
            }
            Allocatormode::_MIMALLOC_ => {
                record_table[_size_type][2] = (time + record_table[_size_type][0]) / 2
            }
            Allocatormode::_MMAP_ => {
                record_table[_size_type][3] = (time + record_table[_size_type][0]) / 2
            }
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

    unsafe fn optimization_mode(&mut self, size: usize) -> Allocatormode {
        //a function to adjust the allocator according to the data collected
        //check the number and see which one cloud work better
        let size_type = Self::size_match(size);
        let record_table = self.records.get_mut().unwrap();
        let allocator_type = Self::position_min(&record_table[size_type]).unwrap();
        let best_allocator = match allocator_type {
            0 => Allocatormode::_SYS_,
            1 => Allocatormode::_JEMALLOC_,
            2 => Allocatormode::_MIMALLOC_,
            3 => Allocatormode::_MMAP_,
            _ => DEFAULT_ALLOCATOR, // in case of error, fall back to default allocator // Used be _SYS_
        };
        best_allocator
    }

    unsafe fn automode_get_allocator(&mut self, ptr: *mut u8) -> Allocatormode {
        //Get the current mode, remove the entry
        let _tree = self.mapping.get_mut().unwrap();
        let _ptr = ptr.clone() as usize;

        let _new_data = match _tree.remove(&_ptr) {
            Some(allocdata) => {
                return allocdata.allocator;
            }
            None => {
                return DEFAULT_ALLOCATOR; //_SYS_       Here could be a problem
            }
        };
    }

    pub unsafe fn enable_monitor() {
        BRUG.monitor_flag.store(true, SeqCst);
        eprintln!("Enable monitor mode, performance may influenced");
    }

    pub unsafe fn change_monitor_limiter(val: i32) {
        //Change the limiter
        BRUG.monitor_size_limiter.store(val, SeqCst);
    }

    pub unsafe fn disable_monitor() {
        BRUG.monitor_flag.store(false, SeqCst);
        BRUG.monitor_map.get_mut().unwrap().clear();
        //Destory the map
    }

    unsafe fn monitor_input(&mut self, address: usize, monitor_data: Monitordata) {
        //record when allocation happens
        let _tree = self.monitor_map.get_mut().unwrap();
        _tree.insert(address, monitor_data); //This insert cause the segamentation fault
    }

    unsafe fn monitor_update(
        //Updates the monitoring data on reallocation
        &mut self,
        old_address: usize,
        new_address: usize,
        duration: Duration,
        new_size: usize,
    ) {
        let _tree = self.monitor_map.get_mut().unwrap();
        let _realloc_counter;
        let _addr_counter;
        let _new_duration;
        let _new_total_size;

        match _tree.remove(&old_address) {
            Some(monitor_data) => {
                _realloc_counter = monitor_data.realloc_counter + 1;
                if old_address != new_address {
                    _addr_counter = monitor_data.addr_counter + 1;
                } else {
                    _addr_counter = monitor_data.addr_counter;
                }
                _new_duration = monitor_data.total_duration + duration;
                _new_total_size = monitor_data.total_size + new_size;
            }
            None => {
                _realloc_counter = 1;
                _addr_counter = 0;
                _new_total_size = new_size;
                _new_duration = duration;
            }
        };
        let _new_data = Monitordata {
            realloc_counter: _realloc_counter,
            addr_counter: _addr_counter,
            total_size: _new_total_size,
            total_duration: _new_duration,
        };
        _tree.insert(new_address, _new_data);
    }

    unsafe fn monitor_release(&mut self, addr: usize) {
        //Currently remove this one after deallocation , But this remain thinking
        let _tree = self.monitor_map.get_mut().unwrap();
        _tree.remove(&addr);
    }

    pub unsafe fn monitor_print() {
        if BRUG.monitor_flag.load(SeqCst) {
            let _monitor_tree = BRUG.monitor_map.get_mut().unwrap();
            for (addr, mointordata) in _monitor_tree {
                eprintln!("Object address: {} with {:?}", addr, mointordata);
            }
        } else {
            eprintln!("Monitor mode not enabled");
        }
    } //Output records
}

unsafe impl GlobalAlloc for BrugAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //allocation function
        let ret: *mut u8;
        let _monitor_flag = BRUG.monitor_flag.load(SeqCst);
        let _start = Instant::now();

        match BRUG.current_alloc {
            Allocatormode::_SYS_ => ret = System.alloc(layout),
            Allocatormode::_MIMALLOC_ => ret = MiMalloc.alloc(layout),
            Allocatormode::_JEMALLOC_ => ret = Jemalloc.alloc(layout),
            Allocatormode::_MMAP_ => {
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
            Allocatormode::_BrugTemplate_ => {
                match BRUG.brug_template_mode(layout.size()) {
                    Allocatormode::_SYS_ => ret = System.alloc(layout),
                    Allocatormode::_MIMALLOC_ => ret = MiMalloc.alloc(layout),
                    Allocatormode::_JEMALLOC_ => ret = Jemalloc.alloc(layout),
                    Allocatormode::_MMAP_ => {
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
            }
            Allocatormode::_BrugAutoOpt_ => {
                ret = System.alloc(layout); // start with _SYS_ as default
                if layout.size() > PTE_PAGE_SIZE {
                    //Put the large object into record tree
                    let _alloc_data = Allocdata {
                        allocator: Allocatormode::_SYS_,
                        counter: 1,
                    };
                    BRUG.input(ret.clone() as usize, _alloc_data);
                }
            } // Allocatormode::_TCMALLOC_ => {
              //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8
              // }
        }

        if _monitor_flag && layout.size() > BRUG.monitor_size_limiter.load(SeqCst) as usize {
            //Adding a filters
            let _ret = ret.clone() as usize;
            let _duration = _start.elapsed();
            let _monitor_data = Monitordata {
                realloc_counter: 0,
                addr_counter: 0,
                total_size: layout.size(),
                total_duration: _duration,
            };
            BRUG.monitor_input(_ret, _monitor_data);
        }

        if ret.is_null() {
            panic!("Allocate_error");
        }

        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //Free function
        match BRUG.current_alloc {
            Allocatormode::_SYS_ => System.dealloc(ptr, layout),
            Allocatormode::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
            Allocatormode::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
            Allocatormode::_MMAP_ => {
                let addr = ptr as *mut c_void;
                libc::munmap(addr, layout.size());
            }
            Allocatormode::_BrugTemplate_ => match BRUG.brug_template_mode(layout.size()) {
                Allocatormode::_SYS_ => System.dealloc(ptr, layout),
                Allocatormode::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
                Allocatormode::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
                Allocatormode::_MMAP_ => {
                    let addr = ptr as *mut c_void;
                    libc::munmap(addr, layout.size());
                }
                _ => (),
            },
            Allocatormode::_BrugAutoOpt_ => {
                if layout.size() < PTE_PAGE_SIZE {
                    System.dealloc(ptr, layout); //Samll objects go to default
                } else {
                    match BRUG.automode_get_allocator(ptr) {
                        Allocatormode::_SYS_ => System.dealloc(ptr, layout),
                        Allocatormode::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
                        Allocatormode::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
                        Allocatormode::_MMAP_ => {
                            let addr = ptr as *mut c_void;
                            libc::munmap(addr, layout.size());
                        }
                        _ => System.dealloc(ptr, layout),
                    }
                }
            } // Allocatormode::_TCMALLOC_ => tcmalloc::tc_free(ptr as *mut c_void),
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        //realloc function
        let ret: *mut u8;
        let _old_addr = ptr.clone() as usize;
        let _start = Instant::now();
        let mut _new_allocator: Allocatormode = DEFAULT_ALLOCATOR; //Initilized _SYS_

        match BRUG.current_alloc {
            Allocatormode::_SYS_ => ret = System.realloc(ptr, layout, new_size),
            Allocatormode::_MIMALLOC_ => ret = MiMalloc.realloc(ptr, layout, new_size),
            Allocatormode::_JEMALLOC_ => ret = Jemalloc.realloc(ptr, layout, new_size),
            Allocatormode::_MMAP_ => {
                let old_address = ptr as *mut c_void;
                ret = libc::mremap(old_address, layout.size(), new_size, libc::MREMAP_MAYMOVE)
                    as *mut u8
            }
            Allocatormode::_BrugTemplate_ | Allocatormode::_BrugAutoOpt_ => {
                let _current_allocator: Allocatormode;
                if BRUG.current_alloc == Allocatormode::_BrugTemplate_ {
                    _current_allocator = BRUG.brug_template_mode(layout.size());
                    _new_allocator = BRUG.brug_template_mode(new_size);
                } else {
                    _new_allocator = BRUG.optimization_mode(new_size);
                    if layout.size() < PTE_PAGE_SIZE {
                        _current_allocator = Allocatormode::_SYS_;
                    } else {
                        _current_allocator = BRUG.automode_get_allocator(ptr);
                        // println!("1.{:?}", _current_allocator);              //This can be use for demo the trainning and performance enhancements
                    }
                }
                let _new = std::alloc::Layout::from_size_align(new_size, layout.align()).unwrap();
                if _current_allocator == _new_allocator {
                    match _new_allocator {
                        //This return the new size to use.
                        Allocatormode::_SYS_ => ret = System.realloc(ptr, layout, new_size),
                        Allocatormode::_MIMALLOC_ => ret = MiMalloc.realloc(ptr, layout, new_size),
                        Allocatormode::_JEMALLOC_ => ret = Jemalloc.realloc(ptr, layout, new_size),
                        Allocatormode::_MMAP_ => {
                            let old_address = ptr as *mut c_void;
                            ret = libc::mremap(
                                old_address,
                                layout.size(),
                                new_size,
                                libc::MREMAP_MAYMOVE,
                            ) as *mut u8
                        }
                        _ => ret = System.realloc(ptr, layout, new_size),
                    }
                } else {
                    ret = match _new_allocator {
                        Allocatormode::_SYS_ => System.alloc(_new),
                        Allocatormode::_MIMALLOC_ => MiMalloc.alloc(_new),
                        Allocatormode::_JEMALLOC_ => Jemalloc.alloc(_new),
                        Allocatormode::_MMAP_ => {
                            const ADDR: *mut c_void = ptr::null_mut::<c_void>();
                            let _ret = libc::mmap(
                                ADDR,
                                new_size,
                                libc::PROT_READ | libc::PROT_WRITE,
                                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                                -1,
                                0,
                            );

                            match libc::madvise(
                                //Tweaking this madvise part
                                _ret,
                                new_size,
                                libc::MADV_WILLNEED | libc::MADV_DONTFORK | libc::MADV_HUGEPAGE,
                            ) {
                                -1 => panic!("madvise_error"),
                                _ => _ret as *mut u8,
                            }
                        }
                        _ => System.alloc(_new),
                    };
                    std::ptr::copy_nonoverlapping(ptr, ret, layout.size());
                    match _current_allocator {
                        Allocatormode::_SYS_ => System.dealloc(ptr, layout),
                        Allocatormode::_MIMALLOC_ => MiMalloc.dealloc(ptr, layout),
                        Allocatormode::_JEMALLOC_ => Jemalloc.dealloc(ptr, layout),
                        Allocatormode::_MMAP_ => {
                            let addr = ptr as *mut c_void;
                            libc::munmap(addr, layout.size());
                        }
                        _ => (),
                    }
                }
            } // Allocatormode::_TCMALLOC_ => {
              //     ret = tcmalloc::tc_memalign(layout.align(), layout.size()) as *mut u8;
              //     std::ptr::copy_nonoverlapping(ptr, ret, layout.size());
              // }
        }

        let _duration = _start.elapsed();
        // println!("{:?}", _duration);

        if BRUG.current_alloc == Allocatormode::_BrugAutoOpt_ && layout.size() >= PTE_PAGE_SIZE {
            let _ret = ret.clone() as usize;
            let _duration = _start.elapsed();
            BRUG.counter_grow(_old_addr, _ret, _new_allocator);
            BRUG.record(new_size, _duration, _new_allocator);
        }

        if BRUG.monitor_flag.load(SeqCst) {
            let _ret = ret.clone() as usize;
            let _duration = _start.elapsed();
            BRUG.monitor_update(_old_addr, _ret, _duration, new_size);
        }

        if ret.is_null() {
            panic!("Reallocae_error");
        }

        ret
    }
}
