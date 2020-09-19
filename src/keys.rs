// Bitcoin Dev Kit Library
//
// Copyright (c) 2020 Bitcoin Dev Kit
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Seed Mnemonic Generation and Extended Private and Public Key Creation
//!
//! TEMPORARY! this will be removed once similar functionality is added to the `bdk` lib
//!
//! This module provides functions to generate a random seed mnemonic, use a generated seed
//! mnemonic to create an extended private key, and create an extended public key from an extended
//! private key.
//!

use bdk::bitcoin::Network;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{Error as Bip32Error, ExtendedPrivKey, ExtendedPubKey};
use bip39::{Error as Bip39Error, Language, Mnemonic};
use rand::RngCore;

/// Generate a random 24 word seed mnemonic code
pub fn generate_mnemonic(word_count: usize) -> Result<Mnemonic, Bip39Error> {
    Mnemonic::generate(word_count)
}

/// Create extended private key from mnemonic words
pub fn create_ext_priv_key(network: Network, mnemonic: &Mnemonic) -> Result<ExtendedPrivKey, Bip32Error> {
    let seed = mnemonic.to_seed("");
    ExtendedPrivKey::new_master(network, &seed)
}

/// Create random extended private key
pub fn create_rand_ext_priv_key(network: Network, entropy_bytes: usize) -> Result<ExtendedPrivKey, Bip32Error> {

    let mut rng = rand::thread_rng();
    let mut entropy = vec![0u8; entropy_bytes];
    RngCore::fill_bytes(&mut rng, &mut entropy);

    ExtendedPrivKey::new_master(network, &entropy)
}

/// Create extended public key from extended private key
pub fn create_ext_pub_key(ext_prv_key: &ExtendedPrivKey) -> ExtendedPubKey {
    ExtendedPubKey::from_private(&Secp256k1::new(), ext_prv_key)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use bdk::bitcoin::Network::Testnet;
    use bdk::electrum_client::bitcoin::util::bip32::ExtendedPrivKey;
    use bip39::Mnemonic;

    use crate::keys::{create_ext_priv_key, create_ext_pub_key, create_rand_ext_priv_key, generate_mnemonic};

    #[test]
    fn test_generate_mnemonic() {

        // Verify generated random bip39 mnemonic words are the expected count
        let mnemonic = generate_mnemonic(24).unwrap();
        assert_eq!(mnemonic.as_str().split_whitespace().count(), 24);
    }

    #[test]
    fn test_create_ext_priv_from_mnemonic() {

        let mnemonic = Mnemonic::from_str("shell bid diary primary focus average truly secret lonely circle radar fall tank action place body wedding sponsor embody glue swing gauge shop penalty").unwrap();

        // Verify created extended private key from mnemonic
        let tprv = create_ext_priv_key(Testnet, &mnemonic).unwrap();
        assert_eq!(tprv.to_string(), "tprv8ZgxMBicQKsPeh5nd4nCDLGh9dLfhqGfUoiQsbThkttjX9oroRY2j5vpEGwkiKiKtzdU7u4eqH2yFicGvz19rMVVXfY8XB9fdoeXWJ7SgVE");
    }

    #[test]
    fn test_create_rand_ext_priv() {

        // Verify two random private key are different
        let tprv1 = create_rand_ext_priv_key(Testnet, 32).unwrap();
        let tprv2 = create_rand_ext_priv_key(Testnet, 32).unwrap();
        assert_ne!(tprv1, tprv2);
    }

    #[test]
    fn test_create_ext_pub_from_priv() {

        let tprv = ExtendedPrivKey::from_str("tprv8ZgxMBicQKsPeh5nd4nCDLGh9dLfhqGfUoiQsbThkttjX9oroRY2j5vpEGwkiKiKtzdU7u4eqH2yFicGvz19rMVVXfY8XB9fdoeXWJ7SgVE").unwrap();

        // Verify created extended public key from extended private key
        let tpub = create_ext_pub_key(&tprv);
        assert_eq!(tpub.to_string(), "tpubD6NzVbkrYhZ4YA7aWiSncjvoierbsATa47KCA7W1BAh8Me4dRpMcuaYgQSoHMctaWGmEGx6qLU557xykguQLMSwv4H72fktpFuVcdjQ1hgw");
    }
}
