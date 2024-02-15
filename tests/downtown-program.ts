import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js"
import { DowntownProgram } from "../target/types/downtown_program";
import { assert } from "chai";

describe("downtown-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DowntownProgram as Program<DowntownProgram>;
  const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet

  const [townAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("town")],
      program.programId
  )

  console.log({program___id: program.programId})

  it.only("Create Town", async () => {
    const townName = "Downtown"
    const tx = await program.methods
        .createTown(townName)
        .accounts({signer: payer.publicKey, town: townAddress})
        .signers([payer.payer])
        .rpc();

    console.log("Your transaction signature", tx);
    const town = await program.account.town.fetch(townAddress)
    console.log({town})
    assert(town.name === townName, `wrong name found: ${town.name}`)
  });

  it("Add house", async () => {
    const initTown = await program.account.town.fetch(townAddress)
    console.log({initTown})
    let positionX = new anchor.BN(0)
    let nft = new PublicKey("BaU4MkdjHRUQqHH1F1TkNmc3cmt4t6dJ2zDUHGGLnVNe")

    const tx = await program.methods
        .insertHouse(1, positionX, positionX, positionX)
        .accounts({signer: payer.publicKey, town: townAddress, nft: nft})
        .rpc()

    console.log("Your transaction signature", tx);
    const town = await program.account.town.fetch(townAddress)
    console.log({town})
    console.assert(town.buildings.length === initTown.buildings.length + 1, "House was not added")
  })

  it("Get town details", async () => {
    const initTown = await program.account.town.fetch(townAddress)
    console.log({town_name: initTown.name})
    console.log({town_buildings: initTown.buildings})
    console.log({town_building_count: initTown.buildings.length})
  })

});
