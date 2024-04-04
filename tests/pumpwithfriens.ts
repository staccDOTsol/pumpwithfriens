import fs from 'fs';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token';

import { Pumpinator } from '../target/types/pumpinator';

describe("Pumpinator", () => {
  // Configure the client to use the local cluster.
  for (const i of [2]){//}, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,15]){
  const provider = new anchor.AnchorProvider(new anchor.web3.Connection("https://jarrett-solana-7ba9.mainnet.rpcpool.com/8d890735-edf2-4a75-af84-92f7c9e31718"),
  new anchor.Wallet(anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(process.env.HOME + "/.config/solana/id.json", {encoding: "utf-8"})))///7i"+i.toString()+".json"
  )), {})
  const program = anchor.workspace.Pumpinator as Program<Pumpinator>;

  it("Is initialized!", async () => {
    // Add your test here.
    const [friend] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("friend")),
    provider.publicKey.toBuffer()],
      program.programId
    );
    const ix = await program.methods.initialize().accounts({
      authority: provider.publicKey,
      friend: friend,
      systemProgram: anchor.web3.SystemProgram.programId})
      .instruction();
      console.log(friend.toBase58())
      const accountMaybe = await provider.connection.getAccountInfo(friend);
      console.log(accountMaybe)
      const balance = await provider.connection.getBalance(provider.publicKey);
      console.log("Your balance is", balance);
      const hexData = "66063d1201daebeac5920e8e0800000035f5b30800000000"
      const data = Buffer.from(hexData, "hex")
      const keys = [
        "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf",
        "CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM",
        "4a7gTZcHJ72LDLPL2SS77ya8yT2AJKmdD2LnAqfqPBN8",
        "52PQBMYkMDvg56GmfQZrgTCpufzgdabpPWaoT3Ket2fR",
        "4Wpd2PgFY37M56btxspMetFvQ89Garrbt9QdSgELfUFk",
        (await getOrCreateAssociatedTokenAccount(provider.connection, anchor.web3.Keypair.fromSecretKey(
          new Uint8Array(JSON.parse(fs.readFileSync(process.env.HOME + "/.config/solana/id.json", {encoding: "utf-8"})))), new anchor.web3.PublicKey("4a7gTZcHJ72LDLPL2SS77ya8yT2AJKmdD2LnAqfqPBN8"),

        friend, true)).address,
        friend.toBase58(),
        "11111111111111111111111111111111",
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        "SysvarRent111111111111111111111111111111111",
        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"
      ].map((x) => new anchor.web3.PublicKey(x))
      const isSigners =  [false, false, false, false, false, false, true, false, false, false];
      let isWritables = [false, true, true, true, true, true, true, false, false, false];
      const txtx = await program.methods.pump(data)
      .accounts({
        jare: new anchor.web3.PublicKey("7ihN8QaTfNoDTRTQGULCzbUT3PHwPDTu5Brcu4iT2paP"),
        authority: provider.publicKey,
        friend: friend,
      })
      .remainingAccounts(
        keys.map((key, i) => {
          return {
            pubkey: key,
            isSigner: isSigners[i],
            isWritable: isWritables[i]
          }
        })
      )
      .instruction();
      const atx = new anchor.web3.Transaction().add(        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({microLamports: 234000})
      ).      add(txtx)
      atx.recentBlockhash = (await provider.connection.getRecentBlockhash()).blockhash;
      atx.feePayer = provider.publicKey;
      atx.sign(anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync(process.env.HOME + "/.config/solana/id.json", {encoding: "utf-8"})))))
        const d = (atx.instructions[1].data)
const hexData2 = Buffer.from(d).toString("hex")
console.log("Your pump data", hexData2);
      const signature = await provider.connection.sendRawTransaction(atx.serialize({requireAllSignatures:false, verifySignatures: false}), {skipPreflight: false, preflightCommitment: "singleGossip"})


console.log("Your pump signature", signature);
      /*
      const ix2 = await program.methods.deposit(new anchor.BN(balance.toString()).div(new anchor.BN(100)).mul(new anchor.BN(90)))
      .accounts({
        authority: provider.publicKey,
        friend: friend,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .instruction();
      const tx = new anchor.web3.Transaction().add(anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({microLamports: 234000}))
      if (accountMaybe == undefined){
        tx.add(ix);
      }
      tx.add(ix2)
      const sig =  await provider.sendAndConfirm(tx, [anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync(process.env.HOME + "/.config/solana/id.json", {encoding: "utf-8"}))))], {skipPreflight: false});
      console.log("Your deposit signature", sig);
     */
     

      if (false){
      const withdrawTx = await program.methods.withdraw(new anchor.BN(balance.toString()).div(new anchor.BN(100)).mul(new anchor.BN(90)))
      .accounts({
        authority: provider.publicKey,
        friend: friend,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();
      console.log("Your withdraw signature, 1", withdrawTx);
    }
      
  });
  }
});
