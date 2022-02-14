import { actor } from "./actor";
import axios from "axios";
import {
  StoredHttpRequest,
  StoredHttpResponse,
} from "../declarations/bridge/bridge.did.js";

export const pollForRequests = async () => {
  try {
    await fetchAndProcessWebRequests();
  } catch (e: any) {
    console.log(e);
  }
  setTimeout(pollForRequests, 1000);
};

const fetchAndProcessWebRequests = async () => {
  // console.log("Fetching web requests");
  try {
    const access_key = process.env.BRIDGE_ACCESS_CODE || "";
    const pulled_web_requests = await actor.pull_web_requests(access_key);
    let num_requests = pulled_web_requests.requests.length;
    if (num_requests > 0) console.log(`${num_requests} requests pulled`);

    for (const req of pulled_web_requests.requests) {
      const result = await processWebRequest(req);
      let saved = await actor.push_web_response(access_key, result);
      if (saved) console.log(`1 result pushed`);
    }
  } catch (e: any) {
    console.log({ e });
  }
};

type ProcessWebRequest = (
  req: StoredHttpRequest
) => Promise<StoredHttpResponse>;

const processWebRequest: ProcessWebRequest = async (req) => {
  let res: StoredHttpResponse = {
    id: req.id,
    body: "",
    headers: [],
    created_at: new Date().valueOf() * 1000000,
    status_code: BigInt(400),
  };
  try {
    if (
      req.method !== "GET" &&
      req.method !== "POST" &&
      req.method !== "PUT" &&
      req.method !== "DELETE"
    ) {
      res.body = "Method not supported";
      res.status_code = BigInt(405);
      return res;
    }
    const result = await axios({
      url: req.url,
      method: req.method,
      headers: Object.fromEntries(req.headers),
      data: JSON.parse(req.body),
    });

    res.body = JSON.stringify(result.data);
    res.status_code = BigInt(200);
    res.headers = req.headers;
    return res;
  } catch (e: any) {
    console.log({ e });
    res.body = e.message;
    res.status_code = BigInt(400);
    return res;
  }
};
