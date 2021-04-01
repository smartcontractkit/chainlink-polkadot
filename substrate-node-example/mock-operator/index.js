const { ApiPromise, Keyring, WsProvider } = require('@polkadot/api');
const { cryptoWaitReady }  = require('@polkadot/util-crypto');

const PHRASE = 'entire material egg meadow latin bargain dutch coral blood melt acoustic thought';

async function fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount) {
  // TODO migrate to 'system.account' ? See https://github.com/paritytech/substrate/pull/4820
  return new Promise(async (resolve) => {
    const balance = await api.query.balances.freeBalance(operatorAccount.address);
    if (balance.isZero()) {
        await api.tx.balances.transfer(operatorAccount.address, 123456666).signAndSend(aliceAccount, async ({ status }) => {
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
    if(operator.isFalse) {
        await api.tx.chainlink.registerOperator().signAndSend(operatorAccount, async ({ status }) => {
          if (status.isFinalized) {
            console.log('Operator registered');
            resolve();
          }
        });
    } else {
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
    const keyring = new Keyring({ type: 'sr25519' });
    const operatorAccount = keyring.addFromUri(PHRASE);
    console.log(`Imported operator with address ${operatorAccount.address}`);

    // Make sure this operator has some funds
    const aliceAccount = keyring.addFromUri('//Alice');

    await fundOperatorAccountIfNeeded(api, aliceAccount, operatorAccount);

    const result = await api.query.example.result();
    console.log(`Result is currently ${result}`);

    // Listen for chainlink.OracleRequest events
    api.query.system.events((events) => {
        events.forEach(({ event })  => {
          if (event.section == "chainlink" && event.method == "OracleRequest") {
            const id = event.data[2].toString();
            const value = Math.floor(Math.random() * Math.floor(100));
            const result = api.createType('i128', value).toHex(true);
            // Respond to the request with a dummy result
            api.tx.chainlink.callback(parseInt(id), result).signAndSend(operatorAccount, async ({ events = [], status }) => {
                if (status.isFinalized) {
                  const updatedResult = await api.query.example.result();
                  console.log(`Result is now ${updatedResult}`);
                  process.exit();
                }
              });
            console.log(`Operator answered to request ${id} with ${value}`);
        }
      });
    });

    await registerOperatorIfNeeded(api, operatorAccount);

    // Then simulate a call from alice
    await api.tx.example.sendRequest(operatorAccount.address, "").signAndSend(aliceAccount);
    console.log(`Request sent`);
}

main().catch(console.error)