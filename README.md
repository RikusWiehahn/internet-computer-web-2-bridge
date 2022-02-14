# Web 2 Bridge for IC Dapps 🌉

This repo lets you make http JSON requests from the internet computer blockchain to the regular internet 🙌

## How it works

1. Submit an http request from any IC canister to the bridge canister. The bridge canister then saves the request in memory and returns an `id` for the request.
2. An off-chain Node.js server continually pulls http requests from the bridge canister and processes them.
3. The off-chain server sends the responses back to the bridge canister, which then saves the response in memory.
4. The original requesting canister can then fetch the response to it's http request from the bridge canister via the `id` that was initially returned.

# Run it locally

🚧 Please note this is a work in progress and hasn't been deployed on the main Internet Computer network yet!

## Configuration

This guide assumes you have the usual dev-tools installed, node, npm, dfx etc.

Start by cloning this repo to your machine.

### 1. Create the canisters

Open a terminal at the project root directory and run the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploy your bridge and demo canister
dfx canister create --all
```

### 2. Create .env file

Create a file named `.env` in this repo's root directory and paste in the following:

```env
LOCAL_DEV_BRIDGE_CANISTER_ID = "<YOUR_LOCAL_BRIDGE_CANISTER_ID>"
PRODUCTION_BRIDGE_CANISTER_ID = "<YOUR_LIVE_IC_BRIDGE_CANISTER_ID>"
LOCAL_DEV_HOST = "http://localhost:8000"
PRODUCTION_HOST = "<YOUR_LIVE_IC_CANISTER_URL>"
BRIDGE_ACCESS_CODE = "<FOR_SECURE_REQUESTS_OVER_THE_BRIDGE>"
```

- Replace `<YOUR_LOCAL_DEV_BRIDGE_CANISTER_ID>` with the newly generated **bridge** canister id (in `.dfx/local/canister_ids.json`).

- Replace `<FOR_SECURE_REQUESTS_OVER_THE_BRIDGE>` with a random access code (using an ssh keygen etc).

### 3. Keys files

Create a file called `keys.rs` in the bridge canister's folder (`src/bridge`) and paste in the following:

```rs

pub const BRIDGE_ACCESS_CODE: &str = "<KEY_FOR_SECURE_REQUESTS_OVER_THE_BRIDGE>";

```

- Replace `<KEY_FOR_SECURE_REQUESTS_OVER_THE_BRIDGE>` with the random access code you generated earlier.

Create a file called `keys.rs` in the demo canister's folder (`src/demo`) and paste in the following:

```rs
pub const BRIDGE_ACCESS_CODE: &str = "<KEY_FOR_SECURE_REQUESTS_OVER_THE_BRIDGE>";
pub const DEV_DEMO_CANISTER_ID: &str = "<THIS_CANISTERS_LOCAL_DEV_ID>";
pub const DEV_BRIDGE_CANISTER_ID: &str = "<YOUR_LOCAL_DEV_BRIDGE_CANISTER_ID>";
pub const PROD_DEMO_CANISTER_ID: &str = "<THIS_CANISTERS_PRODUCTION_ID>";
pub const PROD_BRIDGE_CANISTER_ID: &str = "<YOUR_LIVE_BRIDGE_CANISTER_ID>";
```

- Replace `<KEY_FOR_SECURE_REQUESTS_OVER_THE_BRIDGE>` with the random access code you generated earlier.
- Replace `<THIS_CANISTERS_LOCAL_DEV_ID>` with the **demo** canister id in (`.dfx/local/canister_ids.json`).
- Replace `<YOUR_LOCAL_DEV_BRIDGE_CANISTER_ID>` with the **bridge** canister id in (`.dfx/local/canister_ids.json`).

### 4. Deploy the canisters

Redeploy the canisters now that they are configured:

```bash

# Deploy your bridge and demo canister
dfx deploy

```

### 5. Start the nodejs server

Open a new terminal window and run the following commands:

```bash

npm install

npm run build

npm start

```

- The response may take a while, the demo canister automatically submits the http request every 2 minutes so be patient!
- You should see the local replica print out the api response from the demo api call:

CoinGecko API price info:

`https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd,aud,eur,brl,cad,dkk,hkd,jpy,nzd,nok,sgd,sek,chf,gbp`

```bash

response: Object({"bitcoin": Object({"aud": Number(58993), "brl": Number(220562), "cad": Number(53472), "chf": Number(38845), "dkk": Number(275372), "eur": Number(37017), "gbp": Number(31021), "hkd": Number(327547), "jpy": Number(4849976), "nok": Number(372459), "nzd": Number(63431), "sek": Number(392317), "sgd": Number(56558), "usd": Number(41989)})})

```

### 6. Production

In progress!

- You should be able to run this Node server on any app hosting service like Heroku, Digital Ocean etc without too much hassle.

### 7. Improvements

If you know how to improve this repo please make a pull request! **Especially** if you know how to:

- Simplify config (less manual copy/pasting)
- Improve efficiency (does IC have a delay function like the thread::sleep function?). The current method of submitting a request to the bridge and using a counter to poll for the response is probably very wasteful on cycles.
