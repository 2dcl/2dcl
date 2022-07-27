# WebView Runtime for Decentraland Kernel

Similar to what the official desktop client does, we're using a webview to run 
the decentraland kernel in a separate process.

We use wry to create a hidden webview that connects to the client with 
websockets.

## Protocol

Decentraland relies on some data structures defined in the 
[protocol](https://github.com/decentraland/protocol) repository.

If you want to re-build these, make sure yo have the repository clones alongside
`decentraland-rs` and run `bin/update-protocol`.
