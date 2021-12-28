// @ts-nocheck

import {
  getAuthorFeeAccount,
  getHouse,
  getOperatorFeeAccount,
  getOperatorTreasuryAccount,
  loadHouseProgram,
  loadWalletKey, TOKEN_PROGRAM_ID,
  getConfig
} from "./utils";
import {Keypair, SYSVAR_RENT_PUBKEY, SystemProgram, PublicKey} from "@solana/web3.js";
import * as anchor from '@project-serum/anchor';

async function createOperatorTreasuryAccount(
  anchorProgram,
  configData,
  payerWallet,
  configAccount,
) {
  const size = 10;

  return anchor.web3.SystemProgram.createAccount({
    fromPubkey: payerWallet,
    newAccountPubkey: configAccount,
    space: size,
    lamports:
      await anchorProgram.provider.connection.getMinimumBalanceForRentExemption(
        size,
      ),
    programId: HOUSE_PROGRAM_ID,
  });
}
const createOperatorTreasury = async function (
  anchorProgram: anchor.Program,
  payerWallet: Keypair,
  uuid,
  configData: {
  },
) {
  const configAccount = Keypair.generate();



  return {
    config: configAccount.publicKey,
    uuid,
    txId: await anchorProgram.rpc.initializeOperatorTreasury(
     
      {
        accounts: {
          operatorTreasury: configAccount.publicKey,
          operator: operator.publicKey,
          payer: payerWallet.publicKey,
          systemProgram: SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [payerWallet, configAccount],
        instructions: [
          await createOperatorTreasuryAccount(
            anchorProgram,
            configData,
            payerWallet.publicKey,
            configAccount.publicKey,
          ),
        ],
      },
    ),
  };
};

const HOUSE_PROGRAM_ID = new PublicKey("9pJ55KszBGk1Td3LbRrWLszAaiXg7YLW5oouLABJwsZg");

const walletJson = "../throwaway.json"

const walletKeyPair = loadWalletKey(walletJson);
const operator = Keypair.generate();

async function main() {
  const puppetMaster = await loadHouseProgram(walletKeyPair);
  const author = new anchor.web3.PublicKey("4tui4yfA6MNgLhjXmKBATrPvEUGseEeqQrqAyVHintUQ")
  const [house, houseBump] = await getHouse(author, operator.publicKey);
  const [authorFeeAccount, authorFeeAccountBump] = await getAuthorFeeAccount(house, author, operator.publicKey);
  const [operatorTreasuryAccount, operatorTreasuryAccountBump] = await getConfig(operator.publicKey, "222224");
  const [operatorFeeAccount, operatorFeeAccountBump] = await getOperatorFeeAccount(house, author, operator.publicKey);
  const feeBasisPoints = 350;
  const res = await createOperatorTreasury(puppetMaster, walletKeyPair, "222227", {
    maxNumberOfLines: new anchor.BN(1),
    symbol: 'CMC',
    sellerFeeBasisPoints: 100,
    isMutable: true,
    maxSupply: new anchor.BN(0),
    retainAuthority: true,
    creators: {}
  });
console.log('house:' + house.toBase58())

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
          operatorTreasury: res.config,
          operatorTreasuryDestination: operator.publicKey,
          operatorFeeAccount: operatorFeeAccount,
          operatorFeeDestination: operator.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        },
        signers: [walletKeyPair],
      });
}

main().then(() => console.log("Success"));
