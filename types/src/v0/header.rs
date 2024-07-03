use committable::Commitment;
use serde::{Deserialize, Serialize};
use vbs::version::Version;

use crate::{v0_1, v0_2, v0_3, ChainConfig};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Header {
    V1(v0_1::Header),
    V2(v0_2::Header),
    V3(v0_3::Header),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum EitherOrVersion<ChainConfig, Commitment> {
    Left(ChainConfig),
    Right(Commitment),
    Version(Version),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionedHeader<Fields> {
    pub(crate) version: ResolvableChainConfigOrVersion,
    pub(crate) fields: Fields,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResolvableChainConfigOrVersion {
    pub(crate) chain_config: EitherOrVersion<ChainConfig, Commitment<ChainConfig>>,
}
