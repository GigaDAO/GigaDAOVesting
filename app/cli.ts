import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

// env consts
const IS_DEVNET = false;
const LOCAL_KEYPAIR_FPATH = "/home/alphaprime8/.config/solana/id.json";
const PROGRAM_ID = 'CA2CNT3eyyie4dKVos5ZoaePSAB2jCZaZRi7Zbdx1RNh'; // can also load from file as done with localKeypair below
const AUTH_PDA_SEED = "auth_pda_seed";
/*

 */

// const BUFFER_ADDRESS = "3rkWkQ1dzhVgdUSWqscBQqzBpB6nnzppbnnFaHPVuNwG";

// program consts
async function initProgram() {
    // INIT Web3 Connection Objects
    const localKeypair = anchor.web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require("fs").readFileSync(LOCAL_KEYPAIR_FPATH, {encoding: "utf-8",}))));
    const programId = new anchor.web3.PublicKey(PROGRAM_ID);
    let wallet = new anchor.Wallet(localKeypair);
    let opts = anchor.AnchorProvider.defaultOptions();
    const network = IS_DEVNET ? anchor.web3.clusterApiUrl('devnet') : anchor.web3.clusterApiUrl('mainnet-beta');
    let connection = new anchor.web3.Connection(network, opts.preflightCommitment);
    let provider = new anchor.AnchorProvider(connection, wallet, opts);
    let idl = await anchor.Program.fetchIdl(programId, provider);
    return new anchor.Program(idl, programId, provider);
}

async function log_version() {
    const program = await initProgram();
    const tx = await program.methods.logVersion().rpc();
    console.log("Your transaction signature", tx);
}

async function initialize_contract() {
    const program = await initProgram();
    let [authPda, _] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode(AUTH_PDA_SEED))],
        program.programId
    );

    let vestingContract = anchor.web3.Keypair.generate();
    let gigsVault = anchor.web3.Keypair.generate();

    let gigsMint = new anchor.web3.PublicKey("9U8Bn6zAf6Wyp1YHdXtLyfbN7yMvdvW1qQY475iZ5ftZ");
    let investor = new anchor.web3.PublicKey("413UGdeVRRpMbwM9LUdXQ8twHgPbbLCHZaqStQgueeTS");
    let vesting_rate = new anchor.BN(6025);
    let total_allocated_amount = new anchor.BN(380000000000);
    let vesting_start_timestamp = new anchor.BN(1685548800);

    // @ts-ignore
    const tx = await program.methods.initialize(investor, vesting_rate, total_allocated_amount, vesting_start_timestamp)
        .accounts({
            signer: program.provider.publicKey,
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

}

initialize_contract()
    .then(()=>{
        console.log("done")
    })
