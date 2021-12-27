import {
  getAuthorFeeAccount,
  getHouse,
  getOperatorFeeAccount,
  getOperatorTreasuryAccount,
  loadHouseProgram,
  loadWalletKey, TOKEN_PROGRAM_ID
} from "./utils";
import {Keypair, SYSVAR_RENT_PUBKEY, SystemProgram} from "@solana/web3.js";
import * as anchor from '@project-serum/anchor';

const walletJson = "/home/ohad/.config/solana/id.json";

const walletKeyPair = loadWalletKey(walletJson);

async function main() {
  const puppetMaster = await loadHouseProgram(walletKeyPair);
  const author = new anchor.web3.PublicKey("8RkPXKyE59XkNHAdJAFcgeNgFR4VbpPypgMaBbmxguU3")
  const operator = Keypair.generate();
  const [house, houseBump] = await getHouse(author, operator.publicKey);
  const [authorFeeAccount, authorFeeAccountBump] = await getAuthorFeeAccount(house, author, operator.publicKey);
  const [operatorTreasuryAccount, operatorTreasuryAccountBump] = await getOperatorTreasuryAccount(house, author, operator.publicKey);
  const [operatorFeeAccount, operatorFeeAccountBump] = await getOperatorFeeAccount(house, author, operator.publicKey);
  const feeBasisPoints = 100;
  console.log(walletKeyPair.publicKey)
  const tx = await puppetMaster.rpc.createHouse(
      houseBump,
      authorFeeAccountBump,
      operatorTreasuryAccountBump,
      operatorFeeAccountBump,
      feeBasisPoints,
      {
        accounts: {
          author: walletKeyPair.publicKey,
          operator: operator.publicKey,
          house: house,
          authorFeeAccount: authorFeeAccount,
          authorFeeAccountDestination: author,
          operatorTreasury: operatorTreasuryAccount,
          operatorTreasuryDestination: operator.publicKey,
          operatorFeeAccount: operatorFeeAccount,
          operatorFeeDestination: operator.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
        signers: [walletKeyPair],
      });
  console.log(tx);
}

main().then(() => console.log("Success"));
