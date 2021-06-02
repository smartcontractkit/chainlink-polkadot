# Chainlink Integration Framework

![Tests](https://github.com/smartcontractkit/integrations-framework/actions/workflows/test.yaml/badge.svg)
![Lint](https://github.com/smartcontractkit/integrations-framework/actions/workflows/lint.yaml/badge.svg)

A framework for interacting with chainlink nodes, environments, and other blockchain systems. The framework is primarilly intended to facillitate testing chainlink features and stability.

## How to Test

1. Start a local hardhat network. You can easily do so by using our [docker container](https://hub.docker.com/r/smartcontract/hardhat-network). You could also deploy [your own local version](https://hardhat.org/hardhat-network/), if you are so inclined.
2. Run `go test ./...`

## // TODO

* Add more chainlink node checks
* Enable connecting chainlink node interfaces to actual running nodes in an environment
* Check out [hardhat deploy](https://hardhat.org/plugins/hardhat-deploy.html) to help setup test environments
* Look into [precompiling tests with Ginkgo](https://onsi.github.io/ginkgo/#precompiling-tests) to speed up test execution
* Implement `Ginkgo` CLI into tests, ideally with minimal impact to local test users. This allows us to use some cool stuff like [precompiling tests](https://onsi.github.io/ginkgo/#precompiling-tests), and [running in parallel](https://onsi.github.io/ginkgo/#parallel-specs)

Check out our [clubhouse board](https://app.clubhouse.io/chainlinklabs/project/5690/qa-team?vc_group_by=day) for a look into our progress.
