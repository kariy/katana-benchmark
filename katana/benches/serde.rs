use blockifier::execution::contract_class::ContractClass as CompiledContractClass;
use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use katana::{deserialize_compiled_class, parse_contract_class, serialize_casm};

fn serde_compiled_contract_class(casm: CasmContractClass) -> CompiledContractClass {
    deserialize_compiled_class(serialize_casm(casm))
}

fn storage_serde(c: &mut Criterion) {
    let sierra_class = include_str!("../../resources/sierra/dojo_world_240.json");
    let casm = parse_contract_class(sierra_class);

    c.bench_function("contract class", |b| {
        b.iter_with_large_drop(|| serde_compiled_contract_class(black_box(casm.clone())))
    });
}

criterion_group!(serde, storage_serde);
criterion_main!(serde);
