const {ApiPromise, Keyring, WsProvider} = require('@polkadot/api');
const {cryptoWaitReady} = require('@polkadot/util-crypto');

async function fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount) {
    // TODO migrate to 'system.account' ? See https://github.com/paritytech/substrate/pull/4820
    return new Promise(async (resolve) => {
        const balance = await api.query.balances.freeBalance(operatorAccount.address);
        if (balance.isZero()) {
            await api.tx.balances.transfer(operatorAccount.address, 123456666).signAndSend(aliceAccount, async ({status}) => {
                if (status.isFinalized) {
                    resolve();
                }
            });
            // TODO make sure we wait for block being included
            console.log('Operator funded');
        } else {
            resolve();
        }
    });
}

async function registerOperatorIfNeeded(api, operatorAccount) {
    // Register the operator, this is supposed to be initiated once by the operator itself
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
            DataVersion: "u64"
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
