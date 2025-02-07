use num_bigint::BigUint;
use num_traits::{ToPrimitive};//, FromPrimitive};
use super::hash::Hash;

/// Convert a `Hash` into a `BigUint` (interpreted as a little-endian integer).
pub fn biguint_from_hash(h: &Hash) -> BigUint {
    BigUint::from_bytes_le(h.as_bytes())
}

/// Convert a `BigUint` to its compact representation
/*
pub fn compact_from_biguint(value: BigUint) -> u32 {
    let big_lim = BigUint::from(0x00800000u32); // 24-bit boundary
    let mut tvalue = value.clone();
    let mut exponent: u32 = 0;

    // Shift right until the value fits within 24 bits
    while tvalue >= big_lim {
        tvalue >>= 8;
        exponent += 1;
    }

    // Extract the mantissa and shift the value back if necessary
    let mantissa = tvalue.to_u32().unwrap_or(0);
    if mantissa & 0x00800000 != 0 {
        // If mantissa overflows 24 bits, adjust and increment exponent
        tvalue >>= 8;
        exponent += 1;
    }

    // Compact format: [exponent (8 bits)][mantissa (24 bits)]
    (exponent << 24) | (mantissa & 0x007fffff)
}
*/
pub fn compact_from_biguint(target_value: BigUint) -> u32 {
    let target=&target_value;
    let bytes = target.to_bytes_be();
    if bytes.is_empty() {
        return 0; // Zero target
    }
    
    // Determine exponent: position of the most significant byte
    let exponent = bytes.len() as u32;
    
    // Get the most significant 3 bytes (coefficient)
    let mut coefficient: u32;
    if exponent >= 3 {
        // Take the first 3 bytes from the beginning
        coefficient = u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]);
    } else {
        // For targets smaller than 3 bytes, pad with zeros
        let mut padded = vec![0u8; 3];
        let start = 3 - bytes.len();
        padded[start..].copy_from_slice(&bytes);
        coefficient = u32::from_be_bytes([0, padded[0], padded[1], padded[2]]);
    }
    
    // If the highest bit of the coefficient is set, adjust exponent and shift coefficient
    if coefficient & 0x00800000 != 0 {
        coefficient >>= 8;
        // Only increment exponent if we're not already at max value
        if exponent < 0xFF {
            exponent + 1;
        }
    }
    
    // Encode the result as nBits (8-bit exponent + 24-bit coefficient)
    (exponent << 24) | (coefficient & 0x007FFFFF)
}

/// Convert a compact representation back into a `BigUint`.
/// This is the inverse of `compact_from_biguint`.
/*
pub fn biguint_from_compact(compact: u32) -> BigUint {
    let mantissa = compact & 0x007fffff;
    let exponent = (compact >> 24) as u32;

    let mut value = BigUint::from(mantissa);
    if exponent > 3 {
        value <<= 8 * (exponent - 3);
    } else {
        value >>= 8 * (3 - exponent);
    }
    value
}
*/

pub fn biguint_from_compact(bits: u32) -> BigUint {
    let exponent = (bits >> 24) as usize;
    let coefficient = bits & 0x007FFFFF;

    // Handle special case of zero
    if coefficient == 0 {
        return BigUint::from(0u32);
    }

    // Convert the 24-bit coefficient to a BigUint
    let mut target = BigUint::from(coefficient);

    if exponent <= 3 {
        // For small exponents, right shift
        target >>= 8 * (3 - exponent);
    } else {
        // For larger exponents, left shift
        target <<= 8 * (exponent - 3);
    }

    target
}
/// Convert a 64-bit integer to a `BigUint`.
pub fn biguint_from_u64(value: u64) -> BigUint {
    BigUint::from(value)
}
/// Convert a `BigUint` to a 64-bit integer 
pub fn biguint_squashed_to_f64(value: &BigUint) -> Option<f64> {
    let very_big = BigUint::from(10 as u32).pow(300);

    let result = value / &very_big;
    result.to_f64()
}