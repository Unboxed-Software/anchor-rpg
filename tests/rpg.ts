import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Rpg } from "../target/types/rpg";
import { assert } from "chai";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  TransactionSignature,
  TransactionConfirmationStrategy,
} from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

const GAME_SEED = "GAME";
const PLAYER_SEED = "PLAYER";
const MONSTER_SEED = "MONSTER";
const MAX_ITEMS_PER_PLAYER = 8;
const INITIAL_MONSTER_HITPOINTS = 100;
const AIRDROP_AMOUNT = 10 * LAMPORTS_PER_SOL;
const CREATE_PLAYER_ACTION_POINTS = 100;
const SPAWN_MONSTER_ACTION_POINTS = 5;
const ATTACK_MONSTER_ACTION_POINTS = 1;
const MONSTER_INDEX_BYTE_LENGTH = 8;

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Rpg as Program<Rpg>;
const wallet = provider.wallet as NodeWallet;
const gameMaster = wallet;
const player = wallet;

const treasury = Keypair.generate();

const findProgramAddress = (seeds: Buffer[]): [PublicKey, number] =>
  PublicKey.findProgramAddressSync(seeds, program.programId);

const confirmTransaction = async (
  signature: TransactionSignature,
  provider: anchor.Provider
) => {
  const latestBlockhash = await provider.connection.getLatestBlockhash();
  const confirmationStrategy: TransactionConfirmationStrategy = {
    signature,
    blockhash: latestBlockhash.blockhash,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  };

  try {
    const confirmation = await provider.connection.confirmTransaction(
      confirmationStrategy
    );
    if (confirmation.value.err) {
      throw new Error(
        `Transaction failed: ${confirmation.value.err.toString()}`
      );
    }
  } catch (error) {
    throw new Error(`Transaction confirmation failed: ${error.message}`);
  }
};

const createGameAddress = () =>
  findProgramAddress([Buffer.from(GAME_SEED), treasury.publicKey.toBuffer()]);

const createPlayerAddress = (gameAddress: PublicKey) =>
  findProgramAddress([
    Buffer.from(PLAYER_SEED),
    gameAddress.toBuffer(),
    player.publicKey.toBuffer(),
  ]);

const createMonsterAddress = (
  gameAddress: PublicKey,
  monsterIndex: anchor.BN
) =>
  findProgramAddress([
    Buffer.from(MONSTER_SEED),
    gameAddress.toBuffer(),
    player.publicKey.toBuffer(),
    monsterIndex.toArrayLike(Buffer, "le", MONSTER_INDEX_BYTE_LENGTH),
  ]);

describe("RPG game", () => {
  it("creates a new game", async () => {
    try {
      const [gameAddress] = createGameAddress();

      const createGameSignature = await program.methods
        .createGame(MAX_ITEMS_PER_PLAYER)
        .accounts({
          game: gameAddress,
          gameMaster: gameMaster.publicKey,
          treasury: treasury.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([treasury])
        .rpc();

      await confirmTransaction(createGameSignature, provider);
    } catch (error) {
      throw new Error(`Failed to create game: ${error.message}`);
    }
  });

  it("creates a new player", async () => {
    try {
      const [gameAddress] = createGameAddress();
      const [playerAddress] = createPlayerAddress(gameAddress);

      const createPlayerSignature = await program.methods
        .createPlayer()
        .accounts({
          game: gameAddress,
          playerAccount: playerAddress,
          player: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await confirmTransaction(createPlayerSignature, provider);
    } catch (error) {
      throw new Error(`Failed to create player: ${error.message}`);
    }
  });

  it("spawns a monster", async () => {
    try {
      const [gameAddress] = createGameAddress();
      const [playerAddress] = createPlayerAddress(gameAddress);

      const playerAccount = await program.account.player.fetch(playerAddress);
      const [monsterAddress] = createMonsterAddress(
        gameAddress,
        playerAccount.nextMonsterIndex
      );

      const spawnMonsterSignature = await program.methods
        .spawnMonster()
        .accounts({
          game: gameAddress,
          playerAccount: playerAddress,
          monster: monsterAddress,
          player: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await confirmTransaction(spawnMonsterSignature, provider);
    } catch (error) {
      throw new Error(`Failed to spawn monster: ${error.message}`);
    }
  });

  it("attacks a monster", async () => {
    try {
      const [gameAddress] = createGameAddress();
      const [playerAddress] = createPlayerAddress(gameAddress);

      const playerAccount = await program.account.player.fetch(playerAddress);
      const [monsterAddress] = createMonsterAddress(
        gameAddress,
        playerAccount.nextMonsterIndex.subn(1)
      );

      const attackMonsterSignature = await program.methods
        .attackMonster()
        .accounts({
          playerAccount: playerAddress,
          monster: monsterAddress,
          player: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await confirmTransaction(attackMonsterSignature, provider);

      const monsterAccount = await program.account.monster.fetch(
        monsterAddress
      );
      assert(
        monsterAccount.hitpoints.eqn(INITIAL_MONSTER_HITPOINTS - 1),
        "Monster hitpoints should decrease by 1 after attack"
      );
    } catch (error) {
      throw new Error(`Failed to attack monster: ${error.message}`);
    }
  });

  it("deposits action points", async () => {
    try {
      const [gameAddress] = createGameAddress();
      const [playerAddress] = createPlayerAddress(gameAddress);

      // To show that anyone can deposit the action points
      // Ie, give this to a clockwork bot
      const clockworkWallet = anchor.web3.Keypair.generate();

      // To give it a starting balance
      const clockworkProvider = new anchor.AnchorProvider(
        program.provider.connection,
        new NodeWallet(clockworkWallet),
        anchor.AnchorProvider.defaultOptions()
      );

      // Have to give the accounts some lamports else the tx will fail
      const amountToInitialize = 10000000000;

      const clockworkAirdropTx =
        await clockworkProvider.connection.requestAirdrop(
          clockworkWallet.publicKey,
          amountToInitialize
        );

      await confirmTransaction(clockworkAirdropTx, clockworkProvider);

      const treasuryAirdropTx =
        await clockworkProvider.connection.requestAirdrop(
          treasury.publicKey,
          amountToInitialize
        );

      await confirmTransaction(treasuryAirdropTx, clockworkProvider);

      const depositActionPointsSignature = await program.methods
        .depositActionPoints()
        .accounts({
          game: gameAddress,
          player: playerAddress,
          treasury: treasury.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await confirmTransaction(depositActionPointsSignature, provider);

      const expectedActionPoints =
        CREATE_PLAYER_ACTION_POINTS +
        SPAWN_MONSTER_ACTION_POINTS +
        ATTACK_MONSTER_ACTION_POINTS;
      const treasuryBalance = await provider.connection.getBalance(
        treasury.publicKey
      );
      assert(
        treasuryBalance === AIRDROP_AMOUNT + expectedActionPoints,
        "Treasury balance should match expected action points"
      );

      const gameAccount = await program.account.game.fetch(gameAddress);
      assert(
        gameAccount.actionPointsCollected.eqn(expectedActionPoints),
        "Game action points collected should match expected"
      );

      const playerAccount = await program.account.player.fetch(playerAddress);
      assert(
        playerAccount.actionPointsSpent.eqn(expectedActionPoints),
        "Player action points spent should match expected"
      );
      assert(
        playerAccount.actionPointsToBeCollected.eqn(0),
        "Player should have no action points to be collected"
      );
    } catch (error) {
      throw new Error(`Failed to deposit action points: ${error.message}`);
    }
  });
});
