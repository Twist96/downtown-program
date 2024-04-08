/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import * as beetSolana from '@metaplex-foundation/beet-solana'
import { Building, buildingBeet } from '../types/Building'

/**
 * Arguments used to create {@link Town}
 * @category Accounts
 * @category generated
 */
export type TownArgs = {
  name: string
  buildings: Building[]
}

export const townDiscriminator = [1, 240, 140, 130, 211, 194, 246, 114]
/**
 * Holds the data for the {@link Town} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class Town implements TownArgs {
  private constructor(readonly name: string, readonly buildings: Building[]) {}

  /**
   * Creates a {@link Town} instance from the provided args.
   */
  static fromArgs(args: TownArgs) {
    return new Town(args.name, args.buildings)
  }

  /**
   * Deserializes the {@link Town} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [Town, number] {
    return Town.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link Town} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<Town> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find Town account at ${address}`)
    }
    return Town.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'CgGCmVn7W9zjKjAqw3ypEQfEEiJGSM1u87AzyEC81m5b'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, townBeet)
  }

  /**
   * Deserializes the {@link Town} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [Town, number] {
    return townBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link Town} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return townBeet.serialize({
      accountDiscriminator: townDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link Town} for the provided args.
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   */
  static byteSize(args: TownArgs) {
    const instance = Town.fromArgs(args)
    return townBeet.toFixedFromValue({
      accountDiscriminator: townDiscriminator,
      ...instance,
    }).byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link Town} data from rent
   *
   * @param args need to be provided since the byte size for this account
   * depends on them
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    args: TownArgs,
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      Town.byteSize(args),
      commitment
    )
  }

  /**
   * Returns a readable version of {@link Town} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      name: this.name,
      buildings: this.buildings,
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const townBeet = new beet.FixableBeetStruct<
  Town,
  TownArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['name', beet.utf8String],
    ['buildings', beet.array(buildingBeet)],
  ],
  Town.fromArgs,
  'Town'
)