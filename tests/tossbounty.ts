import * as anchor from "@coral-xyz/anchor";
import { Tossbounty } from "../target/types/tossbounty";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as splToken from "@solana/spl-token";
import assert from "assert";

describe("Bounty Program with SPL Token Rewards", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Tossbounty as Program<Tossbounty>;
    const example = anchor.workspace.Example as Program<Example>;
    const payer = (provider.wallet as NodeWallet).payer;
    let mintPubkey: anchor.web3.PublicKey;
    let fundingAccount: anchor.web3.PublicKey;
    let bountyRewardAmount: anchor.BN;
    let whitehatTokenAccount: anchor.web3.PublicKey;
    const org = "coral-xyz";
    const description = "Fix a bug in our app";

    before(async () => {
      mintPubkey = await splToken.createMint(
        provider.connection,
        payer,
        provider.wallet.publicKey,
        provider.wallet.publicKey,
        6,
      );
    });

    it("Creates a bounty", async () => {
        await provider.connection.requestAirdrop(payer.publicKey, 10*LAMPORTS_PER_SOL);

        const balance = await provider.connection.getBalance(payer.publicKey);

        assert.ok(balance > 0);

        bountyRewardAmount = new anchor.BN(10000);

        fundingAccount = await splToken.createAccount(
          provider.connection,
          payer,
          mintPubkey, 
          payer.publicKey, 
        );

        const [bountyPda, bump] = anchor.web3.PublicKey.findProgramAddressSync([
          anchor.utils.bytes.utf8.encode("bounty"),
          payer.publicKey.toBuffer(),
          example.programId.toBuffer()
        ], program.programId)

        const ix = await program.methods.createBountyExample(description, org, bountyRewardAmount, bump).accounts({
          bounty: bountyPda,
          fundingAccount: fundingAccount, 
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
          programId: example.programId,
        });

        await ix.rpc();
    });

    it("Claims a bounty", async () => {
        const [bountyPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([
          anchor.utils.bytes.utf8.encode("bounty"),
          payer.publicKey.toBuffer(),
          example.programId.toBuffer()
        ], program.programId)

        const whitehatKeypair = anchor.web3.Keypair.generate();

        await provider.connection.requestAirdrop(whitehatKeypair.publicKey, 10 * LAMPORTS_PER_SOL);

        await splToken.mintTo(provider.connection, payer, mintPubkey, fundingAccount, payer.publicKey, bountyRewardAmount.toNumber(), [], undefined, splToken.TOKEN_PROGRAM_ID);

        whitehatTokenAccount = await splToken.createAccount(
          provider.connection,
          whitehatKeypair,
          mintPubkey, 
          whitehatKeypair.publicKey, 
        );

        const ix = await program.methods.claimBountyExample().accounts({
          bounty: bountyPda,
          whitehatTokenAccount: whitehatTokenAccount, 
          fundingAccount: fundingAccount,
          programId: example.programId,
        });

        await ix.rpc();
    });
});

