use crate::{check_args_len, FromRaw, Serialize};

const GLOBAL_CONFIG_DATA_LEN: usize = 296;

/**

    Global config cell only contains data

    Global Config Cell
    Data:
    Type:
        codehash: typeid                // A.S.
        hashtype: type                  // data
        args: unique_id                 // null
    Lock:
        codehash: secp256k1
        args: admin
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct GlobalConfigCellData {
    pub admin_public_key:        [u8; 32],
    /* this is the authenticated admin for
     * sidechain config cell */
    pub code_cell_type_codehash: [u8; 32],
    pub code_cell_type_hashtype: u8,

    pub sidechain_config_cell_type_codehash: [u8; 32],
    pub sidechain_config_cell_type_hashtype: u8,

    pub sidechain_state_cell_type_codehash: [u8; 32],
    pub sidechain_state_cell_type_hashtype: u8,

    pub checker_info_cell_type_codehash: [u8; 32],
    pub checker_info_cell_type_hashtype: u8,

    pub checker_bond_cell_lock_codehash: [u8; 32],
    pub checker_bond_cell_lock_hashtype: u8,

    pub task_cell_type_codehash: [u8; 32],
    pub task_cell_type_hashtype: u8,

    pub sidechain_fee_cell_lock_codehash: [u8; 32],
    pub sidechain_fee_cell_lock_hashtype: u8,

    pub sidechain_bond_cell_lock_codehash: [u8; 32],
    pub sidechain_bond_cell_lock_hashtype: u8,
}

impl FromRaw for GlobalConfigCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<GlobalConfigCellData> {
        check_args_len(cell_raw_data.len(), GLOBAL_CONFIG_DATA_LEN)?;

        let mut admin_public_key = [0u8; 32];
        admin_public_key.copy_from_slice(&cell_raw_data[0..32]);

        let mut code_cell_type_codehash = [0u8; 32];
        code_cell_type_codehash.copy_from_slice(&cell_raw_data[32..64]);
        let code_cell_type_hashtype = u8::from_raw(&cell_raw_data[64..65])?;

        let mut sidechain_config_cell_type_codehash = [0u8; 32];
        sidechain_config_cell_type_codehash.copy_from_slice(&cell_raw_data[65..97]);
        let sidechain_config_cell_type_hashtype = u8::from_raw(&cell_raw_data[97..98])?;

        let mut sidechain_state_cell_type_codehash = [0u8; 32];
        sidechain_state_cell_type_codehash.copy_from_slice(&cell_raw_data[98..130]);
        let sidechain_state_cell_type_hashtype = u8::from_raw(&cell_raw_data[130..131])?;

        let mut checker_info_cell_type_codehash = [0u8; 32];
        checker_info_cell_type_codehash.copy_from_slice(&cell_raw_data[131..163]);
        let checker_info_cell_type_hashtype = u8::from_raw(&cell_raw_data[163..164])?;

        let mut checker_bond_cell_lock_codehash = [0u8; 32];
        checker_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[164..196]);
        let checker_bond_cell_lock_hashtype = u8::from_raw(&cell_raw_data[196..197])?;

        let mut task_cell_type_codehash = [0u8; 32];
        task_cell_type_codehash.copy_from_slice(&cell_raw_data[197..229]);
        let task_cell_type_hashtype = u8::from_raw(&cell_raw_data[229..230])?;

        let mut sidechain_fee_cell_lock_codehash = [0u8; 32];
        sidechain_fee_cell_lock_codehash.copy_from_slice(&cell_raw_data[230..262]);
        let sidechain_fee_cell_lock_hashtype = u8::from_raw(&cell_raw_data[262..263])?;

        let mut sidechain_bond_cell_lock_codehash = [0u8; 32];
        sidechain_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[263..295]);
        let sidechain_bond_cell_lock_hashtype = u8::from_raw(&cell_raw_data[295..296])?;

        Some(GlobalConfigCellData {
            admin_public_key,
            code_cell_type_codehash,
            code_cell_type_hashtype,
            sidechain_config_cell_type_codehash,
            sidechain_config_cell_type_hashtype,
            sidechain_state_cell_type_codehash,
            sidechain_state_cell_type_hashtype,
            checker_info_cell_type_codehash,
            checker_info_cell_type_hashtype,
            checker_bond_cell_lock_codehash,
            checker_bond_cell_lock_hashtype,
            task_cell_type_codehash,
            task_cell_type_hashtype,
            sidechain_fee_cell_lock_codehash,
            sidechain_fee_cell_lock_hashtype,
            sidechain_bond_cell_lock_codehash,
            sidechain_bond_cell_lock_hashtype,
        })
    }
}

impl Serialize for GlobalConfigCellData {
    type RawType = [u8; GLOBAL_CONFIG_DATA_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; GLOBAL_CONFIG_DATA_LEN];

        buf[0..32].copy_from_slice(&self.admin_public_key);

        buf[32..64].copy_from_slice(&self.code_cell_type_codehash);

        buf[64..65].copy_from_slice(&self.code_cell_type_hashtype.serialize());

        buf[65..97].copy_from_slice(&self.sidechain_config_cell_type_codehash);
        buf[97..98].copy_from_slice(&self.sidechain_config_cell_type_hashtype.serialize());

        buf[98..130].copy_from_slice(&self.sidechain_state_cell_type_codehash);
        buf[130..131].copy_from_slice(&self.sidechain_state_cell_type_hashtype.serialize());

        buf[131..163].copy_from_slice(&self.checker_info_cell_type_codehash);
        buf[163..164].copy_from_slice(&self.checker_info_cell_type_hashtype.serialize());

        buf[164..196].copy_from_slice(&self.checker_bond_cell_lock_codehash);
        buf[196..197].copy_from_slice(&self.checker_bond_cell_lock_hashtype.serialize());

        buf[197..229].copy_from_slice(&self.task_cell_type_codehash);
        buf[229..230].copy_from_slice(&self.task_cell_type_hashtype.serialize());

        buf[230..262].copy_from_slice(&self.sidechain_fee_cell_lock_codehash);
        buf[262..263].copy_from_slice(&self.sidechain_fee_cell_lock_hashtype.serialize());

        buf[263..295].copy_from_slice(&self.sidechain_bond_cell_lock_codehash);
        buf[295..296].copy_from_slice(&self.sidechain_bond_cell_lock_hashtype.serialize());

        buf
    }
}
