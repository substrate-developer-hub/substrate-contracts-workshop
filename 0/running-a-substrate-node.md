Running a Canvas Node
===

After successfully installing `canvas`, you can start a local development chain by running:

```bash
canvas --dev
```

> **Note:** If you have run this command in the past, you probably want to purge your chain so that you run through this tutorial with a clean slate. You can do this easily with `canvas purge-chain --dev`.

![An image of the terminal starting a Substrate node](./assets/start-substrate-node.png)

You should start to see blocks being produced by your node in your terminal.

You can interact with your node using the Polkadot UI:

https://polkadot.js.org/apps/

> **Note:** You will need to use a Chromium based browser (Google Chrome) to have this site interact with your local node. The Polkadot UI is hosted on a secure server, and your local node is not, which may cause compatibility issues on Firefox. The other option is to [clone and run the Polkadot UI locally](https://github.com/polkadot-js/apps).

To point the UI to your local node, you need to adjust the **Settings**. Just select 'Local Node (127.0.0.1:9944)' from the endpoint dropdown:

```
Click on the chain name > Development node/endpoint to connect to > Local Node (127.0.0.1:9944) > Switch 🗘
```

![An image of the settings in Polkadot-JS Apps UI](./assets/polkadot-js-settings.png)

>**Important:** The UI needs a data type override for **v2.0.0-rc4** and older versions. The contracts pallet (SEAL) is getting some breaking changes to adjust it for the next version of ink!
>Go to **Settings > Developer** and define the type override.
```JSON
{
  "ContractExecResult": "ContractExecResultTo255"
}
```

If you go into the **Explorer** tab of the UI, you should also see blocks being produced!

![An image of the Substrate UI](./assets/start-substrate-ui.png)
