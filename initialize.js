const fs = require("fs");
const {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
  sendAndConfirmTransaction,
} = require("@solana/web3.js");

const { createHash } = require("crypto");

const RPC = "https://api.devnet.solana.com";

const PROGRAM_ID = new PublicKey(
  "8ZKiCKTeYGq6ArCHTsG4CzRTTJa67rxeYgNpstwavRbt"
);

const walletPath =
  process.env.HOME + "/.config/solana/devnet-wallet.json";

const secretKey = JSON.parse(fs.readFileSync(walletPath, "utf8"));
const payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));

const connection = new Connection(RPC, "confirmed");

const [counter] = PublicKey.findProgramAddressSync(
  [Buffer.from("counter")],
  PROGRAM_ID
);

const discriminator = createHash("sha256")
  .update("global:initialize")
  .digest()
  .subarray(0, 8);

const instruction = new TransactionInstruction({
  programId: PROGRAM_ID,
  keys: [
    {
      pubkey: payer.publicKey,
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: counter,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: SystemProgram.programId,
      isSigner: false,
      isWritable: false,
    },
  ],
  data: discriminator,
});

(async () => {
  console.log("Wallet:", payer.publicKey.toBase58());
  console.log("Counter PDA:", counter.toBase58());

  const signature = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payer]
  );

  console.log("Success!");
  console.log("Signature:", signature);

  const account = await connection.getAccountInfo(counter);

  if (account) {
    console.log("Counter account exists on Devnet!");
    console.log("Owner:", account.owner.toBase58());
  }
})();