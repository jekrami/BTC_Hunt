/// Verification tool to check that mnemonic derivation matches expected output
/// Usage: cargo run --bin verify_mnemonic -- "your mnemonic phrase here"

use bip39::Mnemonic;
use bitcoin::address::Address;
use bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::key::Secp256k1;
use bitcoin::network::constants::Network;
use bitcoin::PrivateKey;
use hex;
use std::env;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin verify_mnemonic -- \"your mnemonic phrase here\"");
        eprintln!("\nExample:");
        eprintln!("cargo run --bin verify_mnemonic -- \"motor venture dilemma quote subject magnet keep large dry gossip bean paper\"");
        std::process::exit(1);
    }

    let mnemonic_str = args[1..].join(" ");
    
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          BTC Hunt - Mnemonic Verification Tool            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let secp = Secp256k1::new();

    // Parse mnemonic
    let mnemonic = Mnemonic::from_str(&mnemonic_str)
        .map_err(|e| format!("Failed to parse mnemonic: {}", e))?;

    // Generate seed
    let seed = mnemonic.to_seed("");

    // Create master key
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
        .map_err(|e| format!("Failed to create master key: {}", e))?;

    println!("BIP39 Mnemonic: {}\n", mnemonic_str);
    println!("BIP39 Seed: {}\n", hex::encode(&seed));
    println!("Coin: BTC\n");
    println!("Derivation Path and outputs\n");
    println!("path,address,public key,private key\n");

    // Define derivation paths (matching old code format)
    let base_paths = vec![
        ("m/0'/0'", "hardened", "P2PKH", 2),
        ("m/44'/0'/0'", "normal_hardened", "P2PKH", 2),
        ("m/49'/0'/0'", "normal_hardened", "P2WPKH nested in P2SH", 2),
        ("m/84'/0'/0'", "normal_hardened", "P2WPKH", 2),
    ];

    for (base_path, suffix_pattern, script_semantics, count) in base_paths {
        for i in 0..count {
            let path_str = match suffix_pattern {
                "hardened" => format!("{}/{}'", base_path, i),
                "normal_hardened" => format!("{}/0/{}'", base_path, i),
                _ => continue,
            };

            let path = DerivationPath::from_str(&path_str)?;
            let child_key = master_key.derive_priv(&secp, &path)?;
            let private_key = PrivateKey::new(child_key.private_key, Network::Bitcoin);
            let public_key = private_key.public_key(&secp);

            let address = match script_semantics {
                "P2PKH" => Address::p2pkh(&public_key, Network::Bitcoin),
                "P2WPKH nested in P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)?,
                "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)?,
                _ => continue,
            };

            println!(
                "{},{},{},{}",
                path_str,
                address,
                public_key,
                private_key.to_wif()
            );
        }
        println!();
    }

    // Additional paths
    let additional_paths = vec![
        ("m/0", "hardened", "P2WPKH nested in P2SH", 2),
        ("m/0", "hardened", "P2WPKH", 2),
    ];

    for (base_path, suffix_pattern, script_semantics, count) in additional_paths {
        println!("Script Semantics: {}", script_semantics);
        for i in 0..count {
            let path_str = match suffix_pattern {
                "hardened" => format!("{}/{}'", base_path, i),
                _ => continue,
            };

            let path = DerivationPath::from_str(&path_str)?;
            let child_key = master_key.derive_priv(&secp, &path)?;
            let private_key = PrivateKey::new(child_key.private_key, Network::Bitcoin);
            let public_key = private_key.public_key(&secp);

            let address = match script_semantics {
                "P2WPKH nested in P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)?,
                "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)?,
                _ => continue,
            };

            println!(
                "{},{},{},{}",
                path_str,
                address,
                public_key,
                private_key.to_wif()
            );
        }
        println!();
    }

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    Verification Complete                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    Ok(())
}



