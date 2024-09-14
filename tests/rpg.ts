import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Rpg } from "../target/types/rpg";
import { assert } from "chai";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

const GAME_SEED = "GAME";
const PLAYER_SEED = "PLAYER";
const MONSTER_SEED = "MONSTER";
const MAX_ITEMS_PER_PLAYER = 8;
const AP_PER_PLAYER_CREATION = new BN(100);
const AP_PER_MONSTER_SPAWN = new BN(5);
const AP_PER_MONSTER_ATTACK = new BN(1);
const AIRDROP_AMOUNT = 10 * LAMPORTS_PER_SOL;
const CREATE_PLAYER_ACTION_POINTS = 100;
const SPAWN_MONSTER_ACTION_POINTS = 5;
const ATTACK_MONSTER_ACTION_POINTS = 1;

describe("RPG game", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Rpg as Program<Rpg>;
  const wallet = anchor.workspace.Rpg.provider.wallet
    .payer as anchor.web3.Keypair;
  const gameMaster = wallet;
  const player = wallet;

  const treasury = Keypair.generate();

  const findProgramAddress = (seeds: Buffer[]): [PublicKey, number] =>
    PublicKey.findProgramAddressSync(seeds, program.programId);

  const confirmTransaction = async (
    signature: string,
    provider: anchor.Provider
  ) => {
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...latestBlockhash,
    });
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
      monsterIndex.toArrayLike(Buffer, "le", 8),
    ]);

  const requestAirdrop = async (
    address: PublicKey,
    amount: number,
    provider: anchor.Provider
  ) => {
    const signature = await provider.connection.requestAirdrop(address, amount);
    await confirmTransaction(signature, program.provider);
  };

  it("creates a new game", async () => {
    try {
      const [gameAddress] = createGameAddress();

      const tx = await program.methods
        .createGame(
          MAX_ITEMS_PER_PLAYER,
          AP_PER_PLAYER_CREATION,
          AP_PER_MONSTER_SPAWN,
          AP_PER_MONSTER_ATTACK
        )
        .accounts({
          game: gameAddress,
          gameMaster: gameMaster.publicKey,
          treasury: treasury.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([treasury])
        .rpc();

      await confirmTransaction(tx, program.provider);
    } catch (error) {
      throw new Error(`Failed to create game: ${error.message}`);
    }
  });

  it("creates a new player", async () => {
    try {
      const [gameAddress] = createGameAddress();
      const [playerAddress] = createPlayerAddress(gameAddress);

      const tx = await program.methods
        .createPlayer()
        .accounts({
          game: gameAddress,
          playerAccount: playerAddress,
          player: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await confirmTransaction(tx, program.provider);
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

      const tx = await program.methods
        .spawnMonster()
        .accounts({
          game: gameAddress,
          playerAccount: playerAddress,
          monster: monsterAddress,
          player: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await confirmTransaction(tx, program.provider);
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

      const tx = await program.methods
        .attackMonster()
        .accounts({
          game: gameAddress,
          playerAccount: playerAddress,
          monster: monsterAddress,
          player: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await confirmTransaction(tx, program.provider);

      const monsterAccount = await program.account.monster.fetch(
        monsterAddress
      );
      assert(
        monsterAccount.hitpoints.eqn(99),
        "Monster hitpoints should be 99 after attack"
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
      const amountToInitialize = 10_000_000_000;

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
          playerWallet: player.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([player])
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
        treasuryBalance === amountToInitialize + expectedActionPoints,
        `Treasury balance (${treasuryBalance}) should match expected (${
          amountToInitialize + expectedActionPoints
        })`
      );

      const gameAccount = await program.account.game.fetch(gameAddress);
      assert(
        gameAccount.actionPointsCollected.eqn(expectedActionPoints),
        "Game action points collected should match expected"
      );

      const playerAccount = await program.account.player.fetch(playerAddress);
      assert(
        playerAccount.actionPointsSpent.eqn(0),
        "Player action points spent should be reset to 0"
      );
      assert(
        playerAccount.actionPointsToBeCollected.eqn(0),
        "Player should have no action points to be collected"
      );
    } catch (error) {
      console.error(`Test failed with error: ${error.message}`);
      throw error;
    }
  });
});
