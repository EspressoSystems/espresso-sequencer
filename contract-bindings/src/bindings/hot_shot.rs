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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockNumber\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidQC\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TooManyBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numQCs\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongNumberOfQCs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_BLOCKS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"blockHeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"input\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bytes32ToUint8Array\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"output\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"expand\",\"outputs\":[{\"internalType\":\"uint8[]\",\"name\":\"\",\"type\":\"uint8[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"input\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hash_to_curve\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash_to_field\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"newCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"qcs\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"newBlocks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"message\",\"type\":\"uint8[]\",\"components\":[]},{\"internalType\":\"struct BN254.G1Point\",\"name\":\"sig\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct BN254.G2Point\",\"name\":\"pk\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"x1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y1\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verify_bls_sig\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b506117ee806100206000396000f3fe608060405234801561001057600080fd5b50600436106100935760003560e01c806349ce89971161006657806349ce89971461010f5780634fa796ed1461012f57806354cac3c5146101525780636769235014610165578063f44ff7121461017a57600080fd5b806326833dcc146100985780632b815b57146100b457806337e232b2146100d45780633b1f56c6146100fc575b600080fd5b6100a16103e881565b6040519081526020015b60405180910390f35b6100c76100c23660046112e9565b610183565b6040516100ab9190611326565b6100e76100e23660046112e9565b610614565b604080519283526020830191909152016100ab565b6100c761010a36600461136d565b6106e7565b6100a161011d36600461136d565b60006020819052908152604090205481565b61014261013d366004611386565b61077e565b60405190151581526020016100ab565b6100a16101603660046112e9565b610a21565b610178610173366004611486565b610c48565b005b6100a160015481565b60606030602060026000600185825b8660ff1681101561020c57806000036101d5576040516001600160f81b031960f886901b16602082015260210160405160208183030381529060405291506101fa565b81846040516020016101e8929190611522565b60405160208183030381529060405291505b8061020481611560565b915050610192565b5060005b8851811015610269578189828151811061022c5761022c611579565b6020026020010151604051602001610245929190611522565b6040516020818303038152906040529150808061026190611560565b915050610210565b50808360405160200161027d929190611522565b604051602081830303815290604052905080866040516020016102a1929190611522565b60408051601f1981840301815290829052915083906102c69083908390602001611522565b60408051808303601f1901815282820182526001808452602080850182905292519195506102f892869282910161158f565b60408051808303601f190181528282528051602091820120818401819052825180850390920182528383019092529450906103399085908790606001611522565b60408051808303601f19018152908290528351602080860151929750610362938893910161158f565b60408051601f1981840301815291905280516020820120909450600060ff8b1667ffffffffffffffff81111561039a5761039a6111c6565b6040519080825280602002602001820160405280156103c3578160200160208202803683370190505b50905060006103d1836106e7565b905060005b815181101561042f578181815181106103f1576103f1611579565b602002602001015183828151811061040b5761040b611579565b60ff909216602092830291909101909101528061042781611560565b9150506103d6565b50600061043b856106e7565b905060005b8c81101561052957806000036104be5782818151811061046257610462611579565b602002602001015182828151811061047c5761047c611579565b6020026020010151186040516020016104a8919060f89190911b6001600160f81b031916815260010190565b6040516020818303038152906040529850610517565b888382815181106104d1576104d1611579565b60200260200101518383815181106104eb576104eb611579565b602002602001015118604051602001610505929190611522565b60405160208183030381529060405298505b8061052181611560565b915050610440565b50878b60405160200161053d929190611522565b60408051808303601f190181529082905287516020808a0151929b50610566938c93910161158f565b60405160208183030381529060405297508780519060200120935061058a846106e7565b915060005b8c8e60ff1661059e91906115c3565b811015610600578281815181106105b7576105b7611579565b602002602001015184828f6105cc91906115dc565b815181106105dc576105dc611579565b60ff90921660209283029190910190910152806105f881611560565b91505061058f565b50919e9d5050505050505050505050505050565b600080600061062284610a21565b905060008051602061179983398151915260036000828485099050828061064b5761064b6115ef565b8482099050828061065e5761065e6115ef565b828208905060008061066f83610db5565b925090505b806106d8578480610687576106876115ef565b600187089550848061069b5761069b6115ef565b868709925084806106ae576106ae6115ef565b868409925084806106c1576106c16115ef565b84840892506106cf83610db5565b92509050610674565b50939793965092945050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b60208110156107785761072281601f6115c3565b61072d906008611605565b610738906002611700565b6107429084611713565b82828151811061075457610754611579565b60ff909216602092830291909101909101528061077081611560565b91505061070e565b50919050565b600080600061078c85610e97565b60408051610480810182526042808252604c602083018190526053938301849052605f606084018190526080840194909452604960a0840152604760c0840181905260e08401859052610100840192909252604e61012084018190526032610140850152603561016085015260346101808501526101a084019290925260316101c08401526101e083018490526058610200840152604d6102208401526044610240840152603a610260840152604b610280840181905260456102a085015260436102c085018190526102e0850181905260416103008601526103208501919091526103408401859052610360840183905261038084015260546103a084015260486103c08401526103e08301849052610400830191909152605561042083015261044082015261046081019190915286516000906108cd906024906115dc565b67ffffffffffffffff8111156108e5576108e56111c6565b60405190808252806020026020018201604052801561090e578160200160208202803683370190505b50905060005b885181101561096d5788818151811061092f5761092f611579565b602002602001015182828151811061094957610949611579565b60ff909216602092830291909101909101528061096581611560565b915050610914565b5060005b60248110156109d35782816024811061098c5761098c611579565b6020020151828a518361099f91906115dc565b815181106109af576109af611579565b60ff90921660209283029190910190910152806109cb81611560565b915050610971565b506109dd81610614565b604080518082019091528281526020810182905291955093506000610a138289610a068c610f26565b610a0e610fa1565b611072565b9a9950505050505050505050565b600080610a2d83610183565b805190915060308114610a4257610a42611727565b60008167ffffffffffffffff811115610a5d57610a5d6111c6565b604051908082528060200260200182016040528015610a86578160200160208202803683370190505b50905060005b82811015610af957836001610aa183866115c3565b610aab91906115c3565b81518110610abb57610abb611579565b6020026020010151828281518110610ad557610ad5611579565b60ff9092166020928302919091019091015280610af181611560565b915050610a8c565b5060408051601f8082526104008201909252600090819083602082016103e08036833701905050905060005b83811015610b91578481610b3986896115c3565b610b4391906115dc565b81518110610b5357610b53611579565b6020026020010151828281518110610b6d57610b6d611579565b60ff9092166020928302919091019091015280610b8981611560565b915050610b25565b50610b9b81611154565b91506101006000805160206117998339815191526000610bbb86896115c3565b905060005b81811015610c38576000886001610bd784866115c3565b610be191906115c3565b81518110610bf157610bf1611579565b602002602001015160ff1690508380610c0c57610c0c6115ef565b85880996508380610c1f57610c1f6115ef565b8188089650508080610c3090611560565b915050610bc0565b50939a9950505050505050505050565b828114610c775760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e8831115610c9d5760405163e082840b60e01b815260048101849052602401610c6e565b60015460005b84811015610d7457610cf8600154878784818110610cc357610cc3611579565b90506020020135868685818110610cdc57610cdc611579565b9050602002810190610cee919061173d565b6001949350505050565b610d1b57600154604051637818671960e01b8152600401610c6e91815260200190565b858582818110610d2d57610d2d611579565b905060200201356000806001548152602001908152602001600020819055506001806000828254610d5e91906115dc565b90915550610d6d905081611560565b9050610ca3565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b6000806000807f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5290506000600080516020611799833981519152905060405160208152602080820152602060408201528660608201528260808201528160a08201526020600060c08360055afa935050600051935082610e775760405162461bcd60e51b815260206004820152601b60248201527f706f7720707265636f6d70696c652063616c6c206661696c65642100000000006044820152606401610c6e565b8080610e8557610e856115ef565b84850991508582149450505050915091565b8051602082015160009160008051602061179983398151915291826003818085800985090883828309148115831517198483108585101616169350505081610f215760405162461bcd60e51b815260206004820152601760248201527f426e3235343a20696e76616c696420473120706f696e740000000000000000006044820152606401610c6e565b505050565b6040805180820190915260008082526020820152815160208301511590151615610f4e575090565b6040518060400160405280836000015181526020016000805160206117998339815191528460200151610f819190611784565b610f99906000805160206117998339815191526115c3565b905292915050565b610fcc6040518060800160405280600081526020016000815260200160008152602001600081525090565b60405180608001604052807f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281526020017f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed81526020017f090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b81526020017f12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa815250905090565b60008060006040518751815260208801516020820152865160408201526020870151606082015260408701516080820152606087015160a0820152855160c0820152602086015160e08201528451610100820152602085015161012082015260408501516101408201526060850151610160820152602060006101808360085afa9150506000519150806111485760405162461bcd60e51b815260206004820152601c60248201527f426e3235343a2050616972696e6720636865636b206661696c656421000000006044820152606401610c6e565b50151595945050505050565b600080805b83518110156111bf5783818151811061117457611174611579565b602002602001015160ff1681600861118c9190611605565b611197906002611700565b6111a19190611605565b6111ab90836115dc565b9150806111b781611560565b915050611159565b5092915050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff811182821017156111ff576111ff6111c6565b60405290565b6040516080810167ffffffffffffffff811182821017156111ff576111ff6111c6565b604051601f8201601f1916810167ffffffffffffffff81118282101715611251576112516111c6565b604052919050565b600082601f83011261126a57600080fd5b8135602067ffffffffffffffff821115611286576112866111c6565b8160051b611295828201611228565b92835284810182019282810190878511156112af57600080fd5b83870192505b848310156112de57823560ff811681146112cf5760008081fd5b825291830191908301906112b5565b979650505050505050565b6000602082840312156112fb57600080fd5b813567ffffffffffffffff81111561131257600080fd5b61131e84828501611259565b949350505050565b6020808252825182820181905260009190848201906040850190845b8181101561136157835160ff1683529284019291840191600101611342565b50909695505050505050565b60006020828403121561137f57600080fd5b5035919050565b600080600083850360e081121561139c57600080fd5b843567ffffffffffffffff8111156113b357600080fd5b6113bf87828801611259565b9450506040601f19820112156113d457600080fd5b6113dc6111dc565b602086810135825260408701359082015292506080605f198201121561140157600080fd5b5061140a611205565b606085013581526080850135602082015260a0850135604082015260c08501356060820152809150509250925092565b60008083601f84011261144c57600080fd5b50813567ffffffffffffffff81111561146457600080fd5b6020830191508360208260051b850101111561147f57600080fd5b9250929050565b6000806000806040858703121561149c57600080fd5b843567ffffffffffffffff808211156114b457600080fd5b6114c08883890161143a565b909650945060208701359150808211156114d957600080fd5b506114e68782880161143a565b95989497509550505050565b6000815160005b8181101561151357602081850181015186830152016114f9565b50600093019283525090919050565b600061152e82856114f2565b60f89390931b6001600160f81b03191683525050600101919050565b634e487b7160e01b600052601160045260246000fd5b6000600182016115725761157261154a565b5060010190565b634e487b7160e01b600052603260045260246000fd5b600061159b82866114f2565b6001600160f81b031960f895861b811682529390941b90921660018401525050600201919050565b818103818111156115d6576115d661154a565b92915050565b808201808211156115d6576115d661154a565b634e487b7160e01b600052601260045260246000fd5b80820281158282048414176115d6576115d661154a565b600181815b8085111561165757816000190482111561163d5761163d61154a565b8085161561164a57918102915b93841c9390800290611621565b509250929050565b60008261166e575060016115d6565b8161167b575060006115d6565b8160018114611691576002811461169b576116b7565b60019150506115d6565b60ff8411156116ac576116ac61154a565b50506001821b6115d6565b5060208310610133831016604e8410600b84101617156116da575081810a6115d6565b6116e4838361161c565b80600019048211156116f8576116f861154a565b029392505050565b600061170c838361165f565b9392505050565b600082611722576117226115ef565b500490565b634e487b7160e01b600052600160045260246000fd5b6000808335601e1984360301811261175457600080fd5b83018035915067ffffffffffffffff82111561176f57600080fd5b60200191503681900382131561147f57600080fd5b600082611793576117936115ef565b50069056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a264697066735822122021e1af724af19b732e10445a84586fbbf7f292e4033abc6a822e3353e84a9bc264736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `verify_bls_sig` (0x4fa796ed) function"]
        pub fn verify_bls_sig(
            &self,
            message: ::std::vec::Vec<u8>,
            sig: G1Point,
            pk: G2Point,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([79, 167, 150, 237], (message, sig, pk))
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
    #[doc = "Container type for all input parameters for the `verify_bls_sig` function with signature `verify_bls_sig(uint8[],(uint256,uint256),(uint256,uint256,uint256,uint256))` and selector `[79, 167, 150, 237]`"]
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
        abi = "verify_bls_sig(uint8[],(uint256,uint256),(uint256,uint256,uint256,uint256))"
    )]
    pub struct VerifyBlsSigCall {
        pub message: ::std::vec::Vec<u8>,
        pub sig: G1Point,
        pub pk: G2Point,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum HotShotCalls {
        MaxBlocks(MaxBlocksCall),
        BlockHeight(BlockHeightCall),
        Bytes32ToUint8Array(Bytes32ToUint8ArrayCall),
        Commitments(CommitmentsCall),
        Expand(ExpandCall),
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
                HotShotCalls::Bytes32ToUint8Array(element) => element.encode(),
                HotShotCalls::Commitments(element) => element.encode(),
                HotShotCalls::Expand(element) => element.encode(),
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
                HotShotCalls::Bytes32ToUint8Array(element) => element.fmt(f),
                HotShotCalls::Commitments(element) => element.fmt(f),
                HotShotCalls::Expand(element) => element.fmt(f),
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
    #[doc = "Container type for all return fields from the `verify_bls_sig` function with signature `verify_bls_sig(uint8[],(uint256,uint256),(uint256,uint256,uint256,uint256))` and selector `[79, 167, 150, 237]`"]
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
