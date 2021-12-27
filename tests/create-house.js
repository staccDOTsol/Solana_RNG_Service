const {
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY
} = require('@solana/web3.js')
const {
  TOKEN_PROGRAM_ID
} = require('@solana/spl-token')
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

// const jare = "6X3oVE5Hq923M2UEJregoA7zLxuc2jXcJJegpy24pb2T";
const walletJson = "/mnt/c/id.json";

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

  const puppetMaster = anchor.workspace.PuppetMaster;
  // const puppet = anchor.workspace.Puppet;
  // Initialize a new puppet account.
  const ohad = new anchor.web3.PublicKey("8RkPXKyE59XkNHAdJAFcgeNgFR4VbpPypgMaBbmxguU3")
  const newHouse = anchor.web3.Keypair.generate();
  console.log(walletKeyPair.publicKey)
  const tx = await puppetMaster.rpc.createHouse(new anchor.BN(255), new anchor.BN(255), new anchor.BN(255), new anchor.BN(255), {
    accounts: {
      payer: walletKeyPair.publicKey,
      authority: walletKeyPair.publicKey,
      feeWithdrawalDestination: ohad,
      treasuryWithdrawalDestination: ohad,
      treasuryWithdrawalDestinationOwner: ohad,
      house: newHouse.publicKey,
      houseFeeAccount: newHouse.publicKey,
      houseTreasury: newHouse.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
    },
    signers: [walletKeyPair],
  });
  console.log(newHouse.publicKey.toBase58())
}, 1)
