import fs from 'fs';

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import {ComputeBudgetProgram, Transaction} from '@solana/web3.js'
import { Pumpinator } from '../target/types/pumpinator';

describe("Pumpinator", () => {
  // Configure the client to use the local cluster.
  for (const i of [0]){//}, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,15]){
  const provider = new anchor.AnchorProvider(new anchor.web3.Connection( process.env.MAINNET_URL as string),
  new anchor.Wallet(anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(process.env.HOME + "/mani.json", {encoding: "utf-8"})))///7i"+i.toString()+".json"
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
      const tx = new Transaction().add(ComputeBudgetProgram.setComputeUnitPrice({microLamports: 64000})).add(ix)
      tx.recentBlockhash = await (await provider.connection.getLatestBlockhash()).blockhash
      tx.feePayer = new anchor.Wallet(anchor.web3.Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync(process.env.HOME + "/mani.json", {encoding: "utf-8"})))///7i"+i.toString()+".json"
      )).publicKey
      await provider.sendAndConfirm(tx)
      console.log(friend.toBase58())
      const accountMaybe = await provider.connection.getAccountInfo(friend);
      console.log(accountMaybe)
      const balance = await provider.connection.getBalance(provider.publicKey);
      console.log("Your balance is", balance);
      const hexData = "66063d1201daebeac5920e8e0800000035f5b30800000000"
      const data = Buffer.from(hexData, "hex")
      /*
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
      ].map((x) => new anchor.web3.PublicKey(x))*/
  });
  }
});
