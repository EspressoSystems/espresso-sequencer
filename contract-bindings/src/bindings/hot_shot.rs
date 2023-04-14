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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidQC\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TooManyBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numQCs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongNumberOfQCs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_BLOCKS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"input\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bytes32ToUint8Array\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"output\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expand\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"input\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"field_from_random_bytes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hash_to_curve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"newCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"qcs\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlocks\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50611273806100206000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c806349ce89971161006657806349ce89971461010f578063676923501461012f578063781588c114610144578063b3111b6d14610157578063f44ff7121461016a57600080fd5b8063025ac515146100985780632491d2aa146100c557806326833dcc146100e55780633b1f56c6146100fc575b600080fd5b6100ab6100a6366004610d01565b610173565b604080519283526020830191909152015b60405180910390f35b6100d86100d3366004610d01565b610246565b6040516100bc9190610d96565b6100ee6103e881565b6040519081526020016100bc565b6100d861010a366004610ddd565b61069e565b6100ee61011d366004610ddd565b60006020819052908152604090205481565b61014261013d366004610e42565b610735565b005b6100ee610152366004610d01565b6108a2565b6100ee610165366004610eae565b610ad1565b6100ee60015481565b6000806000610181846108a2565b905060008051602061121e8339815191526003600082848509905082806101aa576101aa610f67565b848209905082806101bd576101bd610f67565b82820890506000806101ce83610aef565b925090505b806102375784806101e6576101e6610f67565b60018708955084806101fa576101fa610f67565b8687099250848061020d5761020d610f67565b8684099250848061022057610220610f67565b848408925061022e83610aef565b925090506101d3565b50939793965092945050505050565b60606030602060026000600185825b8660ff168110156102cf5780600003610298576040516001600160f81b031960f886901b16602082015260210160405160208183030381529060405291506102bd565b81846040516020016102ab929190610fad565b60405160208183030381529060405291505b806102c781610feb565b915050610255565b5080886040516020016102e3929190611004565b60405160208183030381529060405290508083604051602001610307929190610fad565b6040516020818303038152906040529050808660405160200161032b929190610fad565b60408051601f1981840301815290829052915083906103509083908390602001610fad565b60408051808303601f190181528282018252600180845260208085018290529251919550610382928692829101611037565b60408051808303601f190181528282528051602091820120818401819052825180850390920182528383019092529450906103c39085908790606001610fad565b60408051808303601f190181529082905283516020808601519297506103ec9388939101611037565b60408051601f1981840301815291905280516020820120909450600060ff8b1667ffffffffffffffff81111561042457610424610cba565b60405190808252806020026020018201604052801561044d578160200160208202803683370190505b509050600061045b8361069e565b905060005b81518110156104b95781818151811061047b5761047b611021565b602002602001015183828151811061049557610495611021565b60ff90921660209283029190910190910152806104b181610feb565b915050610460565b5060006104c58561069e565b905060005b8c8110156105b35780600003610548578281815181106104ec576104ec611021565b602002602001015182828151811061050657610506611021565b602002602001015118604051602001610532919060f89190911b6001600160f81b031916815260010190565b60405160208183030381529060405298506105a1565b8883828151811061055b5761055b611021565b602002602001015183838151811061057557610575611021565b60200260200101511860405160200161058f929190610fad565b60405160208183030381529060405298505b806105ab81610feb565b9150506104ca565b50878b6040516020016105c7929190610fad565b60408051808303601f190181529082905287516020808a0151929b506105f0938c939101611037565b6040516020818303038152906040529750878051906020012093506106148461069e565b915060005b8c8e60ff16610628919061106b565b81101561068a5782818151811061064157610641611021565b602002602001015184828f6106569190611084565b8151811061066657610666611021565b60ff909216602092830291909101909101528061068281610feb565b915050610619565b50919e9d5050505050505050505050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b602081101561072f576106d981601f61106b565b6106e4906008611097565b6106ef906002611192565b6106f9908461119e565b82828151811061070b5761070b611021565b60ff909216602092830291909101909101528061072781610feb565b9150506106c5565b50919050565b8281146107645760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e883111561078a5760405163e082840b60e01b81526004810184905260240161075b565b60015460005b84811015610861576107e56001548787848181106107b0576107b0611021565b905060200201358686858181106107c9576107c9611021565b90506020028101906107db91906111c0565b6001949350505050565b61080857600154604051637818671960e01b815260040161075b91815260200190565b85858281811061081a5761081a611021565b90506020020135600080600154815260200190815260200160002081905550600180600082825461084b9190611084565b9091555061085a905081610feb565b9050610790565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b6000806108ae83610246565b8051909150603081146108c3576108c3611207565b60008167ffffffffffffffff8111156108de576108de610cba565b604051908082528060200260200182016040528015610907578160200160208202803683370190505b50905060005b8281101561097a57836001610922838661106b565b61092c919061106b565b8151811061093c5761093c611021565b602002602001015182828151811061095657610956611021565b60ff909216602092830291909101909101528061097281610feb565b91505061090d565b5060408051601f8082526104008201909252600090819083602082016103e08036833701905050905060005b83811015610a125784816109ba868961106b565b6109c49190611084565b815181106109d4576109d4611021565b60200260200101518282815181106109ee576109ee611021565b60ff9092166020928302919091019091015280610a0a81610feb565b9150506109a6565b50610a1c81610ad1565b915061010060008051602061121e8339815191526000610a3c868961106b565b905060005b81811015610ac1576000610a85896001610a5b858761106b565b610a65919061106b565b81518110610a7557610a75611021565b602002602001015160ff16610bea565b90508380610a9557610a95610f67565b85880996508380610aa857610aa8610f67565b8188089650508080610ab990610feb565b915050610a41565b50939a9950505050505050505050565b600080610add83610c48565b9050610ae881610bea565b9392505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f529050600060008051602061121e833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa935050600051935082610bb15760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c6564210000000000604482015260640161075b565b80600185901b1115610bca57610bc7848261106b565b93505b8080610bd857610bd8610f67565b84850991508582149450505050915091565b600060008051602061121e8339815191528203610c0957610c09611207565b7f0e0a77c19a07df2f666ea36f7879462c0a78eb28f5c70b3dd35d438dc58f0d9d60008051602061121e83398151915260008183860995945050505050565b600080805b8351811015610cb357838181518110610c6857610c68611021565b602002602001015160ff16816008610c809190611097565b610c8b906002611192565b610c959190611097565b610c9f9083611084565b915080610cab81610feb565b915050610c4d565b5092915050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715610cf957610cf9610cba565b604052919050565b60006020808385031215610d1457600080fd5b823567ffffffffffffffff80821115610d2c57600080fd5b818501915085601f830112610d4057600080fd5b813581811115610d5257610d52610cba565b610d64601f8201601f19168501610cd0565b91508082528684828501011115610d7a57600080fd5b8084840185840137600090820190930192909252509392505050565b6020808252825182820181905260009190848201906040850190845b81811015610dd157835160ff1683529284019291840191600101610db2565b50909695505050505050565b600060208284031215610def57600080fd5b5035919050565b60008083601f840112610e0857600080fd5b50813567ffffffffffffffff811115610e2057600080fd5b6020830191508360208260051b8501011115610e3b57600080fd5b9250929050565b60008060008060408587031215610e5857600080fd5b843567ffffffffffffffff80821115610e7057600080fd5b610e7c88838901610df6565b90965094506020870135915080821115610e9557600080fd5b50610ea287828801610df6565b95989497509550505050565b60006020808385031215610ec157600080fd5b823567ffffffffffffffff80821115610ed957600080fd5b818501915085601f830112610eed57600080fd5b813581811115610eff57610eff610cba565b8060051b9150610f10848301610cd0565b8181529183018401918481019088841115610f2a57600080fd5b938501935b83851015610f5b578435925060ff83168314610f4b5760008081fd5b8282529385019390850190610f2f565b98975050505050505050565b634e487b7160e01b600052601260045260246000fd5b6000815160005b81811015610f9e5760208185018101518683015201610f84565b50600093019283525090919050565b6000610fb98285610f7d565b60f89390931b6001600160f81b03191683525050600101919050565b634e487b7160e01b600052601160045260246000fd5b600060018201610ffd57610ffd610fd5565b5060010190565b60006110196110138386610f7d565b84610f7d565b949350505050565b634e487b7160e01b600052603260045260246000fd5b60006110438286610f7d565b6001600160f81b031960f895861b811682529390941b90921660018401525050600201919050565b8181038181111561107e5761107e610fd5565b92915050565b8082018082111561107e5761107e610fd5565b808202811582820484141761107e5761107e610fd5565b600181815b808511156110e95781600019048211156110cf576110cf610fd5565b808516156110dc57918102915b93841c93908002906110b3565b509250929050565b6000826111005750600161107e565b8161110d5750600061107e565b8160018114611123576002811461112d57611149565b600191505061107e565b60ff84111561113e5761113e610fd5565b50506001821b61107e565b5060208310610133831016604e8410600b841016171561116c575081810a61107e565b61117683836110ae565b806000190482111561118a5761118a610fd5565b029392505050565b6000610ae883836110f1565b6000826111bb57634e487b7160e01b600052601260045260246000fd5b500490565b6000808335601e198436030181126111d757600080fd5b83018035915067ffffffffffffffff8211156111f257600080fd5b602001915036819003821315610e3b57600080fd5b634e487b7160e01b600052600160045260246000fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212200575fc4bf0f65cb618cacdd61490839ed8a7b91a6c6dc683b0d3107121c82f2e64736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `expand` (0x2491d2aa) function"]
        pub fn expand(
            &self,
            message: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u8>> {
            self.0
                .method_hash([36, 145, 210, 170], message)
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
    #[doc = "Container type for all input parameters for the `expand` function with signature `expand(bytes)` and selector `[36, 145, 210, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "expand", abi = "expand(bytes)")]
    pub struct ExpandCall {
        pub message: ethers::core::types::Bytes,
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
    #[doc = "Container type for all return fields from the `expand` function with signature `expand(bytes)` and selector `[36, 145, 210, 170]`"]
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
}
