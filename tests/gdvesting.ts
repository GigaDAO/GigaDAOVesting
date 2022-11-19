import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Gdvesting } from "../target/types/gdvesting";
import { TOKEN_PROGRAM_ID, createMint} from "@solana/spl-token";

//consts
const AUTH_PDA_SEED = "auth_pda_seed";

// utils
function to_lamports(num_sol) {
    return Math.round(num_sol * 1e9);
}

describe("gdvesting", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Gdvesting as Program<Gdvesting>;

    it("Is initialized!", async () => {

        const owner1 = anchor.web3.Keypair.generate();
        await program.provider.connection.confirmTransaction(
            await program.provider.connection.requestAirdrop(owner1.publicKey, to_lamports(10)),
            "confirmed"
        );

        let [authPda, _] = await anchor.web3.PublicKey.findProgramAddress(
            [Buffer.from(anchor.utils.bytes.utf8.encode(AUTH_PDA_SEED))],
            program.programId
        );

        let gigsMint = await createMint(
            program.provider.connection,
            owner1,
            owner1.publicKey,
            null,
            4,
        );

        let vestingContract = anchor.web3.Keypair.generate();
        let gigsVault = anchor.web3.Keypair.generate();

        let investor = program.provider.publicKey;
        let vesting_rate = new anchor.BN(1);
        let total_allocated_amount = new anchor.BN(1000);
        let vesting_start_timestamp = new anchor.BN(Math.round(Date.now()/1000) - 20);

        // @ts-ignore
        const tx = await program.methods.initialize(investor, vesting_rate, total_allocated_amount, vesting_start_timestamp)
            .accounts({
                signer: investor,
                authPda: authPda,
                vestingContract: vestingContract.publicKey,
                gigsMint: gigsMint,
                gigsVault: gigsVault.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([vestingContract, gigsVault])
            .rpc();

        console.log("Your transaction signature", tx);

    });
});
