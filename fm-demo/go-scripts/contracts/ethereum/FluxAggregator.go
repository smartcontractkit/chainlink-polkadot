// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package ethereum

import (
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
)

// FluxAggregatorABI is the input ABI used to generate the binding from.
const FluxAggregatorABI = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_link\",\"type\":\"address\"},{\"internalType\":\"uint128\",\"name\":\"_paymentAmount\",\"type\":\"uint128\"},{\"internalType\":\"uint32\",\"name\":\"_timeout\",\"type\":\"uint32\"},{\"internalType\":\"address\",\"name\":\"_validator\",\"type\":\"address\"},{\"internalType\":\"int256\",\"name\":\"_minSubmissionValue\",\"type\":\"int256\"},{\"internalType\":\"int256\",\"name\":\"_maxSubmissionValue\",\"type\":\"int256\"},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\"},{\"internalType\":\"string\",\"name\":\"_description\",\"type\":\"string\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"int256\",\"name\":\"current\",\"type\":\"int256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"}],\"name\":\"AnswerUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"AvailableFundsUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"roundId\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"startedBy\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"}],\"name\":\"NewRound\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\"}],\"name\":\"OracleAdminUpdateRequested\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\"}],\"name\":\"OracleAdminUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"bool\",\"name\":\"whitelisted\",\"type\":\"bool\"}],\"name\":\"OraclePermissionsUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"name\":\"OwnershipTransferRequested\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"authorized\",\"type\":\"bool\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"delay\",\"type\":\"uint32\"}],\"name\":\"RequesterPermissionsSet\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint128\",\"name\":\"paymentAmount\",\"type\":\"uint128\"},{\"indexed\":true,\"internalType\":\"uint32\",\"name\":\"minSubmissionCount\",\"type\":\"uint32\"},{\"indexed\":true,\"internalType\":\"uint32\",\"name\":\"maxSubmissionCount\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"restartDelay\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"timeout\",\"type\":\"uint32\"}],\"name\":\"RoundDetailsUpdated\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"int256\",\"name\":\"submission\",\"type\":\"int256\"},{\"indexed\":true,\"internalType\":\"uint32\",\"name\":\"round\",\"type\":\"uint32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"oracle\",\"type\":\"address\"}],\"name\":\"SubmissionReceived\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previous\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"current\",\"type\":\"address\"}],\"name\":\"ValidatorUpdated\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\"}],\"name\":\"acceptAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"acceptOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"allocatedFunds\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"availableFunds\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_removed\",\"type\":\"address[]\"},{\"internalType\":\"address[]\",\"name\":\"_added\",\"type\":\"address[]\"},{\"internalType\":\"address[]\",\"name\":\"_addedAdmins\",\"type\":\"address[]\"},{\"internalType\":\"uint32\",\"name\":\"_minSubmissions\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"_maxSubmissions\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"_restartDelay\",\"type\":\"uint32\"}],\"name\":\"changeOracles\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\"}],\"name\":\"getAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\"}],\"name\":\"getAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getOracles\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\"}],\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\"},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\"}],\"name\":\"getTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestAnswer\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestRound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\"},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\"},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"latestTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"linkToken\",\"outputs\":[{\"internalType\":\"contractLinkTokenInterface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"maxSubmissionCount\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"maxSubmissionValue\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"minSubmissionCount\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"minSubmissionValue\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\"}],\"name\":\"onTokenTransfer\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"oracleCount\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_queriedRoundId\",\"type\":\"uint32\"}],\"name\":\"oracleRoundState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_eligibleToSubmit\",\"type\":\"bool\"},{\"internalType\":\"uint32\",\"name\":\"_roundId\",\"type\":\"uint32\"},{\"internalType\":\"int256\",\"name\":\"_latestSubmission\",\"type\":\"int256\"},{\"internalType\":\"uint64\",\"name\":\"_startedAt\",\"type\":\"uint64\"},{\"internalType\":\"uint64\",\"name\":\"_timeout\",\"type\":\"uint64\"},{\"internalType\":\"uint128\",\"name\":\"_availableFunds\",\"type\":\"uint128\"},{\"internalType\":\"uint8\",\"name\":\"_oracleCount\",\"type\":\"uint8\"},{\"internalType\":\"uint128\",\"name\":\"_paymentAmount\",\"type\":\"uint128\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"paymentAmount\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"requestNewRound\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"\",\"type\":\"uint80\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"restartDelay\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_requester\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"_authorized\",\"type\":\"bool\"},{\"internalType\":\"uint32\",\"name\":\"_delay\",\"type\":\"uint32\"}],\"name\":\"setRequesterPermissions\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newValidator\",\"type\":\"address\"}],\"name\":\"setValidator\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_roundId\",\"type\":\"uint256\"},{\"internalType\":\"int256\",\"name\":\"_submission\",\"type\":\"int256\"}],\"name\":\"submit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"timeout\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_newAdmin\",\"type\":\"address\"}],\"name\":\"transferAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"updateAvailableFunds\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"_paymentAmount\",\"type\":\"uint128\"},{\"internalType\":\"uint32\",\"name\":\"_minSubmissions\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"_maxSubmissions\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"_restartDelay\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"_timeout\",\"type\":\"uint32\"}],\"name\":\"updateFutureRounds\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"validator\",\"outputs\":[{\"internalType\":\"contractAggregatorValidatorInterface\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"withdrawFunds\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"withdrawPayment\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_oracle\",\"type\":\"address\"}],\"name\":\"withdrawablePayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]"

