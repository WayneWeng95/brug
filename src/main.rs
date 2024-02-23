#[cfg(unix)]
mod brug_allocator;
pub use crate::brug_allocator::*;

#[global_allocator]
static GLOBAL: brug_allocator::BrugAllocator = brug_allocator::BrugAllocator;

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

fn main() {}
