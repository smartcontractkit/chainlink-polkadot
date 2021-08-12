const {ApiPromise, Keyring, WsProvider} = require('@polkadot/api');
const {cryptoWaitReady} = require('@polkadot/util-crypto');
const feedConfigs = require('./feeds.json');
const types = require('../substrate-node-example/types.json');


async function fundAccountIfNeeded(api, senderAccount, receiverAddress) {
    return new Promise(async (resolve) => {
        const balance = await api.query.system.account(receiverAddress);
        console.log(`Free balance of ${receiverAddress} is: ${balance.data.free}`);
        if (parseInt(balance.data.free) === 0) {
            await api.tx.balances.transfer(receiverAddress, 123456666000).signAndSend(senderAccount, async ({status}) => {
                if (status.isFinalized) {
                    console.log(`Account ${receiverAddress} funded`);
                    resolve();
                }
            });
        } else {
            resolve();
        }
    });
}

async function registerFeedCreatorIfNeeded(api, aliceAccount, operatorAccount) {
    console.log(`Registering feed creator ${operatorAccount.address}`);
    return new Promise(async (resolve) => {
        const feedCreator = await api.query.chainlinkFeed.feedCreators(operatorAccount.address);
        if (feedCreator.isNone) {
            await api.tx.chainlinkFeed.setFeedCreator(operatorAccount.address).signAndSend(aliceAccount, async ({status}) => {
                if (status.isFinalized) {
                    console.log('Feed creator registered');
                    resolve();
                }
            });
        } else {
            console.log('Feed creator already registered');
            resolve();
        }
    });
}
async function createFeed(api, sender) {
    console.log(`Creating feed with config: ${JSON.stringify(feedConfig, null, 4)}`);

    // make the pallet admin also the oracle admin
    const palletAdmin =  await api.query.chainlinkFeed.palletAdmin();
    feedConfig.oracles = feedConfig.oracles.map(oracle => [oracle, palletAdmin]);

    return new Promise(async (resolve) => {
    await api.tx.chainlinkFeed.createFeed(feedConfig.payment, feedConfig.timeout, feedConfig.submissionValueBounds, feedConfig.minSubmissions, feedConfig.decimals, feedConfig.description, feedConfig.restartDelay,feedConfig.oracles,feedConfig.pruningWindow,feedConfig.maxDebt).signAndSend(sender, ({ status, events }) => {
        if (status.isInBlock || status.isFinalized) {
          events
            // find/filter for failed events
            .filter(({ event }) =>
              api.events.system.ExtrinsicFailed.is(event)
            )
            // we know that data for system.ExtrinsicFailed is
            // (DispatchError, DispatchInfo)
            .forEach(({ event: { data: [error, info] } }) => {
              if (error.isModule) {
                // for module errors, we have the section indexed, lookup
                const decoded = api.registry.findMetaError(error.asModule);
                const { documentation, method, section } = decoded;
    
                console.log(`${section}.${method}: ${documentation.join(' ')}`);
              } else {
                // Other, CannotLookup, BadOrigin, no extra info
                console.log(error.toString());
              }
            });
        }
        if (status.isFinalized) {
            resolve()
        }
      });
    });
}


async function main() {
    await cryptoWaitReady();

    // Connect to the local chain
    const wsProvider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({
        provider: wsProvider,
        types
    });

    // Add an account, straight from mnemonic
    const keyring = new Keyring({type: 'sr25519'});

    for (feedConfig of feedConfigs) {
        const operatorAccount = keyring.addFromUri(feedConfig.operatorSeedPhrase);
        const oracleAddress1 = feedConfig.oracles[0]
        const oracleAddress2 = feedConfig.oracles[1]
    
        console.log(`Using operator with address ${operatorAccount.address}`);
       
        const aliceAccount = keyring.addFromUri('//Alice');


        await fundAccountIfNeeded(api, aliceAccount, operatorAccount.address);
    
        await fundAccountIfNeeded(api, aliceAccount, oracleAddress1);
    
        await fundAccountIfNeeded(api, aliceAccount, oracleAddress2);
    
        await registerFeedCreatorIfNeeded(api, aliceAccount, operatorAccount);
      
        await createFeed(api, operatorAccount);

        // const feed = await api.query.chainlinkFeed.feeds(0)
        // console.log(feed)
    }
  
}

main().catch(console.error).then(() => process.exit());
