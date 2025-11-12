import { Keypair } from "@solana/web3.js";

const keypair = Keypair.generate();

console.log(`Keypair: `, keypair);
console.log(`The Public Key is : `, keypair.publicKey.toBase58());
console.log(`The Secret Key is : `, keypair.secretKey);

