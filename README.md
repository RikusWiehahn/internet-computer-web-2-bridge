# Web 2 Bridge for IC Dapps
# Running the project

Start the bridge canister:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.


Run the nodejs server

```bash

npm install

npm run build

npm start

```

Call the bridge canister with the request you would like to send to an off-chain service. The bridge canister will stash the request in memory and send back a request id.

```rs

```
The off-chain node.js server will periodically pull the latest requests from the bridge canister, process them and send the response back to the bridge canister.

The response can then be pulled from the bridge canister using the request's id