// FluxAggregatorBin is the compiled bytecode used for deploying new contracts.
var FluxAggregatorBin = "0x60c06040523480156200001157600080fd5b5060405162007d8838038062007d8883398181016040526101008110156200003857600080fd5b8101908080519060200190929190805190602001909291908051906020019092919080519060200190929190805190602001909291908051906020019092919080519060200190929190805160405193929190846401000000008211156200009f57600080fd5b83820191506020820185811115620000b657600080fd5b8251866001820283011164010000000082111715620000d457600080fd5b8083526020830192505050908051906020019080838360005b838110156200010a578082015181840152602081019050620000ed565b50505050905090810190601f168015620001385780820380516001836020036101000a031916815260200191505b50604052505050336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555087600260006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550620001d78760008060008a6200029c60201b60201c565b620001e885620007d160201b60201c565b83608081815250508260a0818152505081600560006101000a81548160ff021916908360ff16021790555080600690805190602001906200022b92919062000b00565b506200024c8663ffffffff16426200098f60201b620054371790919060201c565b600960008063ffffffff16815260200190815260200160002060010160086101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550505050505050505062000baf565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146200035f576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b60006200037162000a1960201b60201c565b60ff1690508463ffffffff168463ffffffff161015620003f9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260198152602001807f6d6178206d75737420657175616c2f657863656564206d696e0000000000000081525060200191505060405180910390fd5b8363ffffffff168163ffffffff1610156200047c576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260178152602001807f6d61782063616e6e6f742065786365656420746f74616c00000000000000000081525060200191505060405180910390fd5b60008163ffffffff1614806200049d57508263ffffffff168163ffffffff16115b62000510576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260198152602001807f64656c61792063616e6e6f742065786365656420746f74616c0000000000000081525060200191505060405180910390fd5b62000533866fffffffffffffffffffffffffffffffff1662000a2660201b60201c565b600d60000160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff161015620005dc576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f696e73756666696369656e742066756e647320666f72207061796d656e74000081525060200191505060405180910390fd5b6000620005ee62000a1960201b60201c565b60ff161115620006765760008563ffffffff161162000675576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601a8152602001807f6d696e206d7573742062652067726561746572207468616e203000000000000081525060200191505060405180910390fd5b5b85600460006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555084600460146101000a81548163ffffffff021916908363ffffffff16021790555083600460106101000a81548163ffffffff021916908363ffffffff16021790555082600460186101000a81548163ffffffff021916908363ffffffff160217905550816004601c6101000a81548163ffffffff021916908363ffffffff1602179055508363ffffffff168563ffffffff16600460009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff167f56800c9d1ed723511246614d15e58cfcde15b6a33c245b5c961b689c1890fd8f8686604051808363ffffffff1663ffffffff1681526020018263ffffffff1663ffffffff1681526020019250505060405180910390a4505050505050565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161462000894576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b6000600360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146200098b5781600360006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167fcfac5dc75b8d9a7e074162f59d9adcd33da59f0fe8dfb21580db298fc0fdad0d60405160405180910390a35b5050565b60008282111562000a08576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f536166654d6174683a207375627472616374696f6e206f766572666c6f77000081525060200191505060405180910390fd5b600082840390508091505092915050565b6000600c80549050905090565b600062000a6e600262000a5a62000a4262000a1960201b60201c565b60ff168562000a7560201b620061dd1790919060201c565b62000a7560201b620061dd1790919060201c565b9050919050565b60008083141562000a8a576000905062000afa565b600082840290508284828162000a9c57fe5b041462000af5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602181526020018062007d676021913960400191505060405180910390fd5b809150505b92915050565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f1062000b4357805160ff191683800117855562000b74565b8280016001018555821562000b74579182015b8281111562000b7357825182559160200191906001019062000b56565b5b50905062000b83919062000b87565b5090565b62000bac91905b8082111562000ba857600081600090555060010162000b8e565b5090565b90565b60805160a05161718a62000bdd600039806111ab52806116145250806111155280612b8e525061718a6000f3fe608060405234801561001057600080fd5b50600436106102535760003560e01c8063668a0f0211610146578063a4c0ed36116100c3578063c937450011610087578063c937450014610d73578063d4cc54e414610d9d578063e2e4031714610ddf578063e9ee6eeb14610e37578063f2fde38b14610e9b578063feaf968c14610edf57610253565b8063a4c0ed3614610bbc578063b5ab58dc14610c5f578063b633620c14610ca1578063c107532914610ce3578063c35905c614610d3157610253565b80638205bf6a1161010a5780638205bf6a1461096557806388aa80e7146109835780638da5cb5b14610aa257806398e5b12a14610aec5780639a6fc8f514610b2257610253565b8063668a0f021461087257806370dea79a146108905780637284e416146108ba57806379ba50971461093d5780637c2b0b211461094757610253565b806340884c52116101d457806357970e931161019857806357970e931461071257806358609e441461075c578063613d8fcc14610786578063628806ef146107aa57806364efb22b146107ee57610253565b806340884c521461062b57806346fcff4c1461068a5780634f8fc3b5146106cc57806350d25bcd146106d657806354fd4d50146106f457610253565b8063357ebb021161021b578063357ebb021461037657806338aa4c72146103a05780633969c20f146104205780633a5381b5146105735780633d3d7714146105bd57610253565b80631327d3d814610258578063202ee0ed1461029c57806320ed0275146102d457806323ca290314610334578063313ce56714610352575b600080fd5b61029a6004803603602081101561026e57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610f49565b005b6102d2600480360360408110156102b257600080fd5b810190808035906020019092919080359060200190929190505050611105565b005b610332600480360360608110156102ea57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803515159060200190929190803563ffffffff169060200190929190505050611339565b005b61033c611612565b6040518082815260200191505060405180910390f35b61035a611636565b604051808260ff1660ff16815260200191505060405180910390f35b61037e611649565b604051808263ffffffff1663ffffffff16815260200191505060405180910390f35b61041e600480360360a08110156103b657600080fd5b8101908080356fffffffffffffffffffffffffffffffff169060200190929190803563ffffffff169060200190929190803563ffffffff169060200190929190803563ffffffff169060200190929190803563ffffffff16906020019092919050505061165f565b005b610571600480360360c081101561043657600080fd5b810190808035906020019064010000000081111561045357600080fd5b82018360208201111561046557600080fd5b8035906020019184602083028401116401000000008311171561048757600080fd5b9091929391929390803590602001906401000000008111156104a857600080fd5b8201836020820111156104ba57600080fd5b803590602001918460208302840111640100000000831117156104dc57600080fd5b9091929391929390803590602001906401000000008111156104fd57600080fd5b82018360208201111561050f57600080fd5b8035906020019184602083028401116401000000008311171561053157600080fd5b9091929391929390803563ffffffff169060200190929190803563ffffffff169060200190929190803563ffffffff169060200190929190505050611b74565b005b61057b611e58565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b610629600480360360608110156105d357600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff16906020019092919080359060200190929190505050611e7e565b005b6106336122a4565b6040518080602001828103825283818151815260200191508051906020019060200280838360005b8381101561067657808201518184015260208101905061065b565b505050509050019250505060405180910390f35b610692612332565b60405180826fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b6106d461235b565b005b6106de612597565b6040518082815260200191505060405180910390f35b6106fc6125d3565b6040518082815260200191505060405180910390f35b61071a6125d8565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b6107646125fe565b604051808263ffffffff1663ffffffff16815260200191505060405180910390f35b61078e612614565b604051808260ff1660ff16815260200191505060405180910390f35b6107ec600480360360208110156107c057600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050612621565b005b6108306004803603602081101561080457600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050612884565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b61087a6128f0565b6040518082815260200191505060405180910390f35b610898612910565b604051808263ffffffff1663ffffffff16815260200191505060405180910390f35b6108c2612926565b6040518080602001828103825283818151815260200191508051906020019080838360005b838110156109025780820151818401526020810190506108e7565b50505050905090810190601f16801561092f5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6109456129c4565b005b61094f612b8c565b6040518082815260200191505060405180910390f35b61096d612bb0565b6040518082815260200191505060405180910390f35b6109d56004803603604081101561099957600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803563ffffffff169060200190929190505050612c0a565b60405180891515151581526020018863ffffffff1663ffffffff1681526020018781526020018667ffffffffffffffff1667ffffffffffffffff1681526020018567ffffffffffffffff1667ffffffffffffffff168152602001846fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020018360ff1660ff168152602001826fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020019850505050505050505060405180910390f35b610aaa612e69565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b610af4612e8e565b604051808269ffffffffffffffffffff1669ffffffffffffffffffff16815260200191505060405180910390f35b610b5a60048036036020811015610b3857600080fd5b81019080803569ffffffffffffffffffff169060200190929190505050613064565b604051808669ffffffffffffffffffff1669ffffffffffffffffffff1681526020018581526020018481526020018381526020018269ffffffffffffffffffff1669ffffffffffffffffffff1681526020019550505050505060405180910390f35b610c5d60048036036060811015610bd257600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291908035906020019092919080359060200190640100000000811115610c1957600080fd5b820183602082011115610c2b57600080fd5b80359060200191846001830284011164010000000083111715610c4d57600080fd5b9091929391929390505050613288565b005b610c8b60048036036020811015610c7557600080fd5b810190808035906020019092919050505061330f565b6040518082815260200191505060405180910390f35b610ccd60048036036020811015610cb757600080fd5b8101908080359060200190929190505050613353565b6040518082815260200191505060405180910390f35b610d2f60048036036040811015610cf957600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803590602001909291905050506133b5565b005b610d396136d2565b60405180826fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b610d7b6136f4565b604051808263ffffffff1663ffffffff16815260200191505060405180910390f35b610da561370a565b60405180826fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b610e2160048036036020811015610df557600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050613733565b6040518082815260200191505060405180910390f35b610e9960048036036040811015610e4d57600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803573ffffffffffffffffffffffffffffffffffffffff1690602001909291905050506137ad565b005b610edd60048036036020811015610eb157600080fd5b81019080803573ffffffffffffffffffffffffffffffffffffffff1690602001909291905050506139e3565b005b610ee7613b64565b604051808669ffffffffffffffffffff1669ffffffffffffffffffff1681526020018581526020018481526020018381526020018269ffffffffffffffffffff1669ffffffffffffffffffff1681526020019550505050505060405180910390f35b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461100b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b6000600360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16146111015781600360006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167fcfac5dc75b8d9a7e074162f59d9adcd33da59f0fe8dfb21580db298fc0fdad0d60405160405180910390a35b5050565b60606111113384613b9e565b90507f00000000000000000000000000000000000000000000000000000000000000008212156111a9576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f76616c75652062656c6f77206d696e5375626d697373696f6e56616c7565000081525060200191505060405180910390fd5b7f000000000000000000000000000000000000000000000000000000000000000082131561123f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f76616c75652061626f7665206d61785375626d697373696f6e56616c7565000081525060200191505060405180910390fd5b600081511481906112eb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825283818151815260200191508051906020019080838360005b838110156112b0578082015181840152602081019050611295565b50505050905090810190601f1680156112dd5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b506112f583613f13565b6112ff828461402d565b60008061130b856141e8565b91509150611318856143fb565b61132185614760565b8115611332576113318582614868565b5b5050505050565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146113fb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b811515600b60008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160009054906101000a900460ff161515141561145b5761160d565b81156115215781600b60008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160006101000a81548160ff02191690831515021790555080600b60008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160016101000a81548163ffffffff021916908363ffffffff1602179055506115a6565b600b60008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600080820160006101000a81549060ff02191690556000820160016101000a81549063ffffffff02191690556000820160056101000a81549063ffffffff021916905550505b8273ffffffffffffffffffffffffffffffffffffffff167fc3df5a754e002718f2e10804b99e6605e7c701d95cec9552c7680ca2b6f2820a838360405180831515151581526020018263ffffffff1663ffffffff1681526020019250505060405180910390a25b505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b600560009054906101000a900460ff1681565b600460189054906101000a900463ffffffff1681565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611721576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b600061172b612614565b60ff1690508463ffffffff168463ffffffff1610156117b2576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260198152602001807f6d6178206d75737420657175616c2f657863656564206d696e0000000000000081525060200191505060405180910390fd5b8363ffffffff168163ffffffff161015611834576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260178152602001807f6d61782063616e6e6f742065786365656420746f74616c00000000000000000081525060200191505060405180910390fd5b60008163ffffffff16148061185457508263ffffffff168163ffffffff16115b6118c6576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260198152602001807f64656c61792063616e6e6f742065786365656420746f74616c0000000000000081525060200191505060405180910390fd5b6118e1866fffffffffffffffffffffffffffffffff16614a0a565b600d60000160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff161015611989576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f696e73756666696369656e742066756e647320666f72207061796d656e74000081525060200191505060405180910390fd5b6000611993612614565b60ff161115611a195760008563ffffffff1611611a18576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601a8152602001807f6d696e206d7573742062652067726561746572207468616e203000000000000081525060200191505060405180910390fd5b5b85600460006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555084600460146101000a81548163ffffffff021916908363ffffffff16021790555083600460106101000a81548163ffffffff021916908363ffffffff16021790555082600460186101000a81548163ffffffff021916908363ffffffff160217905550816004601c6101000a81548163ffffffff021916908363ffffffff1602179055508363ffffffff168563ffffffff16600460009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff167f56800c9d1ed723511246614d15e58cfcde15b6a33c245b5c961b689c1890fd8f8686604051808363ffffffff1663ffffffff1681526020018263ffffffff1663ffffffff1681526020019250505060405180910390a4505050505050565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611c36576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b60008090505b89899050811015611c8557611c788a8a83818110611c5657fe5b9050602002013573ffffffffffffffffffffffffffffffffffffffff16614a43565b8080600101915050611c3c565b50848490508787905014611d01576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260208152602001807f6e6565642073616d65206f7261636c6520616e642061646d696e20636f756e7481525060200191505060405180910390fd5b604d611d2388889050611d12612614565b60ff16614d8e90919063ffffffff16565b1115611d97576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260138152602001807f6d6178206f7261636c657320616c6c6f7765640000000000000000000000000081525060200191505060405180910390fd5b60008090505b87879050811015611e0f57611e02888883818110611db757fe5b9050602002013573ffffffffffffffffffffffffffffffffffffffff16878784818110611de057fe5b9050602002013573ffffffffffffffffffffffffffffffffffffffff16614e16565b8080600101915050611d9d565b50611e4d600460009054906101000a90046fffffffffffffffffffffffffffffffff168484846004601c9054906101000a900463ffffffff1661165f565b505050505050505050565b600360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b3373ffffffffffffffffffffffffffffffffffffffff16600860008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614611f81576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f6f6e6c792063616c6c61626c652062792061646d696e0000000000000000000081525060200191505060405180910390fd5b60008190506000600860008673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160009054906101000a90046fffffffffffffffffffffffffffffffff169050816fffffffffffffffffffffffffffffffff16816fffffffffffffffffffffffffffffffff161015612083576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601f8152602001807f696e73756666696369656e7420776974686472617761626c652066756e64730081525060200191505060405180910390fd5b6120a882826fffffffffffffffffffffffffffffffff1661538a90919063ffffffff16565b600860008773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555061216682600d60000160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1661538a90919063ffffffff16565b600d60000160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff160217905550600260009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb85846fffffffffffffffffffffffffffffffff166040518363ffffffff1660e01b8152600401808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200182815260200192505050602060405180830381600087803b15801561225c57600080fd5b505af1158015612270573d6000803e3d6000fd5b505050506040513d602081101561228657600080fd5b810190808051906020019092919050505061229d57fe5b5050505050565b6060600c80548060200260200160405190810160405280929190818152602001828054801561232857602002820191906000526020600020905b8160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116122de575b5050505050905090565b6000600d60000160009054906101000a90046fffffffffffffffffffffffffffffffff16905090565b612363616fac565b600d6040518060400160405290816000820160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020016000820160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250509050600061250a82602001516fffffffffffffffffffffffffffffffff16600260009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff166370a08231306040518263ffffffff1660e01b8152600401808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060206040518083038186803b1580156124c157600080fd5b505afa1580156124d5573d6000803e3d6000fd5b505050506040513d60208110156124eb57600080fd5b810190808051906020019092919050505061543790919063ffffffff16565b90508082600001516fffffffffffffffffffffffffffffffff16146125935780600d60000160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff160217905550807ffe25c73e3b9089fac37d55c4c7efcba6f04af04cebd2fc4d6d7dbb07e1e5234f60405160405180910390a25b5050565b600060096000600760049054906101000a900463ffffffff1663ffffffff1663ffffffff16815260200190815260200160002060000154905090565b600381565b600260009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600460109054906101000a900463ffffffff1681565b6000600c80549050905090565b3373ffffffffffffffffffffffffffffffffffffffff16600860008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060030160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1614612724576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f6f6e6c792063616c6c61626c652062792070656e64696e672061646d696e000081525060200191505060405180910390fd5b6000600860008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060030160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555033600860008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160026101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055503373ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f0c5055390645c15a4be9a21b3f8d019153dcb4a0c125685da6eb84048e2fe90460405160405180910390a350565b6000600860008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160029054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050919050565b6000600760049054906101000a900463ffffffff1663ffffffff16905090565b6004601c9054906101000a900463ffffffff1681565b60068054600181600116156101000203166002900480601f0160208091040260200160405190810160405280929190818152602001828054600181600116156101000203166002900480156129bc5780601f10612991576101008083540402835291602001916129bc565b820191906000526020600020905b81548152906001019060200180831161299f57829003601f168201915b505050505081565b600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612a87576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4d7573742062652070726f706f736564206f776e65720000000000000000000081525060200191505060405180910390fd5b60008060009054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506000600160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055503373ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a350565b7f000000000000000000000000000000000000000000000000000000000000000081565b600060096000600760049054906101000a900463ffffffff1663ffffffff1663ffffffff16815260200190815260200160002060010160089054906101000a900467ffffffffffffffff1667ffffffffffffffff16905090565b6000806000806000806000803273ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612cb7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f6f66662d636861696e2072656164696e67206f6e6c790000000000000000000081525060200191505060405180910390fd5b60008963ffffffff161115612e42576000600960008b63ffffffff1663ffffffff16815260200190815260200160002090506000600a60008c63ffffffff1663ffffffff1681526020019081526020016000209050612d168c8c6154c0565b8b600860008f73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600101548460010160009054906101000a900467ffffffffffffffff168460010160089054906101000a900463ffffffff16600d60000160009054906101000a90046fffffffffffffffffffffffffffffffff16612db2612614565b60008960010160009054906101000a900467ffffffffffffffff1667ffffffffffffffff1611612e0057600460009054906101000a90046fffffffffffffffffffffffffffffffff16612e22565b87600101600c9054906101000a90046fffffffffffffffffffffffffffffffff165b8363ffffffff169350995099509950995099509950995099505050612e5c565b612e4b8a615559565b975097509750975097509750975097505b9295985092959890939650565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6000600b60003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160009054906101000a900460ff16612f52576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260188152602001807f6e6f7420617574686f72697a656420726571756573746572000000000000000081525060200191505060405180910390fd5b6000600760009054906101000a900463ffffffff1690506000600960008363ffffffff1663ffffffff16815260200190815260200160002060010160089054906101000a900467ffffffffffffffff1667ffffffffffffffff161180612fbd5750612fbc8161580a565b5b61302f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601f8152602001807f7072657620726f756e64206d75737420626520737570657273656461626c650081525060200191505060405180910390fd5b600061304b60018363ffffffff166158dd90919063ffffffff16565b905061305681615971565b8063ffffffff169250505090565b6000806000806000613074616fea565b600960008863ffffffff1663ffffffff168152602001908152602001600020604051806080016040529081600082015481526020016001820160009054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020016001820160089054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020016001820160109054906101000a900463ffffffff1663ffffffff1663ffffffff168152505090506000816060015163ffffffff1611801561315f575061315e8769ffffffffffffffffffff16615b2f565b5b6040518060400160405280600f81526020017f4e6f20646174612070726573656e7400000000000000000000000000000000008152509061323b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825283818151815260200191508051906020019080838360005b838110156132005780820151818401526020810190506131e5565b50505050905090810190601f16801561322d5780820380516001836020036101000a031916815260200191505b509250505060405180910390fd5b508681600001518260200151836040015184606001518267ffffffffffffffff1692508167ffffffffffffffff1691508063ffffffff169050955095509550955095505091939590929450565b60008282905014613301576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260208152602001807f7472616e7366657220646f65736e2774206163636570742063616c6c6461746181525060200191505060405180910390fd5b61330961235b565b50505050565b600061331a82615b2f565b1561334957600960008363ffffffff1663ffffffff16815260200190815260200160002060000154905061334e565b600090505b919050565b600061335e82615b2f565b156133ab57600960008363ffffffff1663ffffffff16815260200190815260200160002060010160089054906101000a900467ffffffffffffffff1667ffffffffffffffff1690506133b0565b600090505b919050565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613477576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b6000600d60000160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff169050816134fb6134ec600460009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16614a0a565b8361543790919063ffffffff16565b101561356f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601a8152602001807f696e73756666696369656e7420726573657276652066756e647300000000000081525060200191505060405180910390fd5b600260009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663a9059cbb84846040518363ffffffff1660e01b8152600401808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200182815260200192505050602060405180830381600087803b15801561361857600080fd5b505af115801561362c573d6000803e3d6000fd5b505050506040513d602081101561364257600080fd5b81019080805190602001909291905050506136c5576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260158152602001807f746f6b656e207472616e73666572206661696c6564000000000000000000000081525060200191505060405180910390fd5b6136cd61235b565b505050565b600460009054906101000a90046fffffffffffffffffffffffffffffffff1681565b600460149054906101000a900463ffffffff1681565b6000600d60000160109054906101000a90046fffffffffffffffffffffffffffffffff16905090565b6000600860008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff169050919050565b3373ffffffffffffffffffffffffffffffffffffffff16600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16146138b0576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f6f6e6c792063616c6c61626c652062792061646d696e0000000000000000000081525060200191505060405180910390fd5b80600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060030160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff167fb79bf2e89c2d70dde91d2991fb1ea69b7e478061ad7c04ed5b02b96bc52b81043383604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019250505060405180910390a25050565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613aa5576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f4f6e6c792063616c6c61626c65206279206f776e65720000000000000000000081525060200191505060405180910390fd5b80600160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508073ffffffffffffffffffffffffffffffffffffffff166000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff167fed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae127860405160405180910390a350565b6000806000806000613b8d600760049054906101000a900463ffffffff1663ffffffff16613064565b945094509450945094509091929394565b60606000600860008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160109054906101000a900463ffffffff1690506000600760009054906101000a900463ffffffff16905060008263ffffffff161415613c5c576040518060400160405280601281526020017f6e6f7420656e61626c6564206f7261636c65000000000000000000000000000081525092505050613f0d565b8363ffffffff168263ffffffff161115613caf576040518060400160405280601681526020017f6e6f742079657420656e61626c6564206f7261636c650000000000000000000081525092505050613f0d565b8363ffffffff16600860008773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160149054906101000a900463ffffffff1663ffffffff161015613d54576040518060400160405280601881526020017f6e6f206c6f6e67657220616c6c6f776564206f7261636c65000000000000000081525092505050613f0d565b8363ffffffff16600860008773ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160189054906101000a900463ffffffff1663ffffffff1610613df8576040518060400160405280602081526020017f63616e6e6f74207265706f7274206f6e2070726576696f757320726f756e647381525092505050613f0d565b8063ffffffff168463ffffffff1614158015613e395750613e2960018263ffffffff166158dd90919063ffffffff16565b63ffffffff168463ffffffff1614155b8015613e4c5750613e4a8482615b42565b155b15613e90576040518060400160405280601781526020017f696e76616c696420726f756e6420746f207265706f727400000000000000000081525092505050613f0d565b60018463ffffffff1614158015613ec65750613ec4613ebf60018663ffffffff16615bc090919063ffffffff16565b615c55565b155b15613f0a576040518060400160405280601f81526020017f70726576696f757320726f756e64206e6f7420737570657273656461626c650081525092505050613f0d565b50505b92915050565b613f1c81615cb1565b613f255761402a565b6000600860003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600001601c9054906101000a900463ffffffff1663ffffffff169050600460189054906101000a900463ffffffff1663ffffffff1681018263ffffffff1611158015613fb3575060008114155b15613fbe575061402a565b613fc782615cf4565b81600860003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600001601c6101000a81548163ffffffff021916908363ffffffff160217905550505b50565b61403681615ffd565b6140a8576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601f8152602001807f726f756e64206e6f7420616363657074696e67207375626d697373696f6e730081525060200191505060405180910390fd5b600a60008263ffffffff1663ffffffff16815260200190815260200160002060000182908060018154018082558091505060019003906000526020600020016000909190919091505580600860003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160186101000a81548163ffffffff021916908363ffffffff16021790555081600860003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600101819055503373ffffffffffffffffffffffffffffffffffffffff168163ffffffff16837f92e98423f8adac6e64d0608e519fd1cefb861498385c6dee70d58fc926ddc68c60405160405180910390a45050565b600080600a60008463ffffffff1663ffffffff16815260200190815260200160002060010160049054906101000a900463ffffffff1663ffffffff16600a60008563ffffffff1663ffffffff16815260200190815260200160002060000180549050101561425f57600080809050915091506143f6565b60006142da600a60008663ffffffff1663ffffffff1681526020019081526020016000206000018054806020026020016040519081016040528092919081815260200182805480156142d057602002820191906000526020600020905b8154815260200190600101908083116142bc575b5050505050616042565b905080600960008663ffffffff1663ffffffff1681526020019081526020016000206000018190555042600960008663ffffffff1663ffffffff16815260200190815260200160002060010160086101000a81548167ffffffffffffffff021916908367ffffffffffffffff16021790555083600960008663ffffffff1663ffffffff16815260200190815260200160002060010160106101000a81548163ffffffff021916908363ffffffff16021790555083600760046101000a81548163ffffffff021916908363ffffffff1602179055508363ffffffff16817f0559884fd3a460db3073b7fc896cc77986f16e378210ded43186175bf646fc5f426040518082815260200191505060405180910390a360018192509250505b915091565b6000600a60008363ffffffff1663ffffffff168152602001908152602001600020600101600c9054906101000a90046fffffffffffffffffffffffffffffffff169050614446616fac565b600d6040518060400160405290816000820160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1681526020016000820160109054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff168152505090506145128282600001516fffffffffffffffffffffffffffffffff1661538a90919063ffffffff16565b81600001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505061456a8282602001516fffffffffffffffffffffffffffffffff1661613190919063ffffffff16565b81602001906fffffffffffffffffffffffffffffffff1690816fffffffffffffffffffffffffffffffff168152505080600d60008201518160000160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555060208201518160000160106101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff1602179055509050506146a082600860003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff1661613190919063ffffffff16565b600860003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160006101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555080600001516fffffffffffffffffffffffffffffffff167ffe25c73e3b9089fac37d55c4c7efcba6f04af04cebd2fc4d6d7dbb07e1e5234f60405160405180910390a2505050565b600a60008263ffffffff1663ffffffff16815260200190815260200160002060010160009054906101000a900463ffffffff1663ffffffff16600a60008363ffffffff1663ffffffff1681526020019081526020016000206000018054905010156147ca57614865565b600a60008263ffffffff1663ffffffff168152602001908152602001600020600080820160006147fa919061702c565b6001820160006101000a81549063ffffffff02191690556001820160046101000a81549063ffffffff02191690556001820160086101000a81549063ffffffff021916905560018201600c6101000a8154906fffffffffffffffffffffffffffffffff021916905550505b50565b6000600360009054906101000a900473ffffffffffffffffffffffffffffffffffffffff169050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614156148ca5750614a06565b60006148e660018563ffffffff16615bc090919063ffffffff16565b90506000600960008363ffffffff1663ffffffff16815260200190815260200160002060010160109054906101000a900463ffffffff1690506000600960008463ffffffff1663ffffffff1681526020019081526020016000206000015490508373ffffffffffffffffffffffffffffffffffffffff1663beed9b51620186a084848a8a6040518663ffffffff1660e01b8152600401808563ffffffff1681526020018481526020018363ffffffff168152602001828152602001945050505050602060405180830381600088803b1580156149c157600080fd5b5087f1935050505080156149f657506040513d60208110156149e257600080fd5b810190808051906020019092919050505060015b6149ff57614a01565b505b505050505b5050565b6000614a3c6002614a2e614a1c612614565b60ff16856161dd90919063ffffffff16565b6161dd90919063ffffffff16565b9050919050565b614a4c81616263565b614abe576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260128152602001807f6f7261636c65206e6f7420656e61626c6564000000000000000000000000000081525060200191505060405180910390fd5b614aea6001600760009054906101000a900463ffffffff1663ffffffff166158dd90919063ffffffff16565b600860008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160146101000a81548163ffffffff021916908363ffffffff1602179055506000600c614b6c6001614b5b612614565b60ff1661543790919063ffffffff16565b81548110614b7657fe5b9060005260206000200160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690506000600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160009054906101000a900461ffff16905080600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160006101000a81548161ffff021916908361ffff160217905550600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160006101000a81549061ffff021916905581600c8261ffff1681548110614cba57fe5b9060005260206000200160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600c805480614d0d57fe5b6001900381819060005260206000200160006101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690559055600015158373ffffffffffffffffffffffffffffffffffffffff167f18dd09695e4fbdae8d1a5edb11221eb04564269c29a089b9753a6535c54ba92e60405160405180910390a3505050565b600080828401905083811015614e0c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b8091505092915050565b614e1f82616263565b15614e92576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f6f7261636c6520616c726561647920656e61626c65640000000000000000000081525060200191505060405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161415614f35576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260158152602001807f63616e6e6f74207365742061646d696e20746f2030000000000000000000000081525060200191505060405180910390fd5b600073ffffffffffffffffffffffffffffffffffffffff16600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16148061505f57508073ffffffffffffffffffffffffffffffffffffffff16600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16145b6150d1576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601c8152602001807f6f776e65722063616e6e6f74206f76657277726974652061646d696e0000000081525060200191505060405180910390fd5b6150da826162cd565b600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160106101000a81548163ffffffff021916908363ffffffff16021790555063ffffffff600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160146101000a81548163ffffffff021916908363ffffffff160217905550600c80549050600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160006101000a81548161ffff021916908361ffff160217905550600c829080600181540180825580915050600190039060005260206000200160009091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555080600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060020160026101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600115158273ffffffffffffffffffffffffffffffffffffffff167f18dd09695e4fbdae8d1a5edb11221eb04564269c29a089b9753a6535c54ba92e60405160405180910390a38073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff167f0c5055390645c15a4be9a21b3f8d019153dcb4a0c125685da6eb84048e2fe90460405160405180910390a35050565b6000826fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff161115615426576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f536166654d6174683a207375627472616374696f6e206f766572666c6f77000081525060200191505060405180910390fd5b600082840390508091505092915050565b6000828211156154af576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f536166654d6174683a207375627472616374696f6e206f766572666c6f77000081525060200191505060405180910390fd5b600082840390508091505092915050565b600080600960008463ffffffff1663ffffffff16815260200190815260200160002060010160009054906101000a900467ffffffffffffffff1667ffffffffffffffff1611156155305761551382615ffd565b8015615529575060006155268484613b9e565b51145b9050615553565b61553a838361638a565b80156155505750600061554d8484613b9e565b51145b90505b92915050565b6000806000806000806000806000600960008063ffffffff16815260200190815260200160002090506000600860008c73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002090506000600760009054906101000a900463ffffffff1663ffffffff168260000160189054906101000a900463ffffffff1663ffffffff16148061561f575061561d600760009054906101000a900463ffffffff16615ffd565b155b905061563c600760009054906101000a900463ffffffff16615c55565b80156156455750805b156156cb576156766001600760009054906101000a900463ffffffff1663ffffffff166158dd90919063ffffffff16565b9950600960008b63ffffffff1663ffffffff1681526020019081526020016000209250600460009054906101000a90046fffffffffffffffffffffffffffffffff1693506156c48c8b61638a565b9a5061574e565b600760009054906101000a900463ffffffff169950600960008b63ffffffff1663ffffffff1681526020019081526020016000209250600a60008b63ffffffff1663ffffffff168152602001908152602001600020600101600c9054906101000a90046fffffffffffffffffffffffffffffffff16935061574b8a615ffd565b9a505b600061575a8d8c613b9e565b51146157655760009a505b8a8a83600101548560010160009054906101000a900467ffffffffffffffff16600a60008f63ffffffff1663ffffffff16815260200190815260200160002060010160089054906101000a900463ffffffff16600d60000160009054906101000a90046fffffffffffffffffffffffffffffffff166157e2612614565b8a8363ffffffff1693509a509a509a509a509a509a509a509a50505050919395975091939597565b600080600960008463ffffffff1663ffffffff16815260200190815260200160002060010160009054906101000a900467ffffffffffffffff1690506000600a60008563ffffffff1663ffffffff16815260200190815260200160002060010160089054906101000a900463ffffffff16905060008267ffffffffffffffff1611801561589d575060008163ffffffff16115b80156158d45750426158c88263ffffffff168467ffffffffffffffff1661641f90919063ffffffff16565b67ffffffffffffffff16105b92505050919050565b60008082840190508363ffffffff168163ffffffff161015615967576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b8091505092915050565b61597a81615cb1565b61598357615b2c565b6000600b60003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160059054906101000a900463ffffffff1663ffffffff169050600b60003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160019054906101000a900463ffffffff1663ffffffff1681018263ffffffff161180615a4e5750600081145b615ac0576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260138152602001807f6d7573742064656c61792072657175657374730000000000000000000000000081525060200191505060405180910390fd5b615ac982615cf4565b81600b60003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160056101000a81548163ffffffff021916908363ffffffff160217905550505b50565b600063ffffffff80168211159050919050565b60008163ffffffff16615b6560018563ffffffff166158dd90919063ffffffff16565b63ffffffff16148015615bb857506000600960008463ffffffff1663ffffffff16815260200190815260200160002060010160089054906101000a900467ffffffffffffffff1667ffffffffffffffff16145b905092915050565b60008263ffffffff168263ffffffff161115615c44576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601e8152602001807f536166654d6174683a207375627472616374696f6e206f766572666c6f77000081525060200191505060405180910390fd5b600082840390508091505092915050565b600080600960008463ffffffff1663ffffffff16815260200190815260200160002060010160089054906101000a900467ffffffffffffffff1667ffffffffffffffff161180615caa5750615ca98261580a565b5b9050919050565b6000615cdf6001600760009054906101000a900463ffffffff1663ffffffff166158dd90919063ffffffff16565b63ffffffff168263ffffffff16149050919050565b615d16615d1160018363ffffffff16615bc090919063ffffffff16565b6164bb565b80600760006101000a81548163ffffffff021916908363ffffffff160217905550615d3f61704d565b6040518060a00160405280600067ffffffffffffffff81118015615d6257600080fd5b50604051908082528060200260200182016040528015615d915781602001602082028036833780820191505090505b508152602001600460109054906101000a900463ffffffff1663ffffffff168152602001600460149054906101000a900463ffffffff1663ffffffff1681526020016004601c9054906101000a900463ffffffff1663ffffffff168152602001600460009054906101000a90046fffffffffffffffffffffffffffffffff166fffffffffffffffffffffffffffffffff16815250905080600a60008463ffffffff1663ffffffff1681526020019081526020016000206000820151816000019080519060200190615e639291906170a0565b5060208201518160010160006101000a81548163ffffffff021916908363ffffffff16021790555060408201518160010160046101000a81548163ffffffff021916908363ffffffff16021790555060608201518160010160086101000a81548163ffffffff021916908363ffffffff160217905550608082015181600101600c6101000a8154816fffffffffffffffffffffffffffffffff02191690836fffffffffffffffffffffffffffffffff16021790555090505042600960008463ffffffff1663ffffffff16815260200190815260200160002060010160006101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055503373ffffffffffffffffffffffffffffffffffffffff168263ffffffff167f0109fc6f55cf40689f02fbaad7af7fe7bbac8a3d2186600afc7d3e10cac60271600960008663ffffffff1663ffffffff16815260200190815260200160002060010160009054906101000a900467ffffffffffffffff16604051808267ffffffffffffffff16815260200191505060405180910390a35050565b600080600a60008463ffffffff1663ffffffff16815260200190815260200160002060010160009054906101000a900463ffffffff1663ffffffff1614159050919050565b600081516000106160bb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260168152602001807f6c697374206d757374206e6f7420626520656d7074790000000000000000000081525060200191505060405180910390fd5b6000825190506000600282816160cd57fe5b0490506000600283816160dc57fe5b061415616117576000806160fa86600060018703600187038761668f565b809250819350505061610c828261677c565b94505050505061612c565b6161278460006001850384616819565b925050505b919050565b6000808284019050836fffffffffffffffffffffffffffffffff16816fffffffffffffffffffffffffffffffff1610156161d3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b8091505092915050565b6000808314156161f0576000905061625d565b600082840290508284828161620157fe5b0414616258576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260218152602001806171346021913960400191505060405180910390fd5b809150505b92915050565b600063ffffffff8016600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160149054906101000a900463ffffffff1663ffffffff16149050919050565b600080600760009054906101000a900463ffffffff16905060008163ffffffff16141580156163595750600860008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060000160149054906101000a900463ffffffff1663ffffffff168163ffffffff16145b156163675780915050616385565b61638160018263ffffffff166158dd90919063ffffffff16565b9150505b919050565b600080600860008573ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001908152602001600020600001601c9054906101000a900463ffffffff1663ffffffff169050600460189054906101000a900463ffffffff1663ffffffff1681018363ffffffff1611806164165750600081145b91505092915050565b60008082840190508367ffffffffffffffff168167ffffffffffffffff1610156164b1576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252601b8152602001807f536166654d6174683a206164646974696f6e206f766572666c6f77000000000081525060200191505060405180910390fd5b8091505092915050565b6164c48161580a565b6164cd5761668c565b60006164e960018363ffffffff16615bc090919063ffffffff16565b9050600960008263ffffffff1663ffffffff16815260200190815260200160002060000154600960008463ffffffff1663ffffffff16815260200190815260200160002060000181905550600960008263ffffffff1663ffffffff16815260200190815260200160002060010160109054906101000a900463ffffffff16600960008463ffffffff1663ffffffff16815260200190815260200160002060010160106101000a81548163ffffffff021916908363ffffffff16021790555042600960008463ffffffff1663ffffffff16815260200190815260200160002060010160086101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550600a60008363ffffffff1663ffffffff16815260200190815260200160002060008082016000616620919061702c565b6001820160006101000a81549063ffffffff02191690556001820160046101000a81549063ffffffff02191690556001820160086101000a81549063ffffffff021916905560018201600c6101000a8154906fffffffffffffffffffffffffffffffff02191690555050505b50565b60008082841061669e57600080fd5b8386111580156166ae5750848411155b6166b757600080fd5b8286111580156166c75750848311155b6166d057600080fd5b5b60011561677157600786860310156166f9576166f087878787876168b3565b91509150616772565b6000616706888888616e27565b90508084116167175780955061676b565b8481101561672a5760018101965061676a565b80851115801561673957508381105b61673f57fe5b61674b88888388616819565b925061675c88600183018887616819565b915082829250925050616772565b5b506166d1565b5b9550959350505050565b6000808312801561678d5750600082135b806167a457506000831380156167a35750600082125b5b156167c45760026167b58484616f1e565b816167bc57fe5b059050616813565b600060028084816167d157fe5b07600286816167dc57fe5b0701816167e557fe5b05905061680f616809600286816167f857fe5b056002868161680357fe5b05616f1e565b82616f1e565b9150505b92915050565b60008184111561682857600080fd5b8282111561683557600080fd5b5b82841015616894576007848403101561686957600061685886868686876168b3565b8092508193505050819150506168ab565b6000616876868686616e27565b90508083116168875780935061688e565b6001810194505b50616836565b8484815181106168a057fe5b602002602001015190505b949350505050565b6000806000866001870103905060008860008901815181106168d157fe5b6020026020010151905060008260011061690b577f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff616923565b8960018a018151811061691a57fe5b60200260200101515b9050600083600210616955577f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61696d565b8a60028b018151811061696457fe5b60200260200101515b905060008460031061699f577f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6169b7565b8b60038c01815181106169ae57fe5b60200260200101515b90506000856004106169e9577f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff616a01565b8c60048d01815181106169f857fe5b60200260200101515b9050600086600510616a33577f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff616a4b565b8d60058e0181518110616a4257fe5b60200260200101515b9050600087600610616a7d577f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff616a95565b8e60068f0181518110616a8c57fe5b60200260200101515b905085871315616aaa57858780975081985050505b83851315616abd57838580955081965050505b81831315616ad057818380935081945050505b84871315616ae357848780965081985050505b83861315616af657838680955081975050505b80831315616b0957808380925081945050505b84861315616b1c57848680965081975050505b80821315616b2f57808280925081935050505b82871315616b4257828780945081985050505b81861315616b5557818680935081975050505b80851315616b6857808580925081965050505b82861315616b7b57828680945081975050505b80841315616b8e57808480925081955050505b82851315616ba157828580945081965050505b81841315616bb457818480935081955050505b82841315616bc757828480945081955050505b60008e8d0390506000811415616bdf57879a50616cb9565b6001811415616bf057869a50616cb8565b6002811415616c0157859a50616cb7565b6003811415616c1257849a50616cb6565b6004811415616c2357839a50616cb5565b6005811415616c3457829a50616cb4565b6006811415616c4557819a50616cb3565b6040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260108152602001807f6b31206f7574206f6620626f756e64730000000000000000000000000000000081525060200191505060405180910390fd5b5b5b5b5b5b5b60008f8d0390508c8e1415616cdd578b8c9b509b5050505050505050505050616e1d565b6000811415616cfb578b899b509b5050505050505050505050616e1d565b6001811415616d19578b889b509b5050505050505050505050616e1d565b6002811415616d37578b879b509b5050505050505050505050616e1d565b6003811415616d55578b869b509b5050505050505050505050616e1d565b6004811415616d73578b859b509b5050505050505050505050616e1d565b6005811415616d91578b849b509b5050505050505050505050616e1d565b6006811415616daf578b839b509b5050505050505050505050616e1d565b6040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260108152602001807f6b32206f7574206f6620626f756e64730000000000000000000000000000000081525060200191505060405180910390fd5b9550959350505050565b60008084600284860181616e3757fe5b0481518110616e4257fe5b602002602001015190506001840393506001830192505b600115616f15575b60018401935080858581518110616e7457fe5b602002602001015112616e61575b60018303925080858481518110616e9557fe5b602002602001015113616e825782841015616f0757848381518110616eb657fe5b6020026020010151858581518110616eca57fe5b6020026020010151868681518110616ede57fe5b60200260200101878681518110616ef157fe5b6020026020010182815250828152505050616f10565b82915050616f17565b616e59565b505b9392505050565b600080828401905060008312158015616f375750838112155b80616f4d5750600083128015616f4c57508381125b5b616fa2576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260218152602001806171136021913960400191505060405180910390fd5b8091505092915050565b604051806040016040528060006fffffffffffffffffffffffffffffffff16815260200160006fffffffffffffffffffffffffffffffff1681525090565b604051806080016040528060008152602001600067ffffffffffffffff168152602001600067ffffffffffffffff168152602001600063ffffffff1681525090565b508054600082559060005260206000209081019061704a91906170ed565b50565b6040518060a0016040528060608152602001600063ffffffff168152602001600063ffffffff168152602001600063ffffffff16815260200160006fffffffffffffffffffffffffffffffff1681525090565b8280548282559060005260206000209081019282156170dc579160200282015b828111156170db5782518255916020019190600101906170c0565b5b5090506170e991906170ed565b5090565b61710f91905b8082111561710b5760008160009055506001016170f3565b5090565b9056fe5369676e6564536166654d6174683a206164646974696f6e206f766572666c6f77536166654d6174683a206d756c7469706c69636174696f6e206f766572666c6f77a26469706673582212209f5fb30d2b83c38bb8aa60d7754d0c6b819315773b533a105e963f7ea276f7b664736f6c63430006060033536166654d6174683a206d756c7469706c69636174696f6e206f766572666c6f77"

