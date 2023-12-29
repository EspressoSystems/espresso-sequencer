extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::Result;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, ExprArray, ExprPath, ExprTuple, Ident, LitBool, Token,
};

/// Supported consensus types by macro
#[derive(Debug, Clone)]
enum SupportedConsensusTypes {
    Consensus,
}

/// description of a crosstest
#[derive(derive_builder::Builder, Debug, Clone)]
struct CrossTestData {
    /// consensus time impls
    time_types: ExprArray,
    /// demo type list of tuples
    demo_types: ExprArray,
    /// signature key impls
    signature_key_types: ExprArray,
    /// communication channel impls
    comm_channels: ExprArray,
    /// storage impls
    storages: ExprArray,
    /// name of the test
    test_name: Ident,
    /// test description/spec
    test_builder: Expr,
    /// whether or not to hide behind slow feature flag
    slow: LitBool,
}

/// we internally choose types
#[derive(derive_builder::Builder, Debug, Clone)]
struct CrossAllTypesSpec {
    /// name of the test
    test_name: Ident,
    /// test description/spec
    test_builder: Expr,
    /// whether or not to hide behind slow feature flag
    slow: LitBool,
}

impl Parse for CrossAllTypesSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut description = CrossAllTypesSpecBuilder::create_empty();
        while !description.is_ready() {
            if input.peek(keywords::TestName) {
                let _ = input.parse::<keywords::TestName>()?;
                input.parse::<Token![:]>()?;
                let test_name = input.parse::<Ident>()?;
                description.test_name(test_name);
            } else if input.peek(keywords::TestBuilder) {
                let _ = input.parse::<keywords::TestBuilder>()?;
                input.parse::<Token![:]>()?;
                let test_builder = input.parse::<Expr>()?;
                description.test_builder(test_builder);
            } else if input.peek(keywords::Slow) {
                let _ = input.parse::<keywords::Slow>()?;
                input.parse::<Token![:]>()?;
                let slow = input.parse::<LitBool>()?;
                description.slow(slow);
            } else {
                panic!("Unexpected token. Expected one f: Time, DemoType, SignatureKey, CommChannel, Storage, TestName, TestBuilder, Slow");
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        description
            .build()
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("{}", e)))
    }
}

impl CrossAllTypesSpecBuilder {
    fn is_ready(&self) -> bool {
        self.test_name.is_some() && self.test_builder.is_some() && self.slow.is_some()
    }
}

impl CrossTestDataBuilder {
    fn is_ready(&self) -> bool {
        self.time_types.is_some()
            && self.demo_types.is_some()
            && self.signature_key_types.is_some()
            && self.comm_channels.is_some()
            && self.storages.is_some()
            && self.test_name.is_some()
            && self.test_builder.is_some()
            && self.slow.is_some()
    }
}

/// requisite data to generate a single test
#[derive(derive_builder::Builder, Debug, Clone)]
struct TestData {
    time_type: ExprPath,
    demo_types: ExprTuple,
    signature_key_type: ExprPath,
    comm_channel: ExprPath,
    storage: ExprPath,
    test_name: Ident,
    test_builder: Expr,
    slow: LitBool,
}

/// trait make a string lower and snake case
trait ToLowerSnakeStr {
    /// make a lower and snake case string
    fn to_lower_snake_str(&self) -> String;
}

impl ToLowerSnakeStr for ExprPath {
    fn to_lower_snake_str(&self) -> String {
        self.path
            .segments
            .iter()
            .fold("".to_string(), |mut acc, s| {
                acc.push_str(&s.ident.to_string().to_lowercase());
                acc.push('_');
                acc
            })
            .to_lowercase()
    }
}

impl ToLowerSnakeStr for ExprTuple {
    fn to_lower_snake_str(&self) -> String {
        self.elems
            .iter()
            .map(|x| {
                let Expr::Path(expr_path) = x else {
                    panic!("Expected path expr, got {:?}", x)
                };
                expr_path
            })
            .fold("".to_string(), |mut acc, s| {
                acc.push_str(&s.to_lower_snake_str());
                acc
            })
    }
}

