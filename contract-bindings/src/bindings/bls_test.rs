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
            "0x608060405234801561001057600080fd5b506111bb806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063025ac51514610046578063781588c114610073578063f929c51f14610094575b600080fd5b610059610054366004610da1565b6100b7565b604080519283526020830191909152015b60405180910390f35b610086610081366004610da1565b6100cc565b60405190815260200161006a565b6100a76100a2366004610dd6565b6100dd565b604051901515815260200161006a565b6000806100c3836100f2565b91509150915091565b60006100d7826101c5565b92915050565b60006100ea8484846103f8565b949350505050565b6000806000610100846101c5565b905060008051602061116683398151915260036000828485099050828061012957610129610e8a565b8482099050828061013c5761013c610e8a565b828208905060008061014d8361047e565b925090505b806101b657848061016557610165610e8a565b600187089550848061017957610179610e8a565b8687099250848061018c5761018c610e8a565b8684099250848061019f5761019f610e8a565b84840892506101ad8361047e565b92509050610152565b50939793965092945050505050565b6000806101d18361057e565b8051909150603081146101e6576101e6610ea0565b60008167ffffffffffffffff81111561020157610201610cb2565b6040519080825280601f01601f19166020018201604052801561022b576020820181803683370190505b50905060005b828110156102a6578360016102468386610ecc565b6102509190610ecc565b8151811061026057610260610edf565b602001015160f81c60f81b82828151811061027d5761027d610edf565b60200101906001600160f81b031916908160001a9053508061029e81610ef5565b915050610231565b5060408051601f808252610400820190925260009082602082016103e08036833701905050905060005b828110156103425783816102e48588610ecc565b6102ee9190610f0e565b815181106102fe576102fe610edf565b602001015160f81c60f81b60f81c82828151811061031e5761031e610edf565b60ff909216602092830291909101909101528061033a81610ef5565b9150506102d0565b50600061034e82610906565b9050610100600080516020611166833981519152600061036e8689610ecc565b905060005b818110156103e857600088600161038a8486610ecc565b6103949190610ecc565b815181106103a4576103a4610edf565b016020015160f81c905083806103bc576103bc610e8a565b858709955083806103cf576103cf610e8a565b81870895505080806103e090610ef5565b915050610373565b50929a9950505050505050505050565b600080600061040685610978565b6000604051806060016040528060248152602001611142602491399050600061042f8883610a07565b905061043a816100f2565b60408051808201909152828152602081018290529195509350600061047082896104638c610a84565b61046b610aff565b610bd0565b9a9950505050505050505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5290506000600080516020611166833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa9350506000519350826105455760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c656421000000000060448201526064015b60405180910390fd5b80600185901b111561055e5761055b8482610ecc565b93505b808061056c5761056c610e8a565b84850991508582149450505050915091565b604080516030808252606082810190935290602090600160f91b90600090600160f81b9082908660208201818036833701905050905080886040516020016105c7929190610f51565b604051602081830303815290604052905080838760f81b6040516020016105f093929190610f66565b60408051601f1981840301815290829052915083906106159083908390602001610f93565b60408051601f1981840301815290829052925061010160f01b9061063f9084908390602001610fb8565b60408051808303601f1901815282825280516020918201208184018190526001600160f81b03198816848401526001600160f01b03198516604185015282516023818603018152604390940190925282519083012091945090600060ff8b1667ffffffffffffffff8111156106b6576106b6610cb2565b6040519080825280601f01601f1916602001820160405280156106e0576020820181803683370190505b5090506000826040516020016106f891815260200190565b604051602081830303815290604052905060005b815181101561076d5781818151811061072757610727610edf565b602001015160f81c60f81b83828151811061074457610744610edf565b60200101906001600160f81b031916908160001a9053508061076581610ef5565b91505061070c565b5060008460405160200161078391815260200190565b60408051601f1981840301815260208301909152600080835291995091505b8c8110156108215760008382815181106107be576107be610edf565b602001015160f81c60f81b8383815181106107db576107db610edf565b602001015160f81c60f81b18905089816040516020016107fc929190610f93565b604051602081830303815290604052995050808061081990610ef5565b9150506107a2565b50878b8760405160200161083793929190610fdd565b6040516020818303038152906040529750878051906020012093508360405160200161086591815260200190565b604051602081830303815290604052915060005b8c8e60ff166108889190610ecc565b8110156108f2578281815181106108a1576108a1610edf565b602001015160f81c60f81b84828f6108b99190610f0e565b815181106108c9576108c9610edf565b60200101906001600160f81b031916908160001a905350806108ea81610ef5565b915050610879565b50919e9d5050505050505050505050505050565b600080805b83518110156109715783818151811061092657610926610edf565b602002602001015160ff1681600861093e9190611011565b61094990600261110c565b6109539190611011565b61095d9083610f0e565b91508061096981610ef5565b91505061090b565b5092915050565b8051602082015160009160008051602061116683398151915291826003818085800985090883828309148115831517198483108585101616169350505081610a025760405162461bcd60e51b815260206004820152601760248201527f426e3235343a20696e76616c696420473120706f696e74000000000000000000604482015260640161053c565b505050565b6060806040519050835180825260208201818101602087015b81831015610a38578051835260209283019201610a20565b50855184518101855292509050808201602086015b81831015610a65578051835260209283019201610a4d565b508651929092011591909101601f01601f191660405250905092915050565b6040805180820190915260008082526020820152815160208301511590151615610aac575090565b6040518060400160405280836000015181526020016000805160206111668339815191528460200151610adf919061111f565b610af790600080516020611166833981519152610ecc565b905292915050565b610b2a6040518060800160405280600081526020016000815260200160008152602001600081525090565b60405180608001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed81526020017f090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b81526020017f12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa815250905090565b60008060006040518751815260208801516020820152865160408201526020870151606082015260408701516080820152606087015160a0820152855160c0820152602086015160e08201528451610100820152602085015161012082015260408501516101408201526060850151610160820152602060006101808360085afa915050600051915080610ca65760405162461bcd60e51b815260206004820152601c60248201527f426e3235343a2050616972696e6720636865636b206661696c65642100000000604482015260640161053c565b50151595945050505050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715610ceb57610ceb610cb2565b60405290565b6040516080810167ffffffffffffffff81118282101715610ceb57610ceb610cb2565b600082601f830112610d2557600080fd5b813567ffffffffffffffff80821115610d4057610d40610cb2565b604051601f8301601f19908116603f01168101908282118183101715610d6857610d68610cb2565b81604052838152866020858801011115610d8157600080fd5b836020870160208301376000602085830101528094505050505092915050565b600060208284031215610db357600080fd5b813567ffffffffffffffff811115610dca57600080fd5b6100ea84828501610d14565b600080600083850360e0811215610dec57600080fd5b843567ffffffffffffffff811115610e0357600080fd5b610e0f87828801610d14565b9450506040601f1982011215610e2457600080fd5b610e2c610cc8565b602086810135825260408701359082015292506080605f1982011215610e5157600080fd5b50610e5a610cf1565b606085013581526080850135602082015260a0850135604082015260c08501356060820152809150509250925092565b634e487b7160e01b600052601260045260246000fd5b634e487b7160e01b600052600160045260246000fd5b634e487b7160e01b600052601160045260246000fd5b818103818111156100d7576100d7610eb6565b634e487b7160e01b600052603260045260246000fd5b600060018201610f0757610f07610eb6565b5060010190565b808201808211156100d7576100d7610eb6565b6000815160005b81811015610f425760208185018101518683015201610f28565b50600093019283525090919050565b60006100ea610f608386610f21565b84610f21565b6000610f728286610f21565b6001600160f81b031994851681529290931660018301525060020192915050565b6000610f9f8285610f21565b6001600160f81b03199390931683525050600101919050565b6000610fc48285610f21565b6001600160f01b03199390931683525050600201919050565b6000610fe98286610f21565b6001600160f81b031994909416845250506001600160f01b0319166001820152600301919050565b80820281158282048414176100d7576100d7610eb6565b600181815b8085111561106357816000190482111561104957611049610eb6565b8085161561105657918102915b93841c939080029061102d565b509250929050565b60008261107a575060016100d7565b81611087575060006100d7565b816001811461109d57600281146110a7576110c3565b60019150506100d7565b60ff8411156110b8576110b8610eb6565b50506001821b6100d7565b5060208310610133831016604e8410600b84101617156110e6575081810a6100d7565b6110f08383611028565b806000190482111561110457611104610eb6565b029392505050565b6000611118838361106b565b9392505050565b60008261113c57634e487b7160e01b600052601260045260246000fd5b50069056fe424c535f5349475f424e32353447315f584d443a4b454343414b5f4e4354485f4e554c5f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220b752e8f4cf22da34d995ea44f01df4ab6130befc5271a8baa3ade8b22f0ec24764736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
