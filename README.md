# typto
dApp for sending tips to web apps and donations to charities

## How to use
### Prerequisites
1. Install [node & npm](https://nodejs.dev/learn/how-to-install-nodejs).
2. Install the [yarn package manager](https://yarnpkg.com/getting-started/install).
3. Use yarn to deploy the site to `localhost:1234` and the contract to the near testnet: `yarn dev:deploy`.

### Typto
1. Navigate to `localhost:1234`.
2. Sign into your near testnet wallet.
3. Enter the contract you'd like to tip and the amount to tip.
4. Click `Send Tip`!

## TODO
- Allow receivers to register themselves
- Allow users to search for registered recipients
- Linkdrop to allow users to proactively register recipients
- Convert front-end to chrome/firefox extension
- Parameterize front-end to point to localnet/testnet/mainnet based on `package.json`
- Keep form values in index.html after signing into near wallet
