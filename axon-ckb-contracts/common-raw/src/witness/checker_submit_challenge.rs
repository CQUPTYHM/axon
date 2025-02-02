use crate::{pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CheckerSubmitChallengeWitness {
    pattern:        Pattern,
    pub chain_id:   u8,
    pub checker_id: u8,
}

impl FromRaw for CheckerSubmitChallengeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerSubmitChallengeWitness> {
        if witness_raw_data.len() < 3 {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let checker_id = u8::from_raw(&witness_raw_data[2..3])?;

        Some(CheckerSubmitChallengeWitness {
            pattern,
            chain_id,
            checker_id,
        })
    }
}
