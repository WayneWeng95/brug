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
// use datafusion::error::Result;
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

use anyhow::Result;
use std::fs;
use std::io::Read;
use wasmtime::*;

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

// fn main() {
//     let datasize = 100_000_0000;

//     // let allocator = brug::Allocatormode::_SYS_;
//     // let allocator = brug::Allocatormode::_JEMALLOC_;
//     // let allocator = brug::Allocatormode::_MIMALLOC_;
//     // let allocator = brug::Allocatormode::_MMAP_;
//     // let allocator = brug::Allocatormode::_BrugTemplate_;
//     // let allocator = brug::Allocatormode::_BrugAutoOpt_;

//     // read_file_vec("/home/weikang/Documents/Brug/Wikidump/enwiki-20230201-pages-articles-multistream1.xml-p1p41242").unwrap();
//     // set_allocator_mode!(Allocatormode::_SYS_,read_file_vec("/home/weikang/Documents/Brug/Wikidump/test.xml"));
//     // set_allocator_mode!(Allocatormode::_MMAP_,read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml"));
//     // read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml");
//     // set_allocator_mode!(Allocatormode::_BrugTemplate_,running(datasize));

//     let mut n = 0;

//     let _start = Instant::now();

//     // data_fusion_example();
//     // set_allocator_mode!(Allocatormode::_MMAP_,data_fusion_example());

//     // while n < 5 {
//     //     thread::sleep(time::Duration::from_secs(1));
//     //     //     read_file_buffer("/home/weikang/Documents/Brug/Wikidump/test.xml");
//     //     // set_allocator_mode!(Allocatormode::_JEMALLOC_,arrow_functional(datasize));
//     //     // arrow_functional(datasize);

//     //     // running(datasize);
//     //     set_allocator_mode!(Allocatormode::_SYS_,running(datasize));

//     //     // arrow_slice("/home/weikang/Documents/Brug/Wikidump/test.xml");
//     //     //     Allocatormode::_MIMALLOC_,
//     //     //     arrow_slice("/home/weikang/Documents/Brug/Wikidump/test.xml")
//     //     // );

//     //     // data_fusion_example();
//     //     // set_allocator_mode!(Allocatormode::_MMAP_,data_fusion_example());

//     //     //     println!("      ");

//     //     n += 1;

//     //     // brug::BrugStruct::end_set();
//     //     // brug::BrugStruct::monitor_print();
//     //     // brug::BrugStruct::disable_monitor();
//     // }

//     let _duration = _start.elapsed();
//     println!("total time : {:?}", _duration);
// }

// fn main() -> Result<()> {
//     // Modules can be compiled through either the text or binary format
//     let engine = Engine::default();
//     let wat = r#"
//         (module
//             (import "host" "host_func" (func $host_hello (param i32)))

//             (func (export "hello")
//                 i32.const 3
//                 call $host_hello)
//         )
//     "#;
//     let module = Module::new(&engine, wat)?;

//     // Create a `Linker` which will be later used to instantiate this module.
//     // Host functionality is defined by name within the `Linker`.
//     let mut linker = Linker::new(&engine);
//     linker.func_wrap(
//         "host",
//         "host_func",
//         |caller: Caller<'_, u32>, param: i32| {
//             println!("Got {} from WebAssembly", param);
//             println!("my host state is: {}", caller.data());
//         },
//     )?;

//     // All wasm objects operate within the context of a "store". Each
//     // `Store` has a type parameter to store host-specific data, which in
//     // this case we're using `4` for.
//     let mut store = Store::new(&engine, 4);
//     let instance = linker.instantiate(&mut store, &module)?;
//     let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;

//     let _start = Instant::now();

//     // And finally we can call the wasm!
//     // hello.call(&mut store, ())?;

//     set_allocator_mode!(Allocatormode::_MMAP_,hello.call(&mut store, ())?);

//     let _duration = _start.elapsed();
//     println!("total time : {:?}", _duration);

//     Ok(())
// }

use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() {
    // let mut i = 0;
    // while i < 5 {
    //     // let _ = linktest();
    //     let _ = bench_wasi(&mut criterion::Criterion::default());
    //     i = i + 1;
    // }
    let _ = bench_wasi(&mut criterion::Criterion::default());
}

