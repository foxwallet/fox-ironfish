/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
pub use bls12_381::Scalar;
use crate::Sapling;
use once_cell::sync::OnceCell;

pub static SAPLING: OnceCell<Sapling> = OnceCell::new();

pub struct SaplingWrapper {}

impl SaplingWrapper {
    pub fn global() -> &'static Sapling {
        SAPLING.get().expect("Sapling is not initialized")
    }

    pub fn load(mint_params_path: String, spend_params_path: String, output_params_path: String) -> Result<bool, String> {
        SAPLING.get_or_try_init(|| -> Result<Sapling, String> {
            Sapling::load(mint_params_path, spend_params_path, output_params_path)
        }).map(|_| true)
    }
}
