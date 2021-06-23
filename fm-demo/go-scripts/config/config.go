package config

import (
	"errors"
	"time"

	"github.com/spf13/viper"
)

// ConfigurationType refers to the different ways that configurations can be set
type ConfigurationType string

const (
	LocalConfig  ConfigurationType = "local"
	SecretConfig ConfigurationType = "secret"
)

// Config is the overall config for the framework, holding configurations for supported networks
type Config struct {
	Networks        map[string]*NetworkConfig `mapstructure:"networks" yaml:"networks"`
	DefaultKeyStore string
}

// GetNetworkConfig finds a specified network config based on its name
func (c *Config) GetNetworkConfig(name string) (*NetworkConfig, error) {
	if network, ok := c.Networks[name]; ok {
		return network, nil
	}
	return nil, errors.New("no supported network of name " + name + " was found. Ensure that the config for it exists.")
}

// NetworkConfig holds the basic values that identify a blockchain network and contains private keys on the network
type NetworkConfig struct {
	Name                 string        `mapstructure:"name" yaml:"name"`
	URL                  string        `mapstructure:"url" yaml:"url"`
	ChainID              int64         `mapstructure:"chain_id" yaml:"chain_id"`
	PrivateKeys          []string      `mapstructure:"private_keys" yaml:"private_keys"`
	TransactionLimit     uint64        `mapstructure:"transaction_limit" yaml:"transaction_limit"`
	Timeout              time.Duration `mapstructure:"transaction_timeout" yaml:"transaction_timeout"`
	LinkTokenAddress     string        `mapstructure:"link_token_address" yaml:"link_token_address"`
	MinimumConfirmations int           `mapstructure:"minimum_confirmations" yaml:"minimum_confirmations"`
	GasEstimationBuffer  uint64        `mapstructure:"gas_estimation_buffer" yaml:"gas_estimation_buffer"`
	PrivateKeyStore      PrivateKeyStore
}

// NewConfig creates a new configuration instance via viper from env vars, config file, or a secret store
func NewConfig(configType ConfigurationType) (*Config, error) {
	return NewWithPath(configType, "")
}

// NewWithPath creates a new configuration with a specified path for the config file
func NewWithPath(configType ConfigurationType, configFilePath string) (*Config, error) {
	v := viper.New()
	v.AutomaticEnv()
	v.SetConfigName("config")
	v.SetConfigType("yml")

	if len(configFilePath) == 0 {
		configFilePath = "./" // Use default
	}
	v.AddConfigPath(configFilePath)

	if err := v.ReadInConfig(); err != nil {
		return nil, err
	}

	conf := &Config{}
	err := v.Unmarshal(conf)
	for _, networkConf := range conf.Networks {
		networkConf.PrivateKeyStore = NewPrivateKeyStore(configType, networkConf)
	}
	return conf, err
}

// PrivateKeyStore enables access, through a variety of methods, to private keys for use in blockchain networks
type PrivateKeyStore interface {
	Fetch() ([]string, error)
}

// NewPrivateKeyStore returns a keystore of a specific type, depending on where it should source its keys from
func NewPrivateKeyStore(configType ConfigurationType, network *NetworkConfig) PrivateKeyStore {
	switch configType {
	case LocalConfig:
		return &LocalStore{network.PrivateKeys}
	case SecretConfig:
		return &SecretStore{network.Name}
	}
	return nil
}

// LocalStore retrieves keys defined in a config.yml file, or from environment variables
type LocalStore struct {
	rawKeys []string
}

// Fetch private keys from local environment variables or a config file
func (l *LocalStore) Fetch() ([]string, error) {
	if l.rawKeys == nil {
		return nil, errors.New("no keys found, ensure your configuration is properly set")
	}
	return l.rawKeys, nil
}

// SecretStore retrieves keys from an encrypted secret storage service TBD
type SecretStore struct {
	networkName string
}

// Fetch private keys from env variables or a secret management system
func (s *SecretStore) Fetch() ([]string, error) {
	// TODO: Set up connection with whatever secret store we choose
	// Connect to secrets service / local encryption setup
	// Fetch keys based on the networkName
	// Return them
	return []string{""}, nil
}
