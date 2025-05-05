import hre from "hardhat";
import dotenv from "dotenv";

dotenv.config();

const NEW_PRIVATE_KEY = process.env.NEW_PRIVATE_KEY || "";

if (!NEW_PRIVATE_KEY) {
  throw new Error("Wallet private key is not configured in .env file!");
}

const ADDRESS = process.env.ADDRESS || "";

if (!ADDRESS) {
  throw new Error("Address key is not configured in .env file!");
}

async function main() {
  console.info(`Running increment`);
  // Use a new wallet
  const wallet = await hre.zksyncEthers.getWallet(NEW_PRIVATE_KEY);
  // Deploy contract
  const counter = await hre.zksyncEthers.getContractAt("Counter",ADDRESS, wallet);
  // Call contract function
  const tx = await counter.increment();
  // Wait for transaction
  await tx.wait();
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
