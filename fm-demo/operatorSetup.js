const {ApiPromise, Keyring, WsProvider} = require('@polkadot/api');
const {cryptoWaitReady} = require('@polkadot/util-crypto');

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount) {
    return new Promise(async (resolve) => {
        const balance = await api.query.system.account(operatorAccount.address);
        console.log(`Free balance is: ${balance.data.free}`);
        if (parseInt(balance.data.free) === 0) {
            await api.tx.balances.transfer(operatorAccount.address, 123456666000).signAndSend(aliceAccount, async ({status}) => {
                if (status.isFinalized) {
                    console.log("OK!")
                    resolve();
                }
            });
            // TODO rather than waiting arbitrarily, find a more proactive approach to
            //  the block being included
            await sleep(6000);
            console.log('Operator funded');
        } else {
            resolve();
        }
    });
}

async function registerOperatorIfNeeded(api, operatorAccount) {
    // Register the operator, this is supposed to be initiated once by the operator itself
    console.log(`Registering operator ${operatorAccount.address}`);
    return new Promise(async (resolve) => {
        const operator = await api.query.chainlink.operators(operatorAccount.address);
        if (operator.isFalse) {
            await api.tx.chainlink.registerOperator().signAndSend(operatorAccount, async ({status}) => {
                if (status.isFinalized) {
                    console.log('Operator registered');
                    resolve();
                }
            });
    
        } else {
            console.log('Operator already registered');
            resolve();
        }
    });
}
async function createFeed(api, sender) {
    console.log(`Creating feed`);
    return new Promise(async (resolve) => {
    await api.tx.chainlinkFeed.createFeed(0.01, 600, (0, 99999999999999999999999999999999), 1, 8, "Test", 0, ["0x7c522c8273973e7bcf4a5dbfcc745dba4a3ab08c1e410167d7b1bdf9cb924f6c", "0x06f0d58c43477508c0e5d5901342acf93a0208088816ff303996564a1d8c1c54"],1,0).signAndSend(sender, ({ status, events }) => {
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
    console.log(`Imported operator with address ${operatorAccount.address}`);

    // Make sure this operator has some funds
    const aliceAccount = keyring.addFromUri('//Alice');

    await fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount);

    await registerOperatorIfNeeded(api, operatorAccount);
  
    await createFeed(api, aliceAccount);
}

main().catch(console.error).then(() => process.exit());
