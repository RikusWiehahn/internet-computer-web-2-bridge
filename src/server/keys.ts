export let canisterId = "";
export let host = "";
if (process.env.NODE_ENV === "production") {
  canisterId = process.env.PRODUCTION_BRIDGE_CANISTER_ID || "";
  host = process.env.PRODUCTION_HOST || "";
} else {
  host = process.env.LOCAL_DEV_HOST || "";
  canisterId = process.env.LOCAL_DEV_BRIDGE_CANISTER_ID || "";
}