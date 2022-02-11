export const idlFactory = ({ IDL }) => {
  const HeaderField = IDL.Vec(IDL.Text);
  const StoredHttpRequest = IDL.Record({
    'id' : IDL.Text,
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Text,
    'headers' : IDL.Vec(HeaderField),
    'pulled' : IDL.Bool,
  });
  const PulledWebRequests = IDL.Record({
    'message' : IDL.Text,
    'requests' : IDL.Vec(StoredHttpRequest),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Text,
    'headers' : IDL.Vec(HeaderField),
    'status_code' : IDL.Nat,
  });
  const PulledWebResponse = IDL.Record({
    'pending' : IDL.Bool,
    'message' : IDL.Text,
    'response' : IDL.Opt(HttpResponse),
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Text,
    'headers' : IDL.Vec(HeaderField),
  });
  const PushedWebRequest = IDL.Record({
    'id' : IDL.Opt(IDL.Text),
    'message' : IDL.Text,
  });
  const StoredHttpResponse = IDL.Record({
    'id' : IDL.Text,
    'body' : IDL.Text,
    'headers' : IDL.Vec(HeaderField),
    'created_at' : IDL.Float64,
    'status_code' : IDL.Nat,
  });
  const PushedWebResponse = IDL.Record({
    'id' : IDL.Opt(IDL.Text),
    'message' : IDL.Text,
  });
  return IDL.Service({
    'ping' : IDL.Func([], [IDL.Text], ['query']),
    'pull_web_requests' : IDL.Func([IDL.Text], [PulledWebRequests], []),
    'pull_web_response' : IDL.Func(
        [IDL.Text, IDL.Text],
        [PulledWebResponse],
        [],
      ),
    'push_web_request' : IDL.Func(
        [IDL.Text, HttpRequest],
        [PushedWebRequest],
        [],
      ),
    'push_web_response' : IDL.Func(
        [IDL.Text, StoredHttpResponse],
        [PushedWebResponse],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
