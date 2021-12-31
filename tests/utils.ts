import {Connection, Keypair, PublicKey,} from '@solana/web3.js';
import {Program} from "@project-serum/anchor";
const anchor = require("@project-serum/anchor");
const fs = require('fs');

export const TOKEN_PROGRAM_ID = new PublicKey(
    'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
);

const HOUSE_PROGRAM_ID = new PublicKey("9pJ55KszBGk1Td3LbRrWLszAaiXg7YLW5oouLABJwsZg");
const PUPP_PROGRAM_ID = new PublicKey("39W6qnEQhdaWE25ANNauVesPV1d81QwbMCL5GRwAoymy");

const PREFIX = 'rng_house';
const FEES = "fees";
const TREASURY = 'treasury';

export function loadWalletKey(keypair: string): Keypair {
  if (!keypair || keypair === '') {
    throw new Error('Keypair is required!');
  }
  return Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(keypair).toString())));
}

export async function loadHouseProgram(walletKeyPair: Keypair): Promise<Program> {
  const solConnection = new Connection("https://api.devnet.solana.com");
  const walletWrapper = new anchor.Wallet(walletKeyPair);
  const provider = new anchor.Provider(solConnection, walletWrapper, {
    preflightCommitment: 'confirmed', commitment: 'confirmed'
  });
  const idl = await anchor.Program.fetchIdl(
    HOUSE_PROGRAM_ID,
    provider,
  );
  
  // const idl = await anchor.Program.fetchIdl(HOUSE_PROGRAM_ID, provider);

  return new anchor.Program(idl, HOUSE_PROGRAM_ID, provider);
}
export async function loadPuppProgram(walletKeyPair: Keypair): Promise<Program> {
  const solConnection = new Connection("https://api.devnet.solana.com");
  const walletWrapper = new anchor.Wallet(walletKeyPair);
  const provider = new anchor.Provider(solConnection, walletWrapper, {
    preflightCommitment: 'confirmed', commitment: 'confirmed'
  });
  const idl = await anchor.Program.fetchIdl(
    PUPP_PROGRAM_ID,
    provider,
  );
  
  // const idl = await anchor.Program.fetchIdl(HOUSE_PROGRAM_ID, provider);

  return new anchor.Program(idl, PUPP_PROGRAM_ID, provider);
}
export async function getHouse(author: PublicKey, operator: PublicKey): Promise<[PublicKey, number]> {
  // #[account(init, seeds=[PREFIX.as_bytes(), author.key().as_ref(), operator.key().as_ref()], bump=house_bump, space=HOUSE_SIZE, payer=author)]
  // house: Account<'info, House>,
  return await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(PREFIX),
        author.toBuffer(),
        operator.toBuffer()],
      HOUSE_PROGRAM_ID,
  );
}

export async function getAuthorFeeAccount(house: PublicKey, author: PublicKey, operator: PublicKey): Promise<[PublicKey, number]> {
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

export async function getOperatorTreasuryAccount(house: PublicKey, author: PublicKey, operator: PublicKey): Promise<[PublicKey, number]> {
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


export async function getOperatorFeeAccount(house: PublicKey, author: PublicKey, operator: PublicKey): Promise<[PublicKey, number]> {
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


export async function getConfig (
  operator: PublicKey,
  uuid: string,
) {
  return await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(PREFIX), operator.toBuffer(), Buffer.from(uuid)],
    HOUSE_PROGRAM_ID,
  );
};