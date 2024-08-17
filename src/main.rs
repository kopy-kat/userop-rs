use alloy::primitives::{hex, Address, Bytes, B256, U256};
use std::env;

#[derive(Debug, Clone)]
pub struct PackedUserOperation {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub account_gas_limits: B256,
    pub pre_verification_gas: U256,
    pub gas_fees: B256,
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut userOp = PackedUserOperation {
        sender: Address::default(),
        nonce: U256::from(0),
        init_code: Bytes::default(),
        call_data: Bytes::default(),
        account_gas_limits: B256::default(),
        pre_verification_gas: U256::from(0),
        gas_fees: B256::default(),
        paymaster_and_data: Bytes::default(),
        signature: Bytes::default(),
    };

    for (index, argument) in args.iter().enumerate().skip(1) {
        match argument.strip_prefix("--").unwrap_or(argument) {
            "sender" => {
                let bytes = hex::decode(&args[index + 1]).unwrap();
                let mut array = [0u8; 20];
                array.copy_from_slice(&bytes);
                userOp.sender = Address::from(array)
            }
            "nonce" => userOp.nonce = U256::from(args[index + 1].parse::<u64>().unwrap()),
            "init_code" => {
                let bytes = hex::decode(&args[index + 1]).unwrap();
                userOp.init_code = Bytes::from(bytes)
            }
            "call_data" => {
                let bytes = hex::decode(&args[index + 1]).unwrap();
                userOp.call_data = Bytes::from(bytes)
            }
            "account_gas_limits" => {
                let bytes = hex::decode(&args[index + 1]).unwrap();
                let mut array = [0u8; 32];
                array.copy_from_slice(&bytes);
                userOp.account_gas_limits = B256::from(array)
            }
            "pre_verification_gas" => {
                userOp.pre_verification_gas = U256::from(args[index + 1].parse::<u64>().unwrap())
            }
            "gas_fees" => {
                let bytes = hex::decode(&args[index + 1]).unwrap();
                let mut array = [0u8; 32];
                array.copy_from_slice(&bytes);
                userOp.gas_fees = B256::from(array)
            }
            "paymaster_and_data" => {
                let bytes = hex::decode(&args[index + 1]).unwrap();
                userOp.paymaster_and_data = Bytes::from(bytes)
            }
            "signature" => {
                let bytes = hex::decode(&args[index + 1]).unwrap();
                userOp.signature = Bytes::from(bytes)
            }
            _ => (),
        }
    }

    println!("userOp: {:?}", userOp);
}
