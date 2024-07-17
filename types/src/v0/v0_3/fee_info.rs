use crate::{FeeAccount, FeeAmount};

/// Methods for use w/ Vec<FeeInfo>
pub trait IterableFeeInfo {
    fn amount(&self) -> Option<FeeAmount>;
    fn accounts(&self) -> Vec<FeeAccount>;
}
