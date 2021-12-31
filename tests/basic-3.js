const {
  Keypair,
  SystemProgram,
  PublicKey,
  Connection
} = require('@solana/web3.js')

const TOKEN_PROGRAM_ID = new PublicKey(
  'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
);
const PUPP_PROGRAM_ID = new PublicKey("39W6qnEQhdaWE25ANNauVesPV1d81QwbMCL5GRwAoymy");

const HOUSE_PROGRAM_ID = new PublicKey("9pJ55KszBGk1Td3LbRrWLszAaiXg7YLW5oouLABJwsZg");
const PREFIX = 'rng_house';
const FEES = "fees";
const TREASURY = 'treasury';

const fs = require('fs')
const anchor = require("@project-serum/anchor");
async function loadPuppProgram(walletKeyPair){
  const solConnection = new Connection("https://api.devnet.solana.com");
  const walletWrapper = new anchor.Wallet(walletKeyPair);
  const provider = new anchor.Provider(solConnection, walletWrapper, {
    preflightCommitment: 'confirmed', commitment: 'confirmed',
  });
  const idl = await anchor.Program.fetchIdl(
    PUPP_PROGRAM_ID,
    provider,
  );
  
  // const idl = await anchor.Program.fetchIdl(HOUSE_PROGRAM_ID, provider);

  return new anchor.Program(idl, PUPP_PROGRAM_ID, provider);
}
async function getHouse(author, operator){
  // #[account(init, seeds=[PREFIX.as_bytes(), author.key().as_ref(), operator.key().as_ref()], bump=house_bump, space=HOUSE_SIZE, payer=author)]
  // house: Account<'info, House>,
  return await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(PREFIX),
        author.toBuffer(),
        operator.toBuffer()],
      HOUSE_PROGRAM_ID,
  );
}
 async function getAuthorFeeAccount(house, author, operator) {
  // #[account(mut, seeds=[PREFIX.as_bytes(), FEES.as_bytes(), house.key().as_ref(), author.key.as_ref(), operator.key.as_ref()], bump=author_fee_bump)]
  // author_fee_account: UncheckedAccount<'info>,
  return await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(PREFIX),
        Buffer.from(FEES),
        house.toBuffer(),
        author.toBuffer(),
        operator.toBuffer(),
      ],
      HOUSE_PROGRAM_ID,
  );
}

 async function getOperatorTreasuryAccount(house, author, operator) {
  // #[account(mut, seeds=[PREFIX.as_bytes(), TREASURY.as_bytes(), house.key().as_ref(), author.key.as_ref(), operator.key.as_ref()], bump=operator_treasury_bump)]
  // operator_treasury: UncheckedAccount<'info>,
  return await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(PREFIX),
        Buffer.from(TREASURY),
        house.toBuffer(),
        author.toBuffer(),
        operator.toBuffer(),
      ],
      HOUSE_PROGRAM_ID,
  );
}


 async function getOperatorFeeAccount(house, author, operator) {
  // #[account(mut, seeds=[PREFIX.as_bytes(), FEES.as_bytes(), house.key().as_ref(), author.key.as_ref(), operator.key.as_ref()], bump=operator_fee_bump)]
  // operator_fee_account: UncheckedAccount<'info>,
  return await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(PREFIX),
        Buffer.from(FEES),
        house.toBuffer(),
        author.toBuffer(),
        operator.toBuffer(),
      ],
      HOUSE_PROGRAM_ID,
  );
}


