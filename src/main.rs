use std::time::Instant;
use std::vec;

#[cfg(unix)]
mod brug_allocator;

#[cfg(unix)]
pub use crate::brug_allocator::*;

#[global_allocator]
static GLOBAL: brug_allocator::BrugAllocator = brug_allocator::BrugAllocator;

fn running(datasize: i32) {
    let mut vec = vec::Vec::new();
    for n in 0..datasize {
        vec.push(n);
        // println!("{}", n);
    }
}

use arrow::ipc::Bool;
use arrow::{array, buffer, record_batch};
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

use brug;

use arrow::record_batch::*;
use datafusion::datasource::file_format::file_type::FileCompressionType;
use datafusion::error::Result;
use datafusion::prelude::*;
async fn data_fusion_example() -> Result<()> {
    let ctx = SessionContext::new();

    let testdata = datafusion::test_util::arrow_test_data();

    // register csv file with the execution context
    ctx.register_csv(
        "aggregate_test_100",
        &format!("{testdata}/csv/aggregate_test_100.csv"),
        CsvReadOptions::new(),
    )
    .await?;

    // execute the query
    let df = ctx
        .sql(
            "SELECT c1, MIN(c12), MAX(c12) \
        FROM aggregate_test_100 \
        WHERE c11 > 0.1 AND c11 < 0.9 \
        GROUP BY c1",
        )
        .await?;

    // print the results
    df.show().await?;

    // query compressed CSV with specific options
    let csv_options = CsvReadOptions::default()
        .has_header(true)
        .file_compression_type(FileCompressionType::GZIP)
        .file_extension("csv.gz");
    let df = ctx
        .read_csv(
            &format!("{testdata}/csv/aggregate_test_100.csv.gz"),
            csv_options,
        )
        .await?;
    let df = df
        .filter(col("c1").eq(lit("a")))?
        .select_columns(&["c2", "c3"])?;

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
    println!("I'm using the library:");

    let datasize = 1000000;

    // let allocator = brug::Allocatormode::_SYS_;
    // let allocator = brug::Allocatormode::_JEMALLOC_;
    // let allocator = brug::Allocatormode::_MIMALLOC_;
    // let allocator = brug::Allocatormode::_MMAP_;
    // let allocator = brug::Allocatormode::_BrugTemplate_;
    let allocator = brug::Allocatormode::_BrugAutoOpt_;

    // running(datasize);

    // read_file_vec("/home/weikang/Documents/Brug/Wikidump/enwiki-20230201-pages-articles-multistream1.xml-p1p41242").unwrap();
    // read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml");

    unsafe {
        brug::BrugStruct::enable_monitor();
        brug::BrugStruct::set_mode(allocator);

        let mut n = 0;

        while n < 15 {
            thread::sleep(time::Duration::from_secs(1));
            //     read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml");
            arrow_functional(datasize);
            //     println!("      ");

            n += 1;
        }

        brug::BrugStruct::end_set();
        brug::BrugStruct::monitor_print();
        brug::BrugStruct::disable_monitor();
    }
}

// Object address: 139961451940224 with Monitordata { realloc_counter: 0, addr_counter: 0, total_size: 4000000000, total_duration: 22.66µs }
// Object address: 139967900682240 with Monitordata { realloc_counter: 0, addr_counter: 0, total_size: 4000000000, total_duration: 12.79µs }
// Object address: 139973269391488 with Monitordata { realloc_counter: 0, addr_counter: 0, total_size: 125000000, total_duration: 8.67µs }
// Object address: 139973269393024 with Monitordata { realloc_counter: 0, addr_counter: 0, total_size: 125000000, total_duration: 56.01µs }

// Object address: 140152464097280 with Monitordata { realloc_counter: 0, addr_counter: 0, total_size: 4000000000, total_duration: 1.78µs }
// Object address: 140156464099328 with Monitordata { realloc_counter: 0, addr_counter: 0, total_size: 4000000000, total_duration: 2.15µs }
// Object address: 140160464101376 with Monitordata { realloc_counter: 0, addr_counter: 0, total_size: 125000000, total_duration: 2.43µs }
