use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub type TxTableEntryWord = u32;

#[derive(Derivative, Deserialize, Serialize)]
#[derivative(
    Clone(bound = ""),
    Hash(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Default(bound = ""),
    Debug(bound = "")
)]
#[serde(bound = "")]
pub struct NameSpaceTable<TableWord> {
    #[serde(with = "base64_bytes")]
    pub(crate) bytes: Vec<u8>,
    #[serde(skip)]
    pub(crate) phantom: PhantomData<TableWord>,
}
