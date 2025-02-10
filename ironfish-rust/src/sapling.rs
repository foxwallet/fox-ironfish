/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use blstrs::Bls12;
use ironfish_bellperson::groth16;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(not(doc))]
macro_rules! include_params {
    ( $name:literal ) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/sapling_params/", $name))
    };
}

// When building documentation (especially on docs.rs), it's quite possible that the parameter
// files won't be available, so don't even attempt to include them. This will also speed up
// documentation builds.
#[cfg(doc)]
macro_rules! include_params {
    ( $name:literal ) => {
        b""
    };
}

// static SAPLING_SPEND_PARAMS: &[u8] = include_params!("sapling-spend.params");
// static SAPLING_OUTPUT_PARAMS: &[u8] = include_params!("sapling-output.params");
// static SAPLING_MINT_PARAMS: &[u8] = include_params!("sapling-mint.params");

// The main entry-point to the sapling API. Construct this with loaded parameters, and then call
// methods on it to do the actual work.
//
// spend and output are two arithmetic circuits for use in zksnark calculations provided by bellperson.
// Though the *_params have a verifying key on them, they are not the prepared verifying keys,
// so we store the prepared keys separately at the time of loading the params.
//
// The values are all loaded from a file in serialized form.
pub struct Sapling {
    pub spend_params: groth16::Parameters<Bls12>,
    pub output_params: groth16::Parameters<Bls12>,
    pub mint_params: groth16::Parameters<Bls12>,
    pub spend_verifying_key: groth16::PreparedVerifyingKey<Bls12>,
    pub output_verifying_key: groth16::PreparedVerifyingKey<Bls12>,
    pub mint_verifying_key: groth16::PreparedVerifyingKey<Bls12>,
}

impl Sapling {
    fn load_file(file_path: &str) -> Result<Vec<u8>, String> {
        let path = Path::new(&file_path);
        let mut file = File::open(&path).map_err(|e| e.to_string())?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).map_err(|e| e.to_string())?;
        Ok(data)
    }

    /// Initialize a Sapling instance and prepare for proving. Load the parameters from files
    /// at a known location (`$OUT_DIR/sapling_params`).
    pub fn load(mint_params_path: String, spend_params_path: String, output_params_path: String) -> Result<Self, String> {
        let spend_bytes = Sapling::load_file(&spend_params_path)?;
        let output_bytes = Sapling::load_file(&output_params_path)?;
        let mint_bytes = Sapling::load_file(&mint_params_path)?;

        let spend_params = Sapling::load_params(&spend_bytes[..]);
        let output_params = Sapling::load_params(&output_bytes[..]);
        let mint_params = Sapling::load_params(&mint_bytes[..]);

        let spend_verifying_key = groth16::prepare_verifying_key(&spend_params.vk);
        let output_verifying_key = groth16::prepare_verifying_key(&output_params.vk);
        let mint_verifying_key = groth16::prepare_verifying_key(&mint_params.vk);

        Ok(Sapling {
            spend_verifying_key,
            output_verifying_key,
            mint_verifying_key,
            spend_params,
            output_params,
            mint_params,
        })
    }

    /// Load sapling parameters from a provided filename. The parameters are huge and take a
    /// couple seconds to load. They primarily contain the "toxic waste" for a specific sapling
    /// curve.
    ///
    /// NOTE: If this is stupidly slow for you, try compiling in --release mode
    fn load_params(bytes: &[u8]) -> groth16::Parameters<Bls12> {
        groth16::Parameters::read(bytes, false).unwrap()
    }
}
