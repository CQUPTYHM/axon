use core::result::Result;



use crate::error::Error;

use common::pattern::{check_code_cell};







pub fn main() -> Result<(), Error> {
    check_code_cell()?;
    Ok(())
}