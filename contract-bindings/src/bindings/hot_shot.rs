pub use hot_shot::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod hot_shot {
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
    #[doc = "HotShot was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidQC\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TooManyBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numQCs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongNumberOfQCs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_BLOCKS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"input\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bytes32ToUint8Array\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"output\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expand\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"field_square\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"input\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hash_to_curve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"newCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"qcs\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlocks\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061118c806100206000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c806349ce89971161006657806349ce89971461010f57806354cac3c51461012f57806355569ece146101425780636769235014610155578063f44ff7121461016a57600080fd5b806326833dcc146100985780632b815b57146100b457806337e232b2146100d45780633b1f56c6146100fc575b600080fd5b6100a16103e881565b6040519081526020015b60405180910390f35b6100c76100c2366004610cb9565b610173565b6040516100ab9190610d7e565b6100e76100e2366004610cb9565b610604565b604080519283526020830191909152016100ab565b6100c761010a366004610dc5565b6106d7565b6100a161011d366004610dc5565b60006020819052908152604090205481565b6100a161013d366004610cb9565b61076e565b6100a1610150366004610dc5565b610995565b610168610163366004610e2a565b6109b3565b005b6100a160015481565b60606030602060026000600185825b8660ff168110156101fc57806000036101c5576040516001600160f81b031960f886901b16602082015260210160405160208183030381529060405291506101ea565b81846040516020016101d8929190610ec6565b60405160208183030381529060405291505b806101f481610f04565b915050610182565b5060005b8851811015610259578189828151811061021c5761021c610f1d565b6020026020010151604051602001610235929190610ec6565b6040516020818303038152906040529150808061025190610f04565b915050610200565b50808360405160200161026d929190610ec6565b60405160208183030381529060405290508086604051602001610291929190610ec6565b60408051601f1981840301815290829052915083906102b69083908390602001610ec6565b60408051808303601f1901815282820182526001808452602080850182905292519195506102e8928692829101610f33565b60408051808303601f190181528282528051602091820120818401819052825180850390920182528383019092529450906103299085908790606001610ec6565b60408051808303601f190181529082905283516020808601519297506103529388939101610f33565b60408051601f1981840301815291905280516020820120909450600060ff8b1667ffffffffffffffff81111561038a5761038a610c8d565b6040519080825280602002602001820160405280156103b3578160200160208202803683370190505b50905060006103c1836106d7565b905060005b815181101561041f578181815181106103e1576103e1610f1d565b60200260200101518382815181106103fb576103fb610f1d565b60ff909216602092830291909101909101528061041781610f04565b9150506103c6565b50600061042b856106d7565b905060005b8c81101561051957806000036104ae5782818151811061045257610452610f1d565b602002602001015182828151811061046c5761046c610f1d565b602002602001015118604051602001610498919060f89190911b6001600160f81b031916815260010190565b6040516020818303038152906040529850610507565b888382815181106104c1576104c1610f1d565b60200260200101518383815181106104db576104db610f1d565b6020026020010151186040516020016104f5929190610ec6565b60405160208183030381529060405298505b8061051181610f04565b915050610430565b50878b60405160200161052d929190610ec6565b60408051808303601f190181529082905287516020808a0151929b50610556938c939101610f33565b60405160208183030381529060405297508780519060200120935061057a846106d7565b915060005b8c8e60ff1661058e9190610f67565b8110156105f0578281815181106105a7576105a7610f1d565b602002602001015184828f6105bc9190610f80565b815181106105cc576105cc610f1d565b60ff90921660209283029190910190910152806105e881610f04565b91505061057f565b50919e9d5050505050505050505050505050565b60008060006106128461076e565b905060008051602061113783398151915260036000828485099050828061063b5761063b610f93565b8482099050828061064e5761064e610f93565b828208905060008061065f83610b20565b925090505b806106c857848061067757610677610f93565b600187089550848061068b5761068b610f93565b8687099250848061069e5761069e610f93565b868409925084806106b1576106b1610f93565b84840892506106bf83610b20565b92509050610664565b50939793965092945050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b60208110156107685761071281601f610f67565b61071d906008610fa9565b6107289060026110a4565b61073290846110b7565b82828151811061074457610744610f1d565b60ff909216602092830291909101909101528061076081610f04565b9150506106fe565b50919050565b60008061077a83610173565b80519091506030811461078f5761078f6110d9565b60008167ffffffffffffffff8111156107aa576107aa610c8d565b6040519080825280602002602001820160405280156107d3578160200160208202803683370190505b50905060005b82811015610846578360016107ee8386610f67565b6107f89190610f67565b8151811061080857610808610f1d565b602002602001015182828151811061082257610822610f1d565b60ff909216602092830291909101909101528061083e81610f04565b9150506107d9565b5060408051601f8082526104008201909252600090819083602082016103e08036833701905050905060005b838110156108de5784816108868689610f67565b6108909190610f80565b815181106108a0576108a0610f1d565b60200260200101518282815181106108ba576108ba610f1d565b60ff90921660209283029190910190910152806108d681610f04565b915050610872565b506108e881610c1b565b915061010060008051602061113783398151915260006109088689610f67565b905060005b818110156109855760008860016109248486610f67565b61092e9190610f67565b8151811061093e5761093e610f1d565b602002602001015160ff169050838061095957610959610f93565b8588099650838061096c5761096c610f93565b818808965050808061097d90610f04565b91505061090d565b50939a9950505050505050505050565b60006000805160206111378339815191528181848509949350505050565b8281146109e25760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e8831115610a085760405163e082840b60e01b8152600481018490526024016109d9565b60015460005b84811015610adf57610a63600154878784818110610a2e57610a2e610f1d565b90506020020135868685818110610a4757610a47610f1d565b9050602002810190610a5991906110ef565b6001949350505050565b610a8657600154604051637818671960e01b81526004016109d991815260200190565b858582818110610a9857610a98610f1d565b905060200201356000806001548152602001908152602001600020819055506001806000828254610ac99190610f80565b90915550610ad8905081610f04565b9050610a0e565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5290506000600080516020611137833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa935050600051935082610be25760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c656421000000000060448201526064016109d9565b80600185901b1115610bfb57610bf88482610f67565b93505b8080610c0957610c09610f93565b84850991508582149450505050915091565b600080805b8351811015610c8657838181518110610c3b57610c3b610f1d565b602002602001015160ff16816008610c539190610fa9565b610c5e9060026110a4565b610c689190610fa9565b610c729083610f80565b915080610c7e81610f04565b915050610c20565b5092915050565b634e487b7160e01b600052604160045260246000fd5b803560ff81168114610cb457600080fd5b919050565b60006020808385031215610ccc57600080fd5b823567ffffffffffffffff80821115610ce457600080fd5b818501915085601f830112610cf857600080fd5b813581811115610d0a57610d0a610c8d565b8060051b604051601f19603f83011681018181108582111715610d2f57610d2f610c8d565b604052918252848201925083810185019188831115610d4d57600080fd5b938501935b82851015610d7257610d6385610ca3565b84529385019392850192610d52565b98975050505050505050565b6020808252825182820181905260009190848201906040850190845b81811015610db957835160ff1683529284019291840191600101610d9a565b50909695505050505050565b600060208284031215610dd757600080fd5b5035919050565b60008083601f840112610df057600080fd5b50813567ffffffffffffffff811115610e0857600080fd5b6020830191508360208260051b8501011115610e2357600080fd5b9250929050565b60008060008060408587031215610e4057600080fd5b843567ffffffffffffffff80821115610e5857600080fd5b610e6488838901610dde565b90965094506020870135915080821115610e7d57600080fd5b50610e8a87828801610dde565b95989497509550505050565b6000815160005b81811015610eb75760208185018101518683015201610e9d565b50600093019283525090919050565b6000610ed28285610e96565b60f89390931b6001600160f81b03191683525050600101919050565b634e487b7160e01b600052601160045260246000fd5b600060018201610f1657610f16610eee565b5060010190565b634e487b7160e01b600052603260045260246000fd5b6000610f3f8286610e96565b6001600160f81b031960f895861b811682529390941b90921660018401525050600201919050565b81810381811115610f7a57610f7a610eee565b92915050565b80820180821115610f7a57610f7a610eee565b634e487b7160e01b600052601260045260246000fd5b8082028115828204841417610f7a57610f7a610eee565b600181815b80851115610ffb578160001904821115610fe157610fe1610eee565b80851615610fee57918102915b93841c9390800290610fc5565b509250929050565b60008261101257506001610f7a565b8161101f57506000610f7a565b8160018114611035576002811461103f5761105b565b6001915050610f7a565b60ff84111561105057611050610eee565b50506001821b610f7a565b5060208310610133831016604e8410600b841016171561107e575081810a610f7a565b6110888383610fc0565b806000190482111561109c5761109c610eee565b029392505050565b60006110b08383611003565b9392505050565b6000826110d457634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052600160045260246000fd5b6000808335601e1984360301811261110657600080fd5b83018035915067ffffffffffffffff82111561112157600080fd5b602001915036819003821315610e2357600080fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a264697066735822122077fd7290841ed329220935363193e17fe74ec230d858a9ac55d4e11d4a923fa064736f6c63430008120033" . parse () . expect ("invalid bytecode")
        });
    pub struct HotShot<M>(ethers::contract::Contract<M>);
    impl<M> Clone for HotShot<M> {
        fn clone(&self) -> Self {
            HotShot(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for HotShot<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for HotShot<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(HotShot))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> HotShot<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), HOTSHOT_ABI.clone(), client).into()
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
                HOTSHOT_ABI.clone(),
                HOTSHOT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `MAX_BLOCKS` (0x26833dcc) function"]
        pub fn max_blocks(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([38, 131, 61, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blockHeight` (0xf44ff712) function"]
        pub fn block_height(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([244, 79, 247, 18], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bytes32ToUint8Array` (0x3b1f56c6) function"]
        pub fn bytes_32_to_uint_8_array(
            &self,
            input: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u8>> {
            self.0
                .method_hash([59, 31, 86, 198], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commitments` (0x49ce8997) function"]
        pub fn commitments(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([73, 206, 137, 151], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expand` (0x2b815b57) function"]
        pub fn expand(
            &self,
            message: ::std::vec::Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u8>> {
            self.0
                .method_hash([43, 129, 91, 87], message)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `field_square` (0x55569ece) function"]
        pub fn field_square(
            &self,
            x: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([85, 86, 158, 206], x)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hash_to_curve` (0x37e232b2) function"]
        pub fn hash_to_curve(
            &self,
            input: ::std::vec::Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([55, 226, 50, 178], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hash_to_field` (0x54cac3c5) function"]
        pub fn hash_to_field(
            &self,
            message: ::std::vec::Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 202, 195, 197], message)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `newBlocks` (0x67692350) function"]
        pub fn new_blocks(
            &self,
            new_commitments: ::std::vec::Vec<ethers::core::types::U256>,
            qcs: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 105, 35, 80], (new_commitments, qcs))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewBlocks` event"]
        pub fn new_blocks_filter(&self) -> ethers::contract::builders::Event<M, NewBlocksFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NewBlocksFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for HotShot<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `InvalidQC` with signature `InvalidQC(uint256)` and selector `[120, 24, 103, 25]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidQC", abi = "InvalidQC(uint256)")]
    pub struct InvalidQC {
        pub block_number: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `TooManyBlocks` with signature `TooManyBlocks(uint256)` and selector `[224, 130, 132, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "TooManyBlocks", abi = "TooManyBlocks(uint256)")]
    pub struct TooManyBlocks {
        pub num_blocks: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `WrongNumberOfQCs` with signature `WrongNumberOfQCs(uint256,uint256)` and selector `[199, 27, 235, 162]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "WrongNumberOfQCs", abi = "WrongNumberOfQCs(uint256,uint256)")]
    pub struct WrongNumberOfQCs {
        pub num_blocks: ethers::core::types::U256,
        pub num_q_cs: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum HotShotErrors {
        InvalidQC(InvalidQC),
        TooManyBlocks(TooManyBlocks),
        WrongNumberOfQCs(WrongNumberOfQCs),
    }
    impl ethers::core::abi::AbiDecode for HotShotErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <InvalidQC as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotErrors::InvalidQC(decoded));
            }
            if let Ok(decoded) =
                <TooManyBlocks as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotErrors::TooManyBlocks(decoded));
            }
            if let Ok(decoded) =
                <WrongNumberOfQCs as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotErrors::WrongNumberOfQCs(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for HotShotErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                HotShotErrors::InvalidQC(element) => element.encode(),
                HotShotErrors::TooManyBlocks(element) => element.encode(),
                HotShotErrors::WrongNumberOfQCs(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for HotShotErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                HotShotErrors::InvalidQC(element) => element.fmt(f),
                HotShotErrors::TooManyBlocks(element) => element.fmt(f),
                HotShotErrors::WrongNumberOfQCs(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<InvalidQC> for HotShotErrors {
        fn from(var: InvalidQC) -> Self {
            HotShotErrors::InvalidQC(var)
        }
    }
    impl ::std::convert::From<TooManyBlocks> for HotShotErrors {
        fn from(var: TooManyBlocks) -> Self {
            HotShotErrors::TooManyBlocks(var)
        }
    }
    impl ::std::convert::From<WrongNumberOfQCs> for HotShotErrors {
        fn from(var: WrongNumberOfQCs) -> Self {
            HotShotErrors::WrongNumberOfQCs(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "NewBlocks", abi = "NewBlocks(uint256,uint256)")]
    pub struct NewBlocksFilter {
        pub first_block_number: ethers::core::types::U256,
        pub num_blocks: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `MAX_BLOCKS` function with signature `MAX_BLOCKS()` and selector `[38, 131, 61, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_BLOCKS", abi = "MAX_BLOCKS()")]
    pub struct MaxBlocksCall;
    #[doc = "Container type for all input parameters for the `blockHeight` function with signature `blockHeight()` and selector `[244, 79, 247, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "blockHeight", abi = "blockHeight()")]
    pub struct BlockHeightCall;
    #[doc = "Container type for all input parameters for the `bytes32ToUint8Array` function with signature `bytes32ToUint8Array(bytes32)` and selector `[59, 31, 86, 198]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bytes32ToUint8Array", abi = "bytes32ToUint8Array(bytes32)")]
    pub struct Bytes32ToUint8ArrayCall {
        pub input: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `commitments` function with signature `commitments(uint256)` and selector `[73, 206, 137, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "commitments", abi = "commitments(uint256)")]
    pub struct CommitmentsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `expand` function with signature `expand(uint8[])` and selector `[43, 129, 91, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "expand", abi = "expand(uint8[])")]
    pub struct ExpandCall {
        pub message: ::std::vec::Vec<u8>,
    }
    #[doc = "Container type for all input parameters for the `field_square` function with signature `field_square(uint256)` and selector `[85, 86, 158, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "field_square", abi = "field_square(uint256)")]
    pub struct FieldSquareCall {
        pub x: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hash_to_curve` function with signature `hash_to_curve(uint8[])` and selector `[55, 226, 50, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hash_to_curve", abi = "hash_to_curve(uint8[])")]
    pub struct HashToCurveCall {
        pub input: ::std::vec::Vec<u8>,
    }
    #[doc = "Container type for all input parameters for the `hash_to_field` function with signature `hash_to_field(uint8[])` and selector `[84, 202, 195, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hash_to_field", abi = "hash_to_field(uint8[])")]
    pub struct HashToFieldCall {
        pub message: ::std::vec::Vec<u8>,
    }
    #[doc = "Container type for all input parameters for the `newBlocks` function with signature `newBlocks(uint256[],bytes[])` and selector `[103, 105, 35, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "newBlocks", abi = "newBlocks(uint256[],bytes[])")]
    pub struct NewBlocksCall {
        pub new_commitments: ::std::vec::Vec<ethers::core::types::U256>,
        pub qcs: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum HotShotCalls {
        MaxBlocks(MaxBlocksCall),
        BlockHeight(BlockHeightCall),
        Bytes32ToUint8Array(Bytes32ToUint8ArrayCall),
        Commitments(CommitmentsCall),
        Expand(ExpandCall),
        FieldSquare(FieldSquareCall),
        HashToCurve(HashToCurveCall),
        HashToField(HashToFieldCall),
        NewBlocks(NewBlocksCall),
    }
    impl ethers::core::abi::AbiDecode for HotShotCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MaxBlocksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::MaxBlocks(decoded));
            }
            if let Ok(decoded) =
                <BlockHeightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::BlockHeight(decoded));
            }
            if let Ok(decoded) =
                <Bytes32ToUint8ArrayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::Bytes32ToUint8Array(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::Commitments(decoded));
            }
            if let Ok(decoded) = <ExpandCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::Expand(decoded));
            }
            if let Ok(decoded) =
                <FieldSquareCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::FieldSquare(decoded));
            }
            if let Ok(decoded) =
                <HashToCurveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::HashToCurve(decoded));
            }
            if let Ok(decoded) =
                <HashToFieldCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::HashToField(decoded));
            }
            if let Ok(decoded) =
                <NewBlocksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::NewBlocks(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for HotShotCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                HotShotCalls::MaxBlocks(element) => element.encode(),
                HotShotCalls::BlockHeight(element) => element.encode(),
                HotShotCalls::Bytes32ToUint8Array(element) => element.encode(),
                HotShotCalls::Commitments(element) => element.encode(),
                HotShotCalls::Expand(element) => element.encode(),
                HotShotCalls::FieldSquare(element) => element.encode(),
                HotShotCalls::HashToCurve(element) => element.encode(),
                HotShotCalls::HashToField(element) => element.encode(),
                HotShotCalls::NewBlocks(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for HotShotCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                HotShotCalls::MaxBlocks(element) => element.fmt(f),
                HotShotCalls::BlockHeight(element) => element.fmt(f),
                HotShotCalls::Bytes32ToUint8Array(element) => element.fmt(f),
                HotShotCalls::Commitments(element) => element.fmt(f),
                HotShotCalls::Expand(element) => element.fmt(f),
                HotShotCalls::FieldSquare(element) => element.fmt(f),
                HotShotCalls::HashToCurve(element) => element.fmt(f),
                HotShotCalls::HashToField(element) => element.fmt(f),
                HotShotCalls::NewBlocks(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxBlocksCall> for HotShotCalls {
        fn from(var: MaxBlocksCall) -> Self {
            HotShotCalls::MaxBlocks(var)
        }
    }
    impl ::std::convert::From<BlockHeightCall> for HotShotCalls {
        fn from(var: BlockHeightCall) -> Self {
            HotShotCalls::BlockHeight(var)
        }
    }
    impl ::std::convert::From<Bytes32ToUint8ArrayCall> for HotShotCalls {
        fn from(var: Bytes32ToUint8ArrayCall) -> Self {
            HotShotCalls::Bytes32ToUint8Array(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for HotShotCalls {
        fn from(var: CommitmentsCall) -> Self {
            HotShotCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<ExpandCall> for HotShotCalls {
        fn from(var: ExpandCall) -> Self {
            HotShotCalls::Expand(var)
        }
    }
    impl ::std::convert::From<FieldSquareCall> for HotShotCalls {
        fn from(var: FieldSquareCall) -> Self {
            HotShotCalls::FieldSquare(var)
        }
    }
    impl ::std::convert::From<HashToCurveCall> for HotShotCalls {
        fn from(var: HashToCurveCall) -> Self {
            HotShotCalls::HashToCurve(var)
        }
    }
    impl ::std::convert::From<HashToFieldCall> for HotShotCalls {
        fn from(var: HashToFieldCall) -> Self {
            HotShotCalls::HashToField(var)
        }
    }
    impl ::std::convert::From<NewBlocksCall> for HotShotCalls {
        fn from(var: NewBlocksCall) -> Self {
            HotShotCalls::NewBlocks(var)
        }
    }
    #[doc = "Container type for all return fields from the `MAX_BLOCKS` function with signature `MAX_BLOCKS()` and selector `[38, 131, 61, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxBlocksReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `blockHeight` function with signature `blockHeight()` and selector `[244, 79, 247, 18]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BlockHeightReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `bytes32ToUint8Array` function with signature `bytes32ToUint8Array(bytes32)` and selector `[59, 31, 86, 198]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct Bytes32ToUint8ArrayReturn {
        pub output: ::std::vec::Vec<u8>,
    }
    #[doc = "Container type for all return fields from the `commitments` function with signature `commitments(uint256)` and selector `[73, 206, 137, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CommitmentsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `expand` function with signature `expand(uint8[])` and selector `[43, 129, 91, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExpandReturn(pub ::std::vec::Vec<u8>);
    #[doc = "Container type for all return fields from the `field_square` function with signature `field_square(uint256)` and selector `[85, 86, 158, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FieldSquareReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hash_to_curve` function with signature `hash_to_curve(uint8[])` and selector `[55, 226, 50, 178]`"]
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
    #[doc = "Container type for all return fields from the `hash_to_field` function with signature `hash_to_field(uint8[])` and selector `[84, 202, 195, 197]`"]
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
}
