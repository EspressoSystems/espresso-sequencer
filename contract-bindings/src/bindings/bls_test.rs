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
            "0x608060405234801561001057600080fd5b506111f4806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c8063025ac51514610046578063781588c114610073578063f929c51f14610094575b600080fd5b610059610054366004610dd4565b6100b7565b604080519283526020830191909152015b60405180910390f35b610086610081366004610dd4565b6100cc565b60405190815260200161006a565b6100a76100a2366004610e09565b6100dd565b604051901515815260200161006a565b6000806100c3836100f2565b91509150915091565b60006100d7826101c5565b92915050565b60006100ea8484846103ec565b949350505050565b6000806000610100846101c5565b905060008051602061119f83398151915260036000828485099050828061012957610129610ebd565b8482099050828061013c5761013c610ebd565b828208905060008061014d83610472565b925090505b806101b657848061016557610165610ebd565b600187089550848061017957610179610ebd565b8687099250848061018c5761018c610ebd565b8684099250848061019f5761019f610ebd565b84840892506101ad83610472565b92509050610152565b50939793965092945050505050565b6000806101d183610572565b8051909150603081146101e6576101e6610ed3565b60008167ffffffffffffffff81111561020157610201610ce5565b60405190808252806020026020018201604052801561022a578160200160208202803683370190505b50905060005b8281101561029d578360016102458386610eff565b61024f9190610eff565b8151811061025f5761025f610f12565b602002602001015182828151811061027957610279610f12565b60ff909216602092830291909101909101528061029581610f28565b915050610230565b5060408051601f8082526104008201909252600090819083602082016103e08036833701905050905060005b838110156103355784816102dd8689610eff565b6102e79190610f41565b815181106102f7576102f7610f12565b602002602001015182828151811061031157610311610f12565b60ff909216602092830291909101909101528061032d81610f28565b9150506102c9565b5061033f816108a2565b915061010060008051602061119f833981519152600061035f8689610eff565b905060005b818110156103dc57600088600161037b8486610eff565b6103859190610eff565b8151811061039557610395610f12565b602002602001015160ff16905083806103b0576103b0610ebd565b858809965083806103c3576103c3610ebd565b81880896505080806103d490610f28565b915050610364565b50939a9950505050505050505050565b60008060006103fa85610914565b600060405180606001604052806024815260200161117b602491399050600061042388836109a3565b905061042e816100f2565b60408051808201909152828152602081018290529195509350600061046482896104578c610a20565b61045f610a9b565b610b6c565b9a9950505050505050505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f529050600060008051602061119f833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa9350506000519350826105395760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c656421000000000060448201526064015b60405180910390fd5b80600185901b11156105525761054f8482610eff565b93505b808061056057610560610ebd565b84850991508582149450505050915091565b604080516030808252606082810190935290602090600160f91b90600090600160f81b9082908660208201818036833701905050905080886040516020016105bb929190610f84565b604051602081830303815290604052905080838760f81b6040516020016105e493929190610f99565b60408051601f1981840301815290829052915083906106099083908390602001610fc6565b60408051601f1981840301815290829052925061010160f01b906106339084908390602001610feb565b60408051808303601f1901815282825280516020918201208184018190526001600160f81b03198816848401526001600160f01b03198516604185015282516023818603018152604390940190925282519083012091945090600060ff8b1667ffffffffffffffff8111156106aa576106aa610ce5565b6040519080825280602002602001820160405280156106d3578160200160208202803683370190505b50905060006106e183610c4e565b905060005b815181101561073f5781818151811061070157610701610f12565b602002602001015183828151811061071b5761071b610f12565b60ff909216602092830291909101909101528061073781610f28565b9150506106e6565b50600061074b85610c4e565b905060405180602001604052806000815250975060005b8c8110156107de57600083828151811061077e5761077e610f12565b602002602001015183838151811061079857610798610f12565b60200260200101511860f81b905089816040516020016107b9929190610fc6565b60405160208183030381529060405299505080806107d690610f28565b915050610762565b50878b876040516020016107f493929190611010565b60405160208183030381529060405297508780519060200120935061081884610c4e565b915060005b8c8e60ff1661082c9190610eff565b81101561088e5782818151811061084557610845610f12565b602002602001015184828f61085a9190610f41565b8151811061086a5761086a610f12565b60ff909216602092830291909101909101528061088681610f28565b91505061081d565b50919e9d5050505050505050505050505050565b600080805b835181101561090d578381815181106108c2576108c2610f12565b602002602001015160ff168160086108da9190611044565b6108e590600261113f565b6108ef9190611044565b6108f99083610f41565b91508061090581610f28565b9150506108a7565b5092915050565b8051602082015160009160008051602061119f8339815191529182600381808580098509088382830914811583151719848310858510161616935050508161099e5760405162461bcd60e51b815260206004820152601760248201527f426e3235343a20696e76616c696420473120706f696e740000000000000000006044820152606401610530565b505050565b6060806040519050835180825260208201818101602087015b818310156109d45780518352602092830192016109bc565b50855184518101855292509050808201602086015b81831015610a015780518352602092830192016109e9565b508651929092011591909101601f01601f191660405250905092915050565b6040805180820190915260008082526020820152815160208301511590151615610a48575090565b60405180604001604052808360000151815260200160008051602061119f8339815191528460200151610a7b9190611152565b610a939060008051602061119f833981519152610eff565b905292915050565b610ac66040518060800160405280600081526020016000815260200160008152602001600081525090565b60405180608001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed81526020017f090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b81526020017f12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa815250905090565b60008060006040518751815260208801516020820152865160408201526020870151606082015260408701516080820152606087015160a0820152855160c0820152602086015160e08201528451610100820152602085015161012082015260408501516101408201526060850151610160820152602060006101808360085afa915050600051915080610c425760405162461bcd60e51b815260206004820152601c60248201527f426e3235343a2050616972696e6720636865636b206661696c656421000000006044820152606401610530565b50151595945050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b6020811015610cdf57610c8981601f610eff565b610c94906008611044565b610c9f90600261113f565b610ca99084611166565b828281518110610cbb57610cbb610f12565b60ff9092166020928302919091019091015280610cd781610f28565b915050610c75565b50919050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715610d1e57610d1e610ce5565b60405290565b6040516080810167ffffffffffffffff81118282101715610d1e57610d1e610ce5565b600082601f830112610d5857600080fd5b813567ffffffffffffffff80821115610d7357610d73610ce5565b604051601f8301601f19908116603f01168101908282118183101715610d9b57610d9b610ce5565b81604052838152866020858801011115610db457600080fd5b836020870160208301376000602085830101528094505050505092915050565b600060208284031215610de657600080fd5b813567ffffffffffffffff811115610dfd57600080fd5b6100ea84828501610d47565b600080600083850360e0811215610e1f57600080fd5b843567ffffffffffffffff811115610e3657600080fd5b610e4287828801610d47565b9450506040601f1982011215610e5757600080fd5b610e5f610cfb565b602086810135825260408701359082015292506080605f1982011215610e8457600080fd5b50610e8d610d24565b606085013581526080850135602082015260a0850135604082015260c08501356060820152809150509250925092565b634e487b7160e01b600052601260045260246000fd5b634e487b7160e01b600052600160045260246000fd5b634e487b7160e01b600052601160045260246000fd5b818103818111156100d7576100d7610ee9565b634e487b7160e01b600052603260045260246000fd5b600060018201610f3a57610f3a610ee9565b5060010190565b808201808211156100d7576100d7610ee9565b6000815160005b81811015610f755760208185018101518683015201610f5b565b50600093019283525090919050565b60006100ea610f938386610f54565b84610f54565b6000610fa58286610f54565b6001600160f81b031994851681529290931660018301525060020192915050565b6000610fd28285610f54565b6001600160f81b03199390931683525050600101919050565b6000610ff78285610f54565b6001600160f01b03199390931683525050600201919050565b600061101c8286610f54565b6001600160f81b031994909416845250506001600160f01b0319166001820152600301919050565b80820281158282048414176100d7576100d7610ee9565b600181815b8085111561109657816000190482111561107c5761107c610ee9565b8085161561108957918102915b93841c9390800290611060565b509250929050565b6000826110ad575060016100d7565b816110ba575060006100d7565b81600181146110d057600281146110da576110f6565b60019150506100d7565b60ff8411156110eb576110eb610ee9565b50506001821b6100d7565b5060208310610133831016604e8410600b8410161715611119575081810a6100d7565b611123838361105b565b806000190482111561113757611137610ee9565b029392505050565b600061114b838361109e565b9392505050565b60008261116157611161610ebd565b500690565b60008261117557611175610ebd565b50049056fe424c535f5349475f424e32353447315f584d443a4b454343414b5f4e4354485f4e554c5f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220d275f8acc705cc6bba50f8657c970a227da57ad7bb14ccc60aa1544ffe6186d964736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
