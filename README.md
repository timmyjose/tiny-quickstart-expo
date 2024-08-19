This is a port of the [React Native Tiny Quickstart](https://github.com/plaid/tiny-quickstart/tree/main/react_native) demo to an `expo` (CNG) app.

## Build and Run

### Setup

```bash
$ yarn setup --clean
```

### Start the server

Environment variables:

```
export PLAID_CLIENT_ID=<client_id>
export PLAID_SECRET=<plaid_secret>
export PLAID_ENV=sandbox
export PLAID_SANDBOX_REDIRECT_URI=https://cdn-testing.plaid.com/link/v2/stable/sandbox-oauth-a2a-react-native-redirect.html
export PLAID_ANDROID_PACKAGE_NAME=<android package name>
export PLAID_VERSION=2020-09-14
export DATABASE_URL=<database-url>
```

```bash
 $ RUST_LOG=info cargo run --release --bin server -- \
    --database-url <url> \
    --plaid-env sandbox \
    --plaid-client-id <client-id> \
    --plaid-secret <secret> \
    --plaid-redirect-uri <redirect-url-for-ios> \
    --plaid-android-package-name <android-package-name-for-android>
```

### Start the app

```bash
$ yarn android
```

or

```bash
$ yarn ios
```
