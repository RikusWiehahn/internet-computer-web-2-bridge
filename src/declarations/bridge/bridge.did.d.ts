import type { Principal } from '@dfinity/principal';
export type HeaderField = Array<string>;
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : string,
  'headers' : Array<HeaderField>,
}
export interface HttpResponse {
  'body' : string,
  'headers' : Array<HeaderField>,
  'status_code' : bigint,
}
export interface PulledWebRequests {
  'message' : string,
  'requests' : Array<StoredHttpRequest>,
}
export interface PulledWebResponse {
  'pending' : boolean,
  'message' : string,
  'response' : [] | [HttpResponse],
}
export interface PushedWebRequest { 'id' : [] | [string], 'message' : string }
export interface PushedWebResponse { 'id' : [] | [string], 'message' : string }
export interface StoredHttpRequest {
  'id' : string,
  'url' : string,
  'method' : string,
  'body' : string,
  'headers' : Array<HeaderField>,
  'pulled' : boolean,
}
export interface StoredHttpResponse {
  'id' : string,
  'body' : string,
  'headers' : Array<HeaderField>,
  'created_at' : number,
  'status_code' : bigint,
}
export interface _SERVICE {
  'ping' : () => Promise<string>,
  'pull_web_requests' : (arg_0: string) => Promise<PulledWebRequests>,
  'pull_web_response' : (arg_0: string, arg_1: string) => Promise<
      PulledWebResponse
    >,
  'push_web_request' : (arg_0: string, arg_1: HttpRequest) => Promise<
      PushedWebRequest
    >,
  'push_web_response' : (arg_0: string, arg_1: StoredHttpResponse) => Promise<
      PushedWebResponse
    >,
}
