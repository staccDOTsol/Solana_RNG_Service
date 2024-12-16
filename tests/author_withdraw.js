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
    const solConnection = new Connection(" http://127.0.0.1:8899");
    const walletWrapper = new anchor.Wallet(walletKeyPair);
    const provider = new anchor.Provider(solConnection, walletWrapper, {
      preflightCommitment: 'confirmed',
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
  async function loadHouseProgram(walletKeyPair) {
    const solConnection = new Connection(" http://127.0.0.1:8899");
    const walletWrapper = new anchor.Wallet(walletKeyPair);
    const provider = new anchor.Provider(solConnection, walletWrapper, {
      preflightCommitment: 'confirmed',
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
  const author = new PublicKey(jare);
  const walletJson = "/mnt/c/id.json";
  
  const walletKeyPair = loadWalletKey(walletJson);
  const walletWrapper = new anchor.Wallet(walletKeyPair);
  
  const solConnection = new anchor.web3.Connection(
      //@ts-ignore
      " http://127.0.0.1:8899",
  );
  const provider = new anchor.Provider(solConnection, walletWrapper, {
    preflightCommitment: 'confirmed',
  });
  setTimeout(async function () {
    // Configure the client to use the local cluster.
    anchor.setProvider(provider);
    const operator = new PublicKey("DQVuVE9CeHP3oc5nFQsq1MvSxyKDtmc9SpadWApbdpGi");
  
    const puppetMaster = await loadHouseProgram(walletKeyPair);
    const puppet = await loadPuppProgram(walletKeyPair);
    //Initialize a new puppet account.
    const [house, houseBump] = await getHouse(author, operator);
    const houseObj = await puppetMaster.account.house.fetch(
      house,
    );
    const newPuppetAccount = anchor.web3.Keypair.generate();
    console.log(newPuppetAccount.publicKey.toBase58())
    await puppet.rpc.initialize({
      accounts: {
        puppet: newPuppetAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [walletKeyPair, newPuppetAccount],
    });
    for (let i = 0; i < 100; i++) {
      await puppetMaster.rpc.pullStrings(new anchor.BN(10 ** 7), houseBump, houseObj.operatorFeeBump, houseObj.authorFeeBump, {
        accounts: {
          author: houseObj.author,
          authorFeeAccount: houseObj.authorFeeAccount,
          operator: houseObj.operator,
          operatorFeeAccount: houseObj.operatorFeeAccount,
          house: house,
          puppet: newPuppetAccount.publicKey,
          puppetProgram: puppet.programId,
          recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
          jare: jare,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [walletKeyPair],
        
      },
      );
  
      // Check the state updated.
      puppetAccount = await puppet.account.data.fetch(newPuppetAccount.publicKey);
      if (puppetAccount.data < 4) {
        console.log(i)    
        console.log(puppetAccount.data.toNumber())
        console.log('lost your bet of 0.01 sol')
        console.log('')
      } else {
        console.log(i)    
        console.log(puppetAccount.data.toNumber())
        console.log('won 0.0185 sol :)')
        console.log('')
      }
    }
  }, 1)
  