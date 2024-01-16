use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use cairo_lang_starknet::contract_class::ContractClass;
use katana_db::{
    codecs::{Compress, Decompress},
    models::class::StoredContractClass,
};
use katana_primitives::contract::{CompiledContractClass, CompiledContractClassV1};

pub fn parse_contract_class(sierra_class: &str) -> CasmContractClass {
    let class: ContractClass = serde_json::from_str(sierra_class).unwrap();
    CasmContractClass::from_contract_class(class, true).unwrap()
}

pub fn serialize_casm(casm: CasmContractClass) -> Vec<u8> {
    let class = CompiledContractClass::V1(CompiledContractClassV1::try_from(casm).unwrap());
    StoredContractClass::from(class).compress()
}

pub fn deserialize_compiled_class(buf: Vec<u8>) -> CompiledContractClass {
    CompiledContractClass::from(StoredContractClass::decompress(buf).unwrap())
}
