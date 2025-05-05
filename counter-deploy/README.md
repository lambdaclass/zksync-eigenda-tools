# Script to deploy a counter into zksync

First we need to deposit funds into the L2

```
npx zksync-cli bridge deposit --rpc=http://127.0.0.1:3150 --l1-rpc=<your_l1_rpc>
```

Install the dependencies

```
npm install -D @matterlabs/hardhat-zksync-deploy hardhat zksync-ethers ethers @matterlabs/hardhat-zksync-ethers dotenv
```

Modify on `hardhat.config.ts` the `ethNetwork` for your L1 URL

Next deploy the counter contract

```
PRIVATE_KEY=<your_pk> npx hardhat deploy-zksync --network zkLocal
```

Run the increment function

```
NEW_PRIVATE_KEY=<your_pk> ADDRESS=<your_counter_address> npx hardhat run scripts/increment.ts
```

Check the value of the counter

```
cast call <your_counter_address> "getNumber()" --rpc-url http://127.0.0.1:3150
```
