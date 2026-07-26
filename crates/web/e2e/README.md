# web/e2e

Headless-Chrome regression scripts for the `web/` app. Not part of the Rust
build or `cargo test` — these drive the actual running app in a browser via
`puppeteer-core`, since UI/typing bugs (caret handling, cursor restoration,
input types) don't show up in Rust unit tests.

## Setup

```sh
cd web/e2e
npm install
```

Requires a local Chrome/Chromium install. Defaults to
`/Applications/Google Chrome.app/Contents/MacOS/Google Chrome` (macOS); override
with `CHROME_PATH=/path/to/chrome`.

## Running

In one terminal:

```sh
cd web && trunk serve
```

In another:

```sh
cd web/e2e && npm test
```

Or run a single file: `node typing.test.js`. Override the app URL with
`APP_URL=http://localhost:8080` (that's already the default).

## Files

- `helpers.js` — shared launch/assert helpers.
- `typing.test.js` — IBAN typing at various speeds must produce the exact
  digits typed (regression for a caret-remapping bug where the space-masked
  display scrambled input).
- `reference.test.js` — QR/SCOR reference-number auto-proposal, clearing on
  IBAN-kind switch, and validation errors.
- `smoke.test.js` — one broad pass over the rest of the app (typing with
  special characters, PLZ autofill, debtor toggle, country picker, amount,
  language switch, PDF download).
