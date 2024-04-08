using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;
using Solana.Unity;
using Solana.Unity.Programs.Abstract;
using Solana.Unity.Programs.Utilities;
using Solana.Unity.Rpc;
using Solana.Unity.Rpc.Builders;
using Solana.Unity.Rpc.Core.Http;
using Solana.Unity.Rpc.Core.Sockets;
using Solana.Unity.Rpc.Types;
using Solana.Unity.Wallet;
using DowntownProgram;
using DowntownProgram.Program;
using DowntownProgram.Errors;
using DowntownProgram.Accounts;
using DowntownProgram.Types;

namespace DowntownProgram
{
    namespace Accounts
    {
        public partial class Town
        {
            public static ulong ACCOUNT_DISCRIMINATOR => 8284022778278768641UL;
            public static ReadOnlySpan<byte> ACCOUNT_DISCRIMINATOR_BYTES => new byte[]{1, 240, 140, 130, 211, 194, 246, 114};
            public static string ACCOUNT_DISCRIMINATOR_B58 => "KpPJyNfCBb";
            public string Name { get; set; }

            public Building[] Buildings { get; set; }

            public static Town Deserialize(ReadOnlySpan<byte> _data)
            {
                int offset = 0;
                ulong accountHashValue = _data.GetU64(offset);
                offset += 8;
                if (accountHashValue != ACCOUNT_DISCRIMINATOR)
                {
                    return null;
                }

                Town result = new Town();
                offset += _data.GetBorshString(offset, out var resultName);
                result.Name = resultName;
                int resultBuildingsLength = (int)_data.GetU32(offset);
                offset += 4;
                result.Buildings = new Building[resultBuildingsLength];
                for (uint resultBuildingsIdx = 0; resultBuildingsIdx < resultBuildingsLength; resultBuildingsIdx++)
                {
                    offset += Building.Deserialize(_data, offset, out var resultBuildingsresultBuildingsIdx);
                    result.Buildings[resultBuildingsIdx] = resultBuildingsresultBuildingsIdx;
                }

                return result;
            }
        }
    }

    namespace Errors
    {
        public enum DowntownProgramErrorKind : uint
        {
            BuildingNotFound = 6000U,
            InsufficientVaultSol = 6001U,
            InsufficientRentVault = 6002U,
            UnauthorizedSigner = 6003U,
            InsufficientVaultAsset = 6004U
        }
    }

    namespace Types
    {
        public partial class Building
        {
            public PublicKey Id { get; set; }

            public PublicKey Owner { get; set; }

            public byte HouseVariant { get; set; }

            public Vector3D Position { get; set; }

            public Vector3D Scale { get; set; }

            public ulong StakeSlot { get; set; }

            public int Serialize(byte[] _data, int initialOffset)
            {
                int offset = initialOffset;
                _data.WritePubKey(Id, offset);
                offset += 32;
                _data.WritePubKey(Owner, offset);
                offset += 32;
                _data.WriteU8(HouseVariant, offset);
                offset += 1;
                offset += Position.Serialize(_data, offset);
                offset += Scale.Serialize(_data, offset);
                _data.WriteU64(StakeSlot, offset);
                offset += 8;
                return offset - initialOffset;
            }

            public static int Deserialize(ReadOnlySpan<byte> _data, int initialOffset, out Building result)
            {
                int offset = initialOffset;
                result = new Building();
                result.Id = _data.GetPubKey(offset);
                offset += 32;
                result.Owner = _data.GetPubKey(offset);
                offset += 32;
                result.HouseVariant = _data.GetU8(offset);
                offset += 1;
                offset += Vector3D.Deserialize(_data, offset, out var resultPosition);
                result.Position = resultPosition;
                offset += Vector3D.Deserialize(_data, offset, out var resultScale);
                result.Scale = resultScale;
                result.StakeSlot = _data.GetU64(offset);
                offset += 8;
                return offset - initialOffset;
            }
        }

        public partial class Vector3D
        {
            public long X { get; set; }

            public long Y { get; set; }

            public long Z { get; set; }

            public int Serialize(byte[] _data, int initialOffset)
            {
                int offset = initialOffset;
                _data.WriteS64(X, offset);
                offset += 8;
                _data.WriteS64(Y, offset);
                offset += 8;
                _data.WriteS64(Z, offset);
                offset += 8;
                return offset - initialOffset;
            }

            public static int Deserialize(ReadOnlySpan<byte> _data, int initialOffset, out Vector3D result)
            {
                int offset = initialOffset;
                result = new Vector3D();
                result.X = _data.GetS64(offset);
                offset += 8;
                result.Y = _data.GetS64(offset);
                offset += 8;
                result.Z = _data.GetS64(offset);
                offset += 8;
                return offset - initialOffset;
            }
        }
    }

    public partial class DowntownProgramClient : TransactionalBaseClient<DowntownProgramErrorKind>
    {
        public DowntownProgramClient(IRpcClient rpcClient, IStreamingRpcClient streamingRpcClient, PublicKey programId) : base(rpcClient, streamingRpcClient, programId)
        {
        }

