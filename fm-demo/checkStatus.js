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

    const feed = (await api.query.chainlinkFeed.feeds(0)).unwrap()
    console.log(feed)

    const latestRoundNumber = feed['latest_round'].words[0]
    console.log(latestRoundNumber)

    const latestRound = (await api.query.chainlinkFeed.rounds(0, latestRoundNumber)).unwrap()
    console.log(latestRound)

    const latestAnswer = latestRound.answer.unwrap().words
    console.log(latestAnswer)
  
}

main().catch(console.error).then(() => process.exit());
