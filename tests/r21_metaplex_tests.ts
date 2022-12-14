import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { R21MetaplexTests } from "../target/types/r21_metaplex_tests";
import {
  getAssociatedTokenAddress,
  AccountLayout,
  MintLayout,
} from "@solana/spl-token";
import { Metadata } from "@metaplex-foundation/mpl-token-metadata";

describe("r21_metaplex_tests", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .R21MetaplexTests as Program<R21MetaplexTests>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const wallet = provider.wallet as Wallet;
  let wallet1Holder: PublicKey;

  const wallet2Keypair = anchor.web3.Keypair.generate();
  const wallet2 = new Wallet(wallet2Keypair);
  let wallet2Holder: PublicKey;

  const wallet3Keypair = anchor.web3.Keypair.generate();
  const wallet3 = new Wallet(wallet3Keypair);
  let wallet3Holder: PublicKey;

  const LAMPORTS = 1000000000;

  // airdrop solana to wallets
  before(async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(wallet.publicKey, LAMPORTS),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(wallet2.publicKey, LAMPORTS),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(wallet3.publicKey, LAMPORTS),
      "confirmed"
    );
  });

  xdescribe("Unit Tests", async () => {
    const minterKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();

    it("Is initialized!", async () => {
      // const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
      // const NftTokenAccount = await getAssociatedTokenAddress(
      //   mintKey.publicKey,
      //   wallet.publicKey
      // );

      // const lamports: number =
      //   await program.provider.connection.getMinimumBalanceForRentExemption(
      //     MINT_SIZE
      //   );
      // console.log("NFT Account: ", NftTokenAccount.toBase58());

      // const mint_tx = new anchor.web3.Transaction().add(
      //   anchor.web3.SystemProgram.createAccount({
      //     fromPubkey: wallet.publicKey,
      //     newAccountPubkey: mintKey.publicKey,
      //     space: MINT_SIZE,
      //     programId: TOKEN_PROGRAM_ID,
      //     lamports,
      //   }),
      //   createInitializeMintInstruction(
      //     mintKey.publicKey,
      //     0,
      //     wallet.publicKey,
      //     wallet.publicKey
      //   ),
      //   createAssociatedTokenAccountInstruction(
      //     wallet.publicKey,
      //     NftTokenAccount,
      //     wallet.publicKey,
      //     mintKey.publicKey
      //   )
      // );

      // const res = await program.provider.sendAndConfirm(mint_tx, [mintKey]);
      // console.log(
      //   await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
      // );

      // console.log("Account: ", res);
      // console.log("Mint key: ", mintKey.publicKey.toString());
      // console.log("User: ", wallet.publicKey.toString());

      await program.methods
        .initializeNft()
        .accounts({
          ownerAccount: wallet2.publicKey,
          minterAccount: minterKeypair.publicKey,
        })
        .signers([wallet2.payer, minterKeypair])
        .rpc();
    });

    it("Is Creating NFT Token Holder! [A]", async () => {
      wallet2Holder = await getAssociatedTokenAddress(
        minterKeypair.publicKey,
        wallet2.publicKey
      );

      await program.methods
        .createNftHolder()
        .accounts({
          userAccount: wallet2.publicKey,
          minterAccount: minterKeypair.publicKey,
          tokenHolderAccount: wallet2Holder,
        })
        .signers([wallet2.payer])
        .rpc();
    });

    it("Is Minting! [A]", async () => {
      if (!wallet2Holder) {
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
        .mintNft("TestNFT", "TestNFT", "TestNFT")
        .accounts({
          authorityAccount: wallet2.publicKey,
          minterAccount: minterKeypair.publicKey,
          tokenHolderAccount: wallet2Holder,
          metadataAccount: metadataAddress,
          masterEditionAccount: masterEditionAddress,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        .signers([wallet2.payer])
        .rpc();

      // get total minted tokens
      const accountInfo = await program.provider.connection.getAccountInfo(
        wallet2Holder
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

    it("Is Creating NFT Token Holder! [B]", async () => {
      wallet3Holder = await getAssociatedTokenAddress(
        minterKeypair.publicKey,
        wallet3.publicKey
      );

      await program.methods
        .createNftHolder()
        .accounts({
          userAccount: wallet3.publicKey,
          minterAccount: minterKeypair.publicKey,
          tokenHolderAccount: wallet3Holder,
        })
        .signers([wallet3.payer])
        .rpc();
    });

    it("Is Transferring! [A -> B]", async () => {
      if (!wallet2Holder || !wallet3Holder) {
        throw new Error("Wallet2 or Wallet3 not found");
      }

      // Mint Attempt One
      await program.methods
        .transferNft()
        .accounts({
          authority: wallet2.publicKey,
          mint: minterKeypair.publicKey,
          sender: wallet2Holder,
          recipient: wallet3Holder,
        })
        .signers([wallet2.payer])
        .rpc();

      // get total minted tokens
      const accountInfo = await program.provider.connection.getAccountInfo(
        wallet3Holder
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

  describe("NFT", () => {
    const utils = {
      createNFT: async (
        mainWallet: Wallet = wallet3,
        minterKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate(),
        metadata = {
          name: "TestNFT",
          symbol: "TestNFT",
          uri: "TestNFT",
        }
      ) => {
        /**
         * So here is the premise:
         *
         * - Metaplex just provides Metadata and Other SPL_Token Extensions
         * - To have an NFT, 1 NFT = 1 SPL Token Program (yes, including the program that mints and provides token holders)
         *
         * 1. Create a Minter Program Keypair
         * 2. Create a Token Holder Account
         * 3. Create a Metadata Account
         * 4. Create a Master Edition Account
         * 5. Mint the NFT
         */

        // Calculate our token holder address
        const tokenHolder = await getAssociatedTokenAddress(
          minterKeypair.publicKey,
          mainWallet.publicKey
        );

        // Calculate our PDA for the metadata
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

        // Calculate our PDA for the master edition
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

        // Minting 1 NFT
        await program.methods
          .createNft(metadata.name, metadata.symbol, metadata.uri)
          .accounts({
            authorityAccount: mainWallet.publicKey, // The Owner of the NFT
            minterAccount: minterKeypair.publicKey, // The Minter Program
            tokenHolderAccount: tokenHolder, // Our NFT Token Holder Account Program
            metadataAccount: metadataAddress, // Our Metadata PDA
            masterEditionAccount: masterEditionAddress, // Our Master Edition PDA
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID, // The Token Metadata Program
          })
          .signers([mainWallet.payer, minterKeypair]) // We bpth pass our main wallet and the minter program keypair
          .rpc();

        return {
          minterKeypair,
          tokenHolder,
          metadataAddress,
          masterEditionAddress,
        };
      },
      getMetadata: async (metadataAddress) => {
        // get nft name and symbol
        const metadataAccount =
          await program.provider.connection.getAccountInfo(metadataAddress);
        if (metadataAccount === null) {
          throw new Error("Metadata account not found");
        }

        const metadata = Metadata.deserialize(metadataAccount.data);
        console.log("Name: ", metadata[0].data.name);
        console.log("Symbol: ", metadata[0].data.name);
        console.log("URI: ", metadata[0].data.uri);
      },
      updateNFT: async (
        mainWallet,
        metadataAddress,
        metadata = {
          name: "TestNFT",
          symbol: "TestNFT",
          uri: "TestNFT",
        }
      ) => {
        await program.methods
          .updateNftMetadata(metadata.name, metadata.symbol, metadata.uri)
          .accounts({
            authorityAccount: mainWallet.publicKey,
            metadataAccount: metadataAddress,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          })
          .signers([mainWallet.payer])
          .rpc();
        await utils.getMetadata(metadataAddress);
      },
    };

    xit("can mint one NFT", async () => {
      await utils.createNFT();
    });
    xit("can mint one NFT and update metadata", async () => {
      const { metadataAddress } = await utils.createNFT();
      await utils.getMetadata(metadataAddress);

      // Update the metadata
      await utils.updateNFT(wallet3, metadataAddress, {
        name: "NewName",
        symbol: "NewSymbol",
        uri: "NewURI",
      });
      await utils.getMetadata(metadataAddress);

      // Update the metadata
      await utils.updateNFT(wallet3, metadataAddress, {
        name: "NewName2",
        symbol: "NewSymbol2",
        uri: "NewURI2",
      });
      await utils.getMetadata(metadataAddress);
    });
  });
});
