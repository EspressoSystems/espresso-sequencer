pub use example_rollup_test::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod example_rollup_test {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ExampleRollupTest was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"blockHeight\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"StateUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeArtifacts\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"excludedArtifacts_\",\"type\":\"string[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeContracts\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"excludedContracts_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeSenders\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"excludedSenders_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hotshot\",\"outputs\":[{\"internalType\":\"contract HotShot\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rollup\",\"outputs\":[{\"internalType\":\"contract ExampleRollup\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetArtifactSelectors\",\"outputs\":[{\"internalType\":\"struct StdInvariant.FuzzSelector[]\",\"name\":\"targetedArtifactSelectors_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4[]\",\"name\":\"selectors\",\"type\":\"bytes4[]\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetArtifacts\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"targetedArtifacts_\",\"type\":\"string[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContracts\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targetedContracts_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetSelectors\",\"outputs\":[{\"internalType\":\"struct StdInvariant.FuzzSelector[]\",\"name\":\"targetedSelectors_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4[]\",\"name\":\"selectors\",\"type\":\"bytes4[]\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetSenders\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targetedSenders_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testStateUpdate\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static EXAMPLEROLLUPTEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static EXAMPLEROLLUPTEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405260008054600160ff19918216811790925560048054909116909117905534801561002d57600080fd5b506118f88061003d6000396000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c8063916a17c61161008c578063ba414fa611610066578063ba414fa614610194578063cb23bcb5146101ac578063e20c9f71146101bf578063fa7626d4146101c757600080fd5b8063916a17c61461017c578063a18dcc4014610184578063b5508aa91461018c57600080fd5b80633e5e3c23116100c85780633e5e3c23146101425780633f7286f41461014a57806366d9a9a01461015257806385226c811461016757600080fd5b80630a9254e4146100ef5780631ed7831c146100f95780632adc8b7614610117575b600080fd5b6100f76101d4565b005b610101610277565b60405161010e9190610eab565b60405180910390f35b601b5461012a906001600160a01b031681565b6040516001600160a01b03909116815260200161010e565b6101016102d9565b610101610339565b61015a610399565b60405161010e9190610ef8565b61016f610488565b60405161010e9190610ffb565b61015a610558565b6100f761063e565b61016f6109ff565b61019c610acf565b604051901515815260200161010e565b601c5461012a906001600160a01b031681565b610101610bfa565b60005461019c9060ff1681565b6040516101e090610e91565b604051809103906000f0801580156101fc573d6000803e3d6000fd5b50601b80546001600160a01b0319166001600160a01b0392909216918217905560405161022890610e9e565b6001600160a01b039091168152602001604051809103906000f080158015610254573d6000803e3d6000fd5b50601c80546001600160a01b0319166001600160a01b0392909216919091179055565b6060600d8054806020026020016040519081016040528092919081815260200182805480156102cf57602002820191906000526020600020905b81546001600160a01b031681526001909101906020018083116102b1575b5050505050905090565b6060600f8054806020026020016040519081016040528092919081815260200182805480156102cf576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116102b1575050505050905090565b6060600e8054806020026020016040519081016040528092919081815260200182805480156102cf576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116102b1575050505050905090565b60606012805480602002602001604051908101604052809291908181526020016000905b8282101561047f5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561046757602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116104295790505b505050505081525050815260200190600101906103bd565b50505050905090565b60606011805480602002602001604051908101604052809291908181526020016000905b8282101561047f5783829060005260206000200180546104cb9061105d565b80601f01602080910402602001604051908101604052809291908181526020018280546104f79061105d565b80156105445780601f1061051957610100808354040283529160200191610544565b820191906000526020600020905b81548152906001019060200180831161052757829003601f168201915b5050505050815260200190600101906104ac565b60606013805480602002602001604051908101604052809291908181526020016000905b8282101561047f5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561062657602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116105e85790505b5050505050815250508152602001906001019061057c565b6040805160028082526060820183526000926020830190803683375050604080516002808252606082019092529293506000929150602082015b6060815260200190600190039081610678579050509050648638237095826000815181106106a8576106a8611097565b6020026020010181815250506040518060400160405280600681526020016530783333333360d01b815250816000815181106106e6576106e6611097565b602002602001018190525064368bd531fe8260018151811061070a5761070a611097565b602002602001018181525050604051806040016040528060068152602001650c1e0d0d0d0d60d21b8152508160018151811061074857610748611097565b6020908102919091010152601b54604051630676923560e41b81526001600160a01b039091169063676923509061078590859085906004016110ad565b600060405180830381600087803b15801561079f57600080fd5b505af11580156107b3573d6000803e3d6000fd5b5050604080518082018252600681526503078303030360d41b6020820152601c5491516381bad6f360e01b8152600060048201819052602482018190526044820152600160648201526001600160a01b03909216608483015292506207fb739150737109709ecfa91a80626ff3989d68f67f5b1dd12d906381bad6f39060a401600060405180830381600087803b15801561084d57600080fd5b505af1158015610861573d6000803e3d6000fd5b505050507f960805e7dfc5cc387e0db0b8f6b4a6a3fafbe87a9e0669d505558889762b00b3600160405161089791815260200190565b60405180910390a1601c54604051633c0bc64560e11b81526001600160a01b03909116906378178c8a906108d19084908690600401611145565b600060405180830381600087803b1580156108eb57600080fd5b505af11580156108ff573d6000803e3d6000fd5b50505050610983601c60009054906101000a90046001600160a01b03166001600160a01b031663d800741e6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610959573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061097d9190611166565b82610c5a565b601c546040805163424c033f60e11b815290516109f9926001600160a01b031691638498067e9160048083019260209291908290030181865afa1580156109ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109f29190611166565b6001610c5a565b50505050565b60606010805480602002602001604051908101604052809291908181526020016000905b8282101561047f578382906000526020600020018054610a429061105d565b80601f0160208091040260200160405190810160405280929190818152602001828054610a6e9061105d565b8015610abb5780601f10610a9057610100808354040283529160200191610abb565b820191906000526020600020905b815481529060010190602001808311610a9e57829003601f168201915b505050505081526020019060010190610a23565b60008054610100900460ff1615610aef5750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610bf55760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091610b7d917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc49160800161117f565b60408051601f1981840301815290829052610b97916111b0565b6000604051808303816000865af19150503d8060008114610bd4576040519150601f19603f3d011682016040523d82523d6000602084013e610bd9565b606091505b5091505080806020019051810190610bf191906111cc565b9150505b919050565b6060600c8054806020026020016040519081016040528092919081815260200182805480156102cf576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116102b1575050505050905090565b808214610d81577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50604051610ccb9060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808115e1c1958dd195960b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a8183015269080808081058dd1d585b60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a1610d81610d85565b5050565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610e805760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f1981840301815290829052610e1f929160200161117f565b60408051601f1981840301815290829052610e39916111b0565b6000604051808303816000865af19150503d8060008114610e76576040519150601f19603f3d011682016040523d82523d6000602084013e610e7b565b606091505b505050505b6000805461ff001916610100179055565b6103e3806111f683390190565b6102ea806115d983390190565b6020808252825182820181905260009190848201906040850190845b81811015610eec5783516001600160a01b031683529284019291840191600101610ec7565b50909695505050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015610f9c57898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015610f875783516001600160e01b0319168252928b019260019290920191908b0190610f5d565b50978a01979550505091870191600101610f20565b50919998505050505050505050565b60005b83811015610fc6578181015183820152602001610fae565b50506000910152565b60008151808452610fe7816020860160208601610fab565b601f01601f19169290920160200192915050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b8281101561105057603f1988860301845261103e858351610fcf565b94509285019290850190600101611022565b5092979650505050505050565b600181811c9082168061107157607f821691505b60208210810361109157634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052603260045260246000fd5b604080825283519082018190526000906020906060840190828701845b828110156110e6578151845292840192908401906001016110ca565b50505083810382850152845180825282820190600581901b8301840187850160005b8381101561113657601f19868403018552611124838351610fcf565b94870194925090860190600101611108565b50909998505050505050505050565b82815260406020820152600061115e6040830184610fcf565b949350505050565b60006020828403121561117857600080fd5b5051919050565b6001600160e01b03198316815281516000906111a2816004850160208701610fab565b919091016004019392505050565b600082516111c2818460208701610fab565b9190910192915050565b6000602082840312156111de57600080fd5b815180151581146111ee57600080fd5b939250505056fe608060405234801561001057600080fd5b506103c3806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c806326833dcc1461005157806349ce89971461006c578063676923501461008c578063f44ff712146100a1575b600080fd5b61005a6103e881565b60405190815260200160405180910390f35b61005a61007a366004610217565b60006020819052908152604090205481565b61009f61009a36600461027c565b6100aa565b005b61005a60015481565b8281146100d95760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e88311156100ff5760405163e082840b60e01b8152600481018490526024016100d0565b60015460005b848110156101d65761015a600154878784818110610125576101256102e8565b9050602002013586868581811061013e5761013e6102e8565b905060200281019061015091906102fe565b6001949350505050565b61017d57600154604051637818671960e01b81526004016100d091815260200190565b85858281811061018f5761018f6102e8565b9050602002013560008060015481526020019081526020016000208190555060018060008282546101c0919061035b565b909155506101cf905081610374565b9050610105565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b60006020828403121561022957600080fd5b5035919050565b60008083601f84011261024257600080fd5b50813567ffffffffffffffff81111561025a57600080fd5b6020830191508360208260051b850101111561027557600080fd5b9250929050565b6000806000806040858703121561029257600080fd5b843567ffffffffffffffff808211156102aa57600080fd5b6102b688838901610230565b909650945060208701359150808211156102cf57600080fd5b506102dc87828801610230565b95989497509550505050565b634e487b7160e01b600052603260045260246000fd5b6000808335601e1984360301811261031557600080fd5b83018035915067ffffffffffffffff82111561033057600080fd5b60200191503681900382131561027557600080fd5b634e487b7160e01b600052601160045260246000fd5b8082018082111561036e5761036e610345565b92915050565b60006001820161038657610386610345565b506001019056fea264697066735822122081e5f1e552003eb066e46e84b64b2a26e729c86cc1671246084378212c6ce0ef64736f6c63430008120033608060405234801561001057600080fd5b506040516102ea3803806102ea83398101604081905261002f9161005c565b600080546001600160a01b0319166001600160a01b0392909216919091178155600181905560025561008c565b60006020828403121561006e57600080fd5b81516001600160a01b038116811461008557600080fd5b9392505050565b61024f8061009b6000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c806378178c8a146100465780638498067e1461005b578063d800741e14610076575b600080fd5b61005961005436600461015d565b61007f565b005b61006460025481565b60405190815260200160405180910390f35b61006460015481565b600080546002546001600160a01b03909116906349ce8997906100a39060016101d9565b6040518263ffffffff1660e01b81526004016100c191815260200190565b602060405180830381865afa1580156100de573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101029190610200565b905060016002600082825461011791906101d9565b909155505060018490556002546040519081527f960805e7dfc5cc387e0db0b8f6b4a6a3fafbe87a9e0669d505558889762b00b39060200160405180910390a150505050565b60008060006040848603121561017257600080fd5b83359250602084013567ffffffffffffffff8082111561019157600080fd5b818601915086601f8301126101a557600080fd5b8135818111156101b457600080fd5b8760208285010111156101c657600080fd5b6020830194508093505050509250925092565b808201808211156101fa57634e487b7160e01b600052601160045260246000fd5b92915050565b60006020828403121561021257600080fd5b505191905056fea2646970667358221220550100959eefb6c420c6b98cca495006fd6b0a1ead01b54d40d0d637fe06c6e564736f6c63430008120033a2646970667358221220d5f6885a9ee3af3fe0a0e6d71612769f29a39dbd51799f5c7047836b9a961d6164736f6c63430008120033" . parse () . expect ("invalid bytecode")
        });
    pub struct ExampleRollupTest<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ExampleRollupTest<M> {
        fn clone(&self) -> Self {
            ExampleRollupTest(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ExampleRollupTest<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ExampleRollupTest<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ExampleRollupTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ExampleRollupTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), EXAMPLEROLLUPTEST_ABI.clone(), client)
                .into()
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
                EXAMPLEROLLUPTEST_ABI.clone(),
                EXAMPLEROLLUPTEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excludeArtifacts` (0xb5508aa9) function"]
        pub fn exclude_artifacts(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<String>> {
            self.0
                .method_hash([181, 80, 138, 169], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excludeContracts` (0xe20c9f71) function"]
        pub fn exclude_contracts(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([226, 12, 159, 113], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `excludeSenders` (0x1ed7831c) function"]
        pub fn exclude_senders(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([30, 215, 131, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hotshot` (0x2adc8b76) function"]
        pub fn hotshot(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([42, 220, 139, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rollup` (0xcb23bcb5) function"]
        pub fn rollup(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([203, 35, 188, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function"]
        pub fn target_artifact_selectors(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzSelector>> {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetArtifacts` (0x85226c81) function"]
        pub fn target_artifacts(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<String>> {
            self.0
                .method_hash([133, 34, 108, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetContracts` (0x3f7286f4) function"]
        pub fn target_contracts(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 114, 134, 244], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetSelectors` (0x916a17c6) function"]
        pub fn target_selectors(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzSelector>> {
            self.0
                .method_hash([145, 106, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `targetSenders` (0x3e5e3c23) function"]
        pub fn target_senders(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([62, 94, 60, 35], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testStateUpdate` (0xa18dcc40) function"]
        pub fn test_state_update(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 141, 204, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `StateUpdate` event"]
        pub fn state_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StateUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log` event"]
        pub fn log_filter(&self) -> ethers::contract::builders::Event<M, LogFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_address` event"]
        pub fn log_address_filter(&self) -> ethers::contract::builders::Event<M, LogAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_1_filter(&self) -> ethers::contract::builders::Event<M, LogArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_2_filter(&self) -> ethers::contract::builders::Event<M, LogArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_3_filter(&self) -> ethers::contract::builders::Event<M, LogArray3Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes` event"]
        pub fn log_bytes_filter(&self) -> ethers::contract::builders::Event<M, LogBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes32` event"]
        pub fn log_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_int` event"]
        pub fn log_int_filter(&self) -> ethers::contract::builders::Event<M, LogIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_address` event"]
        pub fn log_named_address_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_1_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_2_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_3_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray3Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes` event"]
        pub fn log_named_bytes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes32` event"]
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_int` event"]
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_uint` event"]
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_int` event"]
        pub fn log_named_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_string` event"]
        pub fn log_named_string_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_uint` event"]
        pub fn log_named_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_string` event"]
        pub fn log_string_filter(&self) -> ethers::contract::builders::Event<M, LogStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_uint` event"]
        pub fn log_uint_filter(&self) -> ethers::contract::builders::Event<M, LogUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `logs` event"]
        pub fn logs_filter(&self) -> ethers::contract::builders::Event<M, LogsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ExampleRollupTestEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ExampleRollupTest<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[ethevent(name = "StateUpdate", abi = "StateUpdate(uint256)")]
    pub struct StateUpdateFilter {
        pub block_height: ethers::core::types::U256,
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ethers::core::types::Address);
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: Vec<ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: Vec<I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ethers::core::types::Bytes);
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub I256);
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: String,
        pub val: ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: String,
        pub val: Vec<I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: String,
        pub val: ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: String,
        pub val: [u8; 32],
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
    #[ethevent(
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: String,
        pub val: I256,
        pub decimals: ethers::core::types::U256,
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
    #[ethevent(
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
        pub decimals: ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: String,
        pub val: I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: String,
        pub val: String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ethers::core::types::U256);
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ethers::core::types::Bytes);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ExampleRollupTestEvents {
        StateUpdateFilter(StateUpdateFilter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ethers::contract::EthLogDecode for ExampleRollupTestEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = StateUpdateFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::StateUpdateFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(ExampleRollupTestEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ExampleRollupTestEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ExampleRollupTestEvents::StateUpdateFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogAddressFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogArray1Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogArray2Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogArray3Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogBytesFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogBytes32Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogIntFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedAddressFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedArray1Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedArray2Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedArray3Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedBytesFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedIntFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedStringFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogNamedUintFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogStringFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogUintFilter(element) => element.fmt(f),
                ExampleRollupTestEvents::LogsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    #[doc = "Container type for all input parameters for the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `[181, 80, 138, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "excludeArtifacts", abi = "excludeArtifacts()")]
    pub struct ExcludeArtifactsCall;
    #[doc = "Container type for all input parameters for the `excludeContracts` function with signature `excludeContracts()` and selector `[226, 12, 159, 113]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
    #[doc = "Container type for all input parameters for the `excludeSenders` function with signature `excludeSenders()` and selector `[30, 215, 131, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "excludeSenders", abi = "excludeSenders()")]
    pub struct ExcludeSendersCall;
    #[doc = "Container type for all input parameters for the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    #[doc = "Container type for all input parameters for the `hotshot` function with signature `hotshot()` and selector `[42, 220, 139, 118]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hotshot", abi = "hotshot()")]
    pub struct HotshotCall;
    #[doc = "Container type for all input parameters for the `rollup` function with signature `rollup()` and selector `[203, 35, 188, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rollup", abi = "rollup()")]
    pub struct RollupCall;
    #[doc = "Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `[10, 146, 84, 228]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[doc = "Container type for all input parameters for the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `[102, 217, 169, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "targetArtifactSelectors", abi = "targetArtifactSelectors()")]
    pub struct TargetArtifactSelectorsCall;
    #[doc = "Container type for all input parameters for the `targetArtifacts` function with signature `targetArtifacts()` and selector `[133, 34, 108, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "targetArtifacts", abi = "targetArtifacts()")]
    pub struct TargetArtifactsCall;
    #[doc = "Container type for all input parameters for the `targetContracts` function with signature `targetContracts()` and selector `[63, 114, 134, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "targetContracts", abi = "targetContracts()")]
    pub struct TargetContractsCall;
    #[doc = "Container type for all input parameters for the `targetSelectors` function with signature `targetSelectors()` and selector `[145, 106, 23, 198]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "targetSelectors", abi = "targetSelectors()")]
    pub struct TargetSelectorsCall;
    #[doc = "Container type for all input parameters for the `targetSenders` function with signature `targetSenders()` and selector `[62, 94, 60, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
    #[doc = "Container type for all input parameters for the `testStateUpdate` function with signature `testStateUpdate()` and selector `[161, 141, 204, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "testStateUpdate", abi = "testStateUpdate()")]
    pub struct TestStateUpdateCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ExampleRollupTestCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        Hotshot(HotshotCall),
        Rollup(RollupCall),
        SetUp(SetUpCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TestStateUpdate(TestStateUpdateCall),
    }
    impl ethers::core::abi::AbiDecode for ExampleRollupTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::IsTest(decoded));
            }
            if let Ok(decoded) =
                <ExcludeArtifactsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeContractsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::ExcludeContracts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeSendersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::Failed(decoded));
            }
            if let Ok(decoded) =
                <HotshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::Hotshot(decoded));
            }
            if let Ok(decoded) = <RollupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::Rollup(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::SetUp(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactSelectorsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::TargetArtifacts(decoded));
            }
            if let Ok(decoded) =
                <TargetContractsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::TargetContracts(decoded));
            }
            if let Ok(decoded) =
                <TargetSelectorsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::TargetSelectors(decoded));
            }
            if let Ok(decoded) =
                <TargetSendersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::TargetSenders(decoded));
            }
            if let Ok(decoded) =
                <TestStateUpdateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ExampleRollupTestCalls::TestStateUpdate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ExampleRollupTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ExampleRollupTestCalls::IsTest(element) => element.encode(),
                ExampleRollupTestCalls::ExcludeArtifacts(element) => element.encode(),
                ExampleRollupTestCalls::ExcludeContracts(element) => element.encode(),
                ExampleRollupTestCalls::ExcludeSenders(element) => element.encode(),
                ExampleRollupTestCalls::Failed(element) => element.encode(),
                ExampleRollupTestCalls::Hotshot(element) => element.encode(),
                ExampleRollupTestCalls::Rollup(element) => element.encode(),
                ExampleRollupTestCalls::SetUp(element) => element.encode(),
                ExampleRollupTestCalls::TargetArtifactSelectors(element) => element.encode(),
                ExampleRollupTestCalls::TargetArtifacts(element) => element.encode(),
                ExampleRollupTestCalls::TargetContracts(element) => element.encode(),
                ExampleRollupTestCalls::TargetSelectors(element) => element.encode(),
                ExampleRollupTestCalls::TargetSenders(element) => element.encode(),
                ExampleRollupTestCalls::TestStateUpdate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ExampleRollupTestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ExampleRollupTestCalls::IsTest(element) => element.fmt(f),
                ExampleRollupTestCalls::ExcludeArtifacts(element) => element.fmt(f),
                ExampleRollupTestCalls::ExcludeContracts(element) => element.fmt(f),
                ExampleRollupTestCalls::ExcludeSenders(element) => element.fmt(f),
                ExampleRollupTestCalls::Failed(element) => element.fmt(f),
                ExampleRollupTestCalls::Hotshot(element) => element.fmt(f),
                ExampleRollupTestCalls::Rollup(element) => element.fmt(f),
                ExampleRollupTestCalls::SetUp(element) => element.fmt(f),
                ExampleRollupTestCalls::TargetArtifactSelectors(element) => element.fmt(f),
                ExampleRollupTestCalls::TargetArtifacts(element) => element.fmt(f),
                ExampleRollupTestCalls::TargetContracts(element) => element.fmt(f),
                ExampleRollupTestCalls::TargetSelectors(element) => element.fmt(f),
                ExampleRollupTestCalls::TargetSenders(element) => element.fmt(f),
                ExampleRollupTestCalls::TestStateUpdate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for ExampleRollupTestCalls {
        fn from(var: IsTestCall) -> Self {
            ExampleRollupTestCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<ExcludeArtifactsCall> for ExampleRollupTestCalls {
        fn from(var: ExcludeArtifactsCall) -> Self {
            ExampleRollupTestCalls::ExcludeArtifacts(var)
        }
    }
    impl ::std::convert::From<ExcludeContractsCall> for ExampleRollupTestCalls {
        fn from(var: ExcludeContractsCall) -> Self {
            ExampleRollupTestCalls::ExcludeContracts(var)
        }
    }
    impl ::std::convert::From<ExcludeSendersCall> for ExampleRollupTestCalls {
        fn from(var: ExcludeSendersCall) -> Self {
            ExampleRollupTestCalls::ExcludeSenders(var)
        }
    }
    impl ::std::convert::From<FailedCall> for ExampleRollupTestCalls {
        fn from(var: FailedCall) -> Self {
            ExampleRollupTestCalls::Failed(var)
        }
    }
    impl ::std::convert::From<HotshotCall> for ExampleRollupTestCalls {
        fn from(var: HotshotCall) -> Self {
            ExampleRollupTestCalls::Hotshot(var)
        }
    }
    impl ::std::convert::From<RollupCall> for ExampleRollupTestCalls {
        fn from(var: RollupCall) -> Self {
            ExampleRollupTestCalls::Rollup(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for ExampleRollupTestCalls {
        fn from(var: SetUpCall) -> Self {
            ExampleRollupTestCalls::SetUp(var)
        }
    }
    impl ::std::convert::From<TargetArtifactSelectorsCall> for ExampleRollupTestCalls {
        fn from(var: TargetArtifactSelectorsCall) -> Self {
            ExampleRollupTestCalls::TargetArtifactSelectors(var)
        }
    }
    impl ::std::convert::From<TargetArtifactsCall> for ExampleRollupTestCalls {
        fn from(var: TargetArtifactsCall) -> Self {
            ExampleRollupTestCalls::TargetArtifacts(var)
        }
    }
    impl ::std::convert::From<TargetContractsCall> for ExampleRollupTestCalls {
        fn from(var: TargetContractsCall) -> Self {
            ExampleRollupTestCalls::TargetContracts(var)
        }
    }
    impl ::std::convert::From<TargetSelectorsCall> for ExampleRollupTestCalls {
        fn from(var: TargetSelectorsCall) -> Self {
            ExampleRollupTestCalls::TargetSelectors(var)
        }
    }
    impl ::std::convert::From<TargetSendersCall> for ExampleRollupTestCalls {
        fn from(var: TargetSendersCall) -> Self {
            ExampleRollupTestCalls::TargetSenders(var)
        }
    }
    impl ::std::convert::From<TestStateUpdateCall> for ExampleRollupTestCalls {
        fn from(var: TestStateUpdateCall) -> Self {
            ExampleRollupTestCalls::TestStateUpdate(var)
        }
    }
    #[doc = "Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `[181, 80, 138, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExcludeArtifactsReturn {
        pub excluded_artifacts: ::std::vec::Vec<String>,
    }
    #[doc = "Container type for all return fields from the `excludeContracts` function with signature `excludeContracts()` and selector `[226, 12, 159, 113]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all return fields from the `excludeSenders` function with signature `excludeSenders()` and selector `[30, 215, 131, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExcludeSendersReturn {
        pub excluded_senders: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all return fields from the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FailedReturn(pub bool);
    #[doc = "Container type for all return fields from the `hotshot` function with signature `hotshot()` and selector `[42, 220, 139, 118]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HotshotReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `rollup` function with signature `rollup()` and selector `[203, 35, 188, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RollupReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `[102, 217, 169, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    #[doc = "Container type for all return fields from the `targetArtifacts` function with signature `targetArtifacts()` and selector `[133, 34, 108, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TargetArtifactsReturn {
        pub targeted_artifacts: ::std::vec::Vec<String>,
    }
    #[doc = "Container type for all return fields from the `targetContracts` function with signature `targetContracts()` and selector `[63, 114, 134, 244]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TargetContractsReturn {
        pub targeted_contracts: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all return fields from the `targetSelectors` function with signature `targetSelectors()` and selector `[145, 106, 23, 198]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TargetSelectorsReturn {
        pub targeted_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    #[doc = "Container type for all return fields from the `targetSenders` function with signature `targetSenders()` and selector `[62, 94, 60, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<ethers::core::types::Address>,
    }
}