impl TestData {
    fn generate_test(&self) -> TokenStream2 {
        let TestData {
            time_type,
            demo_types,
            signature_key_type,
            test_name,
            test_builder,
            slow,
            comm_channel,
            storage,
        } = self;

        let (supported_consensus_type, demo_state) = {
            let mut tuple = demo_types.elems.iter();
            let first_ele = tuple
                .next()
                .expect("First element of tuple must be the consensus type.");

            let Expr::Path(expr_path) = first_ele else {
                panic!("Expected path expr, got {:?}", first_ele)
            };
            let Some(ident) = expr_path.path.get_ident() else {
                panic!("Expected ident, got {:?}", expr_path.path)
            };
            let consensus_type = if ident == "Consensus" {
                SupportedConsensusTypes::Consensus
            } else {
                panic!("Unsupported consensus type: {ident:?}")
            };

            let demo_state = tuple
                .next()
                .expect("Seecond element of tuple must state type");
            (consensus_type, demo_state)
        };

        let slow_attribute = if slow.value() {
            quote! { #[cfg(feature = "slow-tests")] }
        } else {
            quote! {}
        };

        let (consensus_type, leaf, vote, proposal, consensus_message, exchanges) =
            match supported_consensus_type {
                SupportedConsensusTypes::Consensus => {
                    let consensus_type = quote! {
                        hotshot_types::traits::consensus_type::sequencing_consensus::Consensus
                    };
                    let leaf = quote! {
                        hotshot_types::data::SequencingLeaf<TestTypes>
                    };
                    let vote = quote! {
                        hotshot_types::vote::DAVote<TestTypes>
                    };
                    let proposal = quote! {
                        hotshot_types::data::DAProposal<TestTypes>
                    };
                    let consensus_message = quote! {
                        hotshot_types::message::SequencingMessage<TestTypes, TestNodeImpl>
                    };
                    let committee_exchange = quote! {
                        hotshot_types::traits::election::CommitteeExchange<
                            TestTypes,
                            CommitteeMembership,
                            #comm_channel<
                                TestTypes,
                                TestNodeImpl,
                                #proposal,
                                #vote,
                                CommitteeMembership,
                            >,
                            hotshot_types::message::Message<TestTypes, TestNodeImpl>,
                        >
                    };
                    let exchanges = quote! {
                        hotshot_types::traits::node_implementation::SequencingExchanges<
                            TestTypes,
                            hotshot_types::message::Message<TestTypes, TestNodeImpl>,
                            TestQuorumExchange,
                            #committee_exchange
                        >
                    };

                    (
                        consensus_type,
                        leaf,
                        vote,
                        proposal,
                        consensus_message,
                        exchanges,
                    )
                }
            };

        quote! {

                #[derive(
                    Copy,
                    Clone,
                    Debug,
                    Default,
                    Hash,
                    PartialEq,
                    Eq,
                    PartialOrd,
                    Ord,
                    serde::Serialize,
                    serde::Deserialize,
                    )]
                struct TestTypes;

                #[derive(
                    Copy,
                    Clone,
                    Debug,
                    Default,
                    Hash,
                    PartialEq,
                    Eq,
                    PartialOrd,
                    Ord,
                    serde::Serialize,
                    serde::Deserialize,
                    )]
                struct TestNodeImpl;


                impl hotshot_types::traits::node_implementation::NodeType for TestTypes {
                    type ConsensusType = #consensus_type;
                    type Time = #time_type;
                    type BlockHeader = <#demo_state as hotshot_types::traits::State>::BlockHeader;
                    type SignatureKey = #signature_key_type;
                    type Transaction = <<#demo_state as hotshot_types::traits::State>::BlockPayload as hotshot_types::traits::BlockPayload>::Transaction;
                    type StateType = #demo_state;
                    type VoteTokenType = hotshot::traits::election::static_committee::StaticVoteToken<Self::SignatureKey>;
                    type ElectionConfigType = hotshot::traits::election::static_committee::StaticElectionConfig;
                }

                type CommitteeMembership = hotshot::traits::election::static_committee::GeneralStaticCommittee<TestTypes, #leaf, #signature_key_type>;

                type TestQuorumExchange =
                        hotshot_types::traits::election::QuorumExchange<
                        TestTypes,
                        #leaf,
                        #proposal,
                        CommitteeMembership,
                        #comm_channel<
                            TestTypes,
                            TestNodeImpl,
                            #proposal,
                            #vote,
                            CommitteeMembership,
                        >,
                        hotshot_types::message::Message<TestTypes, TestNodeImpl>,
                    >;

                impl hotshot_types::traits::node_implementation::NodeImplementation<TestTypes> for TestNodeImpl {
                    type Leaf = #leaf;
                    type Storage = #storage<TestTypes, #leaf>;
                    type Exchanges = #exchanges;

                    fn new_channel_maps(
                        start_view: #time_type
                    ) -> (
                        hotshot_types::traits::node_implementation::ChannelMaps<TestTypes, Self>,
                        Option<hotshot_types::traits::node_implementation::ChannelMaps<TestTypes, Self>>
                    ) {
                        let committee_channel_maps = if std::any::type_name::<#consensus_type>() ==
                            std::any::type_name::<hotshot_types::traits::consensus_type::validating_consensus::ValidatingConsensus>()
                        {
                            None
                        } else {
                            Some(hotshot_types::traits::node_implementation::ChannelMaps::new(start_view))
                        };
                        (
                            hotshot_types::traits::node_implementation::ChannelMaps::new(start_view),
                            committee_channel_maps
                        )
                    }
                }

                #slow_attribute
                #[cfg_attr(
                    feature = "tokio-executor",
                    tokio::test(flavor = "multi_thread", worker_threads = 2)
                    )]
                    #[cfg_attr(feature = "async-std-executor", async_std::test)]
                    #[tracing::instrument]
                    async fn #test_name() {
                        hotshot_testing::test_builder::TestBuilder::build::<
                            TestTypes,
                            TestNodeImpl
                        >(#test_builder).launch().run_test().await.unwrap();
                    }
        }
    }
}

