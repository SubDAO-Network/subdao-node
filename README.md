# SubDAO Node

### Setup
[Rust development environment](https://substrate.dev/docs/en/knowledgebase/getting-started).


### Build

Checkout code
```bash
git clone --recursive https://github.com/SubDAO-Network/subdao-node.git
cd subdao-node
cargo build
```

### Connect Node

If you are using Polkadot JS Apps to connect SubDAO node, please fill the config in `Settings>Developer`.
```js
{
  "Address": "<AccountId>",
  "LookupSource": "<AccountId>",
}
```
