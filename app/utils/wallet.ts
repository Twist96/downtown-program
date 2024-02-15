import { web3 } from "@coral-xyz/anchor";
import fs from "fs";


export function loadKeypair() {
    return web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync("/Users/matthewchukwuemeka/.config/solana/id.json").toString()))
        // new Uint8Array()
    )
}
