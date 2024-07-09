use std::fmt;

use committable::Commitment;
use hotshot_query_service::VidCommitment;
use hotshot_types::utils::BuilderCommitment;
use itertools::Either;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::{Map, Value};
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

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, strum::Display)]
#[serde(field_identifier, rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum StructFields {
    Version,
    Fields,
    ChainConfig,
    Height,
    Timestamp,
    L1Head,
    L1Finalized,
    PayloadCommitment,
    BuilderCommitment,
    NsTable,
    BlockMerkleTreeRoot,
    FeeMerkleTreeRoot,
    FeeInfo,
    BuilderSignature,
}

impl StructFields {
    fn list() -> &'static [&'static str] {
        &[
            "fields",
            "chain_config",
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
        ]
    }
}

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HeaderVisitor;

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
                    seq.next_element()?
                        .ok_or_else(|| de::Error::missing_field("chain_config"))?;

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
                // insert all the fields in the serde_map as the map may have out of order fields.
                let mut serde_map: Map<String, Value> = Map::new();

                while let Some(key) = map.next_key::<StructFields>()? {
                    serde_map.insert(key.to_string(), map.next_value()?);
                }

                if let Some(v) = serde_map.get("version") {
                    let fields = serde_map
                        .get(&StructFields::Fields.to_string())
                        .ok_or_else(|| de::Error::missing_field("fields"))?;

                    let version =
                        serde_json::from_value::<ResolvableChainConfigOrVersion>(v.clone())
                            .map_err(de::Error::custom)?
                            .chain_config;
                    let result = match version {
                        EitherOrVersion::Version(Version { major: 0, minor: 2 }) => Ok(Header::V2(
                            serde_json::from_value(fields.clone()).map_err(de::Error::custom)?,
                        )),
                        EitherOrVersion::Version(Version { major: 0, minor: 3 }) => Ok(Header::V3(
                            serde_json::from_value(fields.clone()).map_err(de::Error::custom)?,
                        )),
                        _ => Err(serde::de::Error::custom("invalid version")),
                    };

                    return result;
                }

                Ok(Header::V1(
                    serde_json::from_value(serde_map.into()).map_err(de::Error::custom)?,
                ))
            }
        }

        deserializer.deserialize_struct("Header", StructFields::list(), HeaderVisitor)
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
        fee_info: Vec<FeeInfo>,
        builder_signature: Vec<BuilderSignature>,
        version: Version,
    ) -> Self {
        let Version { major, minor } = version;

        assert!(major == 0, "Invalid major version {major}");
        assert!(fee_info.len() > 0, "Invalid fee_info length: 0");

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
                fee_info: fee_info[0], // NOTE this is asserted to exist above
                builder_signature: builder_signature.first().copied(),
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
                fee_info: fee_info[0], // NOTE this is asserted to exist above
                builder_signature: builder_signature.first().copied(),
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