// DeployFluxAggregator deploys a new Ethereum contract, binding an instance of FluxAggregator to it.
func DeployFluxAggregator(auth *bind.TransactOpts, backend bind.ContractBackend, _link common.Address, _paymentAmount *big.Int, _timeout uint32, _validator common.Address, _minSubmissionValue *big.Int, _maxSubmissionValue *big.Int, _decimals uint8, _description string) (common.Address, *types.Transaction, *FluxAggregator, error) {
	parsed, err := abi.JSON(strings.NewReader(FluxAggregatorABI))
	if err != nil {
		return common.Address{}, nil, nil, err
	}

	address, tx, contract, err := bind.DeployContract(auth, parsed, common.FromHex(FluxAggregatorBin), backend, _link, _paymentAmount, _timeout, _validator, _minSubmissionValue, _maxSubmissionValue, _decimals, _description)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &FluxAggregator{FluxAggregatorCaller: FluxAggregatorCaller{contract: contract}, FluxAggregatorTransactor: FluxAggregatorTransactor{contract: contract}, FluxAggregatorFilterer: FluxAggregatorFilterer{contract: contract}}, nil
}

// FluxAggregator is an auto generated Go binding around an Ethereum contract.
type FluxAggregator struct {
	FluxAggregatorCaller     // Read-only binding to the contract
	FluxAggregatorTransactor // Write-only binding to the contract
	FluxAggregatorFilterer   // Log filterer for contract events
}

// FluxAggregatorCaller is an auto generated read-only Go binding around an Ethereum contract.
type FluxAggregatorCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FluxAggregatorTransactor is an auto generated write-only Go binding around an Ethereum contract.
type FluxAggregatorTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FluxAggregatorFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type FluxAggregatorFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// FluxAggregatorSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type FluxAggregatorSession struct {
	Contract     *FluxAggregator   // Generic contract binding to set the session for
	CallOpts     bind.CallOpts     // Call options to use throughout this session
	TransactOpts bind.TransactOpts // Transaction auth options to use throughout this session
}

// FluxAggregatorCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type FluxAggregatorCallerSession struct {
	Contract *FluxAggregatorCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts         // Call options to use throughout this session
}

// FluxAggregatorTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type FluxAggregatorTransactorSession struct {
	Contract     *FluxAggregatorTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts         // Transaction auth options to use throughout this session
}

// FluxAggregatorRaw is an auto generated low-level Go binding around an Ethereum contract.
type FluxAggregatorRaw struct {
	Contract *FluxAggregator // Generic contract binding to access the raw methods on
}

// FluxAggregatorCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type FluxAggregatorCallerRaw struct {
	Contract *FluxAggregatorCaller // Generic read-only contract binding to access the raw methods on
}

// FluxAggregatorTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type FluxAggregatorTransactorRaw struct {
	Contract *FluxAggregatorTransactor // Generic write-only contract binding to access the raw methods on
}

// NewFluxAggregator creates a new instance of FluxAggregator, bound to a specific deployed contract.
func NewFluxAggregator(address common.Address, backend bind.ContractBackend) (*FluxAggregator, error) {
	contract, err := bindFluxAggregator(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &FluxAggregator{FluxAggregatorCaller: FluxAggregatorCaller{contract: contract}, FluxAggregatorTransactor: FluxAggregatorTransactor{contract: contract}, FluxAggregatorFilterer: FluxAggregatorFilterer{contract: contract}}, nil
}

// NewFluxAggregatorCaller creates a new read-only instance of FluxAggregator, bound to a specific deployed contract.
func NewFluxAggregatorCaller(address common.Address, caller bind.ContractCaller) (*FluxAggregatorCaller, error) {
	contract, err := bindFluxAggregator(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorCaller{contract: contract}, nil
}

// NewFluxAggregatorTransactor creates a new write-only instance of FluxAggregator, bound to a specific deployed contract.
func NewFluxAggregatorTransactor(address common.Address, transactor bind.ContractTransactor) (*FluxAggregatorTransactor, error) {
	contract, err := bindFluxAggregator(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorTransactor{contract: contract}, nil
}

// NewFluxAggregatorFilterer creates a new log filterer instance of FluxAggregator, bound to a specific deployed contract.
func NewFluxAggregatorFilterer(address common.Address, filterer bind.ContractFilterer) (*FluxAggregatorFilterer, error) {
	contract, err := bindFluxAggregator(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorFilterer{contract: contract}, nil
}

// bindFluxAggregator binds a generic wrapper to an already deployed contract.
func bindFluxAggregator(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := abi.JSON(strings.NewReader(FluxAggregatorABI))
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_FluxAggregator *FluxAggregatorRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _FluxAggregator.Contract.FluxAggregatorCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_FluxAggregator *FluxAggregatorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FluxAggregator.Contract.FluxAggregatorTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_FluxAggregator *FluxAggregatorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _FluxAggregator.Contract.FluxAggregatorTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_FluxAggregator *FluxAggregatorCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _FluxAggregator.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_FluxAggregator *FluxAggregatorTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FluxAggregator.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_FluxAggregator *FluxAggregatorTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _FluxAggregator.Contract.contract.Transact(opts, method, params...)
}

// AllocatedFunds is a free data retrieval call binding the contract method 0xd4cc54e4.
//
// Solidity: function allocatedFunds() view returns(uint128)
func (_FluxAggregator *FluxAggregatorCaller) AllocatedFunds(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "allocatedFunds")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// AllocatedFunds is a free data retrieval call binding the contract method 0xd4cc54e4.
//
// Solidity: function allocatedFunds() view returns(uint128)
func (_FluxAggregator *FluxAggregatorSession) AllocatedFunds() (*big.Int, error) {
	return _FluxAggregator.Contract.AllocatedFunds(&_FluxAggregator.CallOpts)
}

// AllocatedFunds is a free data retrieval call binding the contract method 0xd4cc54e4.
//
// Solidity: function allocatedFunds() view returns(uint128)
func (_FluxAggregator *FluxAggregatorCallerSession) AllocatedFunds() (*big.Int, error) {
	return _FluxAggregator.Contract.AllocatedFunds(&_FluxAggregator.CallOpts)
}

// AvailableFunds is a free data retrieval call binding the contract method 0x46fcff4c.
//
// Solidity: function availableFunds() view returns(uint128)
func (_FluxAggregator *FluxAggregatorCaller) AvailableFunds(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "availableFunds")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// AvailableFunds is a free data retrieval call binding the contract method 0x46fcff4c.
//
// Solidity: function availableFunds() view returns(uint128)
func (_FluxAggregator *FluxAggregatorSession) AvailableFunds() (*big.Int, error) {
	return _FluxAggregator.Contract.AvailableFunds(&_FluxAggregator.CallOpts)
}

// AvailableFunds is a free data retrieval call binding the contract method 0x46fcff4c.
//
// Solidity: function availableFunds() view returns(uint128)
func (_FluxAggregator *FluxAggregatorCallerSession) AvailableFunds() (*big.Int, error) {
	return _FluxAggregator.Contract.AvailableFunds(&_FluxAggregator.CallOpts)
}

// Decimals is a free data retrieval call binding the contract method 0x313ce567.
//
// Solidity: function decimals() view returns(uint8)
func (_FluxAggregator *FluxAggregatorCaller) Decimals(opts *bind.CallOpts) (uint8, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "decimals")

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// Decimals is a free data retrieval call binding the contract method 0x313ce567.
//
// Solidity: function decimals() view returns(uint8)
func (_FluxAggregator *FluxAggregatorSession) Decimals() (uint8, error) {
	return _FluxAggregator.Contract.Decimals(&_FluxAggregator.CallOpts)
}

// Decimals is a free data retrieval call binding the contract method 0x313ce567.
//
// Solidity: function decimals() view returns(uint8)
func (_FluxAggregator *FluxAggregatorCallerSession) Decimals() (uint8, error) {
	return _FluxAggregator.Contract.Decimals(&_FluxAggregator.CallOpts)
}

// Description is a free data retrieval call binding the contract method 0x7284e416.
//
// Solidity: function description() view returns(string)
func (_FluxAggregator *FluxAggregatorCaller) Description(opts *bind.CallOpts) (string, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "description")

	if err != nil {
		return *new(string), err
	}

	out0 := *abi.ConvertType(out[0], new(string)).(*string)

	return out0, err

}

// Description is a free data retrieval call binding the contract method 0x7284e416.
//
// Solidity: function description() view returns(string)
func (_FluxAggregator *FluxAggregatorSession) Description() (string, error) {
	return _FluxAggregator.Contract.Description(&_FluxAggregator.CallOpts)
}

// Description is a free data retrieval call binding the contract method 0x7284e416.
//
// Solidity: function description() view returns(string)
func (_FluxAggregator *FluxAggregatorCallerSession) Description() (string, error) {
	return _FluxAggregator.Contract.Description(&_FluxAggregator.CallOpts)
}

// GetAdmin is a free data retrieval call binding the contract method 0x64efb22b.
//
// Solidity: function getAdmin(address _oracle) view returns(address)
func (_FluxAggregator *FluxAggregatorCaller) GetAdmin(opts *bind.CallOpts, _oracle common.Address) (common.Address, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "getAdmin", _oracle)

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// GetAdmin is a free data retrieval call binding the contract method 0x64efb22b.
//
// Solidity: function getAdmin(address _oracle) view returns(address)
func (_FluxAggregator *FluxAggregatorSession) GetAdmin(_oracle common.Address) (common.Address, error) {
	return _FluxAggregator.Contract.GetAdmin(&_FluxAggregator.CallOpts, _oracle)
}

// GetAdmin is a free data retrieval call binding the contract method 0x64efb22b.
//
// Solidity: function getAdmin(address _oracle) view returns(address)
func (_FluxAggregator *FluxAggregatorCallerSession) GetAdmin(_oracle common.Address) (common.Address, error) {
	return _FluxAggregator.Contract.GetAdmin(&_FluxAggregator.CallOpts, _oracle)
}

// GetAnswer is a free data retrieval call binding the contract method 0xb5ab58dc.
//
// Solidity: function getAnswer(uint256 _roundId) view returns(int256)
func (_FluxAggregator *FluxAggregatorCaller) GetAnswer(opts *bind.CallOpts, _roundId *big.Int) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "getAnswer", _roundId)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// GetAnswer is a free data retrieval call binding the contract method 0xb5ab58dc.
//
// Solidity: function getAnswer(uint256 _roundId) view returns(int256)
func (_FluxAggregator *FluxAggregatorSession) GetAnswer(_roundId *big.Int) (*big.Int, error) {
	return _FluxAggregator.Contract.GetAnswer(&_FluxAggregator.CallOpts, _roundId)
}

// GetAnswer is a free data retrieval call binding the contract method 0xb5ab58dc.
//
// Solidity: function getAnswer(uint256 _roundId) view returns(int256)
func (_FluxAggregator *FluxAggregatorCallerSession) GetAnswer(_roundId *big.Int) (*big.Int, error) {
	return _FluxAggregator.Contract.GetAnswer(&_FluxAggregator.CallOpts, _roundId)
}

// GetOracles is a free data retrieval call binding the contract method 0x40884c52.
//
// Solidity: function getOracles() view returns(address[])
func (_FluxAggregator *FluxAggregatorCaller) GetOracles(opts *bind.CallOpts) ([]common.Address, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "getOracles")

	if err != nil {
		return *new([]common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new([]common.Address)).(*[]common.Address)

	return out0, err

}

// GetOracles is a free data retrieval call binding the contract method 0x40884c52.
//
// Solidity: function getOracles() view returns(address[])
func (_FluxAggregator *FluxAggregatorSession) GetOracles() ([]common.Address, error) {
	return _FluxAggregator.Contract.GetOracles(&_FluxAggregator.CallOpts)
}

// GetOracles is a free data retrieval call binding the contract method 0x40884c52.
//
// Solidity: function getOracles() view returns(address[])
func (_FluxAggregator *FluxAggregatorCallerSession) GetOracles() ([]common.Address, error) {
	return _FluxAggregator.Contract.GetOracles(&_FluxAggregator.CallOpts)
}

// GetRoundData is a free data retrieval call binding the contract method 0x9a6fc8f5.
//
// Solidity: function getRoundData(uint80 _roundId) view returns(uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
func (_FluxAggregator *FluxAggregatorCaller) GetRoundData(opts *bind.CallOpts, _roundId *big.Int) (struct {
	RoundId         *big.Int
	Answer          *big.Int
	StartedAt       *big.Int
	UpdatedAt       *big.Int
	AnsweredInRound *big.Int
}, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "getRoundData", _roundId)

	outstruct := new(struct {
		RoundId         *big.Int
		Answer          *big.Int
		StartedAt       *big.Int
		UpdatedAt       *big.Int
		AnsweredInRound *big.Int
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.RoundId = *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	outstruct.Answer = *abi.ConvertType(out[1], new(*big.Int)).(**big.Int)
	outstruct.StartedAt = *abi.ConvertType(out[2], new(*big.Int)).(**big.Int)
	outstruct.UpdatedAt = *abi.ConvertType(out[3], new(*big.Int)).(**big.Int)
	outstruct.AnsweredInRound = *abi.ConvertType(out[4], new(*big.Int)).(**big.Int)

	return *outstruct, err

}

// GetRoundData is a free data retrieval call binding the contract method 0x9a6fc8f5.
//
// Solidity: function getRoundData(uint80 _roundId) view returns(uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
func (_FluxAggregator *FluxAggregatorSession) GetRoundData(_roundId *big.Int) (struct {
	RoundId         *big.Int
	Answer          *big.Int
	StartedAt       *big.Int
	UpdatedAt       *big.Int
	AnsweredInRound *big.Int
}, error) {
	return _FluxAggregator.Contract.GetRoundData(&_FluxAggregator.CallOpts, _roundId)
}

// GetRoundData is a free data retrieval call binding the contract method 0x9a6fc8f5.
//
// Solidity: function getRoundData(uint80 _roundId) view returns(uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
func (_FluxAggregator *FluxAggregatorCallerSession) GetRoundData(_roundId *big.Int) (struct {
	RoundId         *big.Int
	Answer          *big.Int
	StartedAt       *big.Int
	UpdatedAt       *big.Int
	AnsweredInRound *big.Int
}, error) {
	return _FluxAggregator.Contract.GetRoundData(&_FluxAggregator.CallOpts, _roundId)
}

// GetTimestamp is a free data retrieval call binding the contract method 0xb633620c.
//
// Solidity: function getTimestamp(uint256 _roundId) view returns(uint256)
func (_FluxAggregator *FluxAggregatorCaller) GetTimestamp(opts *bind.CallOpts, _roundId *big.Int) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "getTimestamp", _roundId)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// GetTimestamp is a free data retrieval call binding the contract method 0xb633620c.
//
// Solidity: function getTimestamp(uint256 _roundId) view returns(uint256)
func (_FluxAggregator *FluxAggregatorSession) GetTimestamp(_roundId *big.Int) (*big.Int, error) {
	return _FluxAggregator.Contract.GetTimestamp(&_FluxAggregator.CallOpts, _roundId)
}

// GetTimestamp is a free data retrieval call binding the contract method 0xb633620c.
//
// Solidity: function getTimestamp(uint256 _roundId) view returns(uint256)
func (_FluxAggregator *FluxAggregatorCallerSession) GetTimestamp(_roundId *big.Int) (*big.Int, error) {
	return _FluxAggregator.Contract.GetTimestamp(&_FluxAggregator.CallOpts, _roundId)
}

// LatestAnswer is a free data retrieval call binding the contract method 0x50d25bcd.
//
// Solidity: function latestAnswer() view returns(int256)
func (_FluxAggregator *FluxAggregatorCaller) LatestAnswer(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "latestAnswer")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LatestAnswer is a free data retrieval call binding the contract method 0x50d25bcd.
//
// Solidity: function latestAnswer() view returns(int256)
func (_FluxAggregator *FluxAggregatorSession) LatestAnswer() (*big.Int, error) {
	return _FluxAggregator.Contract.LatestAnswer(&_FluxAggregator.CallOpts)
}

// LatestAnswer is a free data retrieval call binding the contract method 0x50d25bcd.
//
// Solidity: function latestAnswer() view returns(int256)
func (_FluxAggregator *FluxAggregatorCallerSession) LatestAnswer() (*big.Int, error) {
	return _FluxAggregator.Contract.LatestAnswer(&_FluxAggregator.CallOpts)
}

// LatestRound is a free data retrieval call binding the contract method 0x668a0f02.
//
// Solidity: function latestRound() view returns(uint256)
func (_FluxAggregator *FluxAggregatorCaller) LatestRound(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "latestRound")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LatestRound is a free data retrieval call binding the contract method 0x668a0f02.
//
// Solidity: function latestRound() view returns(uint256)
func (_FluxAggregator *FluxAggregatorSession) LatestRound() (*big.Int, error) {
	return _FluxAggregator.Contract.LatestRound(&_FluxAggregator.CallOpts)
}

// LatestRound is a free data retrieval call binding the contract method 0x668a0f02.
//
// Solidity: function latestRound() view returns(uint256)
func (_FluxAggregator *FluxAggregatorCallerSession) LatestRound() (*big.Int, error) {
	return _FluxAggregator.Contract.LatestRound(&_FluxAggregator.CallOpts)
}

// LatestRoundData is a free data retrieval call binding the contract method 0xfeaf968c.
//
// Solidity: function latestRoundData() view returns(uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
func (_FluxAggregator *FluxAggregatorCaller) LatestRoundData(opts *bind.CallOpts) (struct {
	RoundId         *big.Int
	Answer          *big.Int
	StartedAt       *big.Int
	UpdatedAt       *big.Int
	AnsweredInRound *big.Int
}, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "latestRoundData")

	outstruct := new(struct {
		RoundId         *big.Int
		Answer          *big.Int
		StartedAt       *big.Int
		UpdatedAt       *big.Int
		AnsweredInRound *big.Int
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.RoundId = *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	outstruct.Answer = *abi.ConvertType(out[1], new(*big.Int)).(**big.Int)
	outstruct.StartedAt = *abi.ConvertType(out[2], new(*big.Int)).(**big.Int)
	outstruct.UpdatedAt = *abi.ConvertType(out[3], new(*big.Int)).(**big.Int)
	outstruct.AnsweredInRound = *abi.ConvertType(out[4], new(*big.Int)).(**big.Int)

	return *outstruct, err

}

// LatestRoundData is a free data retrieval call binding the contract method 0xfeaf968c.
//
// Solidity: function latestRoundData() view returns(uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
func (_FluxAggregator *FluxAggregatorSession) LatestRoundData() (struct {
	RoundId         *big.Int
	Answer          *big.Int
	StartedAt       *big.Int
	UpdatedAt       *big.Int
	AnsweredInRound *big.Int
}, error) {
	return _FluxAggregator.Contract.LatestRoundData(&_FluxAggregator.CallOpts)
}

// LatestRoundData is a free data retrieval call binding the contract method 0xfeaf968c.
//
// Solidity: function latestRoundData() view returns(uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
func (_FluxAggregator *FluxAggregatorCallerSession) LatestRoundData() (struct {
	RoundId         *big.Int
	Answer          *big.Int
	StartedAt       *big.Int
	UpdatedAt       *big.Int
	AnsweredInRound *big.Int
}, error) {
	return _FluxAggregator.Contract.LatestRoundData(&_FluxAggregator.CallOpts)
}

// LatestTimestamp is a free data retrieval call binding the contract method 0x8205bf6a.
//
// Solidity: function latestTimestamp() view returns(uint256)
func (_FluxAggregator *FluxAggregatorCaller) LatestTimestamp(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "latestTimestamp")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LatestTimestamp is a free data retrieval call binding the contract method 0x8205bf6a.
//
// Solidity: function latestTimestamp() view returns(uint256)
func (_FluxAggregator *FluxAggregatorSession) LatestTimestamp() (*big.Int, error) {
	return _FluxAggregator.Contract.LatestTimestamp(&_FluxAggregator.CallOpts)
}

// LatestTimestamp is a free data retrieval call binding the contract method 0x8205bf6a.
//
// Solidity: function latestTimestamp() view returns(uint256)
func (_FluxAggregator *FluxAggregatorCallerSession) LatestTimestamp() (*big.Int, error) {
	return _FluxAggregator.Contract.LatestTimestamp(&_FluxAggregator.CallOpts)
}

// LinkToken is a free data retrieval call binding the contract method 0x57970e93.
//
// Solidity: function linkToken() view returns(address)
func (_FluxAggregator *FluxAggregatorCaller) LinkToken(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "linkToken")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// LinkToken is a free data retrieval call binding the contract method 0x57970e93.
//
// Solidity: function linkToken() view returns(address)
func (_FluxAggregator *FluxAggregatorSession) LinkToken() (common.Address, error) {
	return _FluxAggregator.Contract.LinkToken(&_FluxAggregator.CallOpts)
}

// LinkToken is a free data retrieval call binding the contract method 0x57970e93.
//
// Solidity: function linkToken() view returns(address)
func (_FluxAggregator *FluxAggregatorCallerSession) LinkToken() (common.Address, error) {
	return _FluxAggregator.Contract.LinkToken(&_FluxAggregator.CallOpts)
}

// MaxSubmissionCount is a free data retrieval call binding the contract method 0x58609e44.
//
// Solidity: function maxSubmissionCount() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCaller) MaxSubmissionCount(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "maxSubmissionCount")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// MaxSubmissionCount is a free data retrieval call binding the contract method 0x58609e44.
//
// Solidity: function maxSubmissionCount() view returns(uint32)
func (_FluxAggregator *FluxAggregatorSession) MaxSubmissionCount() (uint32, error) {
	return _FluxAggregator.Contract.MaxSubmissionCount(&_FluxAggregator.CallOpts)
}

// MaxSubmissionCount is a free data retrieval call binding the contract method 0x58609e44.
//
// Solidity: function maxSubmissionCount() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCallerSession) MaxSubmissionCount() (uint32, error) {
	return _FluxAggregator.Contract.MaxSubmissionCount(&_FluxAggregator.CallOpts)
}

// MaxSubmissionValue is a free data retrieval call binding the contract method 0x23ca2903.
//
// Solidity: function maxSubmissionValue() view returns(int256)
func (_FluxAggregator *FluxAggregatorCaller) MaxSubmissionValue(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "maxSubmissionValue")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// MaxSubmissionValue is a free data retrieval call binding the contract method 0x23ca2903.
//
// Solidity: function maxSubmissionValue() view returns(int256)
func (_FluxAggregator *FluxAggregatorSession) MaxSubmissionValue() (*big.Int, error) {
	return _FluxAggregator.Contract.MaxSubmissionValue(&_FluxAggregator.CallOpts)
}

// MaxSubmissionValue is a free data retrieval call binding the contract method 0x23ca2903.
//
// Solidity: function maxSubmissionValue() view returns(int256)
func (_FluxAggregator *FluxAggregatorCallerSession) MaxSubmissionValue() (*big.Int, error) {
	return _FluxAggregator.Contract.MaxSubmissionValue(&_FluxAggregator.CallOpts)
}

// MinSubmissionCount is a free data retrieval call binding the contract method 0xc9374500.
//
// Solidity: function minSubmissionCount() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCaller) MinSubmissionCount(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "minSubmissionCount")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// MinSubmissionCount is a free data retrieval call binding the contract method 0xc9374500.
//
// Solidity: function minSubmissionCount() view returns(uint32)
func (_FluxAggregator *FluxAggregatorSession) MinSubmissionCount() (uint32, error) {
	return _FluxAggregator.Contract.MinSubmissionCount(&_FluxAggregator.CallOpts)
}

// MinSubmissionCount is a free data retrieval call binding the contract method 0xc9374500.
//
// Solidity: function minSubmissionCount() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCallerSession) MinSubmissionCount() (uint32, error) {
	return _FluxAggregator.Contract.MinSubmissionCount(&_FluxAggregator.CallOpts)
}

// MinSubmissionValue is a free data retrieval call binding the contract method 0x7c2b0b21.
//
// Solidity: function minSubmissionValue() view returns(int256)
func (_FluxAggregator *FluxAggregatorCaller) MinSubmissionValue(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "minSubmissionValue")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// MinSubmissionValue is a free data retrieval call binding the contract method 0x7c2b0b21.
//
// Solidity: function minSubmissionValue() view returns(int256)
func (_FluxAggregator *FluxAggregatorSession) MinSubmissionValue() (*big.Int, error) {
	return _FluxAggregator.Contract.MinSubmissionValue(&_FluxAggregator.CallOpts)
}

// MinSubmissionValue is a free data retrieval call binding the contract method 0x7c2b0b21.
//
// Solidity: function minSubmissionValue() view returns(int256)
func (_FluxAggregator *FluxAggregatorCallerSession) MinSubmissionValue() (*big.Int, error) {
	return _FluxAggregator.Contract.MinSubmissionValue(&_FluxAggregator.CallOpts)
}

// OracleCount is a free data retrieval call binding the contract method 0x613d8fcc.
//
// Solidity: function oracleCount() view returns(uint8)
func (_FluxAggregator *FluxAggregatorCaller) OracleCount(opts *bind.CallOpts) (uint8, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "oracleCount")

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// OracleCount is a free data retrieval call binding the contract method 0x613d8fcc.
//
// Solidity: function oracleCount() view returns(uint8)
func (_FluxAggregator *FluxAggregatorSession) OracleCount() (uint8, error) {
	return _FluxAggregator.Contract.OracleCount(&_FluxAggregator.CallOpts)
}

// OracleCount is a free data retrieval call binding the contract method 0x613d8fcc.
//
// Solidity: function oracleCount() view returns(uint8)
func (_FluxAggregator *FluxAggregatorCallerSession) OracleCount() (uint8, error) {
	return _FluxAggregator.Contract.OracleCount(&_FluxAggregator.CallOpts)
}

// OracleRoundState is a free data retrieval call binding the contract method 0x88aa80e7.
//
// Solidity: function oracleRoundState(address _oracle, uint32 _queriedRoundId) view returns(bool _eligibleToSubmit, uint32 _roundId, int256 _latestSubmission, uint64 _startedAt, uint64 _timeout, uint128 _availableFunds, uint8 _oracleCount, uint128 _paymentAmount)
func (_FluxAggregator *FluxAggregatorCaller) OracleRoundState(opts *bind.CallOpts, _oracle common.Address, _queriedRoundId uint32) (struct {
	EligibleToSubmit bool
	RoundId          uint32
	LatestSubmission *big.Int
	StartedAt        uint64
	Timeout          uint64
	AvailableFunds   *big.Int
	OracleCount      uint8
	PaymentAmount    *big.Int
}, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "oracleRoundState", _oracle, _queriedRoundId)

	outstruct := new(struct {
		EligibleToSubmit bool
		RoundId          uint32
		LatestSubmission *big.Int
		StartedAt        uint64
		Timeout          uint64
		AvailableFunds   *big.Int
		OracleCount      uint8
		PaymentAmount    *big.Int
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.EligibleToSubmit = *abi.ConvertType(out[0], new(bool)).(*bool)
	outstruct.RoundId = *abi.ConvertType(out[1], new(uint32)).(*uint32)
	outstruct.LatestSubmission = *abi.ConvertType(out[2], new(*big.Int)).(**big.Int)
	outstruct.StartedAt = *abi.ConvertType(out[3], new(uint64)).(*uint64)
	outstruct.Timeout = *abi.ConvertType(out[4], new(uint64)).(*uint64)
	outstruct.AvailableFunds = *abi.ConvertType(out[5], new(*big.Int)).(**big.Int)
	outstruct.OracleCount = *abi.ConvertType(out[6], new(uint8)).(*uint8)
	outstruct.PaymentAmount = *abi.ConvertType(out[7], new(*big.Int)).(**big.Int)

	return *outstruct, err

}

// OracleRoundState is a free data retrieval call binding the contract method 0x88aa80e7.
//
// Solidity: function oracleRoundState(address _oracle, uint32 _queriedRoundId) view returns(bool _eligibleToSubmit, uint32 _roundId, int256 _latestSubmission, uint64 _startedAt, uint64 _timeout, uint128 _availableFunds, uint8 _oracleCount, uint128 _paymentAmount)
func (_FluxAggregator *FluxAggregatorSession) OracleRoundState(_oracle common.Address, _queriedRoundId uint32) (struct {
	EligibleToSubmit bool
	RoundId          uint32
	LatestSubmission *big.Int
	StartedAt        uint64
	Timeout          uint64
	AvailableFunds   *big.Int
	OracleCount      uint8
	PaymentAmount    *big.Int
}, error) {
	return _FluxAggregator.Contract.OracleRoundState(&_FluxAggregator.CallOpts, _oracle, _queriedRoundId)
}

// OracleRoundState is a free data retrieval call binding the contract method 0x88aa80e7.
//
// Solidity: function oracleRoundState(address _oracle, uint32 _queriedRoundId) view returns(bool _eligibleToSubmit, uint32 _roundId, int256 _latestSubmission, uint64 _startedAt, uint64 _timeout, uint128 _availableFunds, uint8 _oracleCount, uint128 _paymentAmount)
func (_FluxAggregator *FluxAggregatorCallerSession) OracleRoundState(_oracle common.Address, _queriedRoundId uint32) (struct {
	EligibleToSubmit bool
	RoundId          uint32
	LatestSubmission *big.Int
	StartedAt        uint64
	Timeout          uint64
	AvailableFunds   *big.Int
	OracleCount      uint8
	PaymentAmount    *big.Int
}, error) {
	return _FluxAggregator.Contract.OracleRoundState(&_FluxAggregator.CallOpts, _oracle, _queriedRoundId)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_FluxAggregator *FluxAggregatorCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_FluxAggregator *FluxAggregatorSession) Owner() (common.Address, error) {
	return _FluxAggregator.Contract.Owner(&_FluxAggregator.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_FluxAggregator *FluxAggregatorCallerSession) Owner() (common.Address, error) {
	return _FluxAggregator.Contract.Owner(&_FluxAggregator.CallOpts)
}

// PaymentAmount is a free data retrieval call binding the contract method 0xc35905c6.
//
// Solidity: function paymentAmount() view returns(uint128)
func (_FluxAggregator *FluxAggregatorCaller) PaymentAmount(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "paymentAmount")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// PaymentAmount is a free data retrieval call binding the contract method 0xc35905c6.
//
// Solidity: function paymentAmount() view returns(uint128)
func (_FluxAggregator *FluxAggregatorSession) PaymentAmount() (*big.Int, error) {
	return _FluxAggregator.Contract.PaymentAmount(&_FluxAggregator.CallOpts)
}

// PaymentAmount is a free data retrieval call binding the contract method 0xc35905c6.
//
// Solidity: function paymentAmount() view returns(uint128)
func (_FluxAggregator *FluxAggregatorCallerSession) PaymentAmount() (*big.Int, error) {
	return _FluxAggregator.Contract.PaymentAmount(&_FluxAggregator.CallOpts)
}

// RestartDelay is a free data retrieval call binding the contract method 0x357ebb02.
//
// Solidity: function restartDelay() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCaller) RestartDelay(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "restartDelay")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// RestartDelay is a free data retrieval call binding the contract method 0x357ebb02.
//
// Solidity: function restartDelay() view returns(uint32)
func (_FluxAggregator *FluxAggregatorSession) RestartDelay() (uint32, error) {
	return _FluxAggregator.Contract.RestartDelay(&_FluxAggregator.CallOpts)
}

// RestartDelay is a free data retrieval call binding the contract method 0x357ebb02.
//
// Solidity: function restartDelay() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCallerSession) RestartDelay() (uint32, error) {
	return _FluxAggregator.Contract.RestartDelay(&_FluxAggregator.CallOpts)
}

// Timeout is a free data retrieval call binding the contract method 0x70dea79a.
//
// Solidity: function timeout() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCaller) Timeout(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "timeout")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// Timeout is a free data retrieval call binding the contract method 0x70dea79a.
//
// Solidity: function timeout() view returns(uint32)
func (_FluxAggregator *FluxAggregatorSession) Timeout() (uint32, error) {
	return _FluxAggregator.Contract.Timeout(&_FluxAggregator.CallOpts)
}

// Timeout is a free data retrieval call binding the contract method 0x70dea79a.
//
// Solidity: function timeout() view returns(uint32)
func (_FluxAggregator *FluxAggregatorCallerSession) Timeout() (uint32, error) {
	return _FluxAggregator.Contract.Timeout(&_FluxAggregator.CallOpts)
}

// Validator is a free data retrieval call binding the contract method 0x3a5381b5.
//
// Solidity: function validator() view returns(address)
func (_FluxAggregator *FluxAggregatorCaller) Validator(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "validator")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Validator is a free data retrieval call binding the contract method 0x3a5381b5.
//
// Solidity: function validator() view returns(address)
func (_FluxAggregator *FluxAggregatorSession) Validator() (common.Address, error) {
	return _FluxAggregator.Contract.Validator(&_FluxAggregator.CallOpts)
}

// Validator is a free data retrieval call binding the contract method 0x3a5381b5.
//
// Solidity: function validator() view returns(address)
func (_FluxAggregator *FluxAggregatorCallerSession) Validator() (common.Address, error) {
	return _FluxAggregator.Contract.Validator(&_FluxAggregator.CallOpts)
}

// Version is a free data retrieval call binding the contract method 0x54fd4d50.
//
// Solidity: function version() view returns(uint256)
func (_FluxAggregator *FluxAggregatorCaller) Version(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "version")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Version is a free data retrieval call binding the contract method 0x54fd4d50.
//
// Solidity: function version() view returns(uint256)
func (_FluxAggregator *FluxAggregatorSession) Version() (*big.Int, error) {
	return _FluxAggregator.Contract.Version(&_FluxAggregator.CallOpts)
}

// Version is a free data retrieval call binding the contract method 0x54fd4d50.
//
// Solidity: function version() view returns(uint256)
func (_FluxAggregator *FluxAggregatorCallerSession) Version() (*big.Int, error) {
	return _FluxAggregator.Contract.Version(&_FluxAggregator.CallOpts)
}

// WithdrawablePayment is a free data retrieval call binding the contract method 0xe2e40317.
//
// Solidity: function withdrawablePayment(address _oracle) view returns(uint256)
func (_FluxAggregator *FluxAggregatorCaller) WithdrawablePayment(opts *bind.CallOpts, _oracle common.Address) (*big.Int, error) {
	var out []interface{}
	err := _FluxAggregator.contract.Call(opts, &out, "withdrawablePayment", _oracle)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// WithdrawablePayment is a free data retrieval call binding the contract method 0xe2e40317.
//
// Solidity: function withdrawablePayment(address _oracle) view returns(uint256)
func (_FluxAggregator *FluxAggregatorSession) WithdrawablePayment(_oracle common.Address) (*big.Int, error) {
	return _FluxAggregator.Contract.WithdrawablePayment(&_FluxAggregator.CallOpts, _oracle)
}

// WithdrawablePayment is a free data retrieval call binding the contract method 0xe2e40317.
//
// Solidity: function withdrawablePayment(address _oracle) view returns(uint256)
func (_FluxAggregator *FluxAggregatorCallerSession) WithdrawablePayment(_oracle common.Address) (*big.Int, error) {
	return _FluxAggregator.Contract.WithdrawablePayment(&_FluxAggregator.CallOpts, _oracle)
}

// AcceptAdmin is a paid mutator transaction binding the contract method 0x628806ef.
//
// Solidity: function acceptAdmin(address _oracle) returns()
func (_FluxAggregator *FluxAggregatorTransactor) AcceptAdmin(opts *bind.TransactOpts, _oracle common.Address) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "acceptAdmin", _oracle)
}

// AcceptAdmin is a paid mutator transaction binding the contract method 0x628806ef.
//
// Solidity: function acceptAdmin(address _oracle) returns()
func (_FluxAggregator *FluxAggregatorSession) AcceptAdmin(_oracle common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.AcceptAdmin(&_FluxAggregator.TransactOpts, _oracle)
}

// AcceptAdmin is a paid mutator transaction binding the contract method 0x628806ef.
//
// Solidity: function acceptAdmin(address _oracle) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) AcceptAdmin(_oracle common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.AcceptAdmin(&_FluxAggregator.TransactOpts, _oracle)
}

