import {loadKeypair, findAssociatedTokenAddress} from "./utils"
import {createMint, mintTo} from "@solana/spl-token";
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
    const mint = new PublicKey("BaU4MkdjHRUQqHH1F1TkNmc3cmt4t6dJ2zDUHGGLnVNe")

    const destination = findAssociatedTokenAddress(payer.publicKey, mint)
    const tx = await mintTo(
        connection,
        payer,
        mint,
        destination,
        payer.publicKey,
        1
    )

    console.log({mint: mint.toBase58()})
    console.log({mintTx: tx})
    console.log({tokenAccount: destination})
}

// mint: BaU4MkdjHRUQqHH1F1TkNmc3cmt4t6dJ2zDUHGGLnVNe