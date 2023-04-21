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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hash_to_curve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct BN254.G1Point\",\"name\":\"sig\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct BN254.G2Point\",\"name\":\"pk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"x1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y1\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verify_bls_sig\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static BLSTEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static BLSTEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061119b806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063025ac51514610046578063781588c114610073578063f929c51f14610094575b600080fd5b610059610054366004610d79565b6100a9565b604080519283526020830191909152015b60405180910390f35b610086610081366004610d79565b6100be565b60405190815260200161006a565b6100a76100a2366004610db6565b6100cf565b005b6000806100b5836100df565b91509150915091565b60006100c9826101b2565b92915050565b6100da8383836103e5565b505050565b60008060006100ed846101b2565b905060008051602061114683398151915260036000828485099050828061011657610116610e6a565b8482099050828061012957610129610e6a565b828208905060008061013a836104dd565b925090505b806101a357848061015257610152610e6a565b600187089550848061016657610166610e6a565b8687099250848061017957610179610e6a565b8684099250848061018c5761018c610e6a565b848408925061019a836104dd565b9250905061013f565b50939793965092945050505050565b6000806101be836105d8565b8051909150603081146101d3576101d3610e80565b60008167ffffffffffffffff8111156101ee576101ee610c8a565b6040519080825280601f01601f191660200182016040528015610218576020820181803683370190505b50905060005b82811015610293578360016102338386610eac565b61023d9190610eac565b8151811061024d5761024d610ebf565b602001015160f81c60f81b82828151811061026a5761026a610ebf565b60200101906001600160f81b031916908160001a9053508061028b81610ed5565b91505061021e565b5060408051601f808252610400820190925260009082602082016103e08036833701905050905060005b8281101561032f5783816102d18588610eac565b6102db9190610eee565b815181106102eb576102eb610ebf565b602001015160f81c60f81b60f81c82828151811061030b5761030b610ebf565b60ff909216602092830291909101909101528061032781610ed5565b9150506102bd565b50600061033b82610960565b9050610100600080516020611146833981519152600061035b8689610eac565b905060005b818110156103d55760008860016103778486610eac565b6103819190610eac565b8151811061039157610391610ebf565b016020015160f81c905083806103a9576103a9610e6a565b858709955083806103bc576103bc610e6a565b81870895505080806103cd90610ed5565b915050610360565b50929a9950505050505050505050565b6103ee826109d2565b600060405180606001604052806024815260200161112260249139905060008482604051602001610420929190610f31565b604051602081830303815290604052905060008061043d836100df565b60408051808201909152828152602081018290529193509150600061047382886104668b610a5c565b61046e610ad7565b610ba8565b9050806104d25760405162461bcd60e51b815260206004820152602260248201527f424c53207369676e617475726520766572696669636174696f6e206661696c65604482015261321760f11b60648201526084015b60405180910390fd5b505050505050505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5290506000600080516020611146833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa93505060005193508261059f5760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c656421000000000060448201526064016104c9565b80600185901b11156105b8576105b58482610eac565b93505b80806105c6576105c6610e6a565b84850991508582149450505050915091565b604080516030808252606082810190935290602090600160f91b90600090600160f81b908290866020820181803683370190505090508088604051602001610621929190610f31565b604051602081830303815290604052905080838760f81b60405160200161064a93929190610f46565b60408051601f19818403018152908290529150839061066f9083908390602001610f73565b60408051601f1981840301815290829052925061010160f01b906106999084908390602001610f98565b60408051808303601f1901815282825280516020918201208184018190526001600160f81b03198816848401526001600160f01b03198516604185015282516023818603018152604390940190925282519083012091945090600060ff8b1667ffffffffffffffff81111561071057610710610c8a565b6040519080825280601f01601f19166020018201604052801561073a576020820181803683370190505b50905060008260405160200161075291815260200190565b604051602081830303815290604052905060005b81518110156107c75781818151811061078157610781610ebf565b602001015160f81c60f81b83828151811061079e5761079e610ebf565b60200101906001600160f81b031916908160001a905350806107bf81610ed5565b915050610766565b506000846040516020016107dd91815260200190565b60408051601f1981840301815260208301909152600080835291995091505b8c81101561087b57600083828151811061081857610818610ebf565b602001015160f81c60f81b83838151811061083557610835610ebf565b602001015160f81c60f81b1890508981604051602001610856929190610f73565b604051602081830303815290604052995050808061087390610ed5565b9150506107fc565b50878b8760405160200161089193929190610fbd565b604051602081830303815290604052975087805190602001209350836040516020016108bf91815260200190565b604051602081830303815290604052915060005b8c8e60ff166108e29190610eac565b81101561094c578281815181106108fb576108fb610ebf565b602001015160f81c60f81b84828f6109139190610eee565b8151811061092357610923610ebf565b60200101906001600160f81b031916908160001a9053508061094481610ed5565b9150506108d3565b50919e9d5050505050505050505050505050565b600080805b83518110156109cb5783818151811061098057610980610ebf565b602002602001015160ff168160086109989190610ff1565b6109a39060026110ec565b6109ad9190610ff1565b6109b79083610eee565b9150806109c381610ed5565b915050610965565b5092915050565b80516020820151600091600080516020611146833981519152918260038180858009850908838283091481158315171984831085851016161693505050816100da5760405162461bcd60e51b815260206004820152601760248201527f426e3235343a20696e76616c696420473120706f696e7400000000000000000060448201526064016104c9565b6040805180820190915260008082526020820152815160208301511590151615610a84575090565b6040518060400160405280836000015181526020016000805160206111468339815191528460200151610ab791906110ff565b610acf90600080516020611146833981519152610eac565b905292915050565b610b026040518060800160405280600081526020016000815260200160008152602001600081525090565b60405180608001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed81526020017f090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b81526020017f12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa815250905090565b60008060006040518751815260208801516020820152865160408201526020870151606082015260408701516080820152606087015160a0820152855160c0820152602086015160e08201528451610100820152602085015161012082015260408501516101408201526060850151610160820152602060006101808360085afa915050600051915080610c7e5760405162461bcd60e51b815260206004820152601c60248201527f426e3235343a2050616972696e6720636865636b206661696c6564210000000060448201526064016104c9565b50151595945050505050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715610cc357610cc3610c8a565b60405290565b6040516080810167ffffffffffffffff81118282101715610cc357610cc3610c8a565b600082601f830112610cfd57600080fd5b813567ffffffffffffffff80821115610d1857610d18610c8a565b604051601f8301601f19908116603f01168101908282118183101715610d4057610d40610c8a565b81604052838152866020858801011115610d5957600080fd5b836020870160208301376000602085830101528094505050505092915050565b600060208284031215610d8b57600080fd5b813567ffffffffffffffff811115610da257600080fd5b610dae84828501610cec565b949350505050565b600080600083850360e0811215610dcc57600080fd5b843567ffffffffffffffff811115610de357600080fd5b610def87828801610cec565b9450506040601f1982011215610e0457600080fd5b610e0c610ca0565b602086810135825260408701359082015292506080605f1982011215610e3157600080fd5b50610e3a610cc9565b606085013581526080850135602082015260a0850135604082015260c08501356060820152809150509250925092565b634e487b7160e01b600052601260045260246000fd5b634e487b7160e01b600052600160045260246000fd5b634e487b7160e01b600052601160045260246000fd5b818103818111156100c9576100c9610e96565b634e487b7160e01b600052603260045260246000fd5b600060018201610ee757610ee7610e96565b5060010190565b808201808211156100c9576100c9610e96565b6000815160005b81811015610f225760208185018101518683015201610f08565b50600093019283525090919050565b6000610dae610f408386610f01565b84610f01565b6000610f528286610f01565b6001600160f81b031994851681529290931660018301525060020192915050565b6000610f7f8285610f01565b6001600160f81b03199390931683525050600101919050565b6000610fa48285610f01565b6001600160f01b03199390931683525050600201919050565b6000610fc98286610f01565b6001600160f81b031994909416845250506001600160f01b0319166001820152600301919050565b80820281158282048414176100c9576100c9610e96565b600181815b8085111561104357816000190482111561102957611029610e96565b8085161561103657918102915b93841c939080029061100d565b509250929050565b60008261105a575060016100c9565b81611067575060006100c9565b816001811461107d5760028114611087576110a3565b60019150506100c9565b60ff84111561109857611098610e96565b50506001821b6100c9565b5060208310610133831016604e8410600b84101617156110c6575081810a6100c9565b6110d08383611008565b80600019048211156110e4576110e4610e96565b029392505050565b60006110f8838361104b565b9392505050565b60008261111c57634e487b7160e01b600052601260045260246000fd5b50069056fe424c535f5349475f424e32353447315f584d443a4b454343414b5f4e4354485f4e554c5f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220911075784b3cf1f6e457c575388b77d08364abd327d5b578dc0d239020e4c57264736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
