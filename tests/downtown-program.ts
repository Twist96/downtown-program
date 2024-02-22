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

  it.skip("Create Town", async () => {
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
  });

  it("should insert house", async () => {
    const initTown = await program.account.town.fetch(townAddress)
    let positionX = new anchor.BN(0)
    let nft = new PublicKey("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd")

    const tx = await program.methods
        .insertHouse(1, positionX, positionX, positionX)
        .accounts({signer: payer.publicKey, town: townAddress, nft: nft})
        .rpc()

    const town = await program.account.town.fetch(townAddress)
    console.log({tx_INSERT_HOUSE: tx, town})
    assert(town.buildings.length === initTown.buildings.length + 1, "House was not added")
  })

  it('should remove house', async () => {
    const initTown = await program.account.town.fetch(townAddress)
    let nft = new PublicKey("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd")

    const tx = await program.methods
        .withdrawHouse("DWDRomhCxYJhodb5vbYeYGZpLTSC9CFpoUEZ8W4CGaYd")
        .accounts({signer: payer.publicKey, town: townAddress, nft: nft})
        .rpc()

    const town = await program.account.town.fetch(townAddress)
    console.log({tx_REMOVE_HOUSE : tx, town})
    assert(town.buildings.length === initTown.buildings.length - 1, "House was not removed")
  });

  it.skip("Get town details", async () => {
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
