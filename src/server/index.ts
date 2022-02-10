import "dotenv/config";
import http from "http";
global.fetch = require("node-fetch");
import express, { Request, Response } from "express";
import { urlencoded, json } from "body-parser";
import cors from "cors";
import { Actor } from "@dfinity/agent";
import { HttpAgent } from "@dfinity/agent";
// @ts-ignore
import { idlFactory } from "../declarations/bridge/bridge.did.js";

export let canisterId = "";
export let host = "";
if (process.env.NODE_ENV === "production") {
  // we are in production
  canisterId = process.env.PRODUCTION_CANISTER_ID || ""; // live bridge canister id
  host = process.env.PRODUCTION_HOST || ""; // live bridge canister url
} else {
  host = process.env.LOCAL_DEV_HOST || ""; // http://localhost:8000
  canisterId = process.env.LOCAL_DEV_CANISTER_ID || ""; // your local dev bridge canister id
}

const agent = new HttpAgent({ host });
const actor = Actor.createActor(idlFactory, {
  agent,
  canisterId,
});

let callCanister = async () => {
  try {
    // get latest request
    const res = await actor.greet("Hello, world!");
    console.log(res);
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
      <head>
      <meta charset=utf-8>
      <title>API</title>
      </head>
      <body>
      <h3 style="font-family:sans-serif;text-align:center;">I'm an API 🤖!</h3>
      </body>
    </html>`
    )
  );
} catch (e: any) {
  console.log({ e });
}
