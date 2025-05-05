import { Wallet } from "zksync-ethers";
import * as ethers from "ethers";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { Deployer } from "@matterlabs/hardhat-zksync-deploy";
import dotenv from "dotenv";

dotenv.config();

const PRIVATE_KEY = process.env.PRIVATE_KEY || "";

if (!PRIVATE_KEY) {
  throw new Error("Wallet private key is not configured in .env file!");
}

// An example of a deploy script
export default async function (hre: HardhatRuntimeEnvironment) {
  console.log(`Running deploy script for the Counter contract`);

  // Initialize the wallet.
  const wallet = new Wallet(PRIVATE_KEY);

  // Create deployer object and load the artifact of the contract you want to deploy.
  const deployer = new Deployer(hre, wallet);
  const artifact = await deployer.loadArtifact("Counter");

  // Estimate contract deployment fee
  const deploymentFee = await deployer.estimateDeployFee(artifact, []);

  const parsedFee = ethers.formatEther(deploymentFee);
  console.log(`The deployment is estimated to cost ${parsedFee} ETH`);

  // Deploy contract
  const counterContract = await deployer.deploy(artifact, []);

  // Show the contract info.
  const contractAddress = await counterContract.getAddress();
  console.log(`${artifact.contractName} was deployed to ${contractAddress}`);
}