/// macro specific custom keywords
mod keywords {
    syn::custom_keyword!(Time);
    syn::custom_keyword!(DemoType);
    syn::custom_keyword!(SignatureKey);
    syn::custom_keyword!(Vote);
    syn::custom_keyword!(TestName);
    syn::custom_keyword!(TestBuilder);
    syn::custom_keyword!(Slow);
    syn::custom_keyword!(CommChannel);
    syn::custom_keyword!(Storage);
}

impl Parse for CrossTestData {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut description = CrossTestDataBuilder::create_empty();

        while !description.is_ready() {
            if input.peek(keywords::Time) {
                let _ = input.parse::<keywords::Time>()?;
                input.parse::<Token![:]>()?;
                let time_types = input.parse::<ExprArray>()?;
                description.time_types(time_types);
            } else if input.peek(keywords::DemoType) {
                let _ = input.parse::<keywords::DemoType>()?;
                input.parse::<Token![:]>()?;
                let demo_types = input.parse::<ExprArray>()?;
                description.demo_types(demo_types);
            } else if input.peek(keywords::SignatureKey) {
                let _ = input.parse::<keywords::SignatureKey>()?;
                input.parse::<Token![:]>()?;
                let signature_key_types = input.parse::<ExprArray>()?;
                description.signature_key_types(signature_key_types);
            } else if input.peek(keywords::CommChannel) {
                let _ = input.parse::<keywords::CommChannel>()?;
                input.parse::<Token![:]>()?;
                let comm_channels = input.parse::<ExprArray>()?;
                description.comm_channels(comm_channels);
            } else if input.peek(keywords::Storage) {
                let _ = input.parse::<keywords::Storage>()?;
                input.parse::<Token![:]>()?;
                let storages = input.parse::<ExprArray>()?;
                description.storages(storages);
            } else if input.peek(keywords::TestName) {
                let _ = input.parse::<keywords::TestName>()?;
                input.parse::<Token![:]>()?;
                let test_name = input.parse::<Ident>()?;
                description.test_name(test_name);
            } else if input.peek(keywords::TestBuilder) {
                let _ = input.parse::<keywords::TestBuilder>()?;
                input.parse::<Token![:]>()?;
                let test_builder = input.parse::<Expr>()?;
                description.test_builder(test_builder);
            } else if input.peek(keywords::Slow) {
                let _ = input.parse::<keywords::Slow>()?;
                input.parse::<Token![:]>()?;
                let slow = input.parse::<LitBool>()?;
                description.slow(slow);
            } else {
                panic!("Unexpected token. Expected one f: Time, DemoType, SignatureKey, CommChannel, Storage, TestName, TestBuilder, Slow");
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        description
            .build()
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("{}", e)))
    }
}

