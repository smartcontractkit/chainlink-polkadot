const {ApiPromise, Keyring, WsProvider} = require('@polkadot/api');
const {cryptoWaitReady} = require('@polkadot/util-crypto');
const feedConfig = require('./feed.json');

// function sleep(ms) {
//   return new Promise(resolve => setTimeout(resolve, ms));
// }

async function fundAccountIfNeeded(api, sender, receiver) {
    return new Promise(async (resolve) => {
        const balance = await api.query.system.account(receiver.address);
        console.log(`Free balance of ${receiver.address} is: ${balance.data.free}`);
        if (parseInt(balance.data.free) === 0) {
            await api.tx.balances.transfer(receiver.address, 123456666000).signAndSend(sender, async ({status}) => {
                if (status.isFinalized) {
                    console.log(`Account ${receiver.address} funded`);
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
    return new Promise(async (resolve) => {
    await api.tx.chainlinkFeed.createFeed(feedConfig.payment, feedConfig.timeout, (feedConfig.submissionValueBounds[0], feedConfig.submissionValueBounds[1]), feedConfig.minSubmissions, feedConfig.decimals, feedConfig.description, feedConfig.restartDelay, feedConfig.oracles,feedConfig.pruningWindow,feedConfig.maxDebt).signAndSend(sender, ({ status, events }) => {
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
        types: {
            SpecIndex: "Vec<u8>",
            RequestIdentifier: "u64",
            DataVersion: "u64",
            Address: "MultiAddress",
            LookupSource: "MultiAddress",
            FeedId: "u32",
            RoundId: "u32",
            Value: "u128",
            FeedConfig: {
                owner: "AccountId",
                pending_owner: "Option<AccountId>",
                submission_value_bounds: "(Value, Value)",
                submission_count_bounds: "(u32, u32)",
                payment: "Balance",
                timeout: "BlockNumber",
                decimals: "u8",
                description: "Vec<u8>",
                restart_delay: "RoundId",
                reporting_round: "RoundId",
                latest_round: "RoundId",
                first_valid_round: "Option<RoundId>",
                oracle_count: "u32"
            },
            FeedConfigOf: "FeedConfig",
            Round: {
                started_at: "BlockNumber",
                answer: "Option<Value>",
                updated_at: "Option<BlockNumber>",
                answered_in_round: "Option<RoundId>"
            },
            RoundOf: "Round",
            RoundDetails: {
                submissions: "Vec<Value>",
                submission_count_bounds: "(u32, u32)",
                payment: "Balance",
                timeout: "BlockNumber"
            },
            RoundDetailsOf: "RoundDetails",
            OracleMeta: {
                withdrawable: "Balance",
                admin: "AccountId",
                pending_admin: "Option<AccountId>"
            },
            OracleMetaOf: "OracleMeta",
            OracleStatus: {
                starting_round: "RoundId",
                ending_round: "Option<RoundId>",
                last_reported_round: "Option<RoundId>",
                last_started_round: "Option<RoundId>",
                latest_submission: "Option<Value>"
            },
            OracleStatusOf: "OracleStatus",
            Requester: {
                delay: "RoundId",
                last_started_round: "Option<RoundId>"
            },
            RoundData: {
                started_at: "BlockNumber",
                answer: "Value",
                updated_at: "BlockNumber",
                answered_in_round: "RoundId"
            },
            RoundDataOf: "RoundData",
            SubmissionBounds: "(u32, u32)"
        }
    });

    // Add an account, straight from mnemonic
    const keyring = new Keyring({type: 'sr25519'});
    const operatorAccount = keyring.addFromUri(process.argv[2]);
    const oracleAccount1 = keyring.addFromUri(process.argv[3]);
    const oracleAccount2 = keyring.addFromUri(process.argv[4]);

    console.log(`Imported operator with address ${operatorAccount.address}`);
    console.log(`Imported oracle 1 with address ${oracleAccount1.address}`);
    console.log(`Imported oracle 2 with address ${oracleAccount2.address}`);

    const aliceAccount = keyring.addFromUri('//Alice');

    await fundAccountIfNeeded(api, aliceAccount, operatorAccount);

    await fundAccountIfNeeded(api, aliceAccount, oracleAccount1);

    await fundAccountIfNeeded(api, aliceAccount, oracleAccount2);

    await registerFeedCreatorIfNeeded(api, aliceAccount, operatorAccount);
  
    await createFeed(api, operatorAccount);
}

main().catch(console.error).then(() => process.exit());
