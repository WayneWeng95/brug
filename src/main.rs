use std::time::Instant;
use std::vec;
#[cfg(unix)]
mod brug_allocator;

#[cfg(unix)]
pub use crate::brug_allocator::*;

#[global_allocator]
static GLOBAL: brug_allocator::BrugAllocator = brug_allocator::BrugAllocator;

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

fn running(datasize: i32) {
    let mut vec = vec::Vec::new();
    for n in 0..datasize {
        vec.push(n);
        // println!("{}", n);
    }
}

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

use arrow::ipc::Bool;
use arrow::{array, buffer, record_batch};
use jemallocator::Jemalloc;
use mimalloc::MiMalloc;
use std::sync;
fn arrow_functional(datasize: i32) {
    //A simple arrow test to testify functionality

    // println!("Arrow test");
    // let col_1 = sync::Arc::new(array::Int32Array::from_iter(0..datasize)) as _;
    // let col_2 = sync::Arc::new(array::Int32Array::from_iter(0..datasize)) as _;

    // let batch =
    //     record_batch::RecordBatch::try_from_iter([("col1", col_1), ("col_2", col_2)]).unwrap();
    // println!("{:?}", batch);

    let mut buffer = buffer::MutableBuffer::new(0);
    buffer.push(256u32);
    buffer.extend_from_slice(&[1u32]);
    let mut n = 0;

    while n < datasize {
        // thread::sleep(time::Duration::from_secs(1));
        buffer.push(n);

        n += 1;
    }

    let buffer: buffer::Buffer = buffer.into();

    // println!("{:?}", buffer);
}

fn arrow_slice(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    //A simple arrow test to testify functionality

    // println!("Arrow test");
    // let col_1 = sync::Arc::new(array::Int32Array::from_iter(0..datasize)) as _;
    // let col_2 = sync::Arc::new(array::Int32Array::from_iter(0..datasize)) as _;

    // let batch =
    //     record_batch::RecordBatch::try_from_iter([("col1", col_1), ("col_2", col_2)]).unwrap();
    // println!("{:?}", batch);

    const BUFFER_LEN: usize = 4096;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = fs::File::open(filepath)?;
    let mut Arrow_buffer = buffer::MutableBuffer::new(0);

    loop {
        let read_count = file.read(&mut buffer)?;
        // do_something(&buffer[..read_count]);  setup do something

        Arrow_buffer.extend_from_slice(&buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }

    Ok(())
    // println!("{:?}", buffer);
}

use brug;

use arrow::record_batch::*;
use datafusion::datasource::file_format::file_type::FileCompressionType;
use datafusion::error::Result;
use datafusion::prelude::*;
#[tokio::main]
async fn data_fusion_example() -> Result<()> {
    let ctx = SessionContext::new();

    // register parquet file with the execution context
    ctx.register_parquet(
        "yellow_taxi",
        &format!("yellow_tripdata_2023-01.parquet"),
        ParquetReadOptions::default(),
    )
    .await?;

    // execute the query
    // let df = ctx
    //     .sql(
    //         "SELECT int_col, double_col, CAST(date_string_col as VARCHAR) \
    //     FROM alltypes_plain \
    //     WHERE id > 1 AND tinyint_col < double_col",
    //     )
    //     .await?;

    let df = ctx
        .sql(
            "SELECT passenger_count, trip_distance, tip_amount FROM yellow_taxi \
            Where trip_distance < 5 AND passenger_count > 1",
        )
        .await?;
    //"SELECT * FROM alltypes_plain"

    // print the results
    df.show().await?;

    Ok(())
}

use std::fs;
use std::io::Read;

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}

fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    //This function is not that correct
    const BUFFER_LEN: usize = 4096;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = fs::File::open(filepath)?;
    let mut vec: Vec<u8> = vec::Vec::new();

    loop {
        let read_count = file.read(&mut buffer)?;
        // do_something(&buffer[..read_count]);  setup do something

        vec.extend_from_slice(&buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(())
}

use std::{thread, time};

fn main() {
    let datasize = 100_000_0000;

    // let allocator = brug::Allocatormode::_SYS_;
    // let allocator = brug::Allocatormode::_JEMALLOC_;
    // let allocator = brug::Allocatormode::_MIMALLOC_;
    // let allocator = brug::Allocatormode::_MMAP_;
    // let allocator = brug::Allocatormode::_BrugTemplate_;
    // let allocator = brug::Allocatormode::_BrugAutoOpt_;

    // read_file_vec("/home/weikang/Documents/Brug/Wikidump/enwiki-20230201-pages-articles-multistream1.xml-p1p41242").unwrap();
    // set_allocator_mode!(Allocatormode::_SYS_,read_file_vec("/home/weikang/Documents/Brug/Wikidump/test.xml"));
    set_allocator_mode!(Allocatormode::_MMAP_,read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml"));
    // read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml");
    // set_allocator_mode!(Allocatormode::_BrugTemplate_,running(datasize));

    let mut n = 0;

    let _start = Instant::now();

    // data_fusion_example();
    // set_allocator_mode!(Allocatormode::_MMAP_,data_fusion_example());

    // while n < 5 {
    //     thread::sleep(time::Duration::from_secs(1));
    //     //     read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml");
    //     // set_allocator_mode!(Allocatormode::_JEMALLOC_,arrow_functional(datasize));
    //     // arrow_functional(datasize);

    //     // running(datasize);
    //     set_allocator_mode!(Allocatormode::_SYS_,running(datasize));

    //     // arrow_slice("/home/weikang/Documents/Brug/Wikidump/test.xml");
    //     //     Allocatormode::_MIMALLOC_,
    //     //     arrow_slice("/home/weikang/Documents/Brug/Wikidump/test.xml")
    //     // );

    //     // data_fusion_example();
    //     // set_allocator_mode!(Allocatormode::_MMAP_,data_fusion_example());

    //     //     println!("      ");

    //     n += 1;

    //     // brug::BrugStruct::end_set();
    //     // brug::BrugStruct::monitor_print();
    //     // brug::BrugStruct::disable_monitor();
    // }

    let _duration = _start.elapsed();
    println!("total time : {:?}", _duration);
}
