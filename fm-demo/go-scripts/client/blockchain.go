package client

import (
	"fmt"
	"integrations-framework/config"
	"math/big"
	"strings"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
)

type BlockchainNetworkID string

const (
	EthereumHardhatID BlockchainNetworkID = "ethereum_hardhat"
	EthereumKovanID   BlockchainNetworkID = "ethereum_kovan"
	EthereumGoerliID  BlockchainNetworkID = "ethereum_goerli"
)

// BlockchainNetwork is the interface that when implemented, defines a new blockchain network that can be tested against
type BlockchainNetwork interface {
	ID() string
	URL() string
	ChainID() *big.Int
	Wallets() (BlockchainWallets, error)
	Config() *config.NetworkConfig
}

type BlockchainNetworkInit func(conf *config.Config) (BlockchainNetwork, error)

// EthereumNetwork is the implementation of BlockchainNetwork for the local ETH dev server
type EthereumNetwork struct {
	networkID     string
	networkConfig *config.NetworkConfig
}

// NewEthereumNetwork creates a way to interact with any specified EVM blockchain
func newEthereumNetwork(conf *config.Config, networkID BlockchainNetworkID) (BlockchainNetwork, error) {
	networkConf, err := conf.GetNetworkConfig(string(networkID))
	if err != nil {
		return nil, err
	}
	return &EthereumNetwork{
		networkID:     string(networkID),
		networkConfig: networkConf,
	}, nil
}

// NewHardhatNetwork prepares settings for a connection to a hardhat blockchain
func NewHardhatNetwork(conf *config.Config) (BlockchainNetwork, error) {
	return newEthereumNetwork(conf, EthereumHardhatID)
}

// NewKovanNetwork prepares settings for a connection to the kovan testnet
func NewKovanNetwork(conf *config.Config) (BlockchainNetwork, error) {
	return newEthereumNetwork(conf, EthereumKovanID)
}

// NewGoerliNetwork prepares settings for a connection to the Goerli testnet
func NewGoerliNetwork(conf *config.Config) (BlockchainNetwork, error) {
	return newEthereumNetwork(conf, EthereumGoerliID)
}

// ID returns the readable name of the EVM network
func (e *EthereumNetwork) ID() string {
	return e.networkID
}

// URL returns the RPC URL used for connecting to the network
func (e *EthereumNetwork) URL() string {
	return e.networkConfig.URL
}

// ChainID returns the on-chain ID of the network being connected to
func (e *EthereumNetwork) ChainID() *big.Int {
	return big.NewInt(e.networkConfig.ChainID)
}

// Config returns the blockchain network configuration
func (e *EthereumNetwork) Config() *config.NetworkConfig {
	return e.networkConfig
}

// Wallets returns all the viable wallets used for testing on chain
func (e *EthereumNetwork) Wallets() (BlockchainWallets, error) {
	return newEthereumWallets(e.networkConfig.PrivateKeyStore)
}

// BlockchainWallets is an interface that when implemented is a representation of a slice of wallets for
// a specific network
type BlockchainWallets interface {
	Default() BlockchainWallet
	SetDefault(i int) error
	Wallet(i int) (BlockchainWallet, error)
}

// Wallets is the default implementation of BlockchainWallets that holds a slice of wallets with the default
type Wallets struct {
	defaultWallet int
	wallets       []BlockchainWallet
}

// Default returns the default wallet to be used for a transaction on-chain
func (w *Wallets) Default() BlockchainWallet {
	return w.wallets[w.defaultWallet]
}

// SetDefault changes the default wallet to be used for on-chain transactions
func (w *Wallets) SetDefault(i int) error {
	if err := walletSliceIndexInRange(w.wallets, i); err != nil {
		return err
	}
	w.defaultWallet = i
	return nil
}

// Wallet returns a wallet based on a given index in the slice
func (w *Wallets) Wallet(i int) (BlockchainWallet, error) {
	if err := walletSliceIndexInRange(w.wallets, i); err != nil {
		return nil, err
	}
	return w.wallets[i], nil
}

// BlockchainWallet when implemented is the interface to allow multiple wallet implementations for each
// BlockchainNetwork that is supported
type BlockchainWallet interface {
	PrivateKey() string
	Address() string
}

// EthereumWallet is the implementation to allow testing with ETH based wallets
type EthereumWallet struct {
	privateKey string
	address    common.Address
}

// NewEthereumWallet returns the instantiated ETH wallet based on a given private key
func NewEthereumWallet(pk string) (*EthereumWallet, error) {
	privateKey, err := crypto.HexToECDSA(pk)
	if err != nil {
		return nil, fmt.Errorf("invalid private key: %v", err)
	}
	return &EthereumWallet{
		privateKey: pk,
		address:    crypto.PubkeyToAddress(privateKey.PublicKey),
	}, nil
}

// PrivateKey returns the private key for a given Ethereum wallet
func (e *EthereumWallet) PrivateKey() string {
	return e.privateKey
}

// Address returns the ETH address for a given wallet
func (e *EthereumWallet) Address() string {
	return e.address.String()
}

func newEthereumWallets(pkStore config.PrivateKeyStore) (BlockchainWallets, error) {
	// Check private keystore value, create wallets from such
	var processedWallets []BlockchainWallet
	keys, err := pkStore.Fetch()
	if err != nil {
		return nil, err
	}

	for _, key := range keys {
		wallet, err := NewEthereumWallet(strings.TrimSpace(key))
		if err != nil {
			return &Wallets{}, err
		}
		processedWallets = append(processedWallets, wallet)
	}

	return &Wallets{
		defaultWallet: 0,
		wallets:       processedWallets,
	}, nil
}

func walletSliceIndexInRange(wallets []BlockchainWallet, i int) error {
	if i > len(wallets)-1 {
		return fmt.Errorf("invalid index in list of wallets")
	}
	return nil
}
