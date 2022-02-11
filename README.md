# Web 2 Bridge for IC Dapps

## In progress!
## Running the project

Start the bridge canister:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Create a file called .env in your project root directory and save the generated bridge canister id (found in `.dfx/local/canister_ids.json`) to the LOCAL_DEV_CANISTER_ID key:

```.env
LOCAL_DEV_CANISTER_ID="<YOUR_LOCAL_CANISTER_ID"
PRODUCTION_CANISTER_ID="<YOUR_LIVE_IC_CANISTER_ID>"
LOCAL_DEV_HOST="http://localhost:8000"
PRODUCTION_HOST="<YOUR_LIVE_IC_CANISTER_URL>"
BRIDGE_ACCESS_KEY="<RANDOM_KEY_FOR_SECURE_REQUESTS_OVER_THE_BRIDGE>"
```

Create a random key string using a ssh keygen or some other method and paste it into the BRIDGE_ACCESS_KEY variable in your .env file.

Run the nodejs server

```bash

npm install

npm run build

npm start

```

Now you are ready to test the bridge.

Call the bridge canister with the request you would like to send to an off-chain service. The bridge canister will stash the request in memory and send back a request id.

```rs

```
The off-chain node.js server will periodically pull the latest requests from the bridge canister, process them and send the response back to the bridge canister.

The response can then be pulled from the bridge canister using the request's id