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
const {getHouse, getHouseFeeAccount, getHouseTreasuryAccount, loadAuctionHouseProgram} = require("./utils");

function loadWalletKey(keypair) {
  if (!keypair || keypair === '') {
    throw new Error('Keypair is required!');
  }
  return Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(fs.readFileSync(keypair).toString())),
  );
}

// const jare = "6X3oVE5Hq923M2UEJregoA7zLxuc2jXcJJegpy24pb2T";
const walletJson = "/home/ohad/.config/solana/id.json";

const walletKeyPair = loadWalletKey(walletJson);
const walletWrapper = new anchor.Wallet(walletKeyPair);

const solConnection = new anchor.web3.Connection(
    //@ts-ignore
    "https://psytrbhymqlkfrhudd.dev.genesysgo.net:8899/",
);


setTimeout(async function () {
  // Configure the client to use the local cluster.
  const puppetMaster = await loadAuctionHouseProgram(walletKeyPair);
  const author = new anchor.web3.PublicKey("8RkPXKyE59XkNHAdJAFcgeNgFR4VbpPypgMaBbmxguU3")
  const operator = Keypair.generate();
  const [house, houseBump] = await getHouse(author, operator.publicKey);
  const [authorFeeAccount, authorFeeAccountBump] = await getHouseFeeAccount(author, house);
  const [authorTreasuryAccount, , authorTreasuryAccountBump] = await getHouseTreasuryAccount(author, house);
  const [operatorFeeAccount, operatorFeeAccountBump] = await getHouseFeeAccount(author, operator.publicKey);
  const feeBasisPoints = 100;
  console.log(walletKeyPair.publicKey)
  const tx = await puppetMaster.rpc.createHouse(
      houseBump,
      operatorFeeAccountBump,
      authorTreasuryAccountBump,
      feeBasisPoints,
      {
        accounts: {
          payer: walletKeyPair.publicKey,
          authority: walletKeyPair.publicKey,
          feeWithdrawalDestination: author,
          treasuryWithdrawalDestination: author,
          treasuryWithdrawalDestinationOwner: author,
          house: house,
          houseAuthor: author,
          houseOperator: operator.publicKey,
          houseFeeAccount: operatorFeeAccount,
          houseTreasury: authorTreasuryAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
        signers: [walletKeyPair],
      });
  console.log(tx);
}, 1)
