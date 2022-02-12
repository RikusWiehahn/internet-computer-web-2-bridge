import "dotenv/config";
import http from "http";
global.fetch = require("node-fetch");
import cors from "cors";
import express, { Request, Response } from "express";
import { urlencoded, json } from "body-parser";
import { pollForRequests } from "./process-requests";

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
  pollForRequests();

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
