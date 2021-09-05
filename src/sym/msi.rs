//! Encoder for MSI
//!
//! MSI is used primarily for inventory control, marking storage containers and shelves in 
//! warehouse environments.
//! https://en.wikipedia.org/wiki/MSI_Barcode

use sym::{Parse, helpers};
use error::*;
use std::ops::Range;

/// The MSI barcode type.
#[derive(Debug)]
pub struct MSI(Vec<u8>);

/// The left-hand guard pattern.
pub const LEFT_GUARD: [u8; 3] = [1, 1, 0];

/// The right-hand guard pattern.
pub const RIGHT_GUARD: [u8; 4] = [1, 0, 0, 1];

/// 0-9
pub const ENCODINGS: [[u8; 12]; 10] = [
	[1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0], // 0
	[1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0], // 1
	[1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0], // 2
	[1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0], // 3
	[1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0], // 4
	[1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0], // 5
	[1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0], // 6
	[1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0], // 7
	[1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0], // 8
	[1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0], // 9
];

pub const MOD_10: u8 = 10u8;

impl MSI {

	/// Creates a new barcode.
    /// Returns Result<Code128, Error> indicating parse success.
	pub fn new<T: AsRef<str>>(data: T) -> Result<MSI> {
		MSI::parse(data.as_ref()).and_then(|d| {
            let digits = d.chars()
				.map(|c| c.to_digit(10).expect("Unknown character") as u8)
				.collect();
            Ok(MSI(digits))
        })
	}

	pub fn encode_mod10(&self) -> u8 {
		let mut sum: u32 = 0;
		for i in (0 .. self.0.len()).rev() {
			if i % 2 == 0 {
				let multi_2: u8 = *self.0.get(i).unwrap() * 2;
				sum += (multi_2 / MOD_10 + multi_2 % MOD_10) as u32; 
			} else {
				sum += *self.0.get(i).unwrap() as u32;
			}
		}
		MOD_10 - (sum % MOD_10 as u32) as u8
	}

	/// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> Vec<u8> {
    	let mut payload: Vec<u8> = vec![];
    	for b in self.0.iter() {
    		payload.extend_from_slice(&ENCODINGS[*b as usize]);
    	}
    	let check: u8 = self.encode_mod10();
        helpers::join_slices(
        	&[
        		&LEFT_GUARD[..],
        		&payload,
        		&ENCODINGS[check as usize],
        		&RIGHT_GUARD[..],
        	],
       	)
    }
}

impl Parse for MSI {

	/// Returns the valid length of data acceptable in this type of barcode.
	/// MSI has no fixed length. Cap it 5..50 for now.
    fn valid_len() -> Range<u32> {
        1..50
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    /// MSI can display only the number 0-9
    fn valid_chars() -> Vec<char> {
        (0..9).map(|i| char::from_digit(i, 9).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {

	use ::sym::msi::{MSI, ENCODINGS};

	#[test]
    fn msi_encode() {
        let msi_0 = MSI::new("01").unwrap();
        let msi_0_encoded = msi_0.encode();
        let msi_0_encoded_mod10 = msi_0.encode_mod10();
        println!("!!! msi_0 {:?}", msi_0);
        println!("!!! msi_0_encoded {:?}", msi_0_encoded);
        println!("!!! msi_0_encoded_mod10 {:?}", msi_0_encoded_mod10);
        println!("!!! msi_0_encoded_mod10 {:?}", ENCODINGS[msi_0_encoded_mod10 as usize]);
    }
}