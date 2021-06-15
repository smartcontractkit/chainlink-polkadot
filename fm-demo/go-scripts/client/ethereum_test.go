package client

import (
	"integrations-framework/config"
	"math/big"

	"github.com/ethereum/go-ethereum/common"

	. "github.com/onsi/ginkgo"
	. "github.com/onsi/ginkgo/extensions/table"
	. "github.com/onsi/gomega"
)

var _ = Describe("Ethereum functionality", func() {
	var conf *config.Config

	BeforeEach(func() {
		var err error
		conf, err = config.NewWithPath(config.LocalConfig, "../config")
		Expect(err).ShouldNot(HaveOccurred())
	})

	DescribeTable("eth transaction basics", func(
		initFunc BlockchainNetworkInit,
	) {
		// Setup
		networkConfig, err := initFunc(conf)
		Expect(err).ShouldNot(HaveOccurred())
		client, err := NewEthereumClient(networkConfig)
		Expect(err).ShouldNot(HaveOccurred())
		wallets, err := networkConfig.Wallets()
		Expect(err).ShouldNot(HaveOccurred())

		// Actual transaction
		toWallet, err := wallets.Wallet(1)
		Expect(err).ShouldNot(HaveOccurred())
		_, err = client.SendTransaction(
			wallets.Default(),
			common.HexToAddress(toWallet.Address()),
			big.NewInt(0),
			nil,
		)
		Expect(err).ShouldNot(HaveOccurred())
	},
		Entry("on Ethereum Hardhat", NewHardhatNetwork),
	)
})
