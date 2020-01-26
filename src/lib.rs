#![no_std]
extern crate eng_wasm;
extern crate eng_wasm_derive;
use eng_wasm::*;
use eng_wasm_derive::pub_interface;

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct Contract;

// Public trait defining public-facing secret contract functions
#[pub_interface]
pub trait ContractInterface {
    fn string() -> String;
    fn string_array_one() -> Vec<String>;
    fn string_array_two() -> Vec<String>;
    fn int_array() -> Vec<i64>;
}

// Implementation of the public-facing secret contract functions defined in the ContractInterface
// trait implementation for the Contract struct above
impl ContractInterface for Contract {
    #[no_mangle]
    fn string_array_two() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push("string_array_one_member1".to_string());
        return vec;
    }
    #[no_mangle]
    fn string_array_two() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push("string_array_two_member1".to_string());
        vec.push("string_array_two_member2".to_string());
        return vec;
    }
    #[no_mangle]
    fn numvec() -> Vec<i64> {
        let mut vec: Vec<i64> = Vec::new();
        vec.push(0);
        vec.push(1);
        vec.push(2);
        return vec;
    }
}
