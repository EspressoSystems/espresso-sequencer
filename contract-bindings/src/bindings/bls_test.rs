pub use bls_test::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod bls_test {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "BLSTest was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hash_to_curve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct BN254.G1Point\",\"name\":\"sig\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct BN254.G2Point\",\"name\":\"pk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"x1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y1\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verify_bls_sig\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static BLSTEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BLSTEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50611156806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063025ac51514610046578063781588c114610073578063f929c51f14610094575b600080fd5b610059610054366004610d3c565b6100b7565b604080519283526020830191909152015b60405180910390f35b610086610081366004610d3c565b6100cc565b60405190815260200161006a565b6100a76100a2366004610d71565b6100dd565b604051901515815260200161006a565b6000806100c3836100f2565b91509150915091565b60006100d7826101c5565b92915050565b60006100ea8484846103f8565b949350505050565b6000806000610100846101c5565b905060008051602061110183398151915260036000828485099050828061012957610129610e25565b8482099050828061013c5761013c610e25565b828208905060008061014d83610496565b925090505b806101b657848061016557610165610e25565b600187089550848061017957610179610e25565b8687099250848061018c5761018c610e25565b8684099250848061019f5761019f610e25565b84840892506101ad83610496565b92509050610152565b50939793965092945050505050565b6000806101d183610596565b8051909150603081146101e6576101e6610e3b565b60008167ffffffffffffffff81111561020157610201610c4d565b6040519080825280601f01601f19166020018201604052801561022b576020820181803683370190505b50905060005b828110156102a6578360016102468386610e67565b6102509190610e67565b8151811061026057610260610e7a565b602001015160f81c60f81b82828151811061027d5761027d610e7a565b60200101906001600160f81b031916908160001a9053508061029e81610e90565b915050610231565b5060408051601f808252610400820190925260009082602082016103e08036833701905050905060005b828110156103425783816102e48588610e67565b6102ee9190610ea9565b815181106102fe576102fe610e7a565b602001015160f81c60f81b60f81c82828151811061031e5761031e610e7a565b60ff909216602092830291909101909101528061033a81610e90565b9150506102d0565b50600061034e8261091e565b9050610100600080516020611101833981519152600061036e8689610e67565b905060005b818110156103e857600088600161038a8486610e67565b6103949190610e67565b815181106103a4576103a4610e7a565b016020015160f81c905083806103bc576103bc610e25565b858709955083806103cf576103cf610e25565b81870895505080806103e090610e90565b915050610373565b50929a9950505050505050505050565b600061040383610990565b60006040518060600160405280602481526020016110dd60249139905060008582604051602001610435929190610eec565b6040516020818303038152906040529050600080610452836100f2565b604080518082019091528281526020810182905291935091506000610488828961047b8c610a1f565b610483610a9a565b610b6b565b9a9950505050505050505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5290506000600080516020611101833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa93505060005193508261055d5760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c656421000000000060448201526064015b60405180910390fd5b80600185901b1115610576576105738482610e67565b93505b808061058457610584610e25565b84850991508582149450505050915091565b604080516030808252606082810190935290602090600160f91b90600090600160f81b9082908660208201818036833701905050905080886040516020016105df929190610eec565b604051602081830303815290604052905080838760f81b60405160200161060893929190610f01565b60408051601f19818403018152908290529150839061062d9083908390602001610f2e565b60408051601f1981840301815290829052925061010160f01b906106579084908390602001610f53565b60408051808303601f1901815282825280516020918201208184018190526001600160f81b03198816848401526001600160f01b03198516604185015282516023818603018152604390940190925282519083012091945090600060ff8b1667ffffffffffffffff8111156106ce576106ce610c4d565b6040519080825280601f01601f1916602001820160405280156106f8576020820181803683370190505b50905060008260405160200161071091815260200190565b604051602081830303815290604052905060005b81518110156107855781818151811061073f5761073f610e7a565b602001015160f81c60f81b83828151811061075c5761075c610e7a565b60200101906001600160f81b031916908160001a9053508061077d81610e90565b915050610724565b5060008460405160200161079b91815260200190565b60408051601f1981840301815260208301909152600080835291995091505b8c8110156108395760008382815181106107d6576107d6610e7a565b602001015160f81c60f81b8383815181106107f3576107f3610e7a565b602001015160f81c60f81b1890508981604051602001610814929190610f2e565b604051602081830303815290604052995050808061083190610e90565b9150506107ba565b50878b8760405160200161084f93929190610f78565b6040516020818303038152906040529750878051906020012093508360405160200161087d91815260200190565b604051602081830303815290604052915060005b8c8e60ff166108a09190610e67565b81101561090a578281815181106108b9576108b9610e7a565b602001015160f81c60f81b84828f6108d19190610ea9565b815181106108e1576108e1610e7a565b60200101906001600160f81b031916908160001a9053508061090281610e90565b915050610891565b50919e9d5050505050505050505050505050565b600080805b83518110156109895783818151811061093e5761093e610e7a565b602002602001015160ff168160086109569190610fac565b6109619060026110a7565b61096b9190610fac565b6109759083610ea9565b91508061098181610e90565b915050610923565b5092915050565b8051602082015160009160008051602061110183398151915291826003818085800985090883828309148115831517198483108585101616169350505081610a1a5760405162461bcd60e51b815260206004820152601760248201527f426e3235343a20696e76616c696420473120706f696e740000000000000000006044820152606401610554565b505050565b6040805180820190915260008082526020820152815160208301511590151615610a47575090565b6040518060400160405280836000015181526020016000805160206111018339815191528460200151610a7a91906110ba565b610a9290600080516020611101833981519152610e67565b905292915050565b610ac56040518060800160405280600081526020016000815260200160008152602001600081525090565b60405180608001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed81526020017f090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b81526020017f12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa815250905090565b60008060006040518751815260208801516020820152865160408201526020870151606082015260408701516080820152606087015160a0820152855160c0820152602086015160e08201528451610100820152602085015161012082015260408501516101408201526060850151610160820152602060006101808360085afa915050600051915080610c415760405162461bcd60e51b815260206004820152601c60248201527f426e3235343a2050616972696e6720636865636b206661696c656421000000006044820152606401610554565b50151595945050505050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715610c8657610c86610c4d565b60405290565b6040516080810167ffffffffffffffff81118282101715610c8657610c86610c4d565b600082601f830112610cc057600080fd5b813567ffffffffffffffff80821115610cdb57610cdb610c4d565b604051601f8301601f19908116603f01168101908282118183101715610d0357610d03610c4d565b81604052838152866020858801011115610d1c57600080fd5b836020870160208301376000602085830101528094505050505092915050565b600060208284031215610d4e57600080fd5b813567ffffffffffffffff811115610d6557600080fd5b6100ea84828501610caf565b600080600083850360e0811215610d8757600080fd5b843567ffffffffffffffff811115610d9e57600080fd5b610daa87828801610caf565b9450506040601f1982011215610dbf57600080fd5b610dc7610c63565b602086810135825260408701359082015292506080605f1982011215610dec57600080fd5b50610df5610c8c565b606085013581526080850135602082015260a0850135604082015260c08501356060820152809150509250925092565b634e487b7160e01b600052601260045260246000fd5b634e487b7160e01b600052600160045260246000fd5b634e487b7160e01b600052601160045260246000fd5b818103818111156100d7576100d7610e51565b634e487b7160e01b600052603260045260246000fd5b600060018201610ea257610ea2610e51565b5060010190565b808201808211156100d7576100d7610e51565b6000815160005b81811015610edd5760208185018101518683015201610ec3565b50600093019283525090919050565b60006100ea610efb8386610ebc565b84610ebc565b6000610f0d8286610ebc565b6001600160f81b031994851681529290931660018301525060020192915050565b6000610f3a8285610ebc565b6001600160f81b03199390931683525050600101919050565b6000610f5f8285610ebc565b6001600160f01b03199390931683525050600201919050565b6000610f848286610ebc565b6001600160f81b031994909416845250506001600160f01b0319166001820152600301919050565b80820281158282048414176100d7576100d7610e51565b600181815b80851115610ffe578160001904821115610fe457610fe4610e51565b80851615610ff157918102915b93841c9390800290610fc8565b509250929050565b600082611015575060016100d7565b81611022575060006100d7565b816001811461103857600281146110425761105e565b60019150506100d7565b60ff84111561105357611053610e51565b50506001821b6100d7565b5060208310610133831016604e8410600b8410161715611081575081810a6100d7565b61108b8383610fc3565b806000190482111561109f5761109f610e51565b029392505050565b60006110b38383611006565b9392505050565b6000826110d757634e487b7160e01b600052601260045260246000fd5b50069056fe424c535f5349475f424e32353447315f584d443a4b454343414b5f4e4354485f4e554c5f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212204abdcc500e84806496c3e43611c5c906a7934ba0986e8581db6c8974555491e264736f6c63430008120033" . parse () . expect ("invalid bytecode")
        });
    pub struct BLSTest<M>(ethers::contract::Contract<M>);
    impl<M> Clone for BLSTest<M> {
        fn clone(&self) -> Self {
            BLSTest(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for BLSTest<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for BLSTest<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BLSTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> BLSTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), BLSTEST_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                BLSTEST_ABI.clone(),
                BLSTEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `hash_to_curve` (0x025ac515) function"]
        pub fn hash_to_curve(
            &self,
            input: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([2, 90, 197, 21], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hash_to_field` (0x781588c1) function"]
        pub fn hash_to_field(
            &self,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 21, 136, 193], message)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verify_bls_sig` (0xf929c51f) function"]
        pub fn verify_bls_sig(
            &self,
            message: ethers::core::types::Bytes,
            sig: G1Point,
            pk: G2Point,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([249, 41, 197, 31], (message, sig, pk))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for BLSTest<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `hash_to_curve` function with signature `hash_to_curve(bytes)` and selector `[2, 90, 197, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hash_to_curve", abi = "hash_to_curve(bytes)")]
    pub struct HashToCurveCall {
        pub input: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `hash_to_field` function with signature `hash_to_field(bytes)` and selector `[120, 21, 136, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hash_to_field", abi = "hash_to_field(bytes)")]
    pub struct HashToFieldCall {
        pub message: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `verify_bls_sig` function with signature `verify_bls_sig(bytes,(uint256,uint256),(uint256,uint256,uint256,uint256))` and selector `[249, 41, 197, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "verify_bls_sig",
        abi = "verify_bls_sig(bytes,(uint256,uint256),(uint256,uint256,uint256,uint256))"
    )]
    pub struct VerifyBlsSigCall {
        pub message: ethers::core::types::Bytes,
        pub sig: G1Point,
        pub pk: G2Point,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BLSTestCalls {
        HashToCurve(HashToCurveCall),
        HashToField(HashToFieldCall),
        VerifyBlsSig(VerifyBlsSigCall),
    }
    impl ethers::core::abi::AbiDecode for BLSTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <HashToCurveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BLSTestCalls::HashToCurve(decoded));
            }
            if let Ok(decoded) =
                <HashToFieldCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BLSTestCalls::HashToField(decoded));
            }
            if let Ok(decoded) =
                <VerifyBlsSigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BLSTestCalls::VerifyBlsSig(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BLSTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BLSTestCalls::HashToCurve(element) => element.encode(),
                BLSTestCalls::HashToField(element) => element.encode(),
                BLSTestCalls::VerifyBlsSig(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BLSTestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BLSTestCalls::HashToCurve(element) => element.fmt(f),
                BLSTestCalls::HashToField(element) => element.fmt(f),
                BLSTestCalls::VerifyBlsSig(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<HashToCurveCall> for BLSTestCalls {
        fn from(var: HashToCurveCall) -> Self {
            BLSTestCalls::HashToCurve(var)
        }
    }
    impl ::std::convert::From<HashToFieldCall> for BLSTestCalls {
        fn from(var: HashToFieldCall) -> Self {
            BLSTestCalls::HashToField(var)
        }
    }
    impl ::std::convert::From<VerifyBlsSigCall> for BLSTestCalls {
        fn from(var: VerifyBlsSigCall) -> Self {
            BLSTestCalls::VerifyBlsSig(var)
        }
    }
    #[doc = "Container type for all return fields from the `hash_to_curve` function with signature `hash_to_curve(bytes)` and selector `[2, 90, 197, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HashToCurveReturn(pub ethers::core::types::U256, pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hash_to_field` function with signature `hash_to_field(bytes)` and selector `[120, 21, 136, 193]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HashToFieldReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `verify_bls_sig` function with signature `verify_bls_sig(bytes,(uint256,uint256),(uint256,uint256,uint256,uint256))` and selector `[249, 41, 197, 31]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyBlsSigReturn(pub bool);
    #[doc = "`G1Point(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct G1Point {
        pub x: ethers::core::types::U256,
        pub y: ethers::core::types::U256,
    }
    #[doc = "`G2Point(uint256,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct G2Point {
        pub x_0: ethers::core::types::U256,
        pub x_1: ethers::core::types::U256,
        pub y_0: ethers::core::types::U256,
        pub y_1: ethers::core::types::U256,
    }
}
