import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Chronovault } from "../target/types/chronovault";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
dotenv.config();


describe("chronovault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.chronovault as Program<Chronovault>;
  const provider = anchor.AnchorProvider.env();

  const user = anchor.web3.Keypair.fromSecretKey(
  bs58.decode(process.env.DEV_WALLET!));


  const recipient = anchor.web3.Keypair.fromSecretKey(
  bs58.decode(process.env.TEST_WALLET!));
  

  let mint = new anchor.web3.PublicKey("3dwnJTi19QyEPhcPuawzJCgDrJdrVNbtddhD4wjUjmJG");

  let userAta = anchor.utils.token.associatedAddress({
    mint,
    owner: user.publicKey,
  });


  let recipientAta = anchor.utils.token.associatedAddress({
    mint,
    owner: recipient.publicKey,
  });



  // const seed = new anchor.BN(12987);
    // const seed = new anchor.BN(12345);
  const seed = new anchor.BN(12999);


  const seeds = [
        Buffer.from("chrono_vault"),
        user.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8)
      ];
  const depositAmount = new anchor.BN(10000000*1000000); // 1 token (assuming 6 decimals)
  const lockDuration = new anchor.BN(120); 
  const [chronoAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );
  
    const vault = anchor.utils.token.associatedAddress({
    mint,
    owner: chronoAccount,
  });


   xit("Should deposit tokens into ChronoVault", async () => {
    // Get initial balances


  console.log("User ATA:", userAta.toBase58());

  console.log("Chrono Account:", chronoAccount.toBase58());

  console.log("Vault Address:", vault.toBase58());

    const txs = await program.methods.
    deposite(seed, depositAmount, lockDuration)
    .accounts({
      signer: user.publicKey,
      mint: mint,
      //@ts-ignore
      chronoAccount: chronoAccount,
      vault: vault,
      userAta: userAta,
      recipientKey: recipient.publicKey,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user])
    .rpc();

    console.log("Transaction Signature:", txs);
  })


  it("should withdraw amount",  async () => {



  console.log("User ATA:", userAta.toBase58());

  console.log("Chrono Account:", chronoAccount.toBase58());

  console.log("Vault Address:", vault.toBase58());

     const itx = await program.methods.withdraw()
     .accounts({
      //@ts-ignore
      recipient: recipient.publicKey,
      depositer: user.publicKey,
      mint: mint,
      //@ts-ignore
      chronoAccount: chronoAccount,
      vault: vault,
      recipientAta: recipientAta,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
     })
     .signers([recipient])
    .rpc();
  
  
    console.log("Transaction Signature:", itx);

    })


   xit("should fetch pda data", async() => {

    // 1CRnH8SKaWd7WiJQx9SAAr3JgCAqbqcVkNETd8y3hqV
    
      const args = await program.account.chronoVault.fetch(
        new anchor.web3.PublicKey("EwnKuMDUWPuBzqeRCW99JwhhmFfhkCbhmbWrmChJ6bRN")
      );


      if(!args) return;


      Object.entries(args).forEach((val) => {
        console.log(`${val[0]}: ${val[1].toString()}`)
      })

      const currentTimestamp = Math.floor(Date.now() / 1000);
      console.log("Current timestamp:", currentTimestamp);


            console.log("remaining time is : ",( args.unlockTime.toNumber() - currentTimestamp)/60)

      // console.log("here is the data", args);
    })

});
