const {
  Keypair,
} = require('@solana/web3.js')
const fs = require('fs')
const anchor = require("@project-serum/anchor");

function loadWalletKey(keypair) {
  if (!keypair || keypair === '') {
    throw new Error('Keypair is required!');
  }
  return Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(fs.readFileSync(keypair).toString())),
  );
}

// const jare = "8RkPXKyE59XkNHAdJAFcgeNgFR4VbpPypgMaBbmxguU3";
const walletJson = "/home/ohad/.config/solana/id.json";

const walletKeyPair = loadWalletKey(walletJson);
const walletWrapper = new anchor.Wallet(walletKeyPair);

const solConnection = new anchor.web3.Connection(
    //@ts-ignore
    "https://psytrbhymqlkfrhudd.dev.genesysgo.net:8899/",
);
const provider = new anchor.Provider(solConnection, walletWrapper, {
  preflightCommitment: 'recent',
});
setTimeout(async function () {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // const puppetMaster = anchor.workspace.PuppetMaster;
  // const puppet = anchor.workspace.Puppet;
  // Initialize a new puppet account.
  // const newPuppetAccount = anchor.web3.Keypair.generate();
  // const tx = await puppetMaster.rpc.createHouse(
  //     {
  //
  //     },
  // signers: [walletKeyPair]
  // )
}, 1)
