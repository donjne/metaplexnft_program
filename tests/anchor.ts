import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { HelloAnchor } from "../target/types/hello_anchor";
describe("metaplex", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloAnchor as anchor.Program<HelloAnchor>;
  
  it("Init and mint!", async () => {
  //Initializing a new mint:

    // Metaplex Constants
    const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    // Constants
    const METADATA_SEED = "metadata";
    const EDITION_SEED = "edition";
  
    // Data for our tests
    const payer = program.provider.publicKey;


      const name = "Just another Test Token";
      const symbol = "TEST";
      const uri = "https://5vfxc4tr6xoy23qefqbj4qx2adzkzapneebanhcalf7myvn5gzja.arweave.net/7UtxcnH13Y1uBCwCnkL6APKsge0hAgacQFl-zFW9NlI";

    // Test init token
    const MINT_SEED = "mint";
    const [mint] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );
    
    const destination = anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer,
    })

    const [metadataAddress] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const [masterEdition] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        Buffer.from(EDITION_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );


      const info = await program.provider.connection.getAccountInfo(mint);
      if (info) {
        return; // Do not attempt to initialize if already initialized
      }
      console.log("  Mint not found. Attempting to initialize.");
      
      const context = {
        payer: payer,
        mint: mint,
        associatedTokenAccount: destination,
        metadataAccount: metadataAddress,
        masterEditionAccount: masterEdition,
        rent: web3.SYSVAR_RENT_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      };

      console.log(anchor.utils.token.ASSOCIATED_PROGRAM_ID.toBase58())
      console.log(anchor.utils.token.TOKEN_PROGRAM_ID.toBase58())
      console.log(web3.SystemProgram.programId.toBase58())
      console.log(payer)
  
      const txHash = await program.methods
        .initNft(name, symbol, uri)
        .accounts(context)
        .rpc();
  
      await program.provider.connection.confirmTransaction(txHash, 'finalized');
      console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
      const newInfo = await program.provider.connection.getAccountInfo(mint);
      assert(newInfo, "  Mint should be initialized.");

});

});