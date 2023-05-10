use clap::ValueEnum;
use strum_macros::EnumIter;

pub const INITIAL_BALANCE: u64 = 9999;

#[derive(ValueEnum, Clone, Copy, Debug, EnumIter)]
#[value(rename_all = "verbatim")]
pub enum SeedIdentity {
    Bob = 0,
    Alice = 1,
    Charlie = 2,
}