        public async Task<Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<Town>>> GetTownsAsync(string programAddress, Commitment commitment = Commitment.Finalized)
        {
            var list = new List<Solana.Unity.Rpc.Models.MemCmp>{new Solana.Unity.Rpc.Models.MemCmp{Bytes = Town.ACCOUNT_DISCRIMINATOR_B58, Offset = 0}};
            var res = await RpcClient.GetProgramAccountsAsync(programAddress, commitment, memCmpList: list);
            if (!res.WasSuccessful || !(res.Result?.Count > 0))
                return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<Town>>(res);
            List<Town> resultingAccounts = new List<Town>(res.Result.Count);
            resultingAccounts.AddRange(res.Result.Select(result => Town.Deserialize(Convert.FromBase64String(result.Account.Data[0]))));
            return new Solana.Unity.Programs.Models.ProgramAccountsResultWrapper<List<Town>>(res, resultingAccounts);
        }

        public async Task<Solana.Unity.Programs.Models.AccountResultWrapper<Town>> GetTownAsync(string accountAddress, Commitment commitment = Commitment.Finalized)
        {
            var res = await RpcClient.GetAccountInfoAsync(accountAddress, commitment);
            if (!res.WasSuccessful)
                return new Solana.Unity.Programs.Models.AccountResultWrapper<Town>(res);
            var resultingAccount = Town.Deserialize(Convert.FromBase64String(res.Result.Value.Data[0]));
            return new Solana.Unity.Programs.Models.AccountResultWrapper<Town>(res, resultingAccount);
        }

        public async Task<SubscriptionState> SubscribeTownAsync(string accountAddress, Action<SubscriptionState, Solana.Unity.Rpc.Messages.ResponseValue<Solana.Unity.Rpc.Models.AccountInfo>, Town> callback, Commitment commitment = Commitment.Finalized)
        {
            SubscriptionState res = await StreamingRpcClient.SubscribeAccountInfoAsync(accountAddress, (s, e) =>
            {
                Town parsingResult = null;
                if (e.Value?.Data?.Count > 0)
                    parsingResult = Town.Deserialize(Convert.FromBase64String(e.Value.Data[0]));
                callback(s, e, parsingResult);
            }, commitment);
            return res;
        }

        public async Task<RequestResult<string>> SendCreateTownAsync(CreateTownAccounts accounts, string name, PublicKey feePayer, Func<byte[], PublicKey, byte[]> signingCallback, PublicKey programId)
        {
            Solana.Unity.Rpc.Models.TransactionInstruction instr = Program.DowntownProgramProgram.CreateTown(accounts, name, programId);
            return await SignAndSendTransaction(instr, feePayer, signingCallback);
        }

        public async Task<RequestResult<string>> SendInsertHouseAsync(InsertHouseAccounts accounts, byte houseVariant, long positionX, long positionY, long positionZ, PublicKey feePayer, Func<byte[], PublicKey, byte[]> signingCallback, PublicKey programId)
        {
            Solana.Unity.Rpc.Models.TransactionInstruction instr = Program.DowntownProgramProgram.InsertHouse(accounts, houseVariant, positionX, positionY, positionZ, programId);
            return await SignAndSendTransaction(instr, feePayer, signingCallback);
        }

        public async Task<RequestResult<string>> SendWithdrawHouseAsync(WithdrawHouseAccounts accounts, PublicKey feePayer, Func<byte[], PublicKey, byte[]> signingCallback, PublicKey programId)
        {
            Solana.Unity.Rpc.Models.TransactionInstruction instr = Program.DowntownProgramProgram.WithdrawHouse(accounts, programId);
            return await SignAndSendTransaction(instr, feePayer, signingCallback);
        }

        public async Task<RequestResult<string>> SendFundRentVaultAsync(FundRentVaultAccounts accounts, ulong amount, PublicKey feePayer, Func<byte[], PublicKey, byte[]> signingCallback, PublicKey programId)
        {
            Solana.Unity.Rpc.Models.TransactionInstruction instr = Program.DowntownProgramProgram.FundRentVault(accounts, amount, programId);
            return await SignAndSendTransaction(instr, feePayer, signingCallback);
        }

        public async Task<RequestResult<string>> SendWithdrawRentVaultAsync(WithdrawRentVaultAccounts accounts, ulong amount, PublicKey feePayer, Func<byte[], PublicKey, byte[]> signingCallback, PublicKey programId)
        {
            Solana.Unity.Rpc.Models.TransactionInstruction instr = Program.DowntownProgramProgram.WithdrawRentVault(accounts, amount, programId);
            return await SignAndSendTransaction(instr, feePayer, signingCallback);
        }

        public async Task<RequestResult<string>> SendClaimRentAsync(ClaimRentAccounts accounts, PublicKey feePayer, Func<byte[], PublicKey, byte[]> signingCallback, PublicKey programId)
        {
            Solana.Unity.Rpc.Models.TransactionInstruction instr = Program.DowntownProgramProgram.ClaimRent(accounts, programId);
            return await SignAndSendTransaction(instr, feePayer, signingCallback);
        }

