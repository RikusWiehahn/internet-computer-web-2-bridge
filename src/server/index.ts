import "dotenv/config";
import http from "http";
global.fetch = require("node-fetch");
import express, { Request, Response } from "express";
import { urlencoded, json } from "body-parser";
import cors from "cors";
import { HttpAgent, Actor, ActorSubclass } from "@dfinity/agent";
// @ts-ignore
import { idlFactory, StoredHttpResponse, _SERVICE } from "../declarations/bridge/bridge.did.js";
import axios from "axios";

let canisterId = "";
let host = "";
if (process.env.NODE_ENV === "production") {
  canisterId = process.env.PRODUCTION_CANISTER_ID || "";
  host = process.env.PRODUCTION_HOST || "";
} else {
  host = process.env.LOCAL_DEV_HOST || "";
  canisterId = process.env.LOCAL_DEV_CANISTER_ID || "";
}

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

const actor: import("@dfinity/agent").ActorSubclass<
  import("../declarations/bridge/bridge.did.js")._SERVICE
> = createActor();

let callCanister = async () => {
  try {
    const test = await actor.ping();
    console.log(test);

    await fetchAndProcessWebRequests();
  } catch (e: any) {
    console.log(e);
  }
  setTimeout(callCanister, 2000);
};
callCanister();

const app = express();
const httpServer = http.createServer(app);

// middle wares for express
app.use(urlencoded({ extended: true, limit: "2mb" }));
app.use(json({ limit: "2mb" }));
app.use(cors({ origin: true }));

try {
  const PORT = process.env.PORT || 4000;
  httpServer.listen(PORT, () => {
    console.log(`Listening on port ${PORT}`);
  });

  // http api
  app.get("/api", async (req: Request, res: Response) => {
    try {
      res.send("Incoming bridge not implemented yet!");
    } catch (e: any) {
      console.log({ e });
      res.status(400).send(e.message);
    }
  });

  // home page
  app.get("/", (req, res) =>
    res.send(
      `<!doctype html>
      <html lang=en>
      <head><meta charset=utf-8><title>API</title></head>
      <body>
      <h2 style="font-family:sans-serif;text-align:center;">I'm an API 🤖!</h2>
      </body>
    </html>`
    )
  );
} catch (e: any) {
  console.log({ e });
}

const fetchAndProcessWebRequests = async () => {
  try {
    const requests = await actor.pull_web_requests(
      process.env.BRIDGE_ACCESS_KEY || ""
    );

    for (const request of requests.requests) {
      let response: StoredHttpResponse = {
        'id' : request.id,
        'body' : '',
        'headers' : [],
        'created_at' : new Date().valueOf(),
        'status_code' : BigInt(400),
      }
      if (
        request.method !== "GET" &&
        request.method !== "POST" &&
        request.method !== "PUT" &&
        request.method !== "DELETE"
      ) {
        // SEND RESPONSE
        continue;
      }
      const url = request.url;
      const result = await axios({
        url: request.url,
        method: request.method,
        data: JSON.parse(request.body),
      });
    }
    console.log({ requests });
  } catch (e: any) {
    console.log({ e });
  }
};