async function loadHouseProgram(walletKeyPair) {
  const solConnection = new Connection("https://api.devnet.solana.com");
  const walletWrapper = new anchor.Wallet(walletKeyPair);
  const provider = new anchor.Provider(solConnection, walletWrapper, {
    preflightCommitment: 'confirmed', commitment: 'confirmed',
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
let wins = 0;
const startts = new Date().getTime();
let losses = 0;
const jare = "4tui4yfA6MNgLhjXmKBATrPvEUGseEeqQrqAyVHintUQ";
const author = new PublicKey(jare);
const walletJson = "./throwaway.json"

const walletKeyPair = loadWalletKey(walletJson);
const walletWrapper = new anchor.Wallet(walletKeyPair);

const solConnection = new anchor.web3.Connection(
    //@ts-ignore
    "https://api.devnet.solana.com",
);
const provider = new anchor.Provider(solConnection, walletWrapper, {
  preflightCommitment: 'confirmed', commitment: 'confirmed',
});
setTimeout(async function () {
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const puppetMaster = await loadHouseProgram(walletKeyPair);
 // const puppet = await loadPuppProgram(walletKeyPair);
  //Initialize a new puppet account.
  const house = new PublicKey("A7WYs23jj9BF91khBkRJPb3TD5BXGivWjDwi5xXPhcnK")

  const houseObj = await puppetMaster.account.house.fetch(
    house,
  );
  /*
  console.log(houseObj)
  const operator = houseObj.operator;
  const feetx = await puppetMaster.rpc.authorFeeWithdraw( new anchor.BN( 0.00001 * 10 ** 9 ), {
    accounts: {
      house,
      authorFeeAccount: houseObj.authorFeeAccount,
      authorFeeAccountDestination: houseObj.authorFeeAccountDestination,
      author: walletKeyPair.publicKey,
      systemProgram: SystemProgram.programId,
    },remainingAccounts: [
      {
        pubkey: houseObj.authorFeeAccount,
        isSigner: false,
        isWritable: true,
      }
    ],
    signers: [walletKeyPair],
  }); 
  console.log(feetx)
  */
    uuid = (Math.floor((Math.random() * 9)).toString() +  Math.floor((Math.random() * 9)) +  Math.floor((Math.random() * 9)) +  Math.floor((Math.random() * 9)) +  Math.floor((Math.random() * 9)) +  Math.floor((Math.random() * 9)))
 const [newPuppetAccount, newPuppetAccountBump] = await anchor.web3.PublicKey.findProgramAddress(

  // @ts-ignore
  [Buffer.from("rng_house"), walletKeyPair.publicKey.toBuffer(), house.toBuffer(), Buffer.from(uuid)],
  HOUSE_PROGRAM_ID
);
await puppetMaster.rpc.initialize(newPuppetAccountBump, uuid,{
  accounts: {
    puppet: newPuppetAccount,
    user: walletKeyPair.publicKey,
    systemProgram: SystemProgram.programId,
    recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
    house: house,
  },
  signers: [],
}); 

  while (true) {
    await puppetMaster.rpc.pullStrings(new anchor.BN(10 ** 4),       {
      accounts: {
        author: houseObj.author,
        authorFeeAccount: houseObj.authorFeeAccount,
        operator: houseObj.operator,
        operatorFeeAccount: houseObj.operatorFeeAccount,
        house: house,
        puppet: newPuppetAccount,
        operatorTreasury: houseObj.operatorTreasury,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        jare: jare,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },remainingAccounts: [
        {
          pubkey: houseObj.operatorTreasury,
          isSigner: false,
          isWritable: true,
        }
      ],
      signers: [walletKeyPair],
      
    },
    );
    try {
    await puppetMaster.rpc.uncover( {
      accounts: {
        author: houseObj.author,
        authorFeeAccount: houseObj.authorFeeAccount,
        operator: houseObj.operator,
        operatorFeeAccount: houseObj.operatorFeeAccount,
        house: house,
        puppet: newPuppetAccount,
        operatorTreasury: houseObj.operatorTreasury,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        jare: jare,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },remainingAccounts: [
        {
          pubkey: houseObj.operatorTreasury,
          isSigner: false,
          isWritable: true,
        }
      ],
      signers: [walletKeyPair],
      
    },
    );
    wins++;
  } catch(err){
    losses++;
  }
  console.log((wins + losses).toString() + ' games played, ' + ((Math.round((wins / (wins + losses)) * 10000)) / 100).toString() + '% winners! Test has been running: ' + ((new Date().getTime() - startts) / 1000 / 60 / 60).toString() + ' hours :)')
  /*
    // Check the state updated.
    puppetAccount = await puppet.account.data.fetch(newPuppetAccount);
    if (puppetAccount.data < 4) {
      console.log(i)    
      console.log(puppetAccount.data.toNumber())
      console.log('lost your bet of 0.01 sol')
      console.log('')
    } else {
      console.log(i)    
      console.log(puppetAccount.data.toNumber())
      console.log('won 0.02 sol :)')
      console.log('')
    }
    */
  }
  
}, 1)