// Copyright 2019-2023 ChainX Project. Licensed under GPL-3.0.

use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        .import_memory()
        .export_heap_base()
        .build()
}
