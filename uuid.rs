#[link(name = "uuid", vers = "0.1", uuid = "bdad0b69-95e3-4729-b06a-0347df720675")];

#[license = "MIT"];
#[crate_type = "lib"];

#[author = "Jens Nockert"];

#[comment = "Generate random UUIDs with Rust"];
#[desc = "A small struct that allows you to generate UUIDs in Rust"];

use std::io;
use std::str;
use std::str::ToStr;
use std::u64;
use std::cast;
use std::rand;
use std::rand::Rng;

use std::unstable::intrinsics;

pub struct UUID {
    value: [u8, ..16]
}

pub enum Variant {
    NCS = 0,
    DCE = 2,
    Microsoft = 6,
    Unknown = 7
}

impl UUID {
    pub fn null() -> UUID {
        UUID::from_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
    }

    pub fn from_string(value:~str) -> Option<UUID> {
        if (value.len() != 36) { return None; }

        let hex = str::replace(value, "-", "");

        if (hex.len() != 32) { return None; }

        let a = u64::from_str_radix(hex.slice( 0, 16), 16);
        let b = u64::from_str_radix(hex.slice(16, 32), 16);
        
        match (a, b) {
            (Some(av), Some(bv)) => {
                // if (little_endian) {
                let (x, y) = unsafe { (intrinsics::bswap64(av as i64), intrinsics::bswap64(bv as i64)) };
                // else {
                //   let (x, y) = (av, bv);
                // }
                Some(UUID::from_bytes(unsafe { cast::transmute([x, y]) }))
            }
            _ => None
        }
    }
    
    pub fn from_bytes(value:[u8, ..16]) -> UUID {
        UUID { value: value }
    }

    pub fn random() -> UUID {
        let mut rng = rand::IsaacRng::new();

        let mut bytes:[u8, ..16] = unsafe { cast::transmute([rng.next(), rng.next(), rng.next(), rng.next()]) };

        bytes[6] = (bytes[6] & 0x0F) | 0x40;
        bytes[8] = (bytes[8] & 0x3F) | 0x80;

        UUID::from_bytes(bytes)
    }

    pub fn is_null(&self) -> bool {
        self == &UUID::null()
    }

    pub fn version(&self) -> int {
        ((self.value[6] & 0xF0) >> 4) as int
    }

    pub fn variant(&self) -> Variant {
        if ((self.value[8] & 0x80) == 0) {
            NCS
        } else if ((self.value[8] & 0x40) == 0) {
            DCE
        } else if ((self.value[8] & 0x20) == 0) {
            Microsoft
        } else {
            Unknown
        }
    }

    pub fn to_lowercase_string(&self) -> ~str {
        let x:[u8, ..16] = unsafe { cast::transmute(self.value) };
        fmt!("%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
             x[ 0] as uint, x[ 1] as uint, x[ 2] as uint, x[ 3] as uint,
             x[ 4] as uint, x[ 5] as uint, x[ 6] as uint, x[ 7] as uint,
             x[ 8] as uint, x[ 9] as uint, x[10] as uint, x[11] as uint,
             x[12] as uint, x[13] as uint, x[14] as uint, x[15] as uint)
    }

    pub fn to_uppercase_string(&self) -> ~str {
        let x:[u8, ..16] = unsafe { cast::transmute(self.value) };
        fmt!("%02X%02X%02X%02X-%02X%02X-%02X%02X-%02X%02X-%02X%02X%02X%02X%02X%02X",
             x[ 0] as uint, x[ 1] as uint, x[ 2] as uint, x[ 3] as uint,
             x[ 4] as uint, x[ 5] as uint, x[ 6] as uint, x[ 7] as uint,
             x[ 8] as uint, x[ 9] as uint, x[10] as uint, x[11] as uint,
             x[12] as uint, x[13] as uint, x[14] as uint, x[15] as uint)
    }
}

impl Clone for UUID {
    pub fn clone(&self) -> UUID {
        UUID { value: self.value }
    }
}

impl Eq for UUID {
    #[inline(always)] pub fn eq(&self, other: &UUID) -> bool { self.value == other.value }
    #[inline(always)] pub fn ne(&self, other: &UUID) -> bool { self.value != other.value }
}

impl ToStr for UUID {
    fn to_str(&self) -> ~str {
        self.to_lowercase_string()
    }
}