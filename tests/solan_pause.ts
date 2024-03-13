import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanPause } from "../target/types/solan_pause";
import { MockProgram } from '../target/types/mock_exploit_program';
import { execSync } from "child_process";
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js';

describe("solan_pause", () => {
  const program = anchor.workspace.SolanPause as Program<SolanPause>;
  const mockProgram = anchor.workspace.MockProgram as Program<MockProgram>;

  it("registers a new cpi program", async () => {
    // Register a new CPI program
    const tx = await program.rpc.registerCpiProgram(/* program details */);
    console.log("CPI program registration transaction signature:", tx);
    // You'd typically check the program's state or a specific account to verify the registration was successful
  }

  it("verifies that an exploit exists", async () => {
    // Create a new Keypair for the target account
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    //const targetAccount = PublicKey.createWithSeed(provider.wallet.publicKey, "counter", program.programId);

    //const counterAccount = anchor.web3.Keypair.generate();
    //const rentExemption = await provider.connection.getMinimumBalanceForRentExemption(program.account.counter.size);

    const [exploitPda, exploitAccountBump] = await PublicKey.findProgramAddress(
        [Buffer.from("exploit")],
        program.programId
    );

    const exploitData = Buffer.from("your_exploit_data_here", "utf-8");

    console.log('The exploit pda is', exploitPda.toBase58());
    const tx = await program.rpc.submitExploit(exploitData,{
        accounts: {
            exploit: exploitPda,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        }
    });

    console.log('Transaction signature', tx);
    //const seeds = []
    //const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, mockProgram.programId);

    //console.log("the storage account address is", myStorage.toBase58());

    // Assume `TargetData` is the account struct you've defined in your Rust program
    // and you're using it to initialize the account with some default data.
    // Replace `yourProgram` with the actual variable holding your deployed program.
    //const targetDataAccount = await mockProgram.rpc.initializeTargetData(
        //// Pass in initialization parameters here
        //{
            //accounts: {
                //targetAccount: targetAccount.publicKey,
                //user: provider.wallet.publicKey,
                //systemProgram: anchor.web3.SystemProgram.programId,
            //},
            //signers: [targetAccount],
        //}
    //);

    // Now `targetAccount` is initialized and can be used in your tests
    const [targetAccount, targetAccountBump] = await PublicKey.findProgramAddress(
        [Buffer.from("target_account")],
        program.programId
    );

    await mockProgram.methods.simulateExploit(exploitData)
        .accounts({
            targetAccount: targetAccount,
            // Include other necessary accounts
        })
        .rpc();

    const verificationResult = await program.methods.verifyExploit()
        .accounts({
            targetAccount: targetAccount,
            // Add other accounts as needed by `verifyExploit`
        })
        .rpc();

    console.log("Verification transaction signature:", verificationResult);

    //// Fetch the updated state of the target account to verify if `verifyExploit` made any expected changes
    const updatedTargetAccount = await program.account.targetData.fetch(targetAccount.publicKey);

    assert.isTrue(updatedTargetAccount.flagged, "Account was not flagged as expected");
  });

  it("pauses the program upon detecting an exploit", async () => {
    // Here, you'd simulate detecting an exploit and then pausing the program
    await program.methods.detectAndPause().rpc();
    const isPaused = await program.account.someState.fetch(/* program state pubkey */);
    if (!isPaused) {
      throw new Error("Program should be paused but is not.");
    }
  });

  it("allows a white hat to submit an exploit", async () => {
    // Simulate a white hat submitting an exploit
    const tx = await program.methods.submitExploit(/* exploit details */).rpc();
    console.log("Exploit submission transaction signature:", tx);
    // You'd typically check the program's state or a specific account to verify the exploit submission was processed
  });
});

