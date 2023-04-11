pub use hot_shot_test::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod hot_shot_test {
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
    #[doc = "HotShotTest was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeArtifacts\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"excludedArtifacts_\",\"type\":\"string[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeContracts\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"excludedContracts_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeSenders\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"excludedSenders_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hotshot\",\"outputs\":[{\"internalType\":\"contract HotShot\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetArtifactSelectors\",\"outputs\":[{\"internalType\":\"struct StdInvariant.FuzzSelector[]\",\"name\":\"targetedArtifactSelectors_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4[]\",\"name\":\"selectors\",\"type\":\"bytes4[]\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetArtifacts\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"targetedArtifacts_\",\"type\":\"string[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContracts\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targetedContracts_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetSelectors\",\"outputs\":[{\"internalType\":\"struct StdInvariant.FuzzSelector[]\",\"name\":\"targetedSelectors_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4[]\",\"name\":\"selectors\",\"type\":\"bytes4[]\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetSenders\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targetedSenders_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testExpander\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testPublishCommitments\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOTTEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOTTEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405260008054600160ff19918216811790925560048054909116909117905534801561002d57600080fd5b506119a98061003d6000396000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c806385226c811161008c578063b5508aa911610066578063b5508aa914610194578063ba414fa61461019c578063e20c9f71146101b4578063fa7626d4146101bc57600080fd5b806385226c811461016f578063916a17c6146101845780639997afdb1461018c57600080fd5b80633e5e3c23116100c85780633e5e3c23146101425780633f7286f41461014a57806366d9a9a014610152578063802e9f981461016757600080fd5b80630a9254e4146100ef5780631ed7831c146100f95780632adc8b7614610117575b600080fd5b6100f76101c9565b005b610101610214565b60405161010e9190610f21565b60405180910390f35b601b5461012a906001600160a01b031681565b6040516001600160a01b03909116815260200161010e565b610101610276565b6101016102d6565b61015a610336565b60405161010e9190610f6e565b6100f7610425565b61017761050b565b60405161010e9190611071565b61015a6105db565b6100f76106c1565b610177610a8b565b6101a4610b5b565b604051901515815260200161010e565b610101610c86565b6000546101a49060ff1681565b6040516101d590610f14565b604051809103906000f0801580156101f1573d6000803e3d6000fd5b50601b80546001600160a01b0319166001600160a01b0392909216919091179055565b6060600d80548060200260200160405190810160405280929190818152602001828054801561026c57602002820191906000526020600020905b81546001600160a01b0316815260019091019060200180831161024e575b5050505050905090565b6060600f80548060200260200160405190810160405280929190818152602001828054801561026c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161024e575050505050905090565b6060600e80548060200260200160405190810160405280929190818152602001828054801561026c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161024e575050505050905090565b60606012805480602002602001604051908101604052809291908181526020016000905b8282101561041c5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561040457602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116103c65790505b5050505050815250508152602001906001019061035a565b50505050905090565b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061045b5761045b6110d3565b602002602001019060ff16908160ff1681525050600281600181518110610484576104846110d3565b60ff90921660209283029190910190910152601b54604051632b815b5760e01b81526001600160a01b0390911690632b815b57906104c69084906004016110e9565b602060405180830381865afa1580156104e3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105079190611124565b5050565b60606011805480602002602001604051908101604052809291908181526020016000905b8282101561041c57838290600052602060002001805461054e9061113d565b80601f016020809104026020016040519081016040528092919081815260200182805461057a9061113d565b80156105c75780601f1061059c576101008083540402835291602001916105c7565b820191906000526020600020905b8154815290600101906020018083116105aa57829003601f168201915b50505050508152602001906001019061052f565b60606013805480602002602001604051908101604052809291908181526020016000905b8282101561041c5760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156106a957602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161066b5790505b505050505081525050815260200190600101906105ff565b6040805160028082526060820183526000926020830190803683375050604080516002808252606082019092529293506000929150602082015b60608152602001906001900390816106fb5790505090506486382370958260008151811061072b5761072b6110d3565b6020026020010181815250506040518060400160405280600681526020016530783333333360d01b81525081600081518110610769576107696110d3565b602002602001018190525064368bd531fe8260018151811061078d5761078d6110d3565b602002602001018181525050604051806040016040528060068152602001650c1e0d0d0d0d60d21b815250816001815181106107cb576107cb6110d3565b6020908102919091010152601b546040516381bad6f360e01b8152600060048201819052602482018190526044820152600160648201526001600160a01b039091166084820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906381bad6f39060a401600060405180830381600087803b15801561084b57600080fd5b505af115801561085f573d6000803e3d6000fd5b50506040805160008152600260208201527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1935001905060405180910390a1601b54604051630676923560e41b81526001600160a01b03909116906367692350906108d09085908590600401611177565b600060405180830381600087803b1580156108ea57600080fd5b505af11580156108fe573d6000803e3d6000fd5b5050601b546040516349ce899760e01b81526000600482015261099393506001600160a01b0390911691506349ce899790602401602060405180830381865afa15801561094f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109739190611124565b83600081518110610986576109866110d3565b6020026020010151610ce6565b601b546040516349ce899760e01b815260016004820152610a15916001600160a01b0316906349ce899790602401602060405180830381865afa1580156109de573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a029190611124565b83600181518110610986576109866110d3565b601b546040516349ce899760e01b815260026004820152610507916001600160a01b0316906349ce899790602401602060405180830381865afa158015610a60573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a849190611124565b6000610ce6565b60606010805480602002602001604051908101604052809291908181526020016000905b8282101561041c578382906000526020600020018054610ace9061113d565b80601f0160208091040260200160405190810160405280929190818152602001828054610afa9061113d565b8015610b475780601f10610b1c57610100808354040283529160200191610b47565b820191906000526020600020905b815481529060010190602001808311610b2a57829003601f168201915b505050505081526020019060010190610aaf565b60008054610100900460ff1615610b7b5750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c815760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091610c09917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc49160800161120f565b60408051601f1981840301815290829052610c2391611240565b6000604051808303816000865af19150503d8060008114610c60576040519150601f19603f3d011682016040523d82523d6000602084013e610c65565b606091505b5091505080806020019051810190610c7d919061125c565b9150505b919050565b6060600c80548060200260200160405190810160405280929190818152602001828054801561026c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161024e575050505050905090565b808214610507577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50604051610d579060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808115e1c1958dd195960b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a8183015269080808081058dd1d585b60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a1610507737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610f035760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f1981840301815290829052610ea2929160200161120f565b60408051601f1981840301815290829052610ebc91611240565b6000604051808303816000865af19150503d8060008114610ef9576040519150601f19603f3d011682016040523d82523d6000602084013e610efe565b606091505b505050505b6000805461ff001916610100179055565b6106ee8061128683390190565b6020808252825182820181905260009190848201906040850190845b81811015610f625783516001600160a01b031683529284019291840191600101610f3d565b50909695505050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b8481101561101257898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015610ffd5783516001600160e01b0319168252928b019260019290920191908b0190610fd3565b50978a01979550505091870191600101610f96565b50919998505050505050505050565b60005b8381101561103c578181015183820152602001611024565b50506000910152565b6000815180845261105d816020860160208601611021565b601f01601f19169290920160200192915050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b828110156110c657603f198886030184526110b4858351611045565b94509285019290850190600101611098565b5092979650505050505050565b634e487b7160e01b600052603260045260246000fd5b6020808252825182820181905260009190848201906040850190845b81811015610f6257835160ff1683529284019291840191600101611105565b60006020828403121561113657600080fd5b5051919050565b600181811c9082168061115157607f821691505b60208210810361117157634e487b7160e01b600052602260045260246000fd5b50919050565b604080825283519082018190526000906020906060840190828701845b828110156111b057815184529284019290840190600101611194565b50505083810382850152845180825282820190600581901b8301840187850160005b8381101561120057601f198684030185526111ee838351611045565b948701949250908601906001016111d2565b50909998505050505050505050565b6001600160e01b0319831681528151600090611232816004850160208701611021565b919091016004019392505050565b60008251611252818460208701611021565b9190910192915050565b60006020828403121561126e57600080fd5b8151801515811461127e57600080fd5b939250505056fe608060405234801561001057600080fd5b506106ce806100206000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c806326833dcc1461005c5780632b815b571461007757806349ce89971461008a57806367692350146100aa578063f44ff712146100bf575b600080fd5b6100656103e881565b60405190815260200160405180910390f35b610065610085366004610402565b6100c8565b6100656100983660046104c7565b60006020819052908152604090205481565b6100bd6100b836600461052c565b610269565b005b61006560015481565b6040805160006020808301829052835160018185038101825260219094019094529092603092600291859190825b6101016001886105ae565b60ff1681101561014257818460405160200161011e9291906105cd565b6040516020818303038152906040529150808061013a9061060f565b9150506100f6565b5060005b885181101561019f578189828151811061016257610162610628565b602002602001015160405160200161017b9291906105cd565b604051602081830303815290604052915080806101979061060f565b915050610146565b5080836040516020016101b39291906105cd565b604051602081830303815290604052905080866040516020016101d79291906105cd565b60408051601f1981840301815290829052915083906101fc90839083906020016105cd565b604051602081830303815290604052915081836040516020016102209291906105cd565b604051602081830303815290604052915081836040516020016102449291906105cd565b60408051601f1981840301815291905280516020909101209998505050505050505050565b8281146102985760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e88311156102be5760405163e082840b60e01b81526004810184905260240161028f565b60015460005b84811015610395576103196001548787848181106102e4576102e4610628565b905060200201358686858181106102fd576102fd610628565b905060200281019061030f919061063e565b6001949350505050565b61033c57600154604051637818671960e01b815260040161028f91815260200190565b85858281811061034e5761034e610628565b90506020020135600080600154815260200190815260200160002081905550600180600082825461037f9190610685565b9091555061038e90508161060f565b90506102c4565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b634e487b7160e01b600052604160045260246000fd5b803560ff811681146103fd57600080fd5b919050565b6000602080838503121561041557600080fd5b823567ffffffffffffffff8082111561042d57600080fd5b818501915085601f83011261044157600080fd5b813581811115610453576104536103d6565b8060051b604051601f19603f83011681018181108582111715610478576104786103d6565b60405291825284820192508381018501918883111561049657600080fd5b938501935b828510156104bb576104ac856103ec565b8452938501939285019261049b565b98975050505050505050565b6000602082840312156104d957600080fd5b5035919050565b60008083601f8401126104f257600080fd5b50813567ffffffffffffffff81111561050a57600080fd5b6020830191508360208260051b850101111561052557600080fd5b9250929050565b6000806000806040858703121561054257600080fd5b843567ffffffffffffffff8082111561055a57600080fd5b610566888389016104e0565b9096509450602087013591508082111561057f57600080fd5b5061058c878288016104e0565b95989497509550505050565b634e487b7160e01b600052601160045260246000fd5b60ff82811682821603908111156105c7576105c7610598565b92915050565b6000835160005b818110156105ee57602081870181015185830152016105d4565b5060f89390931b6001600160f81b0319169190920190815260010192915050565b60006001820161062157610621610598565b5060010190565b634e487b7160e01b600052603260045260246000fd5b6000808335601e1984360301811261065557600080fd5b83018035915067ffffffffffffffff82111561067057600080fd5b60200191503681900382131561052557600080fd5b808201808211156105c7576105c761059856fea26469706673582212209f6682e1c54db9f9868f5b2a1e3d17741725f658c3313bae6458e8dcec9ed6b964736f6c63430008120033a2646970667358221220e118686227952c67de2c9e1181177af16c6c2088bf804e7ea1be0aee462d733664736f6c63430008120033" . parse () . expect ("invalid bytecode")
        });
    pub struct HotShotTest<M>(ethers::contract::Contract<M>);
    impl<M> Clone for HotShotTest<M> {
        fn clone(&self) -> Self {
            HotShotTest(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for HotShotTest<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for HotShotTest<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(HotShotTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> HotShotTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), HOTSHOTTEST_ABI.clone(), client).into()
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
                HOTSHOTTEST_ABI.clone(),
                HOTSHOTTEST_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `testExpander` (0x802e9f98) function"]
        pub fn test_expander(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 46, 159, 152], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testPublishCommitments` (0x9997afdb) function"]
        pub fn test_publish_commitments(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 151, 175, 219], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewBlocks` event"]
        pub fn new_blocks_filter(&self) -> ethers::contract::builders::Event<M, NewBlocksFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, HotShotTestEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for HotShotTest<M> {
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
    #[ethevent(name = "NewBlocks", abi = "NewBlocks(uint256,uint256)")]
    pub struct NewBlocksFilter {
        pub first_block_number: ethers::core::types::U256,
        pub num_blocks: ethers::core::types::U256,
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
    pub enum HotShotTestEvents {
        NewBlocksFilter(NewBlocksFilter),
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
    impl ethers::contract::EthLogDecode for HotShotTestEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewBlocksFilter::decode_log(log) {
                return Ok(HotShotTestEvents::NewBlocksFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(HotShotTestEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for HotShotTestEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                HotShotTestEvents::NewBlocksFilter(element) => element.fmt(f),
                HotShotTestEvents::LogFilter(element) => element.fmt(f),
                HotShotTestEvents::LogAddressFilter(element) => element.fmt(f),
                HotShotTestEvents::LogArray1Filter(element) => element.fmt(f),
                HotShotTestEvents::LogArray2Filter(element) => element.fmt(f),
                HotShotTestEvents::LogArray3Filter(element) => element.fmt(f),
                HotShotTestEvents::LogBytesFilter(element) => element.fmt(f),
                HotShotTestEvents::LogBytes32Filter(element) => element.fmt(f),
                HotShotTestEvents::LogIntFilter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedAddressFilter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedArray1Filter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedArray2Filter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedArray3Filter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedBytesFilter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedIntFilter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedStringFilter(element) => element.fmt(f),
                HotShotTestEvents::LogNamedUintFilter(element) => element.fmt(f),
                HotShotTestEvents::LogStringFilter(element) => element.fmt(f),
                HotShotTestEvents::LogUintFilter(element) => element.fmt(f),
                HotShotTestEvents::LogsFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `testExpander` function with signature `testExpander()` and selector `[128, 46, 159, 152]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "testExpander", abi = "testExpander()")]
    pub struct TestExpanderCall;
    #[doc = "Container type for all input parameters for the `testPublishCommitments` function with signature `testPublishCommitments()` and selector `[153, 151, 175, 219]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "testPublishCommitments", abi = "testPublishCommitments()")]
    pub struct TestPublishCommitmentsCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum HotShotTestCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        Hotshot(HotshotCall),
        SetUp(SetUpCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TestExpander(TestExpanderCall),
        TestPublishCommitments(TestPublishCommitmentsCall),
    }
    impl ethers::core::abi::AbiDecode for HotShotTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::IsTest(decoded));
            }
            if let Ok(decoded) =
                <ExcludeArtifactsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeContractsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::ExcludeContracts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeSendersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::Failed(decoded));
            }
            if let Ok(decoded) =
                <HotshotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::Hotshot(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::SetUp(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactSelectorsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::TargetArtifacts(decoded));
            }
            if let Ok(decoded) =
                <TargetContractsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::TargetContracts(decoded));
            }
            if let Ok(decoded) =
                <TargetSelectorsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::TargetSelectors(decoded));
            }
            if let Ok(decoded) =
                <TargetSendersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::TargetSenders(decoded));
            }
            if let Ok(decoded) =
                <TestExpanderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::TestExpander(decoded));
            }
            if let Ok(decoded) =
                <TestPublishCommitmentsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(HotShotTestCalls::TestPublishCommitments(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for HotShotTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                HotShotTestCalls::IsTest(element) => element.encode(),
                HotShotTestCalls::ExcludeArtifacts(element) => element.encode(),
                HotShotTestCalls::ExcludeContracts(element) => element.encode(),
                HotShotTestCalls::ExcludeSenders(element) => element.encode(),
                HotShotTestCalls::Failed(element) => element.encode(),
                HotShotTestCalls::Hotshot(element) => element.encode(),
                HotShotTestCalls::SetUp(element) => element.encode(),
                HotShotTestCalls::TargetArtifactSelectors(element) => element.encode(),
                HotShotTestCalls::TargetArtifacts(element) => element.encode(),
                HotShotTestCalls::TargetContracts(element) => element.encode(),
                HotShotTestCalls::TargetSelectors(element) => element.encode(),
                HotShotTestCalls::TargetSenders(element) => element.encode(),
                HotShotTestCalls::TestExpander(element) => element.encode(),
                HotShotTestCalls::TestPublishCommitments(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for HotShotTestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                HotShotTestCalls::IsTest(element) => element.fmt(f),
                HotShotTestCalls::ExcludeArtifacts(element) => element.fmt(f),
                HotShotTestCalls::ExcludeContracts(element) => element.fmt(f),
                HotShotTestCalls::ExcludeSenders(element) => element.fmt(f),
                HotShotTestCalls::Failed(element) => element.fmt(f),
                HotShotTestCalls::Hotshot(element) => element.fmt(f),
                HotShotTestCalls::SetUp(element) => element.fmt(f),
                HotShotTestCalls::TargetArtifactSelectors(element) => element.fmt(f),
                HotShotTestCalls::TargetArtifacts(element) => element.fmt(f),
                HotShotTestCalls::TargetContracts(element) => element.fmt(f),
                HotShotTestCalls::TargetSelectors(element) => element.fmt(f),
                HotShotTestCalls::TargetSenders(element) => element.fmt(f),
                HotShotTestCalls::TestExpander(element) => element.fmt(f),
                HotShotTestCalls::TestPublishCommitments(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for HotShotTestCalls {
        fn from(var: IsTestCall) -> Self {
            HotShotTestCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<ExcludeArtifactsCall> for HotShotTestCalls {
        fn from(var: ExcludeArtifactsCall) -> Self {
            HotShotTestCalls::ExcludeArtifacts(var)
        }
    }
    impl ::std::convert::From<ExcludeContractsCall> for HotShotTestCalls {
        fn from(var: ExcludeContractsCall) -> Self {
            HotShotTestCalls::ExcludeContracts(var)
        }
    }
    impl ::std::convert::From<ExcludeSendersCall> for HotShotTestCalls {
        fn from(var: ExcludeSendersCall) -> Self {
            HotShotTestCalls::ExcludeSenders(var)
        }
    }
    impl ::std::convert::From<FailedCall> for HotShotTestCalls {
        fn from(var: FailedCall) -> Self {
            HotShotTestCalls::Failed(var)
        }
    }
    impl ::std::convert::From<HotshotCall> for HotShotTestCalls {
        fn from(var: HotshotCall) -> Self {
            HotShotTestCalls::Hotshot(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for HotShotTestCalls {
        fn from(var: SetUpCall) -> Self {
            HotShotTestCalls::SetUp(var)
        }
    }
    impl ::std::convert::From<TargetArtifactSelectorsCall> for HotShotTestCalls {
        fn from(var: TargetArtifactSelectorsCall) -> Self {
            HotShotTestCalls::TargetArtifactSelectors(var)
        }
    }
    impl ::std::convert::From<TargetArtifactsCall> for HotShotTestCalls {
        fn from(var: TargetArtifactsCall) -> Self {
            HotShotTestCalls::TargetArtifacts(var)
        }
    }
    impl ::std::convert::From<TargetContractsCall> for HotShotTestCalls {
        fn from(var: TargetContractsCall) -> Self {
            HotShotTestCalls::TargetContracts(var)
        }
    }
    impl ::std::convert::From<TargetSelectorsCall> for HotShotTestCalls {
        fn from(var: TargetSelectorsCall) -> Self {
            HotShotTestCalls::TargetSelectors(var)
        }
    }
    impl ::std::convert::From<TargetSendersCall> for HotShotTestCalls {
        fn from(var: TargetSendersCall) -> Self {
            HotShotTestCalls::TargetSenders(var)
        }
    }
    impl ::std::convert::From<TestExpanderCall> for HotShotTestCalls {
        fn from(var: TestExpanderCall) -> Self {
            HotShotTestCalls::TestExpander(var)
        }
    }
    impl ::std::convert::From<TestPublishCommitmentsCall> for HotShotTestCalls {
        fn from(var: TestPublishCommitmentsCall) -> Self {
            HotShotTestCalls::TestPublishCommitments(var)
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
    #[doc = "`FuzzSelector(address,bytes4[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FuzzSelector {
        pub addr: ethers::core::types::Address,
        pub selectors: Vec<[u8; 4]>,
    }
}
