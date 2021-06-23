package contracts

import (
	"context"
	"integrations-framework/client"
	"integrations-framework/config"
	"math/big"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/ginkgo/extensions/table"
	. "github.com/onsi/gomega"
)

var _ = Describe("Client", func() {
	var conf *config.Config

	BeforeEach(func() {
		var err error
		conf, err = config.NewWithPath(config.LocalConfig, "../config")
		Expect(err).ShouldNot(HaveOccurred())
	})

	DescribeTable("deploy and interact with the storage contract", func(
		initFunc client.BlockchainNetworkInit,
		value *big.Int,
	) {
		// Deploy contract
		networkConfig, err := initFunc(conf)
		Expect(err).ShouldNot(HaveOccurred())
		client, err := client.NewEthereumClient(networkConfig)
		Expect(err).ShouldNot(HaveOccurred())
		wallets, err := networkConfig.Wallets()
		Expect(err).ShouldNot(HaveOccurred())
		storeInstance, err := DeployStorageContract(client, wallets.Default())
		Expect(err).ShouldNot(HaveOccurred())

		// Interact with contract
		err = storeInstance.Set(value)
		Expect(err).ShouldNot(HaveOccurred())
		val, err := storeInstance.Get(context.Background())
		Expect(err).ShouldNot(HaveOccurred())
		Expect(val).To(Equal(value))
	},
		Entry("on Ethereum Hardhat", client.NewHardhatNetwork, big.NewInt(5)),
		// Tested locally successfully. We need to implement secrets system as well as testing wallets for CI use
		// Entry("on Ethereum Kovan", client.NewKovanNetwork, big.NewInt(5)),
		// Entry("on Ethereum Goerli", client.NewGoerliNetwork, big.NewInt(5)),
	)

	DescribeTable("deploy and interact with the FluxAggregator contract", func(
		initFunc client.BlockchainNetworkInit,
		fluxOptions FluxAggregatorOptions,
	) {
		// Setup network and client
		networkConfig, err := initFunc(conf)
		Expect(err).ShouldNot(HaveOccurred())
		client, err := client.NewEthereumClient(networkConfig)
		Expect(err).ShouldNot(HaveOccurred())
		wallets, err := networkConfig.Wallets()
		Expect(err).ShouldNot(HaveOccurred())

		// Deploy LINK contract
		linkInstance, err := DeployLinkTokenContract(client, wallets.Default())
		Expect(err).ShouldNot(HaveOccurred())
		name, err := linkInstance.Name(context.Background())
		Expect(err).ShouldNot(HaveOccurred())
		Expect(name).To(Equal("ChainLink Token"))

		// Deploy FluxMonitor contract
		fluxInstance, err := DeployFluxAggregatorContract(client, wallets.Default(), fluxOptions)
		Expect(err).ShouldNot(HaveOccurred())
		err = fluxInstance.Fund(wallets.Default(), big.NewInt(0), big.NewInt(50000000000))
		Expect(err).ShouldNot(HaveOccurred())

		// Interact with contract
		desc, err := fluxInstance.Description(context.Background())
		Expect(err).ShouldNot(HaveOccurred())
		Expect(desc).To(Equal(fluxOptions.Description))
	},
		Entry("on Ethereum Hardhat", client.NewHardhatNetwork, FluxAggregatorOptions{
			PaymentAmount: big.NewInt(1),
			Timeout:       uint32(5),
			MinSubValue:   big.NewInt(1),
			MaxSubValue:   big.NewInt(10),
			Decimals:      uint8(8),
			Description:   "Hardhat Flux Aggregator",
		}),
		// Tested locally successfully. We need to implement secrets system as well as testing wallets for CI use
		// Entry("on Ethereum Kovan", client.NewKovanNetwork, big.NewInt(5)),
		// Entry("on Ethereum Goerli", client.NewGoerliNetwork, big.NewInt(5)),
	)

	DescribeTable("deploy and interact with the OffChain Aggregator contract", func(
		initFunc client.BlockchainNetworkInit,
		offchainOptions OffchainOptions,
	) {
		// Setup network and client
		networkConfig, err := initFunc(conf)
		Expect(err).ShouldNot(HaveOccurred())
		client, err := client.NewEthereumClient(networkConfig)
		Expect(err).ShouldNot(HaveOccurred())
		wallets, err := networkConfig.Wallets()
		Expect(err).ShouldNot(HaveOccurred())

		// Deploy LINK contract
		linkInstance, err := DeployLinkTokenContract(client, wallets.Default())
		Expect(err).ShouldNot(HaveOccurred())
		name, err := linkInstance.Name(context.Background())
		Expect(err).ShouldNot(HaveOccurred())
		Expect(name).To(Equal("ChainLink Token"))

		// Deploy Offchain contract
		offChainInstance, err := DeployOffChainAggregator(client, wallets.Default(), offchainOptions)
		Expect(err).ShouldNot(HaveOccurred())
		err = offChainInstance.Fund(wallets.Default(), nil, big.NewInt(50000000000))
		Expect(err).ShouldNot(HaveOccurred())
	},
		Entry("on Ethereum Hardhat", client.NewHardhatNetwork, OffchainOptions{
			// Some defaults from Lorenz's project. Not sure if we want this in a config file in future?
			MaximumGasPrice:         uint32(1000),
			ReasonableGasPrice:      uint32(200),
			MicroLinkPerEth:         3.6e7,
			LinkGweiPerObservation:  1e8,
			LinkGweiPerTransmission: 4e8,
			MinimumAnswer:           big.NewInt(1),
			MaximumAnswer:           big.NewInt(100),
			Decimals:                uint8(8),
			Description:             "Hardhat OffChain Aggregator",
		}),
		// Tested locally successfully. We need to implement secrets system as well as testing wallets for CI use
		// Entry("on Ethereum Kovan", client.NewKovanNetwork, big.NewInt(5)),
		// Entry("on Ethereum Goerli", client.NewGoerliNetwork, big.NewInt(5)),
	)
})
