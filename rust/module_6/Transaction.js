import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction
} from "@solana/web3.js";

// Connect to Solana Devnet
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// Create a transaction
const transaction = new Transaction();

// Amount to send in lamports (1 SOL = 1,000,000,000 lamports)
const LAMPORTS_TO_SEND = 5000;

// Example: Replace with actual sender and receiver keypairs/public keys
const senderKeyPair = Keypair.generate(); // replace with your sender's keypair
const fromPubKey = senderKeyPair.publicKey;
const toPubKey = new PublicKey("ENTER_RECEIVER_PUBLIC_KEY_HERE"); // replace this

// Add transfer instruction
const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: fromPubKey,
  toPubkey: toPubKey,
  lamports: LAMPORTS_TO_SEND,
});

transaction.add(sendSolInstruction);

// Send and confirm transaction
(async () => {
  try {
    const signature = await sendAndConfirmTransaction(connection, transaction, [senderKeyPair]);
    console.log("✅ Transaction confirmed with signature:", signature);
  } catch (error) {
    console.error("❌ Transaction failed:", error);
  }
})();