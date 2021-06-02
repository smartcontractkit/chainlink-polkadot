package contracts

import (
	"context"
	"integrations-framework/client"
	"integrations-framework/contracts/ethereum"
	"math/big"

	"github.com/ethereum/go-ethereum/core/types"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
)

// EthereumStorage acts as a conduit for the ethereum version of the storage contract
type EthereumStorage struct {
	client       *client.EthereumClient
	store        *ethereum.Store
	callerWallet client.BlockchainWallet
}

// NewEthereumStorage creates a new instance of the storage contract for ethereum chains
func NewEthereumStorage(
	client *client.EthereumClient,
	store *ethereum.Store,
	callerWallet client.BlockchainWallet,
) Storage {

	return &EthereumStorage{
		client:       client,
		store:        store,
		callerWallet: callerWallet,
	}
}

// DeployStorageContract deploys a vanilla storage contract that is a value store
func DeployStorageContract(ethClient *client.EthereumClient, fromWallet client.BlockchainWallet) (Storage, error) {
	_, _, instance, err := ethClient.DeployContract(fromWallet, func(
		auth *bind.TransactOpts,
		backend bind.ContractBackend,
	) (common.Address, *types.Transaction, interface{}, error) {
		return ethereum.DeployStore(auth, backend)
	})
	if err != nil {
		return nil, err
	}
	return NewEthereumStorage(ethClient, instance.(*ethereum.Store), fromWallet), nil
}

// Set sets a value in the storage contract
func (e *EthereumStorage) Set(ctxt context.Context, value *big.Int) error {
	opts, err := e.client.TransactionOpts(e.callerWallet, common.Address{}, big.NewInt(0), common.Hash{})
	if err != nil {
		return err
	}

	transaction, err := e.store.Set(opts, value)
	if err != nil {
		return err
	}
	return e.client.WaitForTransaction(transaction.Hash())
}

// Get retrieves a set value from the storage contract
func (e *EthereumStorage) Get(ctxt context.Context) (*big.Int, error) {
	opts := &bind.CallOpts{
		From:    common.HexToAddress(e.callerWallet.Address()),
		Pending: true,
		Context: ctxt,
	}
	return e.store.Get(opts)
}

// EthereumFluxAggregator represents the basic flux aggregation contract
type EthereumFluxAggregator struct {
	client         *client.EthereumClient
	fluxAggregator *ethereum.FluxAggregator
	callerWallet   client.BlockchainWallet
}

// NewEthereumFluxAggregator creates a new instances of the Flux Aggregator contract for EVM chains
func NewEthereumFluxAggregator(client *client.EthereumClient,
	f *ethereum.FluxAggregator,
	callerWallet client.BlockchainWallet,
) FluxAggregator {

	return &EthereumFluxAggregator{
		client:         client,
		fluxAggregator: f,
		callerWallet:   callerWallet,
	}
}

// DeployFluxAggregatorContract deploys the Flux Aggregator Contract on an EVM chain
func DeployFluxAggregatorContract(
	ethClient *client.EthereumClient,
	fromWallet client.BlockchainWallet,
) (FluxAggregator, error) {

	_, _, instance, err := ethClient.DeployContract(fromWallet, func(
		auth *bind.TransactOpts,
		backend bind.ContractBackend,
	) (common.Address, *types.Transaction, interface{}, error) {
		// Gets a bit more complicated here, want to break this up
		return common.Address{}, nil, nil, nil
		// return ethereum.DeployFluxAggregator(auth, backend, common.HexToAddress(linkAddress))
	})
	if err != nil {
		return nil, err
	}
	return NewEthereumFluxAggregator(ethClient, instance.(*ethereum.FluxAggregator), fromWallet), nil
}

// Description returns the description of the flux aggregator contract
func (f *EthereumFluxAggregator) Description(ctxt context.Context) (string, error) {
	opts := &bind.CallOpts{
		From:    common.HexToAddress(f.callerWallet.Address()),
		Pending: true,
		Context: ctxt,
	}
	return f.fluxAggregator.Description(opts)
}

// EthereumVRF represents a VRF contract
type EthereumVRF struct {
	client       *client.EthereumClient
	vrf          *ethereum.VRF
	callerWallet client.BlockchainWallet
}

// NewEthereumVRF creates a new VRF contract instance
func NewEthereumVRF(client *client.EthereumClient, vrf *ethereum.VRF, callerWallet client.BlockchainWallet) VRF {
	return &EthereumVRF{
		client:       client,
		vrf:          vrf,
		callerWallet: callerWallet,
	}
}

// DeployVRFContract deploys a VRF contract
func DeployVRFContract(ethClient *client.EthereumClient, fromWallet client.BlockchainWallet) (VRF, error) {
	_, _, instance, err := ethClient.DeployContract(fromWallet, func(
		auth *bind.TransactOpts,
		backend bind.ContractBackend,
	) (common.Address, *types.Transaction, interface{}, error) {
		return ethereum.DeployVRF(auth, backend)
	})
	if err != nil {
		return nil, err
	}
	return NewEthereumVRF(ethClient, instance.(*ethereum.VRF), fromWallet), nil
}

// ProofLength returns the PROOFLENGTH call from the VRF contract
func (v *EthereumVRF) ProofLength(ctxt context.Context) (*big.Int, error) {
	opts := &bind.CallOpts{
		From:    common.HexToAddress(v.callerWallet.Address()),
		Pending: true,
		Context: ctxt,
	}
	return v.vrf.PROOFLENGTH(opts)
}
