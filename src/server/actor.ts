import { HttpAgent, Actor, ActorSubclass } from "@dfinity/agent";
import {
  // @ts-ignore
  idlFactory, 
  _SERVICE,
} from "../declarations/bridge/bridge.did.js";
import { canisterId, host } from "./keys";

const createActor = (): ActorSubclass<_SERVICE> => {
  if (!canisterId) throw new Error("Canister id is not set");
  if (!host) throw new Error("Host is not set");
  const agent = new HttpAgent({ host });
  // Fetch root key for certificate validation during development
  if (process.env.NODE_ENV !== "production") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }
  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });
};

export const actor: import("@dfinity/agent").ActorSubclass<
  import("../declarations/bridge/bridge.did.js")._SERVICE
> = createActor();