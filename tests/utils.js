const anchor = require("@project-serum/anchor");
const {
  PublicKey, Connection,
} = require('@solana/web3.js')

const HOUSE_PROGRAM_ID = new PublicKey("E1a43qQFekTigmYdDn1m2XYYEz6hGUwKc1SxsbRzQuMu");
const PREFIX = 'rng_house';
const FEE_PAYER = "fee_payer";
const TREASURY = 'treasury';

exports.loadAuctionHouseProgram = async (walletKeyPair) => {
  const solConnection = new Connection("https://api.devnet.solana.com");
  const walletWrapper = new anchor.Wallet(walletKeyPair);
  const provider = new anchor.Provider(solConnection, walletWrapper, {
    preflightCommitment: 'recent',
  });
  const idl = await anchor.Program.fetchIdl(HOUSE_PROGRAM_ID, provider);

  return new anchor.Program(idl, HOUSE_PROGRAM_ID, provider);
}

exports.getHouse = async (author, operator) => {
  return await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), author.toBuffer(), operator.toBuffer()],
      HOUSE_PROGRAM_ID,
  );
};

exports.getHouseFeeAccount = async (key, house) => {
  return await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(PREFIX),
        house.toBuffer(),
        key.toBuffer(),
        Buffer.from(FEE_PAYER),
      ],
      HOUSE_PROGRAM_ID,
  );
};


exports.getHouseTreasuryAccount = async (key, house) => {
  return await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(PREFIX),
        house.toBuffer(),
        key.toBuffer(),
        Buffer.from(TREASURY),
      ],
      HOUSE_PROGRAM_ID,
  );
};
