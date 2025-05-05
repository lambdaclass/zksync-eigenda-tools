import "@matterlabs/hardhat-zksync-solc";
import "@matterlabs/hardhat-zksync-deploy";
import "@matterlabs/hardhat-zksync-ethers";
import { HardhatUserConfig } from "hardhat/config";


const config: HardhatUserConfig = {
  zksolc: {
    // By not specifying any options, we are using the default settings of zksolc.
  },
  solidity: {
    version: "0.8.17",
  },
  defaultNetwork: "zkLocal",
  networks: {
    zkLocal: {
      url: "http://127.0.0.1:3150", // The RPC URL of ZKsync Era network.
      ethNetwork: "http://127.0.0.1:8545", // The Ethereum Web3 RPC URL.
      zksync: true, // Flag that targets ZKsync Era.
    },
  },
};
export default config;
