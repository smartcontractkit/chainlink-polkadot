This folder contains different chains-specs you can use to specify the example chain's specifications

To run the example chain with a custom chainspec:

```shell
cargo build --release
./target/release/node-template --chain=./specs/chainlink-feed.json --tmp --alice
```

You can check that the feed is included in the chain state:
- use the polkadot UI -> Developer -> Chain State -> chainlinkFeed -> Feeds(0)

If it shows `<unknown>` you need to register the custom [types](../types.json) by copying the json types into Settings -> Developer in the UI

If the chain blocks access from the UI (`Blocked connection to WebSockets server from untrusted origin`) add the `--rpc-cors all` option


Read more about substrate's [Chain Specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) and [creating private networks](https://substrate.dev/docs/en/tutorials/start-a-private-network)


* [chainlink.json](chainlink.json) is a spec with multiple registered feed creators (Alice, Bob, Charlie, Dave, Eve, Ferdie as well as their stash accounts). pallet admin is Alice
* [chainlink-feed.json](chainlink-feed.json) is a spec with the same feed creators and a feed owned by Alice, several oracles (Bob, Charlie, Dave, Eve) and Ferdie as their admin.
  To add more feeds, add another feed in the `feeds` array.
* [chainlink-no-admin-funds.json](chainlink-no-admin-funds.json) is a spec the same feed creators but the admin is Ferdie which has no endowment.

The genesis config of the `chainlink-feed` pallet can be created like:

```rust
chainlink_feed: ChainlinkFeedConfig {
    pallet_admin: Some(root_key),
    // accounts that are allowed to create feeds, must include the owners of the `feeds`
    feed_creators: vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
    feeds: vec![node_template_runtime::FeedBuilder::new()
        // owner of this feed
        .owner(get_account_id_from_seed::<sr25519::Public>("Alice"))
        .decimals(8)
        // payment of oracle rounds
        .payment(1_000)
        // round initiation delay
        .restart_delay(0)
        // timeout of blocks
        .timeout(10)
        .description(b"LINK".to_vec())
        .value_bounds(1, 1_000)
        .min_submissions(2)
        // make Bob, Charlie, Dave and Eve oracles and Ferdie their admin
        .oracles(vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            )
        ])]
},
```