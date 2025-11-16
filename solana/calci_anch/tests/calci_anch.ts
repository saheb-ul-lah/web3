import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CalciAnch } from "../target/types/calci_anch";

describe("calci_anch", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CalciAnch as Program<CalciAnch>;

  const calciAcc = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize()
      .accounts({
        feePayer: provider.wallet.publicKey,                    // camelCase
        calciAcc: calciAcc.publicKey                            // camelCase
      })
      .signers([calciAcc])
      .rpc();

    console.log("Initialized! Signature:", tx);
  });
});
