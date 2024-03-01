#define ENABLE_PREDICTION 1
#define ENABLE_UNSHRINK_NOW 1 // need enable prediction first
#define MMAP_IN_SMALLSIZE 0
#define ENABLE_X2_ENHANCEMENT 1 // need enable prediction first
#define ENABLE_HUGLETLB 1       // need additional checking before enable this
#define ENABLE_SIZESAVE 0

#define AGGRESIVE 0
#define HEADLESS 4
#define MAPPING_POINT 16
#define SWITCH_POINT 128

#define MALLOC_HOTLEVEL 40
#define MMAP_HOTLEVEL 20
#define HUGE_TLB_POINT 1024 * 1024

#define SHRINKING_LEVEL 2
#define UNSHRINK_THRESHOULD 7

#define SMALL_SIZE_COPY_INCREASE 7

#define HUGE_PAGE_SIZE 2 * 1024 * 1024 // 2048KB as system default
#define MID_PAGE_SIZE 16 * 1024

// need to check the configuation first.  using: cat /proc/filesystems
// When the configure available, check the
// number of huge table in the pool by    using: cat /proc/sys/vm/nr_hugepages
// Adding more hugepage by:   echo 20 > /proc/sys/vm/nr_hugepages
// https://www.kernel.org/doc/html/latest/admin-guide/mm/hugetlbpage.html
// https://github.com/libhugetlbfs/libhugetlbfs

// Users who wish to use hugetlb memory via shared memory segment should be members of a supplementary group and system admin needs to configure
// that gid into /proc/sys/vm/hugetlb_shm_group. It is possible for same or different applications to use any combination of mmaps and shm* calls,
// though the mount of filesystem will be required for using mmap calls without MAP_HUGETLB.

// Syscalls that operate on memory backed by hugetlb pages only have their
// lengths aligned to the native page size of the processor; they will normally fail with errno set to
// EINVAL or exclude hugetlb pages that extend beyond the length if not hugepage aligned. For example, munmap(2)
// will fail if memory is backed by a hugetlb page and the length is smaller than the hugepage size.

//! 3377
//! 3378
