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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidQC\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TooManyBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numQCs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongNumberOfQCs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_BLOCKS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"input\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hash_to_curve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"newCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"qcs\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct BN254.G1Point\",\"name\":\"sig\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct BN254.G2Point\",\"name\":\"pk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"x1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y1\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verify_bls_sig\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061151d806100206000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c8063676923501161005b57806367692350146100e6578063781588c1146100fb578063f44ff7121461010e578063f929c51f1461011757600080fd5b8063025ac5151461008257806326833dcc146100af57806349ce8997146100c6575b600080fd5b610095610090366004611043565b61013a565b604080519283526020830191909152015b60405180910390f35b6100b86103e881565b6040519081526020016100a6565b6100b86100d4366004611078565b60006020819052908152604090205481565b6100f96100f43660046110dd565b61014f565b005b6100b8610109366004611043565b6102bc565b6100b860015481565b61012a610125366004611149565b6102cd565b60405190151581526020016100a6565b600080610146836102e2565b91509150915091565b82811461017e5760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e88311156101a45760405163e082840b60e01b815260048101849052602401610175565b60015460005b8481101561027b576101ff6001548787848181106101ca576101ca6111fd565b905060200201358686858181106101e3576101e36111fd565b90506020028101906101f59190611213565b6001949350505050565b61022257600154604051637818671960e01b815260040161017591815260200190565b858582818110610234576102346111fd565b9050602002013560008060015481526020019081526020016000208190555060018060008282546102659190611270565b90915550610274905081611283565b90506101aa565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b60006102c7826103b5565b92915050565b60006102da8484846105dc565b949350505050565b60008060006102f0846103b5565b90506000805160206114c88339815191526003600082848509905082806103195761031961129c565b8482099050828061032c5761032c61129c565b828208905060008061033d83610662565b925090505b806103a65784806103555761035561129c565b60018708955084806103695761036961129c565b8687099250848061037c5761037c61129c565b8684099250848061038f5761038f61129c565b848408925061039d83610662565b92509050610342565b50939793965092945050505050565b6000806103c18361075d565b8051909150603081146103d6576103d66112b2565b60008167ffffffffffffffff8111156103f1576103f1610f54565b60405190808252806020026020018201604052801561041a578160200160208202803683370190505b50905060005b8281101561048d5783600161043583866112c8565b61043f91906112c8565b8151811061044f5761044f6111fd565b6020026020010151828281518110610469576104696111fd565b60ff909216602092830291909101909101528061048581611283565b915050610420565b5060408051601f8082526104008201909252600090819083602082016103e08036833701905050905060005b838110156105255784816104cd86896112c8565b6104d79190611270565b815181106104e7576104e76111fd565b6020026020010151828281518110610501576105016111fd565b60ff909216602092830291909101909101528061051d81611283565b9150506104b9565b5061052f81610b11565b91506101006000805160206114c8833981519152600061054f86896112c8565b905060005b818110156105cc57600088600161056b84866112c8565b61057591906112c8565b81518110610585576105856111fd565b602002602001015160ff16905083806105a0576105a061129c565b858809965083806105b3576105b361129c565b81880896505080806105c490611283565b915050610554565b50939a9950505050505050505050565b60008060006105ea85610b83565b60006040518060600160405280602481526020016114a460249139905060006106138883610c12565b905061061e816102e2565b60408051808201909152828152602081018290529195509350600061065482896106478c610c8f565b61064f610d0a565b610ddb565b9a9950505050505050505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52905060006000805160206114c8833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa9350506000519350826107245760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c65642100000000006044820152606401610175565b80600185901b111561073d5761073a84826112c8565b93505b808061074b5761074b61129c565b84850991508582149450505050915091565b60408051603080825260608281019093529060209060029060009060019082908660208201818036833701905050905080886040516020016107a092919061130b565b604051602081830303815290604052905080836040516020016107c4929190611320565b604051602081830303815290604052905080866040516020016107e8929190611320565b60408051601f19818403018152908290529150839061080d9083908390602001611320565b60408051601f1981840301815290829052925061010160f01b906108379084908390602001611348565b60408051808303601f190181528282528051602091820120818401819052825180850390920182528383019092529450906108789085908790606001611320565b6040516020818303038152906040529350838260405160200161089c929190611348565b60408051601f1981840301815291905280516020820120909450600060ff8b1667ffffffffffffffff8111156108d4576108d4610f54565b6040519080825280602002602001820160405280156108fd578160200160208202803683370190505b509050600061090b83610ebd565b905060005b81518110156109695781818151811061092b5761092b6111fd565b6020026020010151838281518110610945576109456111fd565b60ff909216602092830291909101909101528061096181611283565b915050610910565b50600061097585610ebd565b905060005b8c811015610a2b576000838281518110610996576109966111fd565b60200260200101518383815181106109b0576109b06111fd565b6020026020010151189050816000036109f3576040516001600160f81b031960f883901b1660208201526021016040516020818303038152906040529950610a18565b8981604051602001610a06929190611320565b60405160208183030381529060405299505b5080610a2381611283565b91505061097a565b50878b604051602001610a3f929190611320565b60405160208183030381529060405297508786604051602001610a63929190611348565b604051602081830303815290604052975087805190602001209350610a8784610ebd565b915060005b8c8e60ff16610a9b91906112c8565b811015610afd57828181518110610ab457610ab46111fd565b602002602001015184828f610ac99190611270565b81518110610ad957610ad96111fd565b60ff9092166020928302919091019091015280610af581611283565b915050610a8c565b50919e9d5050505050505050505050505050565b600080805b8351811015610b7c57838181518110610b3157610b316111fd565b602002602001015160ff16816008610b49919061136d565b610b54906002611468565b610b5e919061136d565b610b689083611270565b915080610b7481611283565b915050610b16565b5092915050565b805160208201516000916000805160206114c883398151915291826003818085800985090883828309148115831517198483108585101616169350505081610c0d5760405162461bcd60e51b815260206004820152601760248201527f426e3235343a20696e76616c696420473120706f696e740000000000000000006044820152606401610175565b505050565b6060806040519050835180825260208201818101602087015b81831015610c43578051835260209283019201610c2b565b50855184518101855292509050808201602086015b81831015610c70578051835260209283019201610c58565b508651929092011591909101601f01601f191660405250905092915050565b6040805180820190915260008082526020820152815160208301511590151615610cb7575090565b6040518060400160405280836000015181526020016000805160206114c88339815191528460200151610cea919061147b565b610d02906000805160206114c88339815191526112c8565b905292915050565b610d356040518060800160405280600081526020016000815260200160008152602001600081525090565b60405180608001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed81526020017f090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b81526020017f12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa815250905090565b60008060006040518751815260208801516020820152865160408201526020870151606082015260408701516080820152606087015160a0820152855160c0820152602086015160e08201528451610100820152602085015161012082015260408501516101408201526060850151610160820152602060006101808360085afa915050600051915080610eb15760405162461bcd60e51b815260206004820152601c60248201527f426e3235343a2050616972696e6720636865636b206661696c656421000000006044820152606401610175565b50151595945050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b6020811015610f4e57610ef881601f6112c8565b610f0390600861136d565b610f0e906002611468565b610f18908461148f565b828281518110610f2a57610f2a6111fd565b60ff9092166020928302919091019091015280610f4681611283565b915050610ee4565b50919050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715610f8d57610f8d610f54565b60405290565b6040516080810167ffffffffffffffff81118282101715610f8d57610f8d610f54565b600082601f830112610fc757600080fd5b813567ffffffffffffffff80821115610fe257610fe2610f54565b604051601f8301601f19908116603f0116810190828211818310171561100a5761100a610f54565b8160405283815286602085880101111561102357600080fd5b836020870160208301376000602085830101528094505050505092915050565b60006020828403121561105557600080fd5b813567ffffffffffffffff81111561106c57600080fd5b6102da84828501610fb6565b60006020828403121561108a57600080fd5b5035919050565b60008083601f8401126110a357600080fd5b50813567ffffffffffffffff8111156110bb57600080fd5b6020830191508360208260051b85010111156110d657600080fd5b9250929050565b600080600080604085870312156110f357600080fd5b843567ffffffffffffffff8082111561110b57600080fd5b61111788838901611091565b9096509450602087013591508082111561113057600080fd5b5061113d87828801611091565b95989497509550505050565b600080600083850360e081121561115f57600080fd5b843567ffffffffffffffff81111561117657600080fd5b61118287828801610fb6565b9450506040601f198201121561119757600080fd5b61119f610f6a565b602086810135825260408701359082015292506080605f19820112156111c457600080fd5b506111cd610f93565b606085013581526080850135602082015260a0850135604082015260c08501356060820152809150509250925092565b634e487b7160e01b600052603260045260246000fd5b6000808335601e1984360301811261122a57600080fd5b83018035915067ffffffffffffffff82111561124557600080fd5b6020019150368190038213156110d657600080fd5b634e487b7160e01b600052601160045260246000fd5b808201808211156102c7576102c761125a565b6000600182016112955761129561125a565b5060010190565b634e487b7160e01b600052601260045260246000fd5b634e487b7160e01b600052600160045260246000fd5b818103818111156102c7576102c761125a565b6000815160005b818110156112fc57602081850181015186830152016112e2565b50600093019283525090919050565b60006102da61131a83866112db565b846112db565b600061132c82856112db565b60f89390931b6001600160f81b03191683525050600101919050565b600061135482856112db565b6001600160f01b03199390931683525050600201919050565b80820281158282048414176102c7576102c761125a565b600181815b808511156113bf5781600019048211156113a5576113a561125a565b808516156113b257918102915b93841c9390800290611389565b509250929050565b6000826113d6575060016102c7565b816113e3575060006102c7565b81600181146113f957600281146114035761141f565b60019150506102c7565b60ff8411156114145761141461125a565b50506001821b6102c7565b5060208310610133831016604e8410600b8410161715611442575081810a6102c7565b61144c8383611384565b80600019048211156114605761146061125a565b029392505050565b600061147483836113c7565b9392505050565b60008261148a5761148a61129c565b500690565b60008261149e5761149e61129c565b50049056fe424c535f5349475f424e32353447315f584d443a4b454343414b5f4e4354485f4e554c5f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220417cb7435f4d869a6eb2c8a34ac7dbc0569a48d8b655677672a1ae4790c453d564736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `commitments` (0x49ce8997) function"]
        pub fn commitments(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([73, 206, 137, 151], p0)
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
    pub enum HotShotCalls {
        MaxBlocks(MaxBlocksCall),
        BlockHeight(BlockHeightCall),
        Commitments(CommitmentsCall),
        HashToCurve(HashToCurveCall),
        HashToField(HashToFieldCall),
        NewBlocks(NewBlocksCall),
        VerifyBlsSig(VerifyBlsSigCall),
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
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::Commitments(decoded));
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
            if let Ok(decoded) =
                <VerifyBlsSigCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotCalls::VerifyBlsSig(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for HotShotCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                HotShotCalls::MaxBlocks(element) => element.encode(),
                HotShotCalls::BlockHeight(element) => element.encode(),
                HotShotCalls::Commitments(element) => element.encode(),
                HotShotCalls::HashToCurve(element) => element.encode(),
                HotShotCalls::HashToField(element) => element.encode(),
                HotShotCalls::NewBlocks(element) => element.encode(),
                HotShotCalls::VerifyBlsSig(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for HotShotCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                HotShotCalls::MaxBlocks(element) => element.fmt(f),
                HotShotCalls::BlockHeight(element) => element.fmt(f),
                HotShotCalls::Commitments(element) => element.fmt(f),
                HotShotCalls::HashToCurve(element) => element.fmt(f),
                HotShotCalls::HashToField(element) => element.fmt(f),
                HotShotCalls::NewBlocks(element) => element.fmt(f),
                HotShotCalls::VerifyBlsSig(element) => element.fmt(f),
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
    impl ::std::convert::From<CommitmentsCall> for HotShotCalls {
        fn from(var: CommitmentsCall) -> Self {
            HotShotCalls::Commitments(var)
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
    impl ::std::convert::From<VerifyBlsSigCall> for HotShotCalls {
        fn from(var: VerifyBlsSigCall) -> Self {
            HotShotCalls::VerifyBlsSig(var)
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
