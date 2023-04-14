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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidQC\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TooManyBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numQCs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongNumberOfQCs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_BLOCKS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"input\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bytes32ToUint8Array\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"output\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expand\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"input\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"field_from_random_bytes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"input\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"from_le_bytes_mod_order\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"newCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"qcs\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlocks\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50611400806100206000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c806351f253dd1161006657806351f253dd1461010757806354cac3c51461011a578063676923501461012d578063b3111b6d14610142578063f44ff7121461015557600080fd5b806326833dcc146100985780632b815b57146100b45780633b1f56c6146100d457806349ce8997146100e7575b600080fd5b6100a16103e881565b6040519081526020015b60405180910390f35b6100c76100c2366004610f4a565b61015e565b6040516100ab919061100f565b6100c76100e2366004611056565b6105ef565b6100a16100f5366004611056565b60006020819052908152604090205481565b6100a1610115366004610f4a565b610686565b6100a1610128366004610f4a565b610809565b61014061013b3660046110bb565b610cc5565b005b6100a1610150366004610f4a565b610e32565b6100a160015481565b60606030602060026000600185825b8660ff168110156101e757806000036101b0576040516001600160f81b031960f886901b16602082015260210160405160208183030381529060405291506101d5565b81846040516020016101c3929190611157565b60405160208183030381529060405291505b806101df81611195565b91505061016d565b5060005b88518110156102445781898281518110610207576102076111ae565b6020026020010151604051602001610220929190611157565b6040516020818303038152906040529150808061023c90611195565b9150506101eb565b508083604051602001610258929190611157565b6040516020818303038152906040529050808660405160200161027c929190611157565b60408051601f1981840301815290829052915083906102a19083908390602001611157565b60408051808303601f1901815282820182526001808452602080850182905292519195506102d39286928291016111c4565b60408051808303601f190181528282528051602091820120818401819052825180850390920182528383019092529450906103149085908790606001611157565b60408051808303601f1901815290829052835160208086015192975061033d93889391016111c4565b60408051601f1981840301815291905280516020820120909450600060ff8b1667ffffffffffffffff81111561037557610375610f1e565b60405190808252806020026020018201604052801561039e578160200160208202803683370190505b50905060006103ac836105ef565b905060005b815181101561040a578181815181106103cc576103cc6111ae565b60200260200101518382815181106103e6576103e66111ae565b60ff909216602092830291909101909101528061040281611195565b9150506103b1565b506000610416856105ef565b905060005b8c81101561050457806000036104995782818151811061043d5761043d6111ae565b6020026020010151828281518110610457576104576111ae565b602002602001015118604051602001610483919060f89190911b6001600160f81b031916815260010190565b60405160208183030381529060405298506104f2565b888382815181106104ac576104ac6111ae565b60200260200101518383815181106104c6576104c66111ae565b6020026020010151186040516020016104e0929190611157565b60405160208183030381529060405298505b806104fc81611195565b91505061041b565b50878b604051602001610518929190611157565b60408051808303601f190181529082905287516020808a0151929b50610541938c9391016111c4565b604051602081830303815290604052975087805190602001209350610565846105ef565b915060005b8c8e60ff1661057991906111f8565b8110156105db57828181518110610592576105926111ae565b602002602001015184828f6105a79190611211565b815181106105b7576105b76111ae565b60ff90921660209283029190910190910152806105d381611195565b91505061056a565b50919e9d5050505050505050505050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b60208110156106805761062a81601f6111f8565b610635906008611224565b61064090600261131f565b61064a908461132b565b82828151811061065c5761065c6111ae565b60ff909216602092830291909101909101528061067881611195565b915050610616565b50919050565b805160408051601f80825261040082019092526000928391829084602082016103e08036833701905050905060005b848110156107215786816106c987866111f8565b6106d39190611211565b815181106106e3576106e36111ae565b60200260200101518282815181106106fd576106fd6111ae565b60ff909216602092830291909101909101528061071981611195565b9150506106b5565b5061072b81610e32565b9250600061073a610100610e50565b9050807f0d791464ef86e357276f48b709e2a0fcad825aed9626b0fffbdb0f2afaec667a1461076b5761076b61134d565b600061077786856111f8565b90506000805b828110156107fb576107bf8a600161079584876111f8565b61079f91906111f8565b815181106107af576107af6111ae565b602002602001015160ff16610e50565b91506000805160206113ab83398151915284880996506000805160206113ab8339815191528288089650806107f381611195565b91505061077d565b509498975050505050505050565b6000806108158361015e565b80519091506030811461082a5761082a61134d565b60008167ffffffffffffffff81111561084557610845610f1e565b60405190808252806020026020018201604052801561086e578160200160208202803683370190505b50905060005b828110156108e15783600161088983866111f8565b61089391906111f8565b815181106108a3576108a36111ae565b60200260200101518282815181106108bd576108bd6111ae565b60ff90921660209283029190910190910152806108d981611195565b915050610874565b5060408051601f8082526104008201909252600090819083602082016103e08036833701905050905060005b8381101561097957848161092186896111f8565b61092b9190611211565b8151811061093b5761093b6111ae565b6020026020010151828281518110610955576109556111ae565b60ff909216602092830291909101909101528061097181611195565b91505061090d565b50604080516103e08101825260ec8152604a6020820152600491810191909152600f60608201526002608082015260c360a082015260f760c0820152606560e082015260cb610100820152602e610120820152602761014082015260e661016082015260e9610180820152602c6101a082015260bf6101c082015260096101e082015260cc610200820152606b610220820152608f6102408201526023610260820152600d61028082015260916102a082015260166102c082015260186102e08201526078610300820152606661032082015260006103408201819052606c610360830152602a610380830152606d6103a0830152602d6103c08301525b84811015610ad557828181518110610a9157610a916111ae565b602002602001015160ff168282601f8110610aae57610aae6111ae565b602002015160ff1614610ac357610ac361134d565b80610acd81611195565b915050610a77565b50604080516102208101825260bf81526004602082015260f09181019190915260c06060820181905260566080830152602b60a083015260f590820152602560e0820152604861010082015260446101208201526064610140820152606e61016082015260b1610180820152604a6101a0820152605a6101c082015260e46101e082015260c161020082015260005b610b6e86896111f8565b811015610bcb57818160118110610b8757610b876111ae565b602002015160ff16878281518110610ba157610ba16111ae565b602002602001015160ff1614610bb957610bb961134d565b80610bc381611195565b915050610b64565b50610bd583610e32565b9350610be083610eac565b7e2d6d2a6c0066781816910d238f6bcc09bf2ce9e6272ecb65f7c3020f044aec14610c0d57610c0d61134d565b6000610c1a610100610e50565b9050807f0d791464ef86e357276f48b709e2a0fcad825aed9626b0fffbdb0f2afaec667a14610c4b57610c4b61134d565b6000610c57878a6111f8565b905060005b81811015610cb4576000610c768a600161079585876111f8565b90506000805160206113ab83398151915284890997506000805160206113ab8339815191528189089750508080610cac90611195565b915050610c5c565b50949b9a5050505050505050505050565b828114610cf45760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e8831115610d1a5760405163e082840b60e01b815260048101849052602401610ceb565b60015460005b84811015610df157610d75600154878784818110610d4057610d406111ae565b90506020020135868685818110610d5957610d596111ae565b9050602002810190610d6b9190611363565b6001949350505050565b610d9857600154604051637818671960e01b8152600401610ceb91815260200190565b858582818110610daa57610daa6111ae565b905060200201356000806001548152602001908152602001600020819055506001806000828254610ddb9190611211565b90915550610dea905081611195565b9050610d20565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b600080610e3e83610eac565b9050610e4981610e50565b9392505050565b60006000805160206113ab8339815191528203610e6f57610e6f61134d565b7f0e0a77c19a07df2f666ea36f7879462c0a78eb28f5c70b3dd35d438dc58f0d9d60006000805160206113ab833981519152828509949350505050565b600080805b8351811015610f1757838181518110610ecc57610ecc6111ae565b602002602001015160ff16816008610ee49190611224565b610eef90600261131f565b610ef99190611224565b610f039083611211565b915080610f0f81611195565b915050610eb1565b5092915050565b634e487b7160e01b600052604160045260246000fd5b803560ff81168114610f4557600080fd5b919050565b60006020808385031215610f5d57600080fd5b823567ffffffffffffffff80821115610f7557600080fd5b818501915085601f830112610f8957600080fd5b813581811115610f9b57610f9b610f1e565b8060051b604051601f19603f83011681018181108582111715610fc057610fc0610f1e565b604052918252848201925083810185019188831115610fde57600080fd5b938501935b8285101561100357610ff485610f34565b84529385019392850192610fe3565b98975050505050505050565b6020808252825182820181905260009190848201906040850190845b8181101561104a57835160ff168352928401929184019160010161102b565b50909695505050505050565b60006020828403121561106857600080fd5b5035919050565b60008083601f84011261108157600080fd5b50813567ffffffffffffffff81111561109957600080fd5b6020830191508360208260051b85010111156110b457600080fd5b9250929050565b600080600080604085870312156110d157600080fd5b843567ffffffffffffffff808211156110e957600080fd5b6110f58883890161106f565b9096509450602087013591508082111561110e57600080fd5b5061111b8782880161106f565b95989497509550505050565b6000815160005b81811015611148576020818501810151868301520161112e565b50600093019283525090919050565b60006111638285611127565b60f89390931b6001600160f81b03191683525050600101919050565b634e487b7160e01b600052601160045260246000fd5b6000600182016111a7576111a761117f565b5060010190565b634e487b7160e01b600052603260045260246000fd5b60006111d08286611127565b6001600160f81b031960f895861b811682529390941b90921660018401525050600201919050565b8181038181111561120b5761120b61117f565b92915050565b8082018082111561120b5761120b61117f565b808202811582820484141761120b5761120b61117f565b600181815b8085111561127657816000190482111561125c5761125c61117f565b8085161561126957918102915b93841c9390800290611240565b509250929050565b60008261128d5750600161120b565b8161129a5750600061120b565b81600181146112b057600281146112ba576112d6565b600191505061120b565b60ff8411156112cb576112cb61117f565b50506001821b61120b565b5060208310610133831016604e8410600b84101617156112f9575081810a61120b565b611303838361123b565b80600019048211156113175761131761117f565b029392505050565b6000610e49838361127e565b60008261134857634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052600160045260246000fd5b6000808335601e1984360301811261137a57600080fd5b83018035915067ffffffffffffffff82111561139557600080fd5b6020019150368190038213156110b457600080fdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220b040662611c2ff4caf6b0d6774a9a8c76549ad6650d84eb58a3093e888d2eada64736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `from_le_bytes_mod_order` (0x51f253dd) function"]
        pub fn from_le_bytes_mod_order(
            &self,
            input: ::std::vec::Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([81, 242, 83, 221], input)
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
    #[doc = "Container type for all input parameters for the `from_le_bytes_mod_order` function with signature `from_le_bytes_mod_order(uint8[])` and selector `[81, 242, 83, 221]`"]
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
        name = "from_le_bytes_mod_order",
        abi = "from_le_bytes_mod_order(uint8[])"
    )]
    pub struct FromLeBytesModOrderCall {
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
        FromLeBytesModOrder(FromLeBytesModOrderCall),
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
                <FromLeBytesModOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::FromLeBytesModOrder(decoded));
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
                HotShotCalls::FromLeBytesModOrder(element) => element.encode(),
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
                HotShotCalls::FromLeBytesModOrder(element) => element.fmt(f),
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
    impl ::std::convert::From<FromLeBytesModOrderCall> for HotShotCalls {
        fn from(var: FromLeBytesModOrderCall) -> Self {
            HotShotCalls::FromLeBytesModOrder(var)
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
    #[doc = "Container type for all return fields from the `from_le_bytes_mod_order` function with signature `from_le_bytes_mod_order(uint8[])` and selector `[81, 242, 83, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FromLeBytesModOrderReturn(pub ethers::core::types::U256);
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
