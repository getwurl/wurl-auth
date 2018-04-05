# wurl-auth

Authenticates by fetching an URL and printing a cookie header using the Set-Cookie header from the response.

```
wurl -H $(wurl-auth https://httpbin.org/cookies/set\?foo=bar) -I wss://echo.websocket.org
```

## Improvments
- [ ] Add -H to set arbitrary headers
- [ ] Add -X to set request method
- [ ] Add -d for sending data with the request.
- [ ] Send data via stdin (for example `wurl-auth example.com < login.json`)
- [ ] Make issues for other common authentication methods (basic auth, authorization, jwt..)
