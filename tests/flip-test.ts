import {loadHouseProgram, loadWalletKey} from "./utils";
import {PublicKey} from "@solana/web3.js";

const fs = require('fs')
const anchor = require("@project-serum/anchor");

const walletJson = "./throwaway.json";
const walletKeyPair = loadWalletKey(walletJson);
const walletWrapper = new anchor.Wallet(walletKeyPair);

async function main() {
  const puppetMaster = await loadHouseProgram(walletKeyPair);
  const newPuppetAccount = anchor.web3.Keypair.generate();
  /*
await puppet.rpc.initialize({
  accounts: {
    puppet: newPuppetAccount.publicKey,
    user: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  },
  signers: [walletKeyPair, newPuppetAccount],
});
*/
  for (let i = 0; i < 100; i++) {
    // Invoke the puppet master to perform a CPI to the puppet.
    // await puppetMaster.rpc.pullStrings(new anchor.BN(10 ** 7), {
    //   accounts: {
    //     house: new PublicKey("B93XdJvZxrRV986K8s8KFPZFQ937nGuxRNmNuJhe7EC6"),
    //     puppet: newPuppetAccount.publicKey,
    //     puppetProgram: puppet.programId,
    // recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
    // jare: jare,
    // user: provider.wallet.publicKey,
    // systemProgram: SystemProgram.programId,
    // },
    // });

    // Check the state updated.
    // puppetAccount = await puppet.account.data.fetch(newPuppetAccount.publicKey);
    // if (puppetAccount.data < 4) {
    //   console.log(puppetAccount.data.toNumber())
    //   console.log('lost your bet of 0.01 sol')
    //   console.log('')
    // } else {
    //   console.log(puppetAccount.data.toNumber())
    //   console.log('won 0.0185 sol :)')
    //   console.log('')
    // }
  }
}

main().then(() => console.log("Success"));
