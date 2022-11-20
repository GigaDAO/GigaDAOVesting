import * as anchor from "@project-serum/anchor";

// env consts
const IS_DEVNET = false;
const LOCAL_KEYPAIR_FPATH = "/home/alphaprime8/.config/solana/id.json";
const PROGRAM_ID = 'CA2CNT3eyyie4dKVos5ZoaePSAB2jCZaZRi7Zbdx1RNh'; // can also load from file as done with localKeypair below

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

async function initialize() {
    const program = await initProgram();
    const tx = await program.methods.logVersion().rpc();
    console.log("Your transaction signature", tx);
}

initialize()
    .then(()=>{
        console.log("done")
    })
