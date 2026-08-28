# Network Safety and Local AI Overlay

Kerosene selects one trusted Hyperliquid environment for native `/info`,
`/exchange`, and WebSocket traffic. Existing configurations default to
**Mainnet**. **TESTNET - simulated funds** is conspicuously labeled and uses
only Hyperliquid's fixed testnet endpoints; users cannot configure an arbitrary
exchange target.

The selected network is captured with asynchronous market, account, stream, and
order work. Switching is refused while exchange requests, Chase orders, or
non-terminal TWAP orders are active. A permitted switch increments the network
generation, clears native market/account caches, rejects stale results, and
reloads data. L1 signing retains chain ID `1337` and binds its phantom-agent
source to Mainnet (`a`) or Testnet (`b`), so signed exchange traffic follows the
same trusted network context.

## Local AI status

The always-visible **LOCAL AI - READ ONLY** overlay is informational. It cannot
create, prefill, sign, submit, cancel, or alter an order. Its configurable
ordinary preference defaults to `http://127.0.0.1:8765` and permits only plain
HTTP loopback bases (`127.0.0.1`, `localhost`, or `[::1]`) without credentials,
paths, queries, or fragments.

Kerosene polls only `GET /v1/status` with no request body and sends no wallet,
account, credential, order, or signed data. The service may return:

```json
{ "decision": "Optional bounded informational text" }
```

An omitted, empty, unreachable, or malformed decision is displayed honestly as
no decision, disconnected, or service error. The local-service address and
network preference are not secrets; API keys and private key material continue
to use the existing OS-keychain/encrypted-config secret flows.
