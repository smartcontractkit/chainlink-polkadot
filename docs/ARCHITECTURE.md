The Substrate/Polkadot <-> Chainlink integration is multi-fold and ethereum LINK tokens integration. Some of those require Polkadot features that are not yet available 

# First step: as a separate module, no token involved

Build a runtime module that can be used by chain users to interact with Chainlink.
This module will emit a specific `OracleEvent` containing all information related to a Chainlink request. It will be picked up by Chainlink nodes that in return insert back the answer via a specific Extrinsic.

_NOTE_: this step is focused on implementing the most simple full workflow (from 3rd party request to getting back the Oracle answer). It doesn't attempt to achieve Chainlink Ethereum feature parity.

## Actions

* Parity: write a reusable pallet-chainlink allowin 3rd party users to interact with Chainlink
* Chainlink: write necessary components (External Initiator, ..) so that Chainlink nodes pick those events then write back a substrate transaction with the associated result

More details can be found in the relevant Github issues.

Estimated delivery: January 10 2020

# Second step: as a separate module, no token involved, Ethereum feature parity

Based on feedback from the first step, implement missing bits allowing to support all Ethereum Chainlink feature parity.

# Third step: as a separate module, Oracle fees paid in LINK tokens

3rd party requesters must pay a fee in LINK tokens to Oracle providers as part of their request.
This step must be secure and involve real Ethereum LINK tokens.

# Tentative step: full parachain

This steps requires Polkadot full parachain implementation, XCMP support and eventually access to an Ethereum bridge.