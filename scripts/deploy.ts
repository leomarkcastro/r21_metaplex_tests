import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { R21MetaplexTests } from "../target/types/r21_metaplex_tests";
import {
  getAssociatedTokenAddress,
  AccountLayout,
  MintLayout,
} from "@solana/spl-token";

describe("r21_metaplex_tests", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .R21MetaplexTests as Program<R21MetaplexTests>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const minterKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();

  const wallet = provider.wallet as Wallet;
  let wallet1Holder: PublicKey;

  it("Is initialized!", async () => {
    await program.methods
      .initializeNft()
      .accounts({
        ownerAccount: wallet.publicKey,
        minterAccount: minterKeypair.publicKey,
      })
      .signers([wallet.payer, minterKeypair])
      .rpc();

    console.log({
      ownerAccount: wallet.publicKey,
      minterAccount: minterKeypair.publicKey,
    });
  });

  it("Is Creating NFT Token Holder! [A]", async () => {
    wallet1Holder = await getAssociatedTokenAddress(
      minterKeypair.publicKey,
      wallet.publicKey
    );

    await program.methods
      .createNftHolder()
      .accounts({
        userAccount: wallet.publicKey,
        minterAccount: minterKeypair.publicKey,
        tokenHolderAccount: wallet1Holder,
      })
      .signers([wallet.payer])
      .rpc();
  });

  it("Is Minting! [A]", async () => {
    if (!wallet1Holder) {
      throw new Error("Wallet2 not found");
    }

    const metadataAddress = (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          minterKeypair.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];

    const masterEditionAddress = (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("metadata"),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          minterKeypair.publicKey.toBuffer(),
          Buffer.from("edition"),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];

    // Mint Attempt One
    await program.methods
      .mintNft(
        "TestNFT4",
        "TestNFT4",
        "https://raw.githubusercontent.com/Coding-and-Crypto/Solana-NFT-Marketplace/master/assets/example.json"
      )
      .accounts({
        authorityAccount: wallet.publicKey,
        minterAccount: minterKeypair.publicKey,
        tokenHolderAccount: wallet1Holder,
        metadataAccount: metadataAddress,
        masterEditionAccount: masterEditionAddress,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([wallet.payer])
      .rpc();

    // get total minted tokens
    const accountInfo = await program.provider.connection.getAccountInfo(
      wallet1Holder
    );
    const mintInfo = await program.provider.connection.getAccountInfo(
      minterKeypair.publicKey
    );
    if (accountInfo === null || mintInfo === null) {
      throw new Error("Account or mint info not found");
    }
    const { amount } = AccountLayout.decode(accountInfo.data);
    const { supply } = MintLayout.decode(mintInfo.data);
    // console.log("Account: ", accountInfo);
    console.log("Amount: ", amount);
    console.log("Supply: ", supply);
  });
});
