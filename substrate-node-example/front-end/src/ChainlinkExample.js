import React, { useState } from 'react';
import { Form, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

function Main (props) {
  const { api } = useSubstrate();
  const [status, setStatus] = useState(null);
  const { accountPair } = props;

  return (
    <Grid.Column>
      <h1>Chainlink</h1>
      <Form>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Initiate Request'
            setStatus={setStatus}
            type='TRANSACTION'
            attrs={{
              tx: api.tx.example.sendRequest
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      </Form>
    </Grid.Column>
  );
}

export default function ChainlinkExample (props) {
  const { api } = useSubstrate();
  return (api.query.balances && api.tx.balances.transfer
    ? <Main {...props} /> : null);
}
