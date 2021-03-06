#![cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#![no_std]
extern crate aesni;

macro_rules! impl_test {
    ($name:ident, $cipher:ident, $data:expr) => {
        #[test]
        fn $name() {
            let key = include_bytes!(concat!("data/", $data, ".key.bin"));
            let iv = include_bytes!(concat!("data/", $data, ".iv.bin"));
            let plaintext = include_bytes!(
                concat!("data/", $data, ".plaintext.bin"));
            let ciphertext = include_bytes!(
                concat!("data/", $data, ".ciphertext.bin"));

            let mut mode = aesni::$cipher::new(key, iv);
            let mut pt = plaintext.to_vec();
            {
                let (a, t) = pt.split_at_mut(17);
                let (b, c) = t.split_at_mut(5);
                mode.xor(a);
                mode.xor(b);
                mode.xor(c);
            }
            assert_eq!(pt, &ciphertext[..]);
        }
    }
}

// Random tests generated by OpenSSL
impl_test!(aes128_ctr, CtrAes128, "ctr_aes128");
impl_test!(aes256_ctr, CtrAes256, "ctr_aes256");