// AcceptOwnership is a paid mutator transaction binding the contract method 0x79ba5097.
//
// Solidity: function acceptOwnership() returns()
func (_FluxAggregator *FluxAggregatorTransactor) AcceptOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "acceptOwnership")
}

// AcceptOwnership is a paid mutator transaction binding the contract method 0x79ba5097.
//
// Solidity: function acceptOwnership() returns()
func (_FluxAggregator *FluxAggregatorSession) AcceptOwnership() (*types.Transaction, error) {
	return _FluxAggregator.Contract.AcceptOwnership(&_FluxAggregator.TransactOpts)
}

// AcceptOwnership is a paid mutator transaction binding the contract method 0x79ba5097.
//
// Solidity: function acceptOwnership() returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) AcceptOwnership() (*types.Transaction, error) {
	return _FluxAggregator.Contract.AcceptOwnership(&_FluxAggregator.TransactOpts)
}

// ChangeOracles is a paid mutator transaction binding the contract method 0x3969c20f.
//
// Solidity: function changeOracles(address[] _removed, address[] _added, address[] _addedAdmins, uint32 _minSubmissions, uint32 _maxSubmissions, uint32 _restartDelay) returns()
func (_FluxAggregator *FluxAggregatorTransactor) ChangeOracles(opts *bind.TransactOpts, _removed []common.Address, _added []common.Address, _addedAdmins []common.Address, _minSubmissions uint32, _maxSubmissions uint32, _restartDelay uint32) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "changeOracles", _removed, _added, _addedAdmins, _minSubmissions, _maxSubmissions, _restartDelay)
}

