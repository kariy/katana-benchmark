use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use cairo_lang_starknet::contract_class::ContractClass;
use papyrus_blockifier::execution::contract_class::{
    ContractClass as CompiledContractClass, ContractClassV1,
};
use papyrus_storage::db::serialization::StorageSerde;
use test_utils::read_json_file;

fn foo(casm: CasmContractClass) -> CompiledContractClass {
    let mut buf: Vec<u8> = Vec::new();
    StorageSerde::serialize_into(&casm, &mut buf).unwrap();
    let casm: CasmContractClass = StorageSerde::deserialize_from(&mut buf.as_slice()).unwrap();
    CompiledContractClass::V1(ContractClassV1::try_from(casm).unwrap())
}
