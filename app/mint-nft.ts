import {findAssociatedTokenAddress, loadKeypair} from "./utils"
import {createMint, getOrCreateAssociatedTokenAccount, mintTo} from "@solana/spl-token";
import web3, {Keypair, PublicKey} from "@solana/web3.js";

const endpoint = "http://localhost:8899"
const connection = new web3.Connection(endpoint)

const keypair = loadKeypair()
const payer = Keypair.fromSecretKey(keypair.secretKey)

export async function createMyMint() {
    console.log("####creating mint####")
    return await createMint(
        connection,
        payer,
        payer.publicKey,
        payer.publicKey,
        0
    )
}

export async function mintNft() {
    // const mint = await createMyMint()
    console.log("####minting NFT####")
    const mint = new PublicKey("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd")
    const destination = await getATA(mint, payer.publicKey)

    const tx = await mintTo(
        connection,
        payer,
        mint,
        destination.address,
        payer.publicKey,
        1
    )

    console.log({mint: mint.toBase58()})
    console.log({tokenAccount: destination.address.toBase58()})
    console.log({mintTx: tx})
}

async function getATA(mint: PublicKey, owner: PublicKey) {
    return await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        owner
    )
}

// mint: DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd
// tokenAccount: 5RTS6WhVqdUPTD2G2Ku1xp1cVeATtZGCAA2AT2tzPWxR
// mintTx: 34aPLy53bNicToM4cFkXnmwgGuqWwiTY3ArXWcv68FFexF5xeZrMeDvqVgrZfqhAe4CK5AgrPQ5JSFpUcvX3rnsW