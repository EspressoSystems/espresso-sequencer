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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"firstBlockNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBlocks\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeArtifacts\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"excludedArtifacts_\",\"type\":\"string[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeContracts\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"excludedContracts_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"excludeSenders\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"excludedSenders_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hotshot\",\"outputs\":[{\"internalType\":\"contract HotShot\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetArtifactSelectors\",\"outputs\":[{\"internalType\":\"struct StdInvariant.FuzzSelector[]\",\"name\":\"targetedArtifactSelectors_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4[]\",\"name\":\"selectors\",\"type\":\"bytes4[]\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetArtifacts\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"targetedArtifacts_\",\"type\":\"string[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetContracts\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targetedContracts_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetSelectors\",\"outputs\":[{\"internalType\":\"struct StdInvariant.FuzzSelector[]\",\"name\":\"targetedSelectors_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4[]\",\"name\":\"selectors\",\"type\":\"bytes4[]\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"targetSenders\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targetedSenders_\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"testExpander\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testPublishCommitments\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static HOTSHOTTEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static HOTSHOTTEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405260008054600160ff19918216811790925560048054909116909117905534801561002d57600080fd5b506122288061003d6000396000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c806385226c811161008c578063b5508aa911610066578063b5508aa914610194578063ba414fa61461019c578063e20c9f71146101b4578063fa7626d4146101bc57600080fd5b806385226c811461016f578063916a17c6146101845780639997afdb1461018c57600080fd5b80633e5e3c23116100c85780633e5e3c23146101425780633f7286f41461014a57806366d9a9a014610152578063802e9f981461016757600080fd5b80630a9254e4146100ef5780631ed7831c146100f95780632adc8b7614610117575b600080fd5b6100f76101c9565b005b610101610214565b60405161010e9190610f25565b60405180910390f35b601b5461012a906001600160a01b031681565b6040516001600160a01b03909116815260200161010e565b610101610276565b6101016102d6565b61015a610336565b60405161010e9190610f72565b6100f7610425565b61017761050f565b60405161010e9190611075565b61015a6105df565b6100f76106c5565b610177610a8f565b6101a4610b5f565b604051901515815260200161010e565b610101610c8a565b6000546101a49060ff1681565b6040516101d590610f18565b604051809103906000f0801580156101f1573d6000803e3d6000fd5b50601b80546001600160a01b0319166001600160a01b0392909216919091179055565b6060600d80548060200260200160405190810160405280929190818152602001828054801561026c57602002820191906000526020600020905b81546001600160a01b0316815260019091019060200180831161024e575b5050505050905090565b6060600f80548060200260200160405190810160405280929190818152602001828054801561026c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161024e575050505050905090565b6060600e80548060200260200160405190810160405280929190818152602001828054801561026c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161024e575050505050905090565b60606012805480602002602001604051908101604052809291908181526020016000905b8282101561041c5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561040457602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116103c65790505b5050505050815250508152602001906001019061035a565b50505050905090565b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061045b5761045b6110ed565b602002602001019060ff16908160ff1681525050600281600181518110610484576104846110ed565b60ff90921660209283029190910190910152601b54604051632b815b5760e01b81526001600160a01b0390911690632b815b57906104c6908490600401611103565b600060405180830381865afa1580156104e3573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261050b919081019061114f565b5050565b60606011805480602002602001604051908101604052809291908181526020016000905b8282101561041c57838290600052602060002001805461055290611214565b80601f016020809104026020016040519081016040528092919081815260200182805461057e90611214565b80156105cb5780601f106105a0576101008083540402835291602001916105cb565b820191906000526020600020905b8154815290600101906020018083116105ae57829003601f168201915b505050505081526020019060010190610533565b60606013805480602002602001604051908101604052809291908181526020016000905b8282101561041c5760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156106ad57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161066f5790505b50505050508152505081526020019060010190610603565b6040805160028082526060820183526000926020830190803683375050604080516002808252606082019092529293506000929150602082015b60608152602001906001900390816106ff5790505090506486382370958260008151811061072f5761072f6110ed565b6020026020010181815250506040518060400160405280600681526020016530783333333360d01b8152508160008151811061076d5761076d6110ed565b602002602001018190525064368bd531fe82600181518110610791576107916110ed565b602002602001018181525050604051806040016040528060068152602001650c1e0d0d0d0d60d21b815250816001815181106107cf576107cf6110ed565b6020908102919091010152601b546040516381bad6f360e01b8152600060048201819052602482018190526044820152600160648201526001600160a01b039091166084820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906381bad6f39060a401600060405180830381600087803b15801561084f57600080fd5b505af1158015610863573d6000803e3d6000fd5b50506040805160008152600260208201527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1935001905060405180910390a1601b54604051630676923560e41b81526001600160a01b03909116906367692350906108d4908590859060040161124e565b600060405180830381600087803b1580156108ee57600080fd5b505af1158015610902573d6000803e3d6000fd5b5050601b546040516349ce899760e01b81526000600482015261099793506001600160a01b0390911691506349ce899790602401602060405180830381865afa158015610953573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061097791906112e6565b8360008151811061098a5761098a6110ed565b6020026020010151610cea565b601b546040516349ce899760e01b815260016004820152610a19916001600160a01b0316906349ce899790602401602060405180830381865afa1580156109e2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a0691906112e6565b8360018151811061098a5761098a6110ed565b601b546040516349ce899760e01b81526002600482015261050b916001600160a01b0316906349ce899790602401602060405180830381865afa158015610a64573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a8891906112e6565b6000610cea565b60606010805480602002602001604051908101604052809291908181526020016000905b8282101561041c578382906000526020600020018054610ad290611214565b80601f0160208091040260200160405190810160405280929190818152602001828054610afe90611214565b8015610b4b5780601f10610b2057610100808354040283529160200191610b4b565b820191906000526020600020905b815481529060010190602001808311610b2e57829003601f168201915b505050505081526020019060010190610ab3565b60008054610100900460ff1615610b7f5750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610c855760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091610c0d917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016112ff565b60408051601f1981840301815290829052610c2791611330565b6000604051808303816000865af19150503d8060008114610c64576040519150601f19603f3d011682016040523d82523d6000602084013e610c69565b606091505b5091505080806020019051810190610c81919061134c565b9150505b919050565b6060600c80548060200260200160405190810160405280929190818152602001828054801561026c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161024e575050505050905090565b80821461050b577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50604051610d5b9060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808115e1c1958dd195960b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a8183015269080808081058dd1d585b60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a161050b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610f075760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f1981840301815290829052610ea692916020016112ff565b60408051601f1981840301815290829052610ec091611330565b6000604051808303816000865af19150503d8060008114610efd576040519150601f19603f3d011682016040523d82523d6000602084013e610f02565b606091505b505050505b6000805461ff001916610100179055565b610e7d8061137683390190565b6020808252825182820181905260009190848201906040850190845b81811015610f665783516001600160a01b031683529284019291840191600101610f41565b50909695505050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b8481101561101657898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b808310156110015783516001600160e01b0319168252928b019260019290920191908b0190610fd7565b50978a01979550505091870191600101610f9a565b50919998505050505050505050565b60005b83811015611040578181015183820152602001611028565b50506000910152565b60008151808452611061816020860160208601611025565b601f01601f19169290920160200192915050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b828110156110ca57603f198886030184526110b8858351611049565b9450928501929085019060010161109c565b5092979650505050505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6020808252825182820181905260009190848201906040850190845b81811015610f6657835160ff168352928401929184019160010161111f565b805160ff81168114610c8557600080fd5b6000602080838503121561116257600080fd5b825167ffffffffffffffff8082111561117a57600080fd5b818501915085601f83011261118e57600080fd5b8151818111156111a0576111a06110d7565b8060051b604051601f19603f830116810181811085821117156111c5576111c56110d7565b6040529182528482019250838101850191888311156111e357600080fd5b938501935b82851015611208576111f98561113e565b845293850193928501926111e8565b98975050505050505050565b600181811c9082168061122857607f821691505b60208210810361124857634e487b7160e01b600052602260045260246000fd5b50919050565b604080825283519082018190526000906020906060840190828701845b828110156112875781518452928401929084019060010161126b565b50505083810382850152845180825282820190600581901b8301840187850160005b838110156112d757601f198684030185526112c5838351611049565b948701949250908601906001016112a9565b50909998505050505050505050565b6000602082840312156112f857600080fd5b5051919050565b6001600160e01b0319831681528151600090611322816004850160208701611025565b919091016004019392505050565b60008251611342818460208701611025565b9190910192915050565b60006020828403121561135e57600080fd5b8151801515811461136e57600080fd5b939250505056fe608060405234801561001057600080fd5b50610e5d806100206000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c806354cac3c51161005b57806354cac3c5146100fc578063676923501461010f578063b3111b6d14610124578063f44ff7121461013757600080fd5b806326833dcc1461008d5780632b815b57146100a95780633b1f56c6146100c957806349ce8997146100dc575b600080fd5b6100966103e881565b6040519081526020015b60405180910390f35b6100bc6100b73660046109c0565b610140565b6040516100a09190610a85565b6100bc6100d7366004610acc565b6105d1565b6100966100ea366004610acc565b60006020819052908152604090205481565b61009661010a3660046109c0565b610668565b61012261011d366004610b31565b61073c565b005b6100966101323660046109c0565b6108a9565b61009660015481565b60606030602060026000600185825b8660ff168110156101c95780600003610192576040516001600160f81b031960f886901b16602082015260210160405160208183030381529060405291506101b7565b81846040516020016101a5929190610bcd565b60405160208183030381529060405291505b806101c181610c0b565b91505061014f565b5060005b885181101561022657818982815181106101e9576101e9610c24565b6020026020010151604051602001610202929190610bcd565b6040516020818303038152906040529150808061021e90610c0b565b9150506101cd565b50808360405160200161023a929190610bcd565b6040516020818303038152906040529050808660405160200161025e929190610bcd565b60408051601f1981840301815290829052915083906102839083908390602001610bcd565b60408051808303601f1901815282820182526001808452602080850182905292519195506102b5928692829101610c3a565b60408051808303601f190181528282528051602091820120818401819052825180850390920182528383019092529450906102f69085908790606001610bcd565b60408051808303601f1901815290829052835160208086015192975061031f9388939101610c3a565b60408051601f1981840301815291905280516020820120909450600060ff8b1667ffffffffffffffff81111561035757610357610994565b604051908082528060200260200182016040528015610380578160200160208202803683370190505b509050600061038e836105d1565b905060005b81518110156103ec578181815181106103ae576103ae610c24565b60200260200101518382815181106103c8576103c8610c24565b60ff90921660209283029190910190910152806103e481610c0b565b915050610393565b5060006103f8856105d1565b905060005b8c8110156104e6578060000361047b5782818151811061041f5761041f610c24565b602002602001015182828151811061043957610439610c24565b602002602001015118604051602001610465919060f89190911b6001600160f81b031916815260010190565b60405160208183030381529060405298506104d4565b8883828151811061048e5761048e610c24565b60200260200101518383815181106104a8576104a8610c24565b6020026020010151186040516020016104c2929190610bcd565b60405160208183030381529060405298505b806104de81610c0b565b9150506103fd565b50878b6040516020016104fa929190610bcd565b60408051808303601f190181529082905287516020808a0151929b50610523938c939101610c3a565b604051602081830303815290604052975087805190602001209350610547846105d1565b915060005b8c8e60ff1661055b9190610c6e565b8110156105bd5782818151811061057457610574610c24565b602002602001015184828f6105899190610c87565b8151811061059957610599610c24565b60ff90921660209283029190910190910152806105b581610c0b565b91505061054c565b50919e9d5050505050505050505050505050565b60408051602080825261042082019092526060918082016104008036833701905050905060005b60208110156106625761060c81601f610c6e565b610617906008610c9a565b610622906002610d95565b61062c9084610da8565b82828151811061063e5761063e610c24565b60ff909216602092830291909101909101528061065a81610c0b565b9150506105f8565b50919050565b60008061067483610140565b805190915060008167ffffffffffffffff81111561069457610694610994565b6040519080825280602002602001820160405280156106bd578160200160208202803683370190505b50905060005b82811015610730578360016106d88386610c6e565b6106e29190610c6e565b815181106106f2576106f2610c24565b602002602001015182828151811061070c5761070c610c24565b60ff909216602092830291909101909101528061072881610c0b565b9150506106c3565b50601695945050505050565b82811461076b5760405163638df5d160e11b815260048101849052602481018290526044015b60405180910390fd5b6103e88311156107915760405163e082840b60e01b815260048101849052602401610762565b60015460005b84811015610868576107ec6001548787848181106107b7576107b7610c24565b905060200201358686858181106107d0576107d0610c24565b90506020028101906107e29190610dca565b6001949350505050565b61080f57600154604051637818671960e01b815260040161076291815260200190565b85858281811061082157610821610c24565b9050602002013560008060015481526020019081526020016000208190555060018060008282546108529190610c87565b90915550610861905081610c0b565b9050610797565b5060408051828152602081018690527f8203a21e4f95f72e5081d5e0929b1a8c52141e123f9a14e1e74b0260fa5f52f1910160405180910390a15050505050565b60007f0e0a77c19a07df2f666ea36f7879462c0a78eb28f5c70b3dd35d438dc58f0d9d81805b8451811015610935578481815181106108ea576108ea610c24565b602002602001015160ff168160086109029190610c9a565b61090d906002610d95565b6109179190610c9a565b6109219083610c87565b91508061092d81610c0b565b9150506108cf565b507f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47810361096557610965610e11565b60007f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4783830995945050505050565b634e487b7160e01b600052604160045260246000fd5b803560ff811681146109bb57600080fd5b919050565b600060208083850312156109d357600080fd5b823567ffffffffffffffff808211156109eb57600080fd5b818501915085601f8301126109ff57600080fd5b813581811115610a1157610a11610994565b8060051b604051601f19603f83011681018181108582111715610a3657610a36610994565b604052918252848201925083810185019188831115610a5457600080fd5b938501935b82851015610a7957610a6a856109aa565b84529385019392850192610a59565b98975050505050505050565b6020808252825182820181905260009190848201906040850190845b81811015610ac057835160ff1683529284019291840191600101610aa1565b50909695505050505050565b600060208284031215610ade57600080fd5b5035919050565b60008083601f840112610af757600080fd5b50813567ffffffffffffffff811115610b0f57600080fd5b6020830191508360208260051b8501011115610b2a57600080fd5b9250929050565b60008060008060408587031215610b4757600080fd5b843567ffffffffffffffff80821115610b5f57600080fd5b610b6b88838901610ae5565b90965094506020870135915080821115610b8457600080fd5b50610b9187828801610ae5565b95989497509550505050565b6000815160005b81811015610bbe5760208185018101518683015201610ba4565b50600093019283525090919050565b6000610bd98285610b9d565b60f89390931b6001600160f81b03191683525050600101919050565b634e487b7160e01b600052601160045260246000fd5b600060018201610c1d57610c1d610bf5565b5060010190565b634e487b7160e01b600052603260045260246000fd5b6000610c468286610b9d565b6001600160f81b031960f895861b811682529390941b90921660018401525050600201919050565b81810381811115610c8157610c81610bf5565b92915050565b80820180821115610c8157610c81610bf5565b8082028115828204841417610c8157610c81610bf5565b600181815b80851115610cec578160001904821115610cd257610cd2610bf5565b80851615610cdf57918102915b93841c9390800290610cb6565b509250929050565b600082610d0357506001610c81565b81610d1057506000610c81565b8160018114610d265760028114610d3057610d4c565b6001915050610c81565b60ff841115610d4157610d41610bf5565b50506001821b610c81565b5060208310610133831016604e8410600b8410161715610d6f575081810a610c81565b610d798383610cb1565b8060001904821115610d8d57610d8d610bf5565b029392505050565b6000610da18383610cf4565b9392505050565b600082610dc557634e487b7160e01b600052601260045260246000fd5b500490565b6000808335601e19843603018112610de157600080fd5b83018035915067ffffffffffffffff821115610dfc57600080fd5b602001915036819003821315610b2a57600080fd5b634e487b7160e01b600052600160045260246000fdfea2646970667358221220234967dda3428c9047488e74f88954de21de3917607a965b847c8171f84658b464736f6c63430008120033a2646970667358221220409a6afa8b6171bf0e72691f46ee0bd02a48020bc6429a47571b6d9a64aab38c64736f6c63430008120033" . parse () . expect ("invalid bytecode")
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
