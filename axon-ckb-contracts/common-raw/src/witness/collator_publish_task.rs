use crate::{pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CollatorPublishTaskWitness {
    pattern:      Pattern,
    pub chain_id: u8,
    pub bond:     u128,
}

impl FromRaw for CollatorPublishTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorPublishTaskWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let bond = u128::from_raw(&witness_raw_data[2..18])?;

        Some(CollatorPublishTaskWitness { pattern, chain_id, bond })
    }
}
