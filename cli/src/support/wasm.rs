use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use anyhow::Context;

pub fn read_wasm(
    root: PathBuf,
    contract_name: &str,
    no_wasm_opt: &bool,
) -> Result<Vec<u8>, anyhow::Error> {
    let wasm_path = if *no_wasm_opt {
        root.as_path()
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("{contract_name}.wasm"))
    } else {
        root.as_path()
            .join("artifacts")
            .join(format!("{contract_name}.wasm"))
    };

    let wasm_path_str = &wasm_path.as_os_str().to_string_lossy();
    let f = File::open(&wasm_path).with_context(|| {
        format!(
            "`{wasm_path_str}` not found, please build and optimize the contract before store code`"
        )
    })?;
    let mut reader = BufReader::new(f);
    let mut wasm = Vec::new();
    reader.read_to_end(&mut wasm)?;
    Ok(wasm)
}
