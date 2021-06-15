const {ApiPromise, Keyring, WsProvider} = require('@polkadot/api');
const {cryptoWaitReady} = require('@polkadot/util-crypto');
const feedConfigs = require('./feeds.json');
const types = require('../substrate-node-example/types.json');

async function main() {
    await cryptoWaitReady();

    // Connect to the local chain
    const wsProvider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({
        provider: wsProvider,
        types
    });
    for (feedId = 0; feedId < feedConfigs.length; feedId++) {
        console.log(`Feed ID: ${feedId}`)
        const feed = (await api.query.chainlinkFeed.feeds(feedId)).unwrap()

        const latestRoundNumber = feed['latest_round'].words[0]

        const latestRound = (await api.query.chainlinkFeed.rounds(0, latestRoundNumber)).unwrap()

        const latestAnswer = latestRound.answer.unwrap().toNumber()
        console.log("Latest answer on chain: ")
        console.log(latestAnswer)
    }
  
}

main().catch(console.error).then(() => process.exit());