fn cross_tests_internal(test_spec: CrossTestData) -> TokenStream {
    let demo_types = test_spec
        .demo_types
        .elems
        .iter()
        .map(|t| {
            let Expr::Tuple(p) = t else {
                panic!("Expected Tuple! Got {:?}", t)
            };
            p
        })
        .collect::<Vec<_>>();

    let comm_channels = test_spec.comm_channels.elems.iter().map(|t| {
        let Expr::Path(p) = t else {
            panic!("Expected Path for Comm Channel! Got {:?}", t)
        };
        p
    });

    let storages = test_spec.storages.elems.iter().map(|t| {
        let Expr::Path(p) = t else {
            panic!("Expected Path for Storage! Got {:?}", t)
        };
        p
    });

    let time_types = test_spec.time_types.elems.iter().map(|t| {
        let Expr::Path(p) = t else {
            panic!("Expected Path for Time Type! Got {:?}", t)
        };
        p
    });

    let signature_key_types = test_spec.signature_key_types.elems.iter().map(|t| {
        let Expr::Path(p) = t else {
            panic!("Expected Path for Signature Key Type! Got {:?}", t)
        };
        p
    });

    let mut result = quote! {};

    for demo_type in demo_types.clone() {
        let mut demo_mod = quote! {};
        for comm_channel in comm_channels.clone() {
            let mut comm_mod = quote! {};
            for storage in storages.clone() {
                let mut storage_mod = quote! {};
                for time_type in time_types.clone() {
                    let mut time_mod = quote! {};
                    for signature_key_type in signature_key_types.clone() {
                        let test_data = TestDataBuilder::create_empty()
                            .time_type(time_type.clone())
                            .demo_types(demo_type.clone())
                            .signature_key_type(signature_key_type.clone())
                            .comm_channel(comm_channel.clone())
                            .storage(storage.clone())
                            .test_name(test_spec.test_name.clone())
                            .test_builder(test_spec.test_builder.clone())
                            .slow(test_spec.slow.clone())
                            .build()
                            .unwrap();
                        let test = test_data.generate_test();

                        let signature_key_str =
                            format_ident!("{}", signature_key_type.to_lower_snake_str());
                        let sig_result = quote! {
                            pub mod #signature_key_str {
                                use super::*;
                                #test
                            }
                        };
                        time_mod.extend(sig_result);
                    }

                    let time_str = format_ident!("{}", time_type.to_lower_snake_str());
                    let time_result = quote! {
                        pub mod #time_str {
                            use super::*;
                            #time_mod
                        }
                    };
                    storage_mod.extend(time_result);
                }
                let storage_str = format_ident!("{}", storage.to_lower_snake_str());
                let storage_result = quote! {
                    pub mod #storage_str {
                        use super::*;
                        #storage_mod
                    }
                };
                comm_mod.extend(storage_result);
            }
            let comm_channel_str = format_ident!("{}", comm_channel.to_lower_snake_str());
            let comm_result = quote! {
                pub mod #comm_channel_str {
                    use super::*;
                    #comm_mod
                }
            };
            demo_mod.extend(comm_result);
        }
        let demo_str = format_ident!("{}", demo_type.to_lower_snake_str());
        let demo_result = quote! {
            pub mod #demo_str {
                use super::*;
                #demo_mod
            }
        };
        result.extend(demo_result);
    }
    let name = test_spec.test_name;
    quote! {
        pub mod #name {
            use super::*;
            #result
        }
    }
    .into()
}