        protected override Dictionary<uint, ProgramError<DowntownProgramErrorKind>> BuildErrorsDictionary()
        {
            return new Dictionary<uint, ProgramError<DowntownProgramErrorKind>>{{6000U, new ProgramError<DowntownProgramErrorKind>(DowntownProgramErrorKind.BuildingNotFound, "House not found in town")}, {6001U, new ProgramError<DowntownProgramErrorKind>(DowntownProgramErrorKind.InsufficientVaultSol, "Not enough sol in vault")}, {6002U, new ProgramError<DowntownProgramErrorKind>(DowntownProgramErrorKind.InsufficientRentVault, "Not enough tokens to pay rent")}, {6003U, new ProgramError<DowntownProgramErrorKind>(DowntownProgramErrorKind.UnauthorizedSigner, "Asset not owned by signer")}, {6004U, new ProgramError<DowntownProgramErrorKind>(DowntownProgramErrorKind.InsufficientVaultAsset, "Asset not present in vault")}, };
        }
    }

    namespace Program
    {
        public class CreateTownAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey Town { get; set; }

            public PublicKey SystemProgram { get; set; }
        }

        public class InsertHouseAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey Town { get; set; }

            public PublicKey UserNftAta { get; set; }

            public PublicKey NftVault { get; set; }

            public PublicKey NftMint { get; set; }

            public PublicKey SystemProgram { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; }
        }

        public class WithdrawHouseAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey Town { get; set; }

            public PublicKey UserNftAta { get; set; }

            public PublicKey NftVault { get; set; }

            public PublicKey NftMint { get; set; }

            public PublicKey SystemProgram { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; }
        }

        public class FundRentVaultAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey UserTokenAccount { get; set; }

            public PublicKey RentVault { get; set; }

            public PublicKey TokenMint { get; set; }

            public PublicKey SystemProgram { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; }
        }

        public class WithdrawRentVaultAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey UserTokenAccount { get; set; }

            public PublicKey RentVault { get; set; }

            public PublicKey NftMint { get; set; }

            public PublicKey TokenMint { get; set; }

            public PublicKey SystemProgram { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; }
        }

        public class ClaimRentAccounts
        {
            public PublicKey Signer { get; set; }

            public PublicKey UserTokenAccount { get; set; }

            public PublicKey RentVault { get; set; }

            public PublicKey Town { get; set; }

            public PublicKey NftMint { get; set; }

            public PublicKey TokenMint { get; set; }

            public PublicKey SystemProgram { get; set; }

            public PublicKey TokenProgram { get; set; }

            public PublicKey AssociatedTokenProgram { get; set; }
        }

        public static class DowntownProgramProgram
        {
            public static Solana.Unity.Rpc.Models.TransactionInstruction CreateTown(CreateTownAccounts accounts, string name, PublicKey programId)
            {
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Town, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(15957497349440900128UL, offset);
                offset += 8;
                offset += _data.WriteBorshString(name, offset);
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction InsertHouse(InsertHouseAccounts accounts, byte houseVariant, long positionX, long positionY, long positionZ, PublicKey programId)
            {
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Town, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.UserNftAta, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.NftVault, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.NftMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(16117548766784636737UL, offset);
                offset += 8;
                _data.WriteU8(houseVariant, offset);
                offset += 1;
                _data.WriteS64(positionX, offset);
                offset += 8;
                _data.WriteS64(positionY, offset);
                offset += 8;
                _data.WriteS64(positionZ, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction WithdrawHouse(WithdrawHouseAccounts accounts, PublicKey programId)
            {
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Town, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.UserNftAta, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.NftVault, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.NftMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(10612423312654920930UL, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction FundRentVault(FundRentVaultAccounts accounts, ulong amount, PublicKey programId)
            {
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.UserTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.RentVault, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(225003729026153972UL, offset);
                offset += 8;
                _data.WriteU64(amount, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction WithdrawRentVault(WithdrawRentVaultAccounts accounts, ulong amount, PublicKey programId)
            {
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.UserTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.RentVault, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.NftMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(3304478577311187558UL, offset);
                offset += 8;
                _data.WriteU64(amount, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }

            public static Solana.Unity.Rpc.Models.TransactionInstruction ClaimRent(ClaimRentAccounts accounts, PublicKey programId)
            {
                List<Solana.Unity.Rpc.Models.AccountMeta> keys = new()
                {Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.Signer, true), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.UserTokenAccount, false), Solana.Unity.Rpc.Models.AccountMeta.Writable(accounts.RentVault, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.Town, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.NftMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenMint, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.SystemProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.TokenProgram, false), Solana.Unity.Rpc.Models.AccountMeta.ReadOnly(accounts.AssociatedTokenProgram, false)};
                byte[] _data = new byte[1200];
                int offset = 0;
                _data.WriteU64(7285246838288148793UL, offset);
                offset += 8;
                byte[] resultData = new byte[offset];
                Array.Copy(_data, resultData, offset);
                return new Solana.Unity.Rpc.Models.TransactionInstruction{Keys = keys, ProgramId = programId.KeyBytes, Data = resultData};
            }
        }
    }
}