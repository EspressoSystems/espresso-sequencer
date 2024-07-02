use std::fmt;

use committable::Commitment;
use hotshot_query_service::VidCommitment;
use hotshot_types::utils::BuilderCommitment;
use itertools::Either;
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use vbs::version::Version;

use crate::{
    v0_1, v0_2, v0_3, BlockMerkleCommitment, BuilderSignature, ChainConfig, FeeInfo,
    FeeMerkleCommitment, L1BlockInfo, NsTable, ResolvableChainConfig,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Header {
    V1(v0_1::Header),
    V2(v0_2::Header),
    V3(v0_3::Header),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum EitherOrVersion<ChainConfig, Commitment> {
    Left(ChainConfig),
    Right(Commitment),
    Version(Version),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionedHeader<Fields> {
    pub version: ResolvableChainConfigOrVersion,
    pub fields: Fields,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResolvableChainConfigOrVersion {
    chain_config: EitherOrVersion<ChainConfig, Commitment<ChainConfig>>,
}

impl From<Version> for ResolvableChainConfigOrVersion {
    fn from(version: Version) -> Self {
        Self {
            chain_config: EitherOrVersion::Version(version),
        }
    }
}

impl From<ResolvableChainConfig> for ResolvableChainConfigOrVersion {
    fn from(v: ResolvableChainConfig) -> Self {
        let value = match v.chain_config {
            Either::Left(cfg) => EitherOrVersion::Left(cfg),
            Either::Right(commit) => EitherOrVersion::Right(commit),
        };

        Self {
            chain_config: value,
        }
    }
}

impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::V1(header) => header.serialize(serializer),
            Self::V2(fields) => VersionedHeader {
                version: Version { major: 0, minor: 2 }.into(),
                fields: fields.clone(),
            }
            .serialize(serializer),
            Self::V3(fields) => VersionedHeader {
                version: Version { major: 0, minor: 3 }.into(),
                fields: fields.clone(),
            }
            .serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeaderVisitor;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum StructFields {
            Version,
            Fields,
            ChainConfig,
        }

        impl<'de> Visitor<'de> for HeaderVisitor {
            type Value = Header;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Header")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let chain_config_or_version: ResolvableChainConfigOrVersion =
                    seq.next_element()?.unwrap();

                match chain_config_or_version.chain_config {
                    // For v0.1, the first field in the sequence of fields is the first field of the struct, so we call a function to get the rest of
                    // the fields from the sequence and pack them into the struct.
                    EitherOrVersion::Left(cfg) => Ok(Header::V1(
                        v0_1::Header::deserialize_with_chain_config(cfg.into(), seq)?,
                    )),
                    EitherOrVersion::Right(commit) => Ok(Header::V1(
                        v0_1::Header::deserialize_with_chain_config(commit.into(), seq)?,
                    )),
                    // For all versions > 0.1, the first "field" is not actually part of the `Header` struct.
                    // We just delegate directly to the derived deserialization impl for the appropriate version.
                    EitherOrVersion::Version(Version { major: 0, minor: 2 }) => Ok(Header::V2(
                        seq.next_element()?
                            .ok_or_else(|| de::Error::missing_field("fields"))?,
                    )),
                    EitherOrVersion::Version(Version { major: 0, minor: 3 }) => Ok(Header::V3(
                        seq.next_element()?
                            .ok_or_else(|| de::Error::missing_field("fields"))?,
                    )),
                    _ => Err(serde::de::Error::custom("invalid version")),
                }
            }

            fn visit_map<V>(self, mut map: V) -> Result<Header, V::Error>
            where
                V: MapAccess<'de>,
            {
                // retrieving the first entry from the map, which could either be a
                // `chain_config` (for v0.1 headers) or a `version` (for `VersionedHeader` structs).
                let Some((_, chain_config_or_version)) =
                    map.next_entry::<StructFields, ResolvableChainConfigOrVersion>()?
                else {
                    return Err(de::Error::missing_field("version"));
                };

                // Match to determine the appropriate header version.
                match chain_config_or_version.chain_config {
                    EitherOrVersion::Left(cfg) => Ok(Header::V1(
                        v0_1::Header::deserialize_with_chain_config_map(cfg.into(), map)?,
                    )),
                    EitherOrVersion::Right(commit) => Ok(Header::V1(
                        v0_1::Header::deserialize_with_chain_config_map(commit.into(), map)?,
                    )),
                    EitherOrVersion::Version(Version { major: 0, minor: 2 }) => {
                        Ok(Header::V2(map.next_entry::<StructFields, _>()?.unwrap().1))
                    }
                    EitherOrVersion::Version(Version { major: 0, minor: 3 }) => {
                        Ok(Header::V3(map.next_entry::<StructFields, _>()?.unwrap().1))
                    }
                    _ => Err(serde::de::Error::custom("invalid version")),
                }
            }
        }

        const FIELDS: &[&str] = &[
            "chain_config",
            "fields",
            "height",
            "timestamp",
            "l1_head",
            "l1_finalized",
            "payload_commitment",
            "builder_commitment",
            "ns_table",
            "block_merkle_tree_root",
            "fee_merkle_tree_root",
            "fee_info",
            "builder_signature",
        ];

        deserializer.deserialize_struct("Header", FIELDS, HeaderVisitor)
    }
}

impl Header {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        chain_config: ResolvableChainConfig,
        height: u64,
        timestamp: u64,
        l1_head: u64,
        l1_finalized: Option<L1BlockInfo>,
        payload_commitment: VidCommitment,
        builder_commitment: BuilderCommitment,
        ns_table: NsTable,
        fee_merkle_tree_root: FeeMerkleCommitment,
        block_merkle_tree_root: BlockMerkleCommitment,
        fee_info: FeeInfo,
        builder_signature: Option<BuilderSignature>,
        version: Version,
    ) -> Self {
        let Version { major, minor } = version;

        assert!(major == 0, "Invalid major version {major}");

        match minor {
            1 => Self::V1(v0_1::Header {
                chain_config,
                height,
                timestamp,
                l1_head,
                l1_finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                block_merkle_tree_root,
                fee_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
            2 => Self::V2(v0_2::Header {
                chain_config,
                height,
                timestamp,
                l1_head,
                l1_finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                block_merkle_tree_root,
                fee_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
            3 => Self::V3(v0_3::Header {
                chain_config,
                height,
                timestamp,
                l1_head,
                l1_finalized,
                payload_commitment,
                builder_commitment,
                ns_table,
                block_merkle_tree_root,
                fee_merkle_tree_root,
                fee_info,
                builder_signature,
            }),
            _ => panic!("invalid version: {version}"),
        }
    }
}