// ChangeOracles is a paid mutator transaction binding the contract method 0x3969c20f.
//
// Solidity: function changeOracles(address[] _removed, address[] _added, address[] _addedAdmins, uint32 _minSubmissions, uint32 _maxSubmissions, uint32 _restartDelay) returns()
func (_FluxAggregator *FluxAggregatorSession) ChangeOracles(_removed []common.Address, _added []common.Address, _addedAdmins []common.Address, _minSubmissions uint32, _maxSubmissions uint32, _restartDelay uint32) (*types.Transaction, error) {
	return _FluxAggregator.Contract.ChangeOracles(&_FluxAggregator.TransactOpts, _removed, _added, _addedAdmins, _minSubmissions, _maxSubmissions, _restartDelay)
}

// ChangeOracles is a paid mutator transaction binding the contract method 0x3969c20f.
//
// Solidity: function changeOracles(address[] _removed, address[] _added, address[] _addedAdmins, uint32 _minSubmissions, uint32 _maxSubmissions, uint32 _restartDelay) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) ChangeOracles(_removed []common.Address, _added []common.Address, _addedAdmins []common.Address, _minSubmissions uint32, _maxSubmissions uint32, _restartDelay uint32) (*types.Transaction, error) {
	return _FluxAggregator.Contract.ChangeOracles(&_FluxAggregator.TransactOpts, _removed, _added, _addedAdmins, _minSubmissions, _maxSubmissions, _restartDelay)
}

// OnTokenTransfer is a paid mutator transaction binding the contract method 0xa4c0ed36.
//
// Solidity: function onTokenTransfer(address , uint256 , bytes _data) returns()
func (_FluxAggregator *FluxAggregatorTransactor) OnTokenTransfer(opts *bind.TransactOpts, arg0 common.Address, arg1 *big.Int, _data []byte) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "onTokenTransfer", arg0, arg1, _data)
}

// OnTokenTransfer is a paid mutator transaction binding the contract method 0xa4c0ed36.
//
// Solidity: function onTokenTransfer(address , uint256 , bytes _data) returns()
func (_FluxAggregator *FluxAggregatorSession) OnTokenTransfer(arg0 common.Address, arg1 *big.Int, _data []byte) (*types.Transaction, error) {
	return _FluxAggregator.Contract.OnTokenTransfer(&_FluxAggregator.TransactOpts, arg0, arg1, _data)
}

// OnTokenTransfer is a paid mutator transaction binding the contract method 0xa4c0ed36.
//
// Solidity: function onTokenTransfer(address , uint256 , bytes _data) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) OnTokenTransfer(arg0 common.Address, arg1 *big.Int, _data []byte) (*types.Transaction, error) {
	return _FluxAggregator.Contract.OnTokenTransfer(&_FluxAggregator.TransactOpts, arg0, arg1, _data)
}

// RequestNewRound is a paid mutator transaction binding the contract method 0x98e5b12a.
//
// Solidity: function requestNewRound() returns(uint80)
func (_FluxAggregator *FluxAggregatorTransactor) RequestNewRound(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "requestNewRound")
}

// RequestNewRound is a paid mutator transaction binding the contract method 0x98e5b12a.
//
// Solidity: function requestNewRound() returns(uint80)
func (_FluxAggregator *FluxAggregatorSession) RequestNewRound() (*types.Transaction, error) {
	return _FluxAggregator.Contract.RequestNewRound(&_FluxAggregator.TransactOpts)
}

// RequestNewRound is a paid mutator transaction binding the contract method 0x98e5b12a.
//
// Solidity: function requestNewRound() returns(uint80)
func (_FluxAggregator *FluxAggregatorTransactorSession) RequestNewRound() (*types.Transaction, error) {
	return _FluxAggregator.Contract.RequestNewRound(&_FluxAggregator.TransactOpts)
}

// SetRequesterPermissions is a paid mutator transaction binding the contract method 0x20ed0275.
//
// Solidity: function setRequesterPermissions(address _requester, bool _authorized, uint32 _delay) returns()
func (_FluxAggregator *FluxAggregatorTransactor) SetRequesterPermissions(opts *bind.TransactOpts, _requester common.Address, _authorized bool, _delay uint32) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "setRequesterPermissions", _requester, _authorized, _delay)
}

// SetRequesterPermissions is a paid mutator transaction binding the contract method 0x20ed0275.
//
// Solidity: function setRequesterPermissions(address _requester, bool _authorized, uint32 _delay) returns()
func (_FluxAggregator *FluxAggregatorSession) SetRequesterPermissions(_requester common.Address, _authorized bool, _delay uint32) (*types.Transaction, error) {
	return _FluxAggregator.Contract.SetRequesterPermissions(&_FluxAggregator.TransactOpts, _requester, _authorized, _delay)
}

// SetRequesterPermissions is a paid mutator transaction binding the contract method 0x20ed0275.
//
// Solidity: function setRequesterPermissions(address _requester, bool _authorized, uint32 _delay) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) SetRequesterPermissions(_requester common.Address, _authorized bool, _delay uint32) (*types.Transaction, error) {
	return _FluxAggregator.Contract.SetRequesterPermissions(&_FluxAggregator.TransactOpts, _requester, _authorized, _delay)
}

// SetValidator is a paid mutator transaction binding the contract method 0x1327d3d8.
//
// Solidity: function setValidator(address _newValidator) returns()
func (_FluxAggregator *FluxAggregatorTransactor) SetValidator(opts *bind.TransactOpts, _newValidator common.Address) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "setValidator", _newValidator)
}

// SetValidator is a paid mutator transaction binding the contract method 0x1327d3d8.
//
// Solidity: function setValidator(address _newValidator) returns()
func (_FluxAggregator *FluxAggregatorSession) SetValidator(_newValidator common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.SetValidator(&_FluxAggregator.TransactOpts, _newValidator)
}

// SetValidator is a paid mutator transaction binding the contract method 0x1327d3d8.
//
// Solidity: function setValidator(address _newValidator) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) SetValidator(_newValidator common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.SetValidator(&_FluxAggregator.TransactOpts, _newValidator)
}

// Submit is a paid mutator transaction binding the contract method 0x202ee0ed.
//
// Solidity: function submit(uint256 _roundId, int256 _submission) returns()
func (_FluxAggregator *FluxAggregatorTransactor) Submit(opts *bind.TransactOpts, _roundId *big.Int, _submission *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "submit", _roundId, _submission)
}

// Submit is a paid mutator transaction binding the contract method 0x202ee0ed.
//
// Solidity: function submit(uint256 _roundId, int256 _submission) returns()
func (_FluxAggregator *FluxAggregatorSession) Submit(_roundId *big.Int, _submission *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.Contract.Submit(&_FluxAggregator.TransactOpts, _roundId, _submission)
}

// Submit is a paid mutator transaction binding the contract method 0x202ee0ed.
//
// Solidity: function submit(uint256 _roundId, int256 _submission) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) Submit(_roundId *big.Int, _submission *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.Contract.Submit(&_FluxAggregator.TransactOpts, _roundId, _submission)
}

// TransferAdmin is a paid mutator transaction binding the contract method 0xe9ee6eeb.
//
// Solidity: function transferAdmin(address _oracle, address _newAdmin) returns()
func (_FluxAggregator *FluxAggregatorTransactor) TransferAdmin(opts *bind.TransactOpts, _oracle common.Address, _newAdmin common.Address) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "transferAdmin", _oracle, _newAdmin)
}

// TransferAdmin is a paid mutator transaction binding the contract method 0xe9ee6eeb.
//
// Solidity: function transferAdmin(address _oracle, address _newAdmin) returns()
func (_FluxAggregator *FluxAggregatorSession) TransferAdmin(_oracle common.Address, _newAdmin common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.TransferAdmin(&_FluxAggregator.TransactOpts, _oracle, _newAdmin)
}

// TransferAdmin is a paid mutator transaction binding the contract method 0xe9ee6eeb.
//
// Solidity: function transferAdmin(address _oracle, address _newAdmin) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) TransferAdmin(_oracle common.Address, _newAdmin common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.TransferAdmin(&_FluxAggregator.TransactOpts, _oracle, _newAdmin)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _to) returns()
func (_FluxAggregator *FluxAggregatorTransactor) TransferOwnership(opts *bind.TransactOpts, _to common.Address) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "transferOwnership", _to)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _to) returns()
func (_FluxAggregator *FluxAggregatorSession) TransferOwnership(_to common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.TransferOwnership(&_FluxAggregator.TransactOpts, _to)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address _to) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) TransferOwnership(_to common.Address) (*types.Transaction, error) {
	return _FluxAggregator.Contract.TransferOwnership(&_FluxAggregator.TransactOpts, _to)
}

// UpdateAvailableFunds is a paid mutator transaction binding the contract method 0x4f8fc3b5.
//
// Solidity: function updateAvailableFunds() returns()
func (_FluxAggregator *FluxAggregatorTransactor) UpdateAvailableFunds(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "updateAvailableFunds")
}

// UpdateAvailableFunds is a paid mutator transaction binding the contract method 0x4f8fc3b5.
//
// Solidity: function updateAvailableFunds() returns()
func (_FluxAggregator *FluxAggregatorSession) UpdateAvailableFunds() (*types.Transaction, error) {
	return _FluxAggregator.Contract.UpdateAvailableFunds(&_FluxAggregator.TransactOpts)
}

// UpdateAvailableFunds is a paid mutator transaction binding the contract method 0x4f8fc3b5.
//
// Solidity: function updateAvailableFunds() returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) UpdateAvailableFunds() (*types.Transaction, error) {
	return _FluxAggregator.Contract.UpdateAvailableFunds(&_FluxAggregator.TransactOpts)
}

// UpdateFutureRounds is a paid mutator transaction binding the contract method 0x38aa4c72.
//
// Solidity: function updateFutureRounds(uint128 _paymentAmount, uint32 _minSubmissions, uint32 _maxSubmissions, uint32 _restartDelay, uint32 _timeout) returns()
func (_FluxAggregator *FluxAggregatorTransactor) UpdateFutureRounds(opts *bind.TransactOpts, _paymentAmount *big.Int, _minSubmissions uint32, _maxSubmissions uint32, _restartDelay uint32, _timeout uint32) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "updateFutureRounds", _paymentAmount, _minSubmissions, _maxSubmissions, _restartDelay, _timeout)
}

// UpdateFutureRounds is a paid mutator transaction binding the contract method 0x38aa4c72.
//
// Solidity: function updateFutureRounds(uint128 _paymentAmount, uint32 _minSubmissions, uint32 _maxSubmissions, uint32 _restartDelay, uint32 _timeout) returns()
func (_FluxAggregator *FluxAggregatorSession) UpdateFutureRounds(_paymentAmount *big.Int, _minSubmissions uint32, _maxSubmissions uint32, _restartDelay uint32, _timeout uint32) (*types.Transaction, error) {
	return _FluxAggregator.Contract.UpdateFutureRounds(&_FluxAggregator.TransactOpts, _paymentAmount, _minSubmissions, _maxSubmissions, _restartDelay, _timeout)
}

// UpdateFutureRounds is a paid mutator transaction binding the contract method 0x38aa4c72.
//
// Solidity: function updateFutureRounds(uint128 _paymentAmount, uint32 _minSubmissions, uint32 _maxSubmissions, uint32 _restartDelay, uint32 _timeout) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) UpdateFutureRounds(_paymentAmount *big.Int, _minSubmissions uint32, _maxSubmissions uint32, _restartDelay uint32, _timeout uint32) (*types.Transaction, error) {
	return _FluxAggregator.Contract.UpdateFutureRounds(&_FluxAggregator.TransactOpts, _paymentAmount, _minSubmissions, _maxSubmissions, _restartDelay, _timeout)
}

// WithdrawFunds is a paid mutator transaction binding the contract method 0xc1075329.
//
// Solidity: function withdrawFunds(address _recipient, uint256 _amount) returns()
func (_FluxAggregator *FluxAggregatorTransactor) WithdrawFunds(opts *bind.TransactOpts, _recipient common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "withdrawFunds", _recipient, _amount)
}

// WithdrawFunds is a paid mutator transaction binding the contract method 0xc1075329.
//
// Solidity: function withdrawFunds(address _recipient, uint256 _amount) returns()
func (_FluxAggregator *FluxAggregatorSession) WithdrawFunds(_recipient common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.Contract.WithdrawFunds(&_FluxAggregator.TransactOpts, _recipient, _amount)
}

// WithdrawFunds is a paid mutator transaction binding the contract method 0xc1075329.
//
// Solidity: function withdrawFunds(address _recipient, uint256 _amount) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) WithdrawFunds(_recipient common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.Contract.WithdrawFunds(&_FluxAggregator.TransactOpts, _recipient, _amount)
}

// WithdrawPayment is a paid mutator transaction binding the contract method 0x3d3d7714.
//
// Solidity: function withdrawPayment(address _oracle, address _recipient, uint256 _amount) returns()
func (_FluxAggregator *FluxAggregatorTransactor) WithdrawPayment(opts *bind.TransactOpts, _oracle common.Address, _recipient common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.contract.Transact(opts, "withdrawPayment", _oracle, _recipient, _amount)
}

// WithdrawPayment is a paid mutator transaction binding the contract method 0x3d3d7714.
//
// Solidity: function withdrawPayment(address _oracle, address _recipient, uint256 _amount) returns()
func (_FluxAggregator *FluxAggregatorSession) WithdrawPayment(_oracle common.Address, _recipient common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.Contract.WithdrawPayment(&_FluxAggregator.TransactOpts, _oracle, _recipient, _amount)
}

// WithdrawPayment is a paid mutator transaction binding the contract method 0x3d3d7714.
//
// Solidity: function withdrawPayment(address _oracle, address _recipient, uint256 _amount) returns()
func (_FluxAggregator *FluxAggregatorTransactorSession) WithdrawPayment(_oracle common.Address, _recipient common.Address, _amount *big.Int) (*types.Transaction, error) {
	return _FluxAggregator.Contract.WithdrawPayment(&_FluxAggregator.TransactOpts, _oracle, _recipient, _amount)
}

