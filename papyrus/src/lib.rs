use blockifier::execution::contract_class::{
    ContractClass as CompiledContractClass, ContractClassV1,
};
use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use cairo_lang_starknet::contract_class::ContractClass;
use papyrus_storage::db::serialization::StorageSerde;

pub fn parse_contract_class(sierra_class: &str) -> CasmContractClass {
    let class: ContractClass = serde_json::from_str(sierra_class).unwrap();
    CasmContractClass::from_contract_class(class, true).unwrap()
}

pub fn serialize_casm(casm: CasmContractClass) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    StorageSerde::serialize_into(&casm, &mut buf).unwrap();
    buf
}

pub fn deserialize_casm(buf: Vec<u8>) -> CompiledContractClass {
    let casm: CasmContractClass = StorageSerde::deserialize_from(&mut buf.as_slice()).unwrap();
    CompiledContractClass::V1(ContractClassV1::try_from(casm).unwrap())
}
