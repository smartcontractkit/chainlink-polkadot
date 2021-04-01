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
            Address: "AccountId",
            LookupSource: "AccountId"
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
}

main().catch(console.error).then(() => process.exit());
