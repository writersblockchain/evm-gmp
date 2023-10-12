import { SecretNetworkClient, Wallet, coinsFromString } from "secretjs";
// import {
//   AxelarAssetTransfer,
//   AxelarQueryAPI,
//   Environment,
//   CHAINS,
//   AxelarGMPRecoveryAPI,
// } from "@axelar-network/axelarjs-sdk";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm.gz");

// let codeId = 1309;
// let contractCodeHash =
//   "9679a9aae3f9e5cc65a1bfbb9f2613cef7cf97e47bbc41cb8138c64c73e9757a";
// let contractAddress = "secret1z2y0k356wfq8kgf22e7d2dqdz9a5r0kfrvwlev";

const secretjs = new SecretNetworkClient({
  chainId: "secret-4",
  url: "https://lcd.mainnet.secretsaturn.net",
  wallet: wallet,
  walletAddress: wallet.address,
});

// const secretjs = new SecretNetworkClient({
//   chainId: "pulsar-3",
//   url: "https://api.pulsar3.scrttestnet.com",
//   wallet: wallet,
//   walletAddress: wallet.address,
// });

let upload_contract = async () => {
  let tx = await secretjs.tx.compute.storeCode(
    {
      sender: wallet.address,
      wasm_byte_code: contract_wasm,
      source: "",
      builder: "",
    },
    {
      gasLimit: 4_000_000,
    }
  );

  const codeId = Number(
    tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
      .value
  );

  console.log("codeId: ", codeId);

  const contractCodeHash = (
    await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
  ).code_hash;
  console.log(`Contract hash: ${contractCodeHash}`);

  //   console.log(tx);
};

// upload_contract();

let instantiate_contract = async () => {
  // Create an instance of the Counter contract, providing a starting count
  const initMsg = {};
  let tx = await secretjs.tx.compute.instantiateContract(
    {
      code_id: codeId,
      sender: wallet.address,
      code_hash: contractCodeHash,
      init_msg: initMsg,
      label: "Secret EVM AXELAR " + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );

  //Find the contract_address in the logs
  const contractAddress = tx.arrayLog.find(
    (log) => log.type === "message" && log.key === "contract_address"
  ).value;

  console.log(contractAddress);
};

// instantiate_contract();

let send_message_evm = async () => {
  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        send_message_evm: {
          destination_chain: "Polygon",
          destination_address: "0x13ACd5794A3136E7fAc8f9727259930fcab1290F",
          message: "october 11 seanrad",
        },
      },
      code_hash: contractCodeHash,
      sent_funds: coinsFromString("1uscrt"),
    },
    { gasLimit: 100_000 }
  );

  console.log(tx);
};
// send_message_evm();

let get_stored_message = async () => {
  let query = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    query: {
      get_stored_message: {},
    },
    code_hash: contractCodeHash,
  });

  console.log(query);
};

// get_stored_message();

// secretcli tx wasm execute "secret1wpmsu5arwp80hqgekan9j693eshphzfgh9s869" '{"send_message_evm": {"destination_chain": "Polygon", "destination_address":"0x7a26f97170BA95C1C21FBe941902D0Ca49A798dF","message":"hello"}}' --amount 1uscrt --from pulsar3-test

// Polygon Mainnet contract:
// 0x13ACd5794A3136E7fAc8f9727259930fcab1290F

// const sdk = new AxelarQueryAPI({
//   environment: "mainnet",
// });

// const api = new AxelarQueryAPI(sdk);

// async function main() {}

// main();
