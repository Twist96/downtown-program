import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {Keypair, PublicKey} from "@solana/web3.js"
import { DowntownProgram } from "../target/types/downtown_program";
import { assert } from "chai";
import {createMint, getOrCreateAssociatedTokenAccount, mintTo} from "@solana/spl-token";

describe("downtown-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = anchor.getProvider().connection;

  const program = anchor.workspace.DowntownProgram as Program<DowntownProgram>;
  const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet

  const [townAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("town")],
      program.programId
  )

  const mintKeypair = Keypair.fromSecretKey(new Uint8Array( [
    18, 228, 169, 213,  37,  58, 118,  81,  46, 235, 191,
    96, 163, 121, 252, 193,  83, 219, 141,  51, 127, 150,
    22,  76,  61,  51, 177, 211,   6, 113,  30,  60,  76,
    170,  98, 238, 151, 134,  10,  79,  44, 235,  21, 153,
    78, 214,  29,  98, 232,  60, 247, 239, 141,   6, 106,
    45, 252, 150, 135,  34, 208, 154, 247, 145
  ]))

  async function createMintToken() {
    const mint = await createMint(
        connection,
        payer.payer,
        payer.publicKey,
        payer.publicKey,
        9,
        mintKeypair
    )
    console.log(mint)
  }

  async function mint(mint: PublicKey, destination: PublicKey) {
    return await mintTo(
        connection,
        payer.payer,
        mint,
        destination,
        payer.payer,
        1
    )
  }

  async function getATA(mint: PublicKey, owner: PublicKey) {
    const connection = anchor.getProvider().connection
    return  await getOrCreateAssociatedTokenAccount(
        connection,
        payer.payer,
        mint,
        owner
    )
  }

  function getVault(mint: PublicKey) {
    return  PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), mint.toBuffer()],
        program.programId
    )
  }

  it("Create Town", async () => {
    const townName = "Downtown"
    const tx = await program.methods
        .createTown(townName)
        .accounts({signer: payer.publicKey, town: townAddress})
        .signers([payer.payer])
        .rpc();

    const town = await program.account.town.fetch(townAddress)
    console.log({tx_createTown: tx, town})
    assert(town.name === townName, `wrong name found: ${town.name}`)
    assert(town.buildings.length === 0, "No building should be found")

    // await createMintToken()
    // console.log("Mint success")
  });

  it("should insert house", async () => {
    const initTown = await program.account.town.fetch(townAddress)
    let positionX = new anchor.BN(0)
    let nft = mintKeypair.publicKey
    let user_nft_ata = (await getATA(nft, payer.publicKey)).address
    let [nftVault] = getVault(nft)
    // console.log({nftVault, townAddress, nft})

    //await mint(mintKeypair.publicKey, user_nft_ata)

    const tx = await program.methods
        .insertHouse(1, positionX, positionX, positionX)
        .accounts({signer: payer.publicKey, town: townAddress, nftMint: nft, userNftAta: user_nft_ata, nftVault})
        .rpc()

    const town = await program.account.town.fetch(townAddress)
    console.log({tx_INSERT_HOUSE: tx, town})
    assert(town.buildings.length === initTown.buildings.length + 1, "House was not added")
  })

  it.only('should remove house', async () => {
    const initTown = await program.account.town.fetch(townAddress)
    let nft = mintKeypair.publicKey
    let user_nft_ata = (await getATA(nft, payer.publicKey)).address
    let [nftVault] = getVault(nft)

    const tx = await program.methods
        .withdrawHouse()
        .accounts({
          signer: payer.publicKey,
          town: townAddress,
          userNftAta: user_nft_ata,
          nftMint: nft,
          nftVault: nftVault
        }).rpc()

    const town = await program.account.town.fetch(townAddress)
    console.log({tx_REMOVE_HOUSE : tx, town})
    assert(town.buildings.length === initTown.buildings.length - 1, "House was not removed")
  });

  it("Get town details", async () => {
    const initTown = await program.account.town.fetch(townAddress)
    console.log({town_name: initTown.name})
    let buildings = initTown.buildings
    for (var building of buildings) {
      console.log(building.id)
      console.log(building.position)
    }
    console.log({town_building_count: initTown.buildings.length})
  })

});
