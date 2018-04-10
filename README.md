# wurl-auth [![Build Status](https://travis-ci.org/getwurl/wurl-auth.svg?branch=master)](https://travis-ci.org/getwurl/wurl-auth) [<img src="https://github.com/getwurl/wurl/raw/master/assets/logo.png" width="280" align="right" alt="wurl">](https://github.com/getwurl/wurl)

> Generate [wurl][wurl] authentication headers automatically

Often WebSocket endpoints are behind authentication and require a header to
access. The most common example of this is a login form which sets a cookie in
the browser. This utility allows you to fake a login and turn the `Set-Cookie`
header into a `Cookie` header which can be set in [wurl][wurl] to allow access.

Authenticates by fetching an URL and printing a cookie header using the Set-Cookie header from the response.

## Usage

```
USAGE:
    wurl-auth [FLAGS] [OPTIONS] <URL>
FLAGS:
    -h, --help       Prints help information
    -i, --include    Include the HTTP headers in the output
    -V, --version    Prints version information
OPTIONS:
    -d, --data <data>                 Set HTTP request data
    -H, --header <header:value>...    Adds headers to any HTTP request made
    -X, --method <METHOD>             Sets the HTTP request verb
ARGS:
    <URL>    URL of the server to connect to
```

## Example

```
wurl -H $(wurl-auth https://login.example.com) wss://websocket.example.com
```

[wurl]: https://github.com/getwurl/wurl
