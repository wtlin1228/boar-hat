#![feature(maybe_uninit_slice)]
use std::mem::MaybeUninit;

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::sodium_init;

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Sodium;

impl Sodium {
    pub fn new() -> Result<Self, ()> {
        if unsafe { ffi::sodium_init() } < 0 {
            Err(())
        } else {
            Ok(Self)
        }
    }

    pub fn crypto_generichash<'a>(
        self,
        input: &[u8],
        key: Option<&[u8]>,
        out: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut [u8], ()> {
        assert!(out.len() >= usize::try_from(ffi::crypto_generichash_BYTES_MIN).unwrap());
        assert!(out.len() <= usize::try_from(ffi::crypto_generichash_BYTES_MAX).unwrap());
        if let Some(key) = key {
            assert!(key.len() >= usize::try_from(ffi::crypto_generichash_KEYBYTES_MIN).unwrap());
            assert!(key.len() <= usize::try_from(ffi::crypto_generichash_KEYBYTES_MAX).unwrap());
        }
        let (key, keylen) = if let Some(key) = key {
            (key.as_ptr(), key.len())
        } else {
            (std::ptr::null(), 0)
        };
        let res = unsafe {
            ffi::crypto_generichash(
                MaybeUninit::slice_as_mut_ptr(out),
                out.len(),
                input.as_ptr(),
                input.len() as u64,
                key,
                keylen,
            )
        };
        if res < 0 {
            return Err(());
        }
        Ok(unsafe { MaybeUninit::slice_assume_init_mut(out) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Sodium::new().unwrap();
    }

    #[test]
    fn it_hashes() {
        let s = Sodium::new().unwrap();
        let mut out = [MaybeUninit::uninit(); ffi::crypto_generichash_BYTES as usize];
        let bytes = s
            .crypto_generichash(b"Arbitrary data to hash", None, &mut out)
            .unwrap();
        assert_eq!(
            "3dc7925e13e4c5f0f8756af2cc71d5624b58833bb92fa989c3e87d734ee5a600",
            hex::encode(&bytes)
        );
    }
}