fn linktest() -> Result<()> {

    unsafe{
        // brug_allocator::BrugStruct::change_monitor_limiter(4096);
        brug_allocator::BrugStruct::set_mode(Allocatormode::_BrugTemplate_);
    }
    
    let engine = Engine::default();

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Load and compile our two modules
    let linking1 = Module::from_file(&engine, "examples/linking1.wat")?;
    let linking2 = Module::from_file(&engine, "examples/linking2.wat")?;

    // Configure WASI and insert it into a `Store`
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our first module which only uses WASI, then register that
    // instance with the linker since the next linking will use it.
    let linking2 = linker.instantiate(&mut store, &linking2)?;
    linker.instance(&mut store, "linking2", linking2)?;

    // And with that we can perform the final link and the execute the module.
    let linking1 = linker.instantiate(&mut store, &linking1)?;
    let run = linking1.get_typed_func::<(), ()>(&mut store, "run")?;

    let _start = Instant::now();

    // set_allocator_mode!(Allocatormode::_SYS_,run.call(&mut store, ())?);
    run.call(&mut store, ())?;

    let _duration = _start.elapsed();
    println!("total time : {:?}", _duration);
    unsafe{
        brug_allocator::BrugStruct::end_set();
    }
    Ok(())
}

use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs::File, path::Path};
use wasmtime::{Engine, Linker, Module, Store, TypedFunc};
use wasmtime_wasi::{WasiCtx};

fn bench_wasi(c: &mut Criterion) {
    // let _ = env_logger::try_init();

    // Build a zero-filled test file if it does not yet exist.
    let test_file = Path::new("benches/wasi/test.bin");
    if !test_file.is_file() {
        let file = File::create(test_file).unwrap();
        file.set_len(4096).unwrap();
    }

    // Benchmark each `*.wat` file in the `wasi` directory.
    for file in std::fs::read_dir("benches/wasi").unwrap() {
        let path = file.unwrap().path();
        if path.extension().map(|e| e == "wat").unwrap_or(false) {
            let wat = std::fs::read(&path).unwrap();
            let (mut store, run_fn) = instantiate(&wat);
            let bench_name = format!("wasi/{}", path.file_name().unwrap().to_string_lossy());
            // To avoid overhead, the module itself must iterate the expected
            // number of times in a specially-crafted `run` function (see
            // `instantiate` for details).
            c.bench_function(&bench_name, move |b| {
                b.iter_custom(|iters| {
                    let start = Instant::now();
                    let result = run_fn.call(&mut store, iters).unwrap();
                    assert_eq!(iters, result);
                    start.elapsed()
                })
            });
        }
    }
}

/// Compile and instantiate the Wasm module, returning the exported `run`
/// function. This function expects `run` to:
/// - have a single `u64` parameter indicating the number of loop iterations to
///   execute
/// - execute the body of the function for that number of loop iterations
/// - return a single `u64` indicating how many loop iterations were executed
///   (to double-check)
fn instantiate(wat: &[u8]) -> (Store<WasiCtx>, TypedFunc<u64, u64>) {
    let engine = Engine::default();
    let wasi = wasi_context();
    let mut store = Store::new(&engine, wasi);
    let module = Module::new(&engine, wat).unwrap();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).unwrap();
    let instance = linker.instantiate(&mut store, &module).unwrap();
    let run = instance.get_typed_func(&mut store, "run").unwrap();
    (store, run)
}

/// Build a WASI context with some actual data to retrieve.
fn wasi_context() -> WasiCtx {
    WasiCtxBuilder::new()
        .envs(&[
            ("a".to_string(), "b".to_string()),
            ("b".to_string(), "c".to_string()),
            ("c".to_string(), "d".to_string()),
        ])
        .unwrap()
        .args(&[
            "exe".to_string(),
            "--flag1".to_string(),
            "--flag2".to_string(),
            "--flag3".to_string(),
            "--flag4".to_string(),
        ])
        .unwrap()
        .preopened_dir(
            wasmtime_wasi::Dir::open_ambient_dir(
                "benches/wasi",
                wasmtime_wasi::ambient_authority(),
            )
            .unwrap(),
            "/",
        )
        .unwrap()
        .build()
}