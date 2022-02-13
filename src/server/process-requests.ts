
import { actor } from "./actor";
import axios from "axios";
import { StoredHttpRequest, StoredHttpResponse } from "../declarations/bridge/bridge.did.js";

export const pollForRequests = async () => {
  try {
    await fetchAndProcessWebRequests();
  } catch (e: any) {
    console.log(e);
  }
  setTimeout(pollForRequests, 2000);
};

const fetchAndProcessWebRequests = async () => {
  console.log("Fetching web requests");
  try {
    const access_key = process.env.BRIDGE_ACCESS_KEY || "";
    const pulled_web_requests = await actor.pull_web_requests(access_key);
    if (pulled_web_requests.requests.length > 0) console.log(pulled_web_requests);

    for (const req of pulled_web_requests.requests) {
      const result = await processWebRequest(req);
      await actor.push_web_response(access_key, result);
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
    created_at: new Date().valueOf(),
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
    res.headers = Object.fromEntries(req.headers);
    return res;
  } catch (e: any) {
    console.log({ e });
    res.body = e.message;
    res.status_code = BigInt(400);
    return res;
  }
};
