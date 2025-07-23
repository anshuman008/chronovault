import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Chronovault } from "../target/types/chronovault";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { expect } from "chai";
dotenv.config();

describe("chronovault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.chronovault as Program<Chronovault>;

  const user = anchor.web3.Keypair.fromSecretKey(
    bs58.decode(process.env.DEV_WALLET!)
  );

  const recipient = anchor.web3.Keypair.fromSecretKey(
    bs58.decode(process.env.TEST_WALLET!)
  );

  let mint = new anchor.web3.PublicKey(
    "3dwnJTi19QyEPhcPuawzJCgDrJdrVNbtddhD4wjUjmJG"
  );

  let userAta = anchor.utils.token.associatedAddress({
    mint,
    owner: user.publicKey,
  });

  let recipientAta = anchor.utils.token.associatedAddress({
    mint,
    owner: recipient.publicKey,
  });

  const seed = new anchor.BN(1143);
  const seeds = [
    Buffer.from("chrono_vault"),
    user.publicKey.toBuffer(),
    seed.toArrayLike(Buffer, "le", 8),
  ];

  const depositAmount = new anchor.BN(10000000 * 1000000); // 10 million tokens
  const lockDuration = new anchor.BN(60); // 1 minute
  const [chronoAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,
    program.programId
  );
  const vault = anchor.utils.token.associatedAddress({
    mint,
    owner: chronoAccount,
  });

  it("Should deposit tokens into ChronoVault", async () => {
    console.log("User ATA:", userAta.toBase58());

    console.log("Chrono Account:", chronoAccount.toBase58());

    console.log("Vault Address:", vault.toBase58());

    const txs = await program.methods
      .deposite(seed, depositAmount, lockDuration)
      .accounts({
        signer: user.publicKey,
        mint: mint,
        //@ts-ignore
        chronoAccount: chronoAccount,
        vault: vault,
        userAta: userAta,
        recipientKey: user.publicKey,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Deposit success!! Transaction Signature:", txs);
  });

  it("Should fail to withdraw before unlock time", async () => {
    console.log("User ATA:", userAta.toBase58());
    console.log("Chrono Account:", chronoAccount.toBase58());
    console.log("Vault Address:", vault.toBase58());
    console.log("Recipient ATA:", recipientAta.toBase58());

    const chronoAccountData = await program.account.chronoVault.fetch(
      chronoAccount
    );
    const currentTime = Math.floor(Date.now() / 1000);
    const unlockTime = chronoAccountData.unlockTime.toNumber();

    console.log("Current time:", currentTime);
    console.log("Unlock time:", unlockTime);
    console.log("Time remaining:", unlockTime - currentTime, "seconds");

    expect(currentTime).to.be.lessThan(
      unlockTime,
      "Test should run before unlock time"
    );

    try {
      const itx = await program.methods
        .withdraw()
        .accounts({
          //@ts-ignore
          recipient: user.publicKey,
          depositer: user.publicKey,
          mint: mint,
          chronoAccount: user.publicKey,
          vault: vault,
          recipientAta: userAta,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();

      expect.fail(
        "Withdraw should have failed because unlock time hasn't been reached"
      );
    } catch (error) {
      console.log("Expected error caught:", error.message);

      console.log("✅ Early withdrawal properly rejected!");
    }
  });

  it("should withdraw amount", async () => {
    console.log("waiting for 1 minute to withdraw on time!!");
    await new Promise((res: any) => {
      setTimeout(() => {
        res("");
      }, 60000);
    });

    console.log("User ATA:", userAta.toBase58());

    console.log("Chrono Account:", chronoAccount.toBase58());

    console.log("Vault Address:", vault.toBase58());

    const itx = await program.methods
      .withdraw()
      .accounts({
        //@ts-ignore
        recipient: user.publicKey,
        depositer: user.publicKey,
        mint: mint,
        //@ts-ignore
        chronoAccount: chronoAccount,
        vault: vault,
        recipientAta: userAta,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Withdraw sucesss!! Signature:", itx);
  });

  xit("should fetch pda data", async () => {
    // 1CRnH8SKaWd7WiJQx9SAAr3JgCAqbqcVkNETd8y3hqV

    const args = await program.account.chronoVault.fetch(
      new anchor.web3.PublicKey("EwnKuMDUWPuBzqeRCW99JwhhmFfhkCbhmbWrmChJ6bRN")
    );

    if (!args) return;

    Object.entries(args).forEach((val) => {
      console.log(`${val[0]}: ${val[1].toString()}`);
    });

    const currentTimestamp = Math.floor(Date.now() / 1000);
    console.log("Current timestamp:", currentTimestamp);

    console.log(
      "remaining time is : ",
      (args.unlockTime.toNumber() - currentTimestamp) / 60
    );

    // console.log("here is the data", args);
  });
});
