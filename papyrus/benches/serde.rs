use blockifier::execution::contract_class::ContractClass as CompiledContractClass;
use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use papyrus::{deserialize_casm_buffer_as_compiled_contract, parse_contract_class, serialize_casm};

fn serde_compiled_contract_class(casm: CasmContractClass) -> CompiledContractClass {
    deserialize_casm_buffer_as_compiled_contract(serialize_casm(casm))
}

fn storage_serde(c: &mut Criterion) {
    let sierra_class = include_str!("../../resources/sierra/dojo_world_240.json");
    let casm = parse_contract_class(sierra_class);
    let serialized_casm = serialize_casm(casm.clone());

    c.bench_function("serialize contract class", |b| {
        b.iter_with_large_drop(|| serialize_casm(black_box(casm.clone())))
    });

    c.bench_function("deserialize contract class", |b| {
        b.iter_with_large_drop(|| {
            deserialize_casm_buffer_as_compiled_contract(black_box(serialized_casm.clone()))
        })
    });

    c.bench_function("contract class", |b| {
        b.iter_with_large_drop(|| serde_compiled_contract_class(black_box(casm.clone())))
    });
}

criterion_group!(serde, storage_serde);
criterion_main!(serde);
