package client

import (
	"context"
	"fmt"
	"math/big"
	"time"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/rs/zerolog/log"
)

// EthereumClient wraps the client and the BlockChain network to interact with an EVM based Blockchain
type EthereumClient struct {
	Client  *ethclient.Client
	Network BlockchainNetwork
}

// ContractDeployer acts as a go-between function for general contract deployment
type ContractDeployer func(auth *bind.TransactOpts, backend bind.ContractBackend) (
	common.Address,
	*types.Transaction,
	interface{},
	error,
)

// NewEthereumClient returns an instantiated instance of the Ethereum client that has connected to the server
func NewEthereumClient(network BlockchainNetwork) (*EthereumClient, error) {
	cl, err := ethclient.Dial(network.URL())
	if err != nil {
		return nil, err
	}

	return &EthereumClient{
		Client:  cl,
		Network: network,
	}, nil
}

// SendTransaction sends a specified amount of WEI from a selected wallet to an address, and blocks until the
// transaction completes
func (e *EthereumClient) SendTransaction(
	from BlockchainWallet,
	to common.Address,
	value *big.Int,
	data []byte,
) (*common.Hash, error) {
	callMsg, err := e.TransactionCallMessage(from, to, value, data)
	if err != nil {
		return nil, err
	}
	privateKey, err := crypto.HexToECDSA(from.PrivateKey())
	if err != nil {
		return nil, fmt.Errorf("invalid private key: %v", err)
	}
	nonce, err := e.Client.PendingNonceAt(context.Background(), common.HexToAddress(from.Address()))
	if err != nil {
		return nil, err
	}

	tx, err := types.SignNewTx(privateKey, types.NewEIP2930Signer(e.Network.ChainID()), &types.LegacyTx{
		To:       callMsg.To,
		Value:    callMsg.Value,
		Data:     callMsg.Data,
		GasPrice: callMsg.GasPrice,
		Gas:      callMsg.Gas,
		Nonce:    nonce,
	})
	if err != nil {
		return nil, err
	}

	if err := e.Client.SendTransaction(context.Background(), tx); err != nil {
		return nil, err
	}
	log.Info().
		Str("From", from.Address()).
		Str("To", tx.To().Hex()).
		Str("Value", tx.Value().String()).
		Msg("Sending Transaction")

	err = e.WaitForTransaction(tx.Hash())
	hash := tx.Hash()
	return &hash, err
}

// DeployContract acts as a general contract deployment tool to an ethereum chain
func (e *EthereumClient) DeployContract(
	fromWallet BlockchainWallet,
	contractName string,
	deployer ContractDeployer,
) (*common.Address, *types.Transaction, interface{}, error) {
	opts, err := e.TransactionOpts(fromWallet, common.Address{}, big.NewInt(0), nil)
	if err != nil {
		return nil, nil, nil, err
	}
	contractAddress, transaction, contractInstance, err := deployer(opts, e.Client)
	if err != nil {
		return nil, nil, nil, err
	}
	err = e.WaitForTransaction(transaction.Hash())
	if err != nil {
		return nil, nil, nil, err
	}
	log.Info().
		Str("Contract Address", contractAddress.Hex()).
		Str("Contract Name", contractName).
		Str("From", fromWallet.Address()).
		Msg("Deployed contract")
	return &contractAddress, transaction, contractInstance, err
}

// TransactionCallMessage returns a filled Ethereum CallMsg object with suggest gas price and limit
func (e *EthereumClient) TransactionCallMessage(
	from BlockchainWallet,
	to common.Address,
	value *big.Int,
	data []byte,
) (*ethereum.CallMsg, error) {
	gasPrice, err := e.Client.SuggestGasPrice(context.Background())
	if err != nil {
		return nil, err
	}
	log.Debug().Str("Suggested Gas Price", gasPrice.String())
	msg := ethereum.CallMsg{
		From:     common.HexToAddress(from.Address()),
		To:       &to,
		GasPrice: gasPrice,
		Value:    value,
		Data:     data,
	}
	msg.Gas = e.Network.Config().TransactionLimit + e.Network.Config().GasEstimationBuffer
	log.Debug().Uint64("Gas Limit", e.Network.Config().TransactionLimit).Uint64("Limit + Buffer", msg.Gas)
	return &msg, nil
}

// TransactionOpts return the base binding transaction options to create a new valid tx for contract deployment
func (e *EthereumClient) TransactionOpts(
	from BlockchainWallet,
	to common.Address,
	value *big.Int,
	data []byte,
) (*bind.TransactOpts, error) {
	callMsg, err := e.TransactionCallMessage(from, to, value, data)
	if err != nil {
		return nil, err
	}
	nonce, err := e.Client.PendingNonceAt(context.Background(), common.HexToAddress(from.Address()))
	if err != nil {
		return nil, err
	}
	privateKey, err := crypto.HexToECDSA(from.PrivateKey())
	if err != nil {
		return nil, fmt.Errorf("invalid private key: %v", err)
	}
	opts, err := bind.NewKeyedTransactorWithChainID(privateKey, e.Network.ChainID())
	if err != nil {
		return nil, err
	}
	opts.From = callMsg.From
	opts.Nonce = big.NewInt(int64(nonce))
	opts.Value = value
	opts.GasPrice = callMsg.GasPrice
	opts.GasLimit = callMsg.Gas
	opts.Context = context.Background()

	return opts, nil
}

// WaitForTransaction helper function that waits for a specified transaction to clear
func (e *EthereumClient) WaitForTransaction(transactionHash common.Hash) error {
	headerChannel := make(chan *types.Header)
	subscription, err := e.Client.SubscribeNewHead(context.Background(), headerChannel)
	if err != nil {
		return err
	}
	defer subscription.Unsubscribe()

	timeout := e.Network.Config().Timeout
	confirmations := 0

	for {
		select {
		case err := <-subscription.Err():
			return err
		case header := <-headerChannel:
			minConfirmations := e.Network.Config().MinimumConfirmations

			block, err := e.Client.BlockByNumber(context.Background(), header.Number)
			if err != nil {
				return err
			}
			confirmationLog := log.Info().Str("Network", e.Network.Config().Name).
				Str("Block Hash", block.Hash().Hex()).
				Str("Block Number", block.Number().String()).Str("Tx Hash", transactionHash.Hex()).
				Int("Minimum Confirmations", minConfirmations).
				Int("Total Confirmations", confirmations)

			isConfirmed, err := e.isTxConfirmed(transactionHash)
			if err != nil {
				return err
			} else if !isConfirmed {
				confirmationLog.Msg("Transaction still pending, waiting for confirmation")
				continue
			}

			confirmations++
			confirmationLog.Msg("Transaction confirmed, waiting on confirmations")

			if confirmations >= minConfirmations {
				confirmationLog.Msg("Minimum confirmations met")
				return err
			} else {
				confirmationLog.Msg("Waiting on minimum confirmations")
			}
		case <-time.After(timeout):
			isConfirmed, err := e.isTxConfirmed(transactionHash)
			if err != nil {
				return err
			} else if isConfirmed {
				return nil
			}

			err = fmt.Errorf("timeout waiting for transaction after %f seconds", timeout.Seconds())
			log.Error().
				Str("Network", e.Network.Config().Name).
				Err(err)
			return err
		}
	}
}

func (e *EthereumClient) isTxConfirmed(txHash common.Hash) (bool, error) {
	_, isPending, err := e.Client.TransactionByHash(context.Background(), txHash)
	return !isPending, err
}
