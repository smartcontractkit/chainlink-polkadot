package contracts

import (
	"context"
	"integrations-framework/client"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
)

type FluxAggregatorOptions struct {
	PaymentAmount *big.Int       // The amount of LINK paid to each oracle per submission, in wei (units of 10⁻¹⁸ LINK)
	Timeout       uint32         // The number of seconds after the previous round that are allowed to lapse before allowing an oracle to skip an unfinished round
	Validator     common.Address // An optional contract address for validating external validation of answers
	MinSubValue   *big.Int       // An immutable check for a lower bound of what submission values are accepted from an oracle
	MaxSubValue   *big.Int       // An immutable check for an upper bound of what submission values are accepted from an oracle
	Decimals      uint8          // The number of decimals to offset the answer by
	Description   string         // A short description of what is being reported
}

type FluxAggregatorData struct {
	AllocatedFunds  *big.Int         // The amount of payment yet to be withdrawn by oracles
	AvailableFunds  *big.Int         // The amount of future funding available to oracles
	LatestRoundData RoundData        // Data about the latest round
	Oracles         []common.Address // Addresses of oracles on the contract
}

type FluxAggregator interface {
	Fund(fromWallet client.BlockchainWallet, ethAmount *big.Int, linkAmount *big.Int) error
	GetContractData(ctxt context.Context) (*FluxAggregatorData, error)
	SetOracles(
		client.BlockchainWallet,
		[]common.Address,
		[]common.Address,
		[]common.Address,
		uint32,
		uint32,
		uint32,
	) error
	Description(ctxt context.Context) (string, error)
}

type LinkToken interface {
	Fund(fromWallet client.BlockchainWallet, ethAmount *big.Int) error
	Name(context.Context) (string, error)
}

type OffchainOptions struct {
	MaximumGasPrice           uint32         // The highest gas price for which transmitter will be compensated
	ReasonableGasPrice        uint32         // The transmitter will receive reward for gas prices under this value
	MicroLinkPerEth           uint32         // The reimbursement per ETH of gas cost, in 1e-6LINK units
	LinkGweiPerObservation    uint32         // The reward to the oracle for contributing an observation to a successfully transmitted report, in 1e-9LINK units
	LinkGweiPerTransmission   uint32         // The reward to the transmitter of a successful report, in 1e-9LINK units
	MinimumAnswer             *big.Int       // The lowest answer the median of a report is allowed to be
	MaximumAnswer             *big.Int       // The highest answer the median of a report is allowed to be
	BillingAccessController   common.Address // The access controller for billing admin functions
	RequesterAccessController common.Address // The access controller for requesting new rounds
	Decimals                  uint8          // Answers are stored in fixed-point format, with this many digits of precision
	Description               string         // A short description of what is being reported
}

type OffchainAggregatorData struct {
	LatestRoundData RoundData // Data about the latest round
}

type OffchainAggregator interface {
	Fund(client.BlockchainWallet, *big.Int, *big.Int) error
	GetContractData(ctxt context.Context) (*OffchainAggregatorData, error)
	SetConfig(
		fromWallet client.BlockchainWallet,
		signers, transmitters []common.Address,
		threshold uint8,
		encodedConfigVersion uint64,
		encoded []byte,
	) error
	SetPayees(client.BlockchainWallet, []common.Address, []common.Address) error
	Link(ctxt context.Context) (common.Address, error)
}

type Storage interface {
	Get(ctxt context.Context) (*big.Int, error)
	Set(*big.Int) error
}

type VRF interface {
	Fund(client.BlockchainWallet, *big.Int, *big.Int) error
	ProofLength(context.Context) (*big.Int, error)
}

type RoundData struct {
	RoundId         *big.Int
	Answer          *big.Int
	StartedAt       *big.Int
	UpdatedAt       *big.Int
	AnsweredInRound *big.Int
}
