import * as anchor from "@coral-xyz/anchor";
import { InsightFaucet } from "../target/types/insight_faucet";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { airdropIfRequired } from "@solana-developers/helpers";
import { Program } from "@coral-xyz/anchor";
import {
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  getAssociatedTokenAddress,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import "dotenv/config";

describe("insight faucet", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.InsightFaucet as Program<InsightFaucet>;
  const connection = provider.connection;

  const master = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(process.env.PRIVATE_KEY))
  );
  let testToken: PublicKey;

  beforeAll(async () => {
    await airdropIfRequired(
      connection,
      master.publicKey,
      2 * LAMPORTS_PER_SOL,
      1 * LAMPORTS_PER_SOL
    );

    testToken = await createMint(
      connection,
      master,
      master.publicKey,
      null,
      6,
      Keypair.generate(),
      null,
      TOKEN_PROGRAM_ID
    );

    const signerUsdcAccount = await getAssociatedTokenAddress(
      testToken,
      master.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );

    try {
      await getAccount(connection, signerUsdcAccount, null, TOKEN_PROGRAM_ID);
    } catch {
      await createAssociatedTokenAccount(
        connection,
        master,
        testToken,
        master.publicKey,
        null,
        TOKEN_PROGRAM_ID
      );
    }

    await mintTo(
      connection,
      master,
      testToken,
      signerUsdcAccount,
      master,
      10_000_000,
      [],
      null,
      TOKEN_PROGRAM_ID
    );
  });

  it("initialize", async () => {
    let tx: string | null = null;
    try {
      tx = await program.methods
        .initialize()
        .accounts({
          token: testToken,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([master])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    expect(tx).toBeTruthy();
  });
});
