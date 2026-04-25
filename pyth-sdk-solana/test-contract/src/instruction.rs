//! Program instructions for end-to-end testing and instruction counts

use pyth_sdk_solana::Price;

use crate::id;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::instruction::Instruction;

/// Instructions supported by the pyth-client program, used for testing and
/// instruction counts
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PythClientInstruction {
    Divide {
        numerator: Price,
        denominator: Price,
    },
    Multiply {
        x: Price,
        y: Price,
    },
    Add {
        x: Price,
        y: Price,
    },
    ScaleToExponent {
        x: Price,
        expo: i32,
    },
    Normalize {
        x: Price,
    },
    /// Don't do anything for comparison
    ///
    /// No accounts required for this instruction
    Noop,
}

pub fn divide(numerator: Price, denominator: Price) -> Instruction {
    let mut data = vec![];
    PythClientInstruction::Divide {
        numerator,
        denominator,
    }
    .serialize(&mut data)
    .unwrap();

    Instruction {
        data,
        program_id: id(),
        accounts: vec![],
    }
}

pub fn multiply(x: Price, y: Price) -> Instruction {
    let mut data = vec![];
    PythClientInstruction::Multiply { x, y }
        .serialize(&mut data)
        .unwrap();

    Instruction {
        data,
        program_id: id(),
        accounts: vec![],
    }
}

pub fn add(x: Price, y: Price) -> Instruction {
    let mut data = vec![];
    PythClientInstruction::Add { x, y }
        .serialize(&mut data)
        .unwrap();

    Instruction {
        program_id: id(),
        accounts: vec![],
        data,
    }
}

pub fn scale_to_exponent(x: Price, expo: i32) -> Instruction {
    let mut data = vec![];
    PythClientInstruction::ScaleToExponent { x, expo }
        .serialize(&mut data)
        .unwrap();

    Instruction {
        data,
        program_id: id(),
        accounts: vec![],
    }
}

pub fn normalize(x: Price) -> Instruction {
    let mut data = vec![];
    PythClientInstruction::Normalize { x }
        .serialize(&mut data)
        .unwrap();

    Instruction {
        program_id: id(),
        accounts: vec![],
        data,
    }
}

/// Noop instruction for comparison purposes
pub fn noop() -> Instruction {
    let mut data = vec![];
    PythClientInstruction::Noop.serialize(&mut data).unwrap();

    Instruction {
        data,
        program_id: id(),
        accounts: vec![],
    }
}
