use core::convert::{TryFrom, TryInto};
use core::result::Result;

use crate::{check_args_len, FromRaw, Serialize};

const TASK_DATA_LEN: usize = 85;
const TASK_TYPE_ARGS_LEN: usize = 1;
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum TaskCellMode {
    Task = 0,
    Challenge,
}

impl Default for TaskCellMode {
    fn default() -> Self {
        TaskCellMode::Task
    }
}

impl TryFrom<u8> for TaskCellMode {
    type Error = ();

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0u8 => Ok(Self::Task),
            1u8 => Ok(Self::Challenge),
            _ => Err(()),
        }
    }
}

/**
    Task Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct TaskCellData {
    pub chain_id:                u8,
    pub version:                 u8,
    pub check_block_height_from: u128,
    // 应该为ssc committed_height + 1
    pub check_block_height_to:   u128,
    // inclusive 应该为latest_height
    pub check_block_hash_to:     [u8; 32],
    pub check_data_size:         u128,
    pub refresh_interval:        u16,
    pub mode:                    TaskCellMode,
}

impl FromRaw for TaskCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<TaskCellData> {
        check_args_len(cell_raw_data.len(), TASK_DATA_LEN)?;

        let chain_id = u8::from_raw(&cell_raw_data[0..1])?;
        let version = u8::from_raw(&cell_raw_data[1..2])?;
        let check_block_height_from = u128::from_raw(&cell_raw_data[2..18])?;
        let check_block_height_to = u128::from_raw(&cell_raw_data[18..34])?;

        let mut check_block_hash_to = [0u8; 32];
        check_block_hash_to.copy_from_slice(&cell_raw_data[34..66]);

        let check_data_size = u128::from_raw(&cell_raw_data[66..82])?;
        let refresh_interval = u16::from_raw(&cell_raw_data[82..84])?;

        let mode_u8 = u8::from_raw(&cell_raw_data[84..85])?;
        let mode: TaskCellMode = mode_u8.try_into().ok()?;

        Some(TaskCellData {
            chain_id,
            version,
            check_block_height_from,
            check_block_height_to,
            check_block_hash_to,
            check_data_size,
            refresh_interval,
            mode,
        })
    }
}

impl Serialize for TaskCellData {
    type RawType = [u8; TASK_DATA_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; TASK_DATA_LEN];

        buf[0..1].copy_from_slice(&self.chain_id.serialize());
        buf[1..2].copy_from_slice(&self.version.serialize());
        buf[2..18].copy_from_slice(&self.check_block_height_from.serialize());
        buf[18..34].copy_from_slice(&self.check_block_height_to.serialize());

        buf[34..66].copy_from_slice(&self.check_block_hash_to);

        buf[66..82].copy_from_slice(&self.check_data_size.serialize());
        buf[82..84].copy_from_slice(&self.refresh_interval.serialize());

        buf[84..85].copy_from_slice(&(self.mode as u8).serialize());

        buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct TaskCellTypeArgs {
    pub chain_id: u8,
}

impl FromRaw for TaskCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<TaskCellTypeArgs> {
        check_args_len(arg_raw_data.len(), TASK_TYPE_ARGS_LEN)?;

        let chain_id = u8::from_raw(&arg_raw_data[0..1])?;

        Some(TaskCellTypeArgs { chain_id })
    }
}