/// Generate a cartesian product of tests across all types
/// Arguments:
/// - `DemoType: [(ConsensusTypeName1, DemoStateType1), (ConsensusTypeName2, DemoStateType2) ...]` - a list of tuples of cusonsensus state + `State` implementations
/// - `SignatureKey: [SignatureKey1, SignatureKey2, ...]` - a list of `SignatureKey` implementations
/// - `CommChannel: [CommChannel1, CommChannel2, ..]` - a list of `CommunicationChannel` implementations
/// - `Time: [ Time1, Time2, ...]` - a list of `ConsensusTime` implementations]`
/// - `TestName: example_test` - the name of the test
/// - `TestBuilder: { some_test_builder_expression }` - the `TestBuilder` to use
/// - `Storage: Storage1, Storage2, ...` - a list of `Storage` implementations to use
/// - `Slow`: whether or not this set of tests are hidden behind the `slow` feature flag
/// Example usage:
/// ```
/// hotshot_testing_macros::cross_tests!(
///     DemoType: [(ValidatingConsensus, hotshot::demos::vdemo::VDemoState) ],
///     SignatureKey: [ hotshot_types::traits::signature_key::ed25519::Ed25519Pub ],
///     CommChannel: [ hotshot::traits::implementations::MemoryCommChannel ],
///     Storage: [ hotshot::traits::implementations::MemoryStorage ],
///     Time: [ hotshot_types::data::ViewNumber ],
///     TestName: ten_tx_seven_nodes_fast,
///     TestBuilder: hotshot_testing::test_builder::TestBuilder {
///         total_nodes: 7,
///         start_nodes: 7,
///         num_succeeds: 10,
///         txn_ids: either::Either::Right(1),
///         ..hotshot_testing::test_builder::TestBuilder::default()
///     },
///     Slow: false,
/// );
/// ```
#[proc_macro]
pub fn cross_tests(input: TokenStream) -> TokenStream {
    let test_spec = parse_macro_input!(input as CrossTestData);
    cross_tests_internal(test_spec)
}

/// Generate a cartesian product of tests across all impls known to `hotshot_types`
/// Arguments:
/// - `TestName: example_test` - the name of the test
/// - `TestBuilder: { some_test_builder_expression }` - the `TestBuilder` to use
/// - `Slow`: whether or not this set of tests are hidden behind the `slow` feature flag
/// Example usage:
/// ```
/// hotshot_testing_macros::cross_all_types!(
///     TestName: example_test,
///     TestBuilder: hotshot_testing::test_builder::TestBuilder::default(),
///     Slow: false,
/// );
/// ```
#[proc_macro]
pub fn cross_all_types(input: TokenStream) -> TokenStream {
    let CrossAllTypesSpec {
        test_name,
        test_builder,
        slow,
    } = parse_macro_input!(input as CrossAllTypesSpec);
    let tokens = quote! {
            DemoType: [ /* (Consensus, hotshot_testing::demo::DemoState), */ (hotshot::demos::vdemo::VDemoState) ],
            SignatureKey: [ hotshot_types::traits::signature_key::bn254::BLSPubKey ],
            CommChannel: [ hotshot::traits::implementations::Libp2pCommChannel, hotshot::traits::implementations::CentralizedCommChannel ],
            Time: [ hotshot_types::data::ViewNumber ],
            Storage: [ hotshot::traits::implementations::MemoryStorage ],
            TestName: #test_name,
            TestBuilder: #test_builder,
            Slow: #slow,
    }.into();
    let test_spec = parse_macro_input!(tokens as CrossTestData);
    cross_tests_internal(test_spec)
}
