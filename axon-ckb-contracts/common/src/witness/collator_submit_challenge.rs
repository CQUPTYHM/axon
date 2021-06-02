use core::convert::{TryFrom, TryInto};
use core::result::Result;

use ckb_std::error::SysError;

use crate::error::CommonError;
use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};
use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::prelude::{Entity, Unpack};
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};

#[derive(Debug)]
pub struct CollatorSubmitChallengeWitness {
    pub pattern:               u8,
    pub chain_id:              u8,
    pub fee:                   u128,
    pub fee_per_checker:       u128,
    pub punish_checker_bitmap: [u8; 32],
}

impl FromRaw for CollatorSubmitChallengeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<CollatorSubmitChallengeWitness, SysError> {
        if witness_raw_data.len() < 2 {
            return Err(SysError::Encoding);
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;
        let fee = decode_u128(&witness_raw_data[2..18])?;
        let fee_per_checker = decode_u128(&witness_raw_data[18..34])?;

        let mut punish_checker_bitmap = [0u8; 32];
        punish_checker_bitmap.copy_from_slice(&witness_raw_data[34..66]);

        Ok(CollatorSubmitChallengeWitness {
            pattern,
            chain_id,
            fee,
            fee_per_checker,
            punish_checker_bitmap,
        })
    }
}