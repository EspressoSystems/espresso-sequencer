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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidQC\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TooManyBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numQCs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongNumberOfQCs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_BLOCKS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"input\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bytes32ToUint8Array\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"output\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expand\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"input\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"field_from_random_bytes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"field_square\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"input\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hash_to_curve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"newCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"qcs\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlocks\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506111b5806100206000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c806354cac3c51161006657806354cac3c51461013a57806355569ece1461014d5780636769235014610160578063b3111b6d14610175578063f44ff7121461018857600080fd5b806326833dcc146100a35780632b815b57146100bf57806337e232b2146100df5780633b1f56c61461010757806349ce89971461011a575b600080fd5b6100ac6103e881565b6040519081526020015b60405180910390f35b6100d26100cd366004610ce8565b610191565b6040516100b69190610dad565b6100f26100ed366004610ce8565b610622565b604080519283526020830191909152016100b6565b6100d2610115366004610df4565b6106f5565b6100ac610128366004610df4565b60006020819052908152604090205481565b6100ac610148366004610ce8565b61078c565b6100ac61015b366004610df4565b6109b3565b61017361016e366004610e59565b6109d1565b005b6100ac610183366004610ce8565b610b3e565b6100ac60015481565b60606030602060026000600185825b8660ff1681101561021a57806000036101e3576040516001600160f81b031960f886901b1660208201526021016040516020818303038152906040529150610208565b81846040516020016101f6929190610ef5565b60405160208183030381529060405291505b8061021281610f33565b9150506101a0565b5060005b8851811015610277578189828151811061023a5761023a610f4c565b6020026020010151604051602001610253929190610ef5565b6040516020818303038152906040529150808061026f90610f33565b91505061021e565b50808360405160200161028b929190610ef5565b604051602081830303815290604052905080866040516020016102af929190610ef5565b60408051601f1981840301815290829052915083906102d49083908390602001610ef5565b60408051808303601f190181528282018252600180845260208085018290529251919550610306928692829101610f62565b60408051808303601f190181528282528051602091820120818401819052825180850390920182528383019092529450906103479085908790606001610ef5565b60408051808303601f190181529082905283516020808601519297506103709388939101610f62565b60408051601f1981840301815291905280516020820120909450600060ff8b1667ffffffffffffffff8111156103a8576103a8610cbc565b6040519080825280602002602001820160405280156103d1578160200160208202803683370190505b50905060006103df836106f5565b905060005b815181101561043d578181815181106103ff576103ff610f4c565b602002602001015183828151811061041957610419610f4c565b60ff909216602092830291909101909101528061043581610f33565b9150506103e4565b506000610449856106f5565b905060005b8c81101561053757806000036104cc5782818151811061047057610470610f4c565b602002602001015182828151811061048a5761048a610f4c565b6020026020010151186040516020016104b6919060f89190911b6001600160f81b031916815260010190565b6040516020818303038152906040529850610525565b888382815181106104df576104df610f4c565b60200260200101518383815181106104f9576104f9610f4c565b602002602001015118604051602001610513929190610ef5565b60405160208183030381529060405298505b8061052f81610f33565b91505061044e565b50878b60405160200161054b929190610ef5565b60408051808303601f190181529082905287516020808a0151929b50610574938c939101610f62565b604051602081830303815290604052975087805190602001209350610598846106f5565b915060005b8c8e60ff166105ac9190610f96565b81101561060e578281815181106105c5576105c5610f4c565b602002602001015184828f6105da9190610fa9565b815181106105ea576105ea610f4c565b60ff909216602092830291909101909101528061060681610f33565b91505061059d565b50919e9d5050505050505050505050505050565b60008060006106308461078c565b905060008051602061116083398151915260036000828485099050828061065957610659610fbc565b8482099050828061066c5761066c610fbc565b828208905060008061067d83610b4f565b925090505b806106e657848061069557610695610fbc565b60018708955084806106a9576106a9610fbc565b868709925084806106bc576106bc610fbc565b868409925084806106cf576106cf610fbc565b84840892506106dd83610b4f565b92509050610682565b50939793965092945050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b60208110156107865761073081601f610f96565b61073b906008610fd2565b6107469060026110cd565b61075090846110e0565b82828151811061076257610762610f4c565b60ff909216602092830291909101909101528061077e81610f33565b91505061071c565b50919050565b60008061079883610191565b8051909150603081146107ad576107ad611102565b60008167ffffffffffffffff8111156107c8576107c8610cbc565b6040519080825280602002602001820160405280156107f1578160200160208202803683370190505b50905060005b828110156108645783600161080c8386610f96565b6108169190610f96565b8151811061082657610826610f4c565b602002602001015182828151811061084057610840610f4c565b60ff909216602092830291909101909101528061085c81610f33565b9150506107f7565b5060408051601f8082526104008201909252600090819083602082016103e08036833701905050905060005b838110156108fc5784816108a48689610f96565b6108ae9190610fa9565b815181106108be576108be610f4c565b60200260200101518282815181106108d8576108d8610f4c565b60ff90921660209283029190910190910152806108f481610f33565b915050610890565b5061090681610b3e565b915061010060008051602061116083398151915260006109268689610f96565b905060005b818110156109a35760008860016109428486610f96565b61094c9190610f96565b8151811061095c5761095c610f4c565b602002602001015160ff169050838061097757610977610fbc565b8588099650838061098a5761098a610fbc565b818808965050808061099b90610f33565b91505061092b565b50939a9950505050505050505050565b60006000805160206111608339815191528181848509949350505050565b828114610a005760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e8831115610a265760405163e082840b60e01b8152600481018490526024016109f7565b60015460005b84811015610afd57610a81600154878784818110610a4c57610a4c610f4c565b90506020020135868685818110610a6557610a65610f4c565b9050602002810190610a779190611118565b6001949350505050565b610aa457600154604051637818671960e01b81526004016109f791815260200190565b858582818110610ab657610ab6610f4c565b905060200201356000806001548152602001908152602001600020819055506001806000828254610ae79190610fa9565b90915550610af6905081610f33565b9050610a2c565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b6000610b4982610c4a565b92915050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5290506000600080516020611160833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa935050600051935082610c115760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c656421000000000060448201526064016109f7565b80600185901b1115610c2a57610c278482610f96565b93505b8080610c3857610c38610fbc565b84850991508582149450505050915091565b600080805b8351811015610cb557838181518110610c6a57610c6a610f4c565b602002602001015160ff16816008610c829190610fd2565b610c8d9060026110cd565b610c979190610fd2565b610ca19083610fa9565b915080610cad81610f33565b915050610c4f565b5092915050565b634e487b7160e01b600052604160045260246000fd5b803560ff81168114610ce357600080fd5b919050565b60006020808385031215610cfb57600080fd5b823567ffffffffffffffff80821115610d1357600080fd5b818501915085601f830112610d2757600080fd5b813581811115610d3957610d39610cbc565b8060051b604051601f19603f83011681018181108582111715610d5e57610d5e610cbc565b604052918252848201925083810185019188831115610d7c57600080fd5b938501935b82851015610da157610d9285610cd2565b84529385019392850192610d81565b98975050505050505050565b6020808252825182820181905260009190848201906040850190845b81811015610de857835160ff1683529284019291840191600101610dc9565b50909695505050505050565b600060208284031215610e0657600080fd5b5035919050565b60008083601f840112610e1f57600080fd5b50813567ffffffffffffffff811115610e3757600080fd5b6020830191508360208260051b8501011115610e5257600080fd5b9250929050565b60008060008060408587031215610e6f57600080fd5b843567ffffffffffffffff80821115610e8757600080fd5b610e9388838901610e0d565b90965094506020870135915080821115610eac57600080fd5b50610eb987828801610e0d565b95989497509550505050565b6000815160005b81811015610ee65760208185018101518683015201610ecc565b50600093019283525090919050565b6000610f018285610ec5565b60f89390931b6001600160f81b03191683525050600101919050565b634e487b7160e01b600052601160045260246000fd5b600060018201610f4557610f45610f1d565b5060010190565b634e487b7160e01b600052603260045260246000fd5b6000610f6e8286610ec5565b6001600160f81b031960f895861b811682529390941b90921660018401525050600201919050565b81810381811115610b4957610b49610f1d565b80820180821115610b4957610b49610f1d565b634e487b7160e01b600052601260045260246000fd5b8082028115828204841417610b4957610b49610f1d565b600181815b8085111561102457816000190482111561100a5761100a610f1d565b8085161561101757918102915b93841c9390800290610fee565b509250929050565b60008261103b57506001610b49565b8161104857506000610b49565b816001811461105e576002811461106857611084565b6001915050610b49565b60ff84111561107957611079610f1d565b50506001821b610b49565b5060208310610133831016604e8410600b84101617156110a7575081810a610b49565b6110b18383610fe9565b80600019048211156110c5576110c5610f1d565b029392505050565b60006110d9838361102c565b9392505050565b6000826110fd57634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052600160045260246000fd5b6000808335601e1984360301811261112f57600080fd5b83018035915067ffffffffffffffff82111561114a57600080fd5b602001915036819003821315610e5257600080fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220e0c0a5003d67589ebdd9369aa89ad3b2d1c4db713eed81088e1b7fa3debcd40a64736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `field_from_random_bytes` (0xb3111b6d) function"]
        pub fn field_from_random_bytes(
            &self,
            input: ::std::vec::Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([179, 17, 27, 109], input)
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
    #[doc = "Container type for all input parameters for the `field_from_random_bytes` function with signature `field_from_random_bytes(uint8[])` and selector `[179, 17, 27, 109]`"]
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
        name = "field_from_random_bytes",
        abi = "field_from_random_bytes(uint8[])"
    )]
    pub struct FieldFromRandomBytesCall {
        pub input: ::std::vec::Vec<u8>,
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
        FieldFromRandomBytes(FieldFromRandomBytesCall),
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
                <FieldFromRandomBytesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::FieldFromRandomBytes(decoded));
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
                HotShotCalls::FieldFromRandomBytes(element) => element.encode(),
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
                HotShotCalls::FieldFromRandomBytes(element) => element.fmt(f),
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
    impl ::std::convert::From<FieldFromRandomBytesCall> for HotShotCalls {
        fn from(var: FieldFromRandomBytesCall) -> Self {
            HotShotCalls::FieldFromRandomBytes(var)
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
    #[doc = "Container type for all return fields from the `field_from_random_bytes` function with signature `field_from_random_bytes(uint8[])` and selector `[179, 17, 27, 109]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FieldFromRandomBytesReturn(pub ethers::core::types::U256);
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
