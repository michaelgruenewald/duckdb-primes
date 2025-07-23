extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{
    core::{DataChunkHandle, LogicalTypeHandle, LogicalTypeId},
    vtab::{BindInfo, InitInfo, TableFunctionInfo, VTab},
    Connection, Result,
};
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use libduckdb_sys as ffi;
use std::{
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
    vec,
};

#[repr(C)]
struct PrimesBindData {
    limit: i64,
}

#[repr(C)]
struct PrimesInitData {
    done: AtomicBool,
}

struct HelloVTab;

impl VTab for HelloVTab {
    type InitData = PrimesInitData;
    type BindData = PrimesBindData;

    fn bind(bind: &BindInfo) -> Result<Self::BindData, Box<dyn std::error::Error>> {
        bind.add_result_column("no", LogicalTypeHandle::from(LogicalTypeId::Bigint));

        Ok(PrimesBindData {
            limit: bind.get_parameter(0).to_int64(),
        })
    }

    fn init(_: &InitInfo) -> Result<Self::InitData, Box<dyn std::error::Error>> {
        Ok(PrimesInitData {
            done: AtomicBool::new(false),
        })
    }

    fn func(
        func: &TableFunctionInfo<Self>,
        output: &mut DataChunkHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let init_data = func.get_init_data();
        let bind_data = func.get_bind_data();

        if init_data.done.swap(true, Ordering::Relaxed) {
            output.set_len(0);
            return Ok(());
        }

        let limit = bind_data.limit as usize;

        let mut sieve = vec![true; limit];
        let mut total = 0;
        for i in 2..limit {
            if sieve[i] {
                for j in (i..limit).step_by(i).skip(1) {
                    sieve[j] = false;
                }
                total += 1;
            }
        }

        let mut vector = output.flat_vector(0);
        let mut it = vector.as_mut_slice_with_len::<i64>(total).into_iter();

        for i in 2..limit {
            if sieve[i] {
                *it.next().unwrap() = i as i64;
            }
        }
        output.set_len(total);

        Ok(())
    }

    fn parameters() -> Option<Vec<LogicalTypeHandle>> {
        Some(vec![LogicalTypeHandle::from(LogicalTypeId::Bigint)])
    }
}

const EXTENSION_NAME: &str = "primes";

#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    con.register_table_function::<HelloVTab>(EXTENSION_NAME)
        .expect("Failed to register hello table function");
    Ok(())
}
