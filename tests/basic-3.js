const {
  Keypair,
  SystemProgram,
  PublicKey,
  Connection
} = require('@solana/web3.js')

const TOKEN_PROGRAM_ID = new PublicKey(
  'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
);

const HOUSE_PROGRAM_ID = new PublicKey("9pJ55KszBGk1Td3LbRrWLszAaiXg7YLW5oouLABJwsZg");
const PREFIX = 'rng_house';
const FEES = "fees";
const TREASURY = 'treasury';

const fs = require('fs')
const anchor = require("@project-serum/anchor");
async function loadHouseProgram(walletKeyPair) {
  const solConnection = new Connection("https://api.devnet.solana.com");
  const walletWrapper = new anchor.Wallet(walletKeyPair);
  const provider = new anchor.Provider(solConnection, walletWrapper, {
    preflightCommitment: 'recent',
  });
  const idl = await anchor.Program.fetchIdl(
    HOUSE_PROGRAM_ID,
    provider,
  );
  
  // const idl = await anchor.Program.fetchIdl(HOUSE_PROGRAM_ID, provider);

  return new anchor.Program(idl, HOUSE_PROGRAM_ID, provider);
}

function loadWalletKey(keypair) {
  if (!keypair || keypair === '') {
    throw new Error('Keypair is required!');
  }
  return Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(fs.readFileSync(keypair).toString())),
  );
}

const jare = "4tui4yfA6MNgLhjXmKBATrPvEUGseEeqQrqAyVHintUQ";
const walletJson = "../throwaway.json";

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

  const puppetMaster = await loadHouseProgram(walletKeyPair);
  //const puppet = anchor.workspace.Puppet;
  // Initialize a new puppet account.

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
    await puppetMaster.rpc.pullStrings(new anchor.BN(10 ** 7), {
      accounts: {
        house: new PublicKey("B93XdJvZxrRV986K8s8KFPZFQ937nGuxRNmNuJhe7EC6"),
        //puppet: newPuppetAccount.publicKey,
        //puppetProgram: puppet.programId,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        jare: jare,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    // Check the state updated.
    puppetAccount = await puppet.account.data.fetch(newPuppetAccount.publicKey);
    if (puppetAccount.data < 4) {
      console.log(puppetAccount.data.toNumber())
      console.log('lost your bet of 0.01 sol')
      console.log('')
    } else {
      console.log(puppetAccount.data.toNumber())
      console.log('won 0.0185 sol :)')
      console.log('')
    }
  }
}, 1)