// FluxAggregatorAnswerUpdatedIterator is returned from FilterAnswerUpdated and is used to iterate over the raw logs and unpacked data for AnswerUpdated events raised by the FluxAggregator contract.
type FluxAggregatorAnswerUpdatedIterator struct {
	Event *FluxAggregatorAnswerUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorAnswerUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorAnswerUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorAnswerUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorAnswerUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorAnswerUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorAnswerUpdated represents a AnswerUpdated event raised by the FluxAggregator contract.
type FluxAggregatorAnswerUpdated struct {
	Current   *big.Int
	RoundId   *big.Int
	UpdatedAt *big.Int
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterAnswerUpdated is a free log retrieval operation binding the contract event 0x0559884fd3a460db3073b7fc896cc77986f16e378210ded43186175bf646fc5f.
//
// Solidity: event AnswerUpdated(int256 indexed current, uint256 indexed roundId, uint256 updatedAt)
func (_FluxAggregator *FluxAggregatorFilterer) FilterAnswerUpdated(opts *bind.FilterOpts, current []*big.Int, roundId []*big.Int) (*FluxAggregatorAnswerUpdatedIterator, error) {

	var currentRule []interface{}
	for _, currentItem := range current {
		currentRule = append(currentRule, currentItem)
	}
	var roundIdRule []interface{}
	for _, roundIdItem := range roundId {
		roundIdRule = append(roundIdRule, roundIdItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "AnswerUpdated", currentRule, roundIdRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorAnswerUpdatedIterator{contract: _FluxAggregator.contract, event: "AnswerUpdated", logs: logs, sub: sub}, nil
}

// WatchAnswerUpdated is a free log subscription operation binding the contract event 0x0559884fd3a460db3073b7fc896cc77986f16e378210ded43186175bf646fc5f.
//
// Solidity: event AnswerUpdated(int256 indexed current, uint256 indexed roundId, uint256 updatedAt)
func (_FluxAggregator *FluxAggregatorFilterer) WatchAnswerUpdated(opts *bind.WatchOpts, sink chan<- *FluxAggregatorAnswerUpdated, current []*big.Int, roundId []*big.Int) (event.Subscription, error) {

	var currentRule []interface{}
	for _, currentItem := range current {
		currentRule = append(currentRule, currentItem)
	}
	var roundIdRule []interface{}
	for _, roundIdItem := range roundId {
		roundIdRule = append(roundIdRule, roundIdItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "AnswerUpdated", currentRule, roundIdRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorAnswerUpdated)
				if err := _FluxAggregator.contract.UnpackLog(event, "AnswerUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseAnswerUpdated is a log parse operation binding the contract event 0x0559884fd3a460db3073b7fc896cc77986f16e378210ded43186175bf646fc5f.
//
// Solidity: event AnswerUpdated(int256 indexed current, uint256 indexed roundId, uint256 updatedAt)
func (_FluxAggregator *FluxAggregatorFilterer) ParseAnswerUpdated(log types.Log) (*FluxAggregatorAnswerUpdated, error) {
	event := new(FluxAggregatorAnswerUpdated)
	if err := _FluxAggregator.contract.UnpackLog(event, "AnswerUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorAvailableFundsUpdatedIterator is returned from FilterAvailableFundsUpdated and is used to iterate over the raw logs and unpacked data for AvailableFundsUpdated events raised by the FluxAggregator contract.
type FluxAggregatorAvailableFundsUpdatedIterator struct {
	Event *FluxAggregatorAvailableFundsUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorAvailableFundsUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorAvailableFundsUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorAvailableFundsUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorAvailableFundsUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorAvailableFundsUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorAvailableFundsUpdated represents a AvailableFundsUpdated event raised by the FluxAggregator contract.
type FluxAggregatorAvailableFundsUpdated struct {
	Amount *big.Int
	Raw    types.Log // Blockchain specific contextual infos
}

// FilterAvailableFundsUpdated is a free log retrieval operation binding the contract event 0xfe25c73e3b9089fac37d55c4c7efcba6f04af04cebd2fc4d6d7dbb07e1e5234f.
//
// Solidity: event AvailableFundsUpdated(uint256 indexed amount)
func (_FluxAggregator *FluxAggregatorFilterer) FilterAvailableFundsUpdated(opts *bind.FilterOpts, amount []*big.Int) (*FluxAggregatorAvailableFundsUpdatedIterator, error) {

	var amountRule []interface{}
	for _, amountItem := range amount {
		amountRule = append(amountRule, amountItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "AvailableFundsUpdated", amountRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorAvailableFundsUpdatedIterator{contract: _FluxAggregator.contract, event: "AvailableFundsUpdated", logs: logs, sub: sub}, nil
}

// WatchAvailableFundsUpdated is a free log subscription operation binding the contract event 0xfe25c73e3b9089fac37d55c4c7efcba6f04af04cebd2fc4d6d7dbb07e1e5234f.
//
// Solidity: event AvailableFundsUpdated(uint256 indexed amount)
func (_FluxAggregator *FluxAggregatorFilterer) WatchAvailableFundsUpdated(opts *bind.WatchOpts, sink chan<- *FluxAggregatorAvailableFundsUpdated, amount []*big.Int) (event.Subscription, error) {

	var amountRule []interface{}
	for _, amountItem := range amount {
		amountRule = append(amountRule, amountItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "AvailableFundsUpdated", amountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorAvailableFundsUpdated)
				if err := _FluxAggregator.contract.UnpackLog(event, "AvailableFundsUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseAvailableFundsUpdated is a log parse operation binding the contract event 0xfe25c73e3b9089fac37d55c4c7efcba6f04af04cebd2fc4d6d7dbb07e1e5234f.
//
// Solidity: event AvailableFundsUpdated(uint256 indexed amount)
func (_FluxAggregator *FluxAggregatorFilterer) ParseAvailableFundsUpdated(log types.Log) (*FluxAggregatorAvailableFundsUpdated, error) {
	event := new(FluxAggregatorAvailableFundsUpdated)
	if err := _FluxAggregator.contract.UnpackLog(event, "AvailableFundsUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorNewRoundIterator is returned from FilterNewRound and is used to iterate over the raw logs and unpacked data for NewRound events raised by the FluxAggregator contract.
type FluxAggregatorNewRoundIterator struct {
	Event *FluxAggregatorNewRound // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorNewRoundIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorNewRound)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorNewRound)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorNewRoundIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorNewRoundIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorNewRound represents a NewRound event raised by the FluxAggregator contract.
type FluxAggregatorNewRound struct {
	RoundId   *big.Int
	StartedBy common.Address
	StartedAt *big.Int
	Raw       types.Log // Blockchain specific contextual infos
}

// FilterNewRound is a free log retrieval operation binding the contract event 0x0109fc6f55cf40689f02fbaad7af7fe7bbac8a3d2186600afc7d3e10cac60271.
//
// Solidity: event NewRound(uint256 indexed roundId, address indexed startedBy, uint256 startedAt)
func (_FluxAggregator *FluxAggregatorFilterer) FilterNewRound(opts *bind.FilterOpts, roundId []*big.Int, startedBy []common.Address) (*FluxAggregatorNewRoundIterator, error) {

	var roundIdRule []interface{}
	for _, roundIdItem := range roundId {
		roundIdRule = append(roundIdRule, roundIdItem)
	}
	var startedByRule []interface{}
	for _, startedByItem := range startedBy {
		startedByRule = append(startedByRule, startedByItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "NewRound", roundIdRule, startedByRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorNewRoundIterator{contract: _FluxAggregator.contract, event: "NewRound", logs: logs, sub: sub}, nil
}

// WatchNewRound is a free log subscription operation binding the contract event 0x0109fc6f55cf40689f02fbaad7af7fe7bbac8a3d2186600afc7d3e10cac60271.
//
// Solidity: event NewRound(uint256 indexed roundId, address indexed startedBy, uint256 startedAt)
func (_FluxAggregator *FluxAggregatorFilterer) WatchNewRound(opts *bind.WatchOpts, sink chan<- *FluxAggregatorNewRound, roundId []*big.Int, startedBy []common.Address) (event.Subscription, error) {

	var roundIdRule []interface{}
	for _, roundIdItem := range roundId {
		roundIdRule = append(roundIdRule, roundIdItem)
	}
	var startedByRule []interface{}
	for _, startedByItem := range startedBy {
		startedByRule = append(startedByRule, startedByItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "NewRound", roundIdRule, startedByRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorNewRound)
				if err := _FluxAggregator.contract.UnpackLog(event, "NewRound", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseNewRound is a log parse operation binding the contract event 0x0109fc6f55cf40689f02fbaad7af7fe7bbac8a3d2186600afc7d3e10cac60271.
//
// Solidity: event NewRound(uint256 indexed roundId, address indexed startedBy, uint256 startedAt)
func (_FluxAggregator *FluxAggregatorFilterer) ParseNewRound(log types.Log) (*FluxAggregatorNewRound, error) {
	event := new(FluxAggregatorNewRound)
	if err := _FluxAggregator.contract.UnpackLog(event, "NewRound", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorOracleAdminUpdateRequestedIterator is returned from FilterOracleAdminUpdateRequested and is used to iterate over the raw logs and unpacked data for OracleAdminUpdateRequested events raised by the FluxAggregator contract.
type FluxAggregatorOracleAdminUpdateRequestedIterator struct {
	Event *FluxAggregatorOracleAdminUpdateRequested // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorOracleAdminUpdateRequestedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorOracleAdminUpdateRequested)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorOracleAdminUpdateRequested)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorOracleAdminUpdateRequestedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorOracleAdminUpdateRequestedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorOracleAdminUpdateRequested represents a OracleAdminUpdateRequested event raised by the FluxAggregator contract.
type FluxAggregatorOracleAdminUpdateRequested struct {
	Oracle   common.Address
	Admin    common.Address
	NewAdmin common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterOracleAdminUpdateRequested is a free log retrieval operation binding the contract event 0xb79bf2e89c2d70dde91d2991fb1ea69b7e478061ad7c04ed5b02b96bc52b8104.
//
// Solidity: event OracleAdminUpdateRequested(address indexed oracle, address admin, address newAdmin)
func (_FluxAggregator *FluxAggregatorFilterer) FilterOracleAdminUpdateRequested(opts *bind.FilterOpts, oracle []common.Address) (*FluxAggregatorOracleAdminUpdateRequestedIterator, error) {

	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "OracleAdminUpdateRequested", oracleRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorOracleAdminUpdateRequestedIterator{contract: _FluxAggregator.contract, event: "OracleAdminUpdateRequested", logs: logs, sub: sub}, nil
}

// WatchOracleAdminUpdateRequested is a free log subscription operation binding the contract event 0xb79bf2e89c2d70dde91d2991fb1ea69b7e478061ad7c04ed5b02b96bc52b8104.
//
// Solidity: event OracleAdminUpdateRequested(address indexed oracle, address admin, address newAdmin)
func (_FluxAggregator *FluxAggregatorFilterer) WatchOracleAdminUpdateRequested(opts *bind.WatchOpts, sink chan<- *FluxAggregatorOracleAdminUpdateRequested, oracle []common.Address) (event.Subscription, error) {

	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "OracleAdminUpdateRequested", oracleRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorOracleAdminUpdateRequested)
				if err := _FluxAggregator.contract.UnpackLog(event, "OracleAdminUpdateRequested", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOracleAdminUpdateRequested is a log parse operation binding the contract event 0xb79bf2e89c2d70dde91d2991fb1ea69b7e478061ad7c04ed5b02b96bc52b8104.
//
// Solidity: event OracleAdminUpdateRequested(address indexed oracle, address admin, address newAdmin)
func (_FluxAggregator *FluxAggregatorFilterer) ParseOracleAdminUpdateRequested(log types.Log) (*FluxAggregatorOracleAdminUpdateRequested, error) {
	event := new(FluxAggregatorOracleAdminUpdateRequested)
	if err := _FluxAggregator.contract.UnpackLog(event, "OracleAdminUpdateRequested", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorOracleAdminUpdatedIterator is returned from FilterOracleAdminUpdated and is used to iterate over the raw logs and unpacked data for OracleAdminUpdated events raised by the FluxAggregator contract.
type FluxAggregatorOracleAdminUpdatedIterator struct {
	Event *FluxAggregatorOracleAdminUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorOracleAdminUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorOracleAdminUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorOracleAdminUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorOracleAdminUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorOracleAdminUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorOracleAdminUpdated represents a OracleAdminUpdated event raised by the FluxAggregator contract.
type FluxAggregatorOracleAdminUpdated struct {
	Oracle   common.Address
	NewAdmin common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterOracleAdminUpdated is a free log retrieval operation binding the contract event 0x0c5055390645c15a4be9a21b3f8d019153dcb4a0c125685da6eb84048e2fe904.
//
// Solidity: event OracleAdminUpdated(address indexed oracle, address indexed newAdmin)
func (_FluxAggregator *FluxAggregatorFilterer) FilterOracleAdminUpdated(opts *bind.FilterOpts, oracle []common.Address, newAdmin []common.Address) (*FluxAggregatorOracleAdminUpdatedIterator, error) {

	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}
	var newAdminRule []interface{}
	for _, newAdminItem := range newAdmin {
		newAdminRule = append(newAdminRule, newAdminItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "OracleAdminUpdated", oracleRule, newAdminRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorOracleAdminUpdatedIterator{contract: _FluxAggregator.contract, event: "OracleAdminUpdated", logs: logs, sub: sub}, nil
}

// WatchOracleAdminUpdated is a free log subscription operation binding the contract event 0x0c5055390645c15a4be9a21b3f8d019153dcb4a0c125685da6eb84048e2fe904.
//
// Solidity: event OracleAdminUpdated(address indexed oracle, address indexed newAdmin)
func (_FluxAggregator *FluxAggregatorFilterer) WatchOracleAdminUpdated(opts *bind.WatchOpts, sink chan<- *FluxAggregatorOracleAdminUpdated, oracle []common.Address, newAdmin []common.Address) (event.Subscription, error) {

	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}
	var newAdminRule []interface{}
	for _, newAdminItem := range newAdmin {
		newAdminRule = append(newAdminRule, newAdminItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "OracleAdminUpdated", oracleRule, newAdminRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorOracleAdminUpdated)
				if err := _FluxAggregator.contract.UnpackLog(event, "OracleAdminUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOracleAdminUpdated is a log parse operation binding the contract event 0x0c5055390645c15a4be9a21b3f8d019153dcb4a0c125685da6eb84048e2fe904.
//
// Solidity: event OracleAdminUpdated(address indexed oracle, address indexed newAdmin)
func (_FluxAggregator *FluxAggregatorFilterer) ParseOracleAdminUpdated(log types.Log) (*FluxAggregatorOracleAdminUpdated, error) {
	event := new(FluxAggregatorOracleAdminUpdated)
	if err := _FluxAggregator.contract.UnpackLog(event, "OracleAdminUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorOraclePermissionsUpdatedIterator is returned from FilterOraclePermissionsUpdated and is used to iterate over the raw logs and unpacked data for OraclePermissionsUpdated events raised by the FluxAggregator contract.
type FluxAggregatorOraclePermissionsUpdatedIterator struct {
	Event *FluxAggregatorOraclePermissionsUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorOraclePermissionsUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorOraclePermissionsUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorOraclePermissionsUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorOraclePermissionsUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorOraclePermissionsUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorOraclePermissionsUpdated represents a OraclePermissionsUpdated event raised by the FluxAggregator contract.
type FluxAggregatorOraclePermissionsUpdated struct {
	Oracle      common.Address
	Whitelisted bool
	Raw         types.Log // Blockchain specific contextual infos
}

// FilterOraclePermissionsUpdated is a free log retrieval operation binding the contract event 0x18dd09695e4fbdae8d1a5edb11221eb04564269c29a089b9753a6535c54ba92e.
//
// Solidity: event OraclePermissionsUpdated(address indexed oracle, bool indexed whitelisted)
func (_FluxAggregator *FluxAggregatorFilterer) FilterOraclePermissionsUpdated(opts *bind.FilterOpts, oracle []common.Address, whitelisted []bool) (*FluxAggregatorOraclePermissionsUpdatedIterator, error) {

	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}
	var whitelistedRule []interface{}
	for _, whitelistedItem := range whitelisted {
		whitelistedRule = append(whitelistedRule, whitelistedItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "OraclePermissionsUpdated", oracleRule, whitelistedRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorOraclePermissionsUpdatedIterator{contract: _FluxAggregator.contract, event: "OraclePermissionsUpdated", logs: logs, sub: sub}, nil
}

// WatchOraclePermissionsUpdated is a free log subscription operation binding the contract event 0x18dd09695e4fbdae8d1a5edb11221eb04564269c29a089b9753a6535c54ba92e.
//
// Solidity: event OraclePermissionsUpdated(address indexed oracle, bool indexed whitelisted)
func (_FluxAggregator *FluxAggregatorFilterer) WatchOraclePermissionsUpdated(opts *bind.WatchOpts, sink chan<- *FluxAggregatorOraclePermissionsUpdated, oracle []common.Address, whitelisted []bool) (event.Subscription, error) {

	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}
	var whitelistedRule []interface{}
	for _, whitelistedItem := range whitelisted {
		whitelistedRule = append(whitelistedRule, whitelistedItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "OraclePermissionsUpdated", oracleRule, whitelistedRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorOraclePermissionsUpdated)
				if err := _FluxAggregator.contract.UnpackLog(event, "OraclePermissionsUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOraclePermissionsUpdated is a log parse operation binding the contract event 0x18dd09695e4fbdae8d1a5edb11221eb04564269c29a089b9753a6535c54ba92e.
//
// Solidity: event OraclePermissionsUpdated(address indexed oracle, bool indexed whitelisted)
func (_FluxAggregator *FluxAggregatorFilterer) ParseOraclePermissionsUpdated(log types.Log) (*FluxAggregatorOraclePermissionsUpdated, error) {
	event := new(FluxAggregatorOraclePermissionsUpdated)
	if err := _FluxAggregator.contract.UnpackLog(event, "OraclePermissionsUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorOwnershipTransferRequestedIterator is returned from FilterOwnershipTransferRequested and is used to iterate over the raw logs and unpacked data for OwnershipTransferRequested events raised by the FluxAggregator contract.
type FluxAggregatorOwnershipTransferRequestedIterator struct {
	Event *FluxAggregatorOwnershipTransferRequested // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorOwnershipTransferRequestedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorOwnershipTransferRequested)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorOwnershipTransferRequested)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorOwnershipTransferRequestedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorOwnershipTransferRequestedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorOwnershipTransferRequested represents a OwnershipTransferRequested event raised by the FluxAggregator contract.
type FluxAggregatorOwnershipTransferRequested struct {
	From common.Address
	To   common.Address
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferRequested is a free log retrieval operation binding the contract event 0xed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae1278.
//
// Solidity: event OwnershipTransferRequested(address indexed from, address indexed to)
func (_FluxAggregator *FluxAggregatorFilterer) FilterOwnershipTransferRequested(opts *bind.FilterOpts, from []common.Address, to []common.Address) (*FluxAggregatorOwnershipTransferRequestedIterator, error) {

	var fromRule []interface{}
	for _, fromItem := range from {
		fromRule = append(fromRule, fromItem)
	}
	var toRule []interface{}
	for _, toItem := range to {
		toRule = append(toRule, toItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "OwnershipTransferRequested", fromRule, toRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorOwnershipTransferRequestedIterator{contract: _FluxAggregator.contract, event: "OwnershipTransferRequested", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferRequested is a free log subscription operation binding the contract event 0xed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae1278.
//
// Solidity: event OwnershipTransferRequested(address indexed from, address indexed to)
func (_FluxAggregator *FluxAggregatorFilterer) WatchOwnershipTransferRequested(opts *bind.WatchOpts, sink chan<- *FluxAggregatorOwnershipTransferRequested, from []common.Address, to []common.Address) (event.Subscription, error) {

	var fromRule []interface{}
	for _, fromItem := range from {
		fromRule = append(fromRule, fromItem)
	}
	var toRule []interface{}
	for _, toItem := range to {
		toRule = append(toRule, toItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "OwnershipTransferRequested", fromRule, toRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorOwnershipTransferRequested)
				if err := _FluxAggregator.contract.UnpackLog(event, "OwnershipTransferRequested", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferRequested is a log parse operation binding the contract event 0xed8889f560326eb138920d842192f0eb3dd22b4f139c87a2c57538e05bae1278.
//
// Solidity: event OwnershipTransferRequested(address indexed from, address indexed to)
func (_FluxAggregator *FluxAggregatorFilterer) ParseOwnershipTransferRequested(log types.Log) (*FluxAggregatorOwnershipTransferRequested, error) {
	event := new(FluxAggregatorOwnershipTransferRequested)
	if err := _FluxAggregator.contract.UnpackLog(event, "OwnershipTransferRequested", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the FluxAggregator contract.
type FluxAggregatorOwnershipTransferredIterator struct {
	Event *FluxAggregatorOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorOwnershipTransferred represents a OwnershipTransferred event raised by the FluxAggregator contract.
type FluxAggregatorOwnershipTransferred struct {
	From common.Address
	To   common.Address
	Raw  types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed from, address indexed to)
func (_FluxAggregator *FluxAggregatorFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, from []common.Address, to []common.Address) (*FluxAggregatorOwnershipTransferredIterator, error) {

	var fromRule []interface{}
	for _, fromItem := range from {
		fromRule = append(fromRule, fromItem)
	}
	var toRule []interface{}
	for _, toItem := range to {
		toRule = append(toRule, toItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "OwnershipTransferred", fromRule, toRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorOwnershipTransferredIterator{contract: _FluxAggregator.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed from, address indexed to)
func (_FluxAggregator *FluxAggregatorFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *FluxAggregatorOwnershipTransferred, from []common.Address, to []common.Address) (event.Subscription, error) {

	var fromRule []interface{}
	for _, fromItem := range from {
		fromRule = append(fromRule, fromItem)
	}
	var toRule []interface{}
	for _, toItem := range to {
		toRule = append(toRule, toItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "OwnershipTransferred", fromRule, toRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorOwnershipTransferred)
				if err := _FluxAggregator.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed from, address indexed to)
func (_FluxAggregator *FluxAggregatorFilterer) ParseOwnershipTransferred(log types.Log) (*FluxAggregatorOwnershipTransferred, error) {
	event := new(FluxAggregatorOwnershipTransferred)
	if err := _FluxAggregator.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorRequesterPermissionsSetIterator is returned from FilterRequesterPermissionsSet and is used to iterate over the raw logs and unpacked data for RequesterPermissionsSet events raised by the FluxAggregator contract.
type FluxAggregatorRequesterPermissionsSetIterator struct {
	Event *FluxAggregatorRequesterPermissionsSet // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorRequesterPermissionsSetIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorRequesterPermissionsSet)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorRequesterPermissionsSet)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorRequesterPermissionsSetIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorRequesterPermissionsSetIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorRequesterPermissionsSet represents a RequesterPermissionsSet event raised by the FluxAggregator contract.
type FluxAggregatorRequesterPermissionsSet struct {
	Requester  common.Address
	Authorized bool
	Delay      uint32
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterRequesterPermissionsSet is a free log retrieval operation binding the contract event 0xc3df5a754e002718f2e10804b99e6605e7c701d95cec9552c7680ca2b6f2820a.
//
// Solidity: event RequesterPermissionsSet(address indexed requester, bool authorized, uint32 delay)
func (_FluxAggregator *FluxAggregatorFilterer) FilterRequesterPermissionsSet(opts *bind.FilterOpts, requester []common.Address) (*FluxAggregatorRequesterPermissionsSetIterator, error) {

	var requesterRule []interface{}
	for _, requesterItem := range requester {
		requesterRule = append(requesterRule, requesterItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "RequesterPermissionsSet", requesterRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorRequesterPermissionsSetIterator{contract: _FluxAggregator.contract, event: "RequesterPermissionsSet", logs: logs, sub: sub}, nil
}

// WatchRequesterPermissionsSet is a free log subscription operation binding the contract event 0xc3df5a754e002718f2e10804b99e6605e7c701d95cec9552c7680ca2b6f2820a.
//
// Solidity: event RequesterPermissionsSet(address indexed requester, bool authorized, uint32 delay)
func (_FluxAggregator *FluxAggregatorFilterer) WatchRequesterPermissionsSet(opts *bind.WatchOpts, sink chan<- *FluxAggregatorRequesterPermissionsSet, requester []common.Address) (event.Subscription, error) {

	var requesterRule []interface{}
	for _, requesterItem := range requester {
		requesterRule = append(requesterRule, requesterItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "RequesterPermissionsSet", requesterRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorRequesterPermissionsSet)
				if err := _FluxAggregator.contract.UnpackLog(event, "RequesterPermissionsSet", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRequesterPermissionsSet is a log parse operation binding the contract event 0xc3df5a754e002718f2e10804b99e6605e7c701d95cec9552c7680ca2b6f2820a.
//
// Solidity: event RequesterPermissionsSet(address indexed requester, bool authorized, uint32 delay)
func (_FluxAggregator *FluxAggregatorFilterer) ParseRequesterPermissionsSet(log types.Log) (*FluxAggregatorRequesterPermissionsSet, error) {
	event := new(FluxAggregatorRequesterPermissionsSet)
	if err := _FluxAggregator.contract.UnpackLog(event, "RequesterPermissionsSet", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorRoundDetailsUpdatedIterator is returned from FilterRoundDetailsUpdated and is used to iterate over the raw logs and unpacked data for RoundDetailsUpdated events raised by the FluxAggregator contract.
type FluxAggregatorRoundDetailsUpdatedIterator struct {
	Event *FluxAggregatorRoundDetailsUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorRoundDetailsUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorRoundDetailsUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorRoundDetailsUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorRoundDetailsUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorRoundDetailsUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorRoundDetailsUpdated represents a RoundDetailsUpdated event raised by the FluxAggregator contract.
type FluxAggregatorRoundDetailsUpdated struct {
	PaymentAmount      *big.Int
	MinSubmissionCount uint32
	MaxSubmissionCount uint32
	RestartDelay       uint32
	Timeout            uint32
	Raw                types.Log // Blockchain specific contextual infos
}

// FilterRoundDetailsUpdated is a free log retrieval operation binding the contract event 0x56800c9d1ed723511246614d15e58cfcde15b6a33c245b5c961b689c1890fd8f.
//
// Solidity: event RoundDetailsUpdated(uint128 indexed paymentAmount, uint32 indexed minSubmissionCount, uint32 indexed maxSubmissionCount, uint32 restartDelay, uint32 timeout)
func (_FluxAggregator *FluxAggregatorFilterer) FilterRoundDetailsUpdated(opts *bind.FilterOpts, paymentAmount []*big.Int, minSubmissionCount []uint32, maxSubmissionCount []uint32) (*FluxAggregatorRoundDetailsUpdatedIterator, error) {

	var paymentAmountRule []interface{}
	for _, paymentAmountItem := range paymentAmount {
		paymentAmountRule = append(paymentAmountRule, paymentAmountItem)
	}
	var minSubmissionCountRule []interface{}
	for _, minSubmissionCountItem := range minSubmissionCount {
		minSubmissionCountRule = append(minSubmissionCountRule, minSubmissionCountItem)
	}
	var maxSubmissionCountRule []interface{}
	for _, maxSubmissionCountItem := range maxSubmissionCount {
		maxSubmissionCountRule = append(maxSubmissionCountRule, maxSubmissionCountItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "RoundDetailsUpdated", paymentAmountRule, minSubmissionCountRule, maxSubmissionCountRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorRoundDetailsUpdatedIterator{contract: _FluxAggregator.contract, event: "RoundDetailsUpdated", logs: logs, sub: sub}, nil
}

// WatchRoundDetailsUpdated is a free log subscription operation binding the contract event 0x56800c9d1ed723511246614d15e58cfcde15b6a33c245b5c961b689c1890fd8f.
//
// Solidity: event RoundDetailsUpdated(uint128 indexed paymentAmount, uint32 indexed minSubmissionCount, uint32 indexed maxSubmissionCount, uint32 restartDelay, uint32 timeout)
func (_FluxAggregator *FluxAggregatorFilterer) WatchRoundDetailsUpdated(opts *bind.WatchOpts, sink chan<- *FluxAggregatorRoundDetailsUpdated, paymentAmount []*big.Int, minSubmissionCount []uint32, maxSubmissionCount []uint32) (event.Subscription, error) {

	var paymentAmountRule []interface{}
	for _, paymentAmountItem := range paymentAmount {
		paymentAmountRule = append(paymentAmountRule, paymentAmountItem)
	}
	var minSubmissionCountRule []interface{}
	for _, minSubmissionCountItem := range minSubmissionCount {
		minSubmissionCountRule = append(minSubmissionCountRule, minSubmissionCountItem)
	}
	var maxSubmissionCountRule []interface{}
	for _, maxSubmissionCountItem := range maxSubmissionCount {
		maxSubmissionCountRule = append(maxSubmissionCountRule, maxSubmissionCountItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "RoundDetailsUpdated", paymentAmountRule, minSubmissionCountRule, maxSubmissionCountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorRoundDetailsUpdated)
				if err := _FluxAggregator.contract.UnpackLog(event, "RoundDetailsUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseRoundDetailsUpdated is a log parse operation binding the contract event 0x56800c9d1ed723511246614d15e58cfcde15b6a33c245b5c961b689c1890fd8f.
//
// Solidity: event RoundDetailsUpdated(uint128 indexed paymentAmount, uint32 indexed minSubmissionCount, uint32 indexed maxSubmissionCount, uint32 restartDelay, uint32 timeout)
func (_FluxAggregator *FluxAggregatorFilterer) ParseRoundDetailsUpdated(log types.Log) (*FluxAggregatorRoundDetailsUpdated, error) {
	event := new(FluxAggregatorRoundDetailsUpdated)
	if err := _FluxAggregator.contract.UnpackLog(event, "RoundDetailsUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorSubmissionReceivedIterator is returned from FilterSubmissionReceived and is used to iterate over the raw logs and unpacked data for SubmissionReceived events raised by the FluxAggregator contract.
type FluxAggregatorSubmissionReceivedIterator struct {
	Event *FluxAggregatorSubmissionReceived // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorSubmissionReceivedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorSubmissionReceived)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorSubmissionReceived)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorSubmissionReceivedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorSubmissionReceivedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorSubmissionReceived represents a SubmissionReceived event raised by the FluxAggregator contract.
type FluxAggregatorSubmissionReceived struct {
	Submission *big.Int
	Round      uint32
	Oracle     common.Address
	Raw        types.Log // Blockchain specific contextual infos
}

// FilterSubmissionReceived is a free log retrieval operation binding the contract event 0x92e98423f8adac6e64d0608e519fd1cefb861498385c6dee70d58fc926ddc68c.
//
// Solidity: event SubmissionReceived(int256 indexed submission, uint32 indexed round, address indexed oracle)
func (_FluxAggregator *FluxAggregatorFilterer) FilterSubmissionReceived(opts *bind.FilterOpts, submission []*big.Int, round []uint32, oracle []common.Address) (*FluxAggregatorSubmissionReceivedIterator, error) {

	var submissionRule []interface{}
	for _, submissionItem := range submission {
		submissionRule = append(submissionRule, submissionItem)
	}
	var roundRule []interface{}
	for _, roundItem := range round {
		roundRule = append(roundRule, roundItem)
	}
	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "SubmissionReceived", submissionRule, roundRule, oracleRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorSubmissionReceivedIterator{contract: _FluxAggregator.contract, event: "SubmissionReceived", logs: logs, sub: sub}, nil
}

// WatchSubmissionReceived is a free log subscription operation binding the contract event 0x92e98423f8adac6e64d0608e519fd1cefb861498385c6dee70d58fc926ddc68c.
//
// Solidity: event SubmissionReceived(int256 indexed submission, uint32 indexed round, address indexed oracle)
func (_FluxAggregator *FluxAggregatorFilterer) WatchSubmissionReceived(opts *bind.WatchOpts, sink chan<- *FluxAggregatorSubmissionReceived, submission []*big.Int, round []uint32, oracle []common.Address) (event.Subscription, error) {

	var submissionRule []interface{}
	for _, submissionItem := range submission {
		submissionRule = append(submissionRule, submissionItem)
	}
	var roundRule []interface{}
	for _, roundItem := range round {
		roundRule = append(roundRule, roundItem)
	}
	var oracleRule []interface{}
	for _, oracleItem := range oracle {
		oracleRule = append(oracleRule, oracleItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "SubmissionReceived", submissionRule, roundRule, oracleRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorSubmissionReceived)
				if err := _FluxAggregator.contract.UnpackLog(event, "SubmissionReceived", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseSubmissionReceived is a log parse operation binding the contract event 0x92e98423f8adac6e64d0608e519fd1cefb861498385c6dee70d58fc926ddc68c.
//
// Solidity: event SubmissionReceived(int256 indexed submission, uint32 indexed round, address indexed oracle)
func (_FluxAggregator *FluxAggregatorFilterer) ParseSubmissionReceived(log types.Log) (*FluxAggregatorSubmissionReceived, error) {
	event := new(FluxAggregatorSubmissionReceived)
	if err := _FluxAggregator.contract.UnpackLog(event, "SubmissionReceived", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// FluxAggregatorValidatorUpdatedIterator is returned from FilterValidatorUpdated and is used to iterate over the raw logs and unpacked data for ValidatorUpdated events raised by the FluxAggregator contract.
type FluxAggregatorValidatorUpdatedIterator struct {
	Event *FluxAggregatorValidatorUpdated // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *FluxAggregatorValidatorUpdatedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(FluxAggregatorValidatorUpdated)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(FluxAggregatorValidatorUpdated)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *FluxAggregatorValidatorUpdatedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *FluxAggregatorValidatorUpdatedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// FluxAggregatorValidatorUpdated represents a ValidatorUpdated event raised by the FluxAggregator contract.
type FluxAggregatorValidatorUpdated struct {
	Previous common.Address
	Current  common.Address
	Raw      types.Log // Blockchain specific contextual infos
}

// FilterValidatorUpdated is a free log retrieval operation binding the contract event 0xcfac5dc75b8d9a7e074162f59d9adcd33da59f0fe8dfb21580db298fc0fdad0d.
//
// Solidity: event ValidatorUpdated(address indexed previous, address indexed current)
func (_FluxAggregator *FluxAggregatorFilterer) FilterValidatorUpdated(opts *bind.FilterOpts, previous []common.Address, current []common.Address) (*FluxAggregatorValidatorUpdatedIterator, error) {

	var previousRule []interface{}
	for _, previousItem := range previous {
		previousRule = append(previousRule, previousItem)
	}
	var currentRule []interface{}
	for _, currentItem := range current {
		currentRule = append(currentRule, currentItem)
	}

	logs, sub, err := _FluxAggregator.contract.FilterLogs(opts, "ValidatorUpdated", previousRule, currentRule)
	if err != nil {
		return nil, err
	}
	return &FluxAggregatorValidatorUpdatedIterator{contract: _FluxAggregator.contract, event: "ValidatorUpdated", logs: logs, sub: sub}, nil
}

// WatchValidatorUpdated is a free log subscription operation binding the contract event 0xcfac5dc75b8d9a7e074162f59d9adcd33da59f0fe8dfb21580db298fc0fdad0d.
//
// Solidity: event ValidatorUpdated(address indexed previous, address indexed current)
func (_FluxAggregator *FluxAggregatorFilterer) WatchValidatorUpdated(opts *bind.WatchOpts, sink chan<- *FluxAggregatorValidatorUpdated, previous []common.Address, current []common.Address) (event.Subscription, error) {

	var previousRule []interface{}
	for _, previousItem := range previous {
		previousRule = append(previousRule, previousItem)
	}
	var currentRule []interface{}
	for _, currentItem := range current {
		currentRule = append(currentRule, currentItem)
	}

	logs, sub, err := _FluxAggregator.contract.WatchLogs(opts, "ValidatorUpdated", previousRule, currentRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(FluxAggregatorValidatorUpdated)
				if err := _FluxAggregator.contract.UnpackLog(event, "ValidatorUpdated", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseValidatorUpdated is a log parse operation binding the contract event 0xcfac5dc75b8d9a7e074162f59d9adcd33da59f0fe8dfb21580db298fc0fdad0d.
//
// Solidity: event ValidatorUpdated(address indexed previous, address indexed current)
func (_FluxAggregator *FluxAggregatorFilterer) ParseValidatorUpdated(log types.Log) (*FluxAggregatorValidatorUpdated, error) {
	event := new(FluxAggregatorValidatorUpdated)
	if err := _FluxAggregator.contract.UnpackLog(event, "ValidatorUpdated", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
