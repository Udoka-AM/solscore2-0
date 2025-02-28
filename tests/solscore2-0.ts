import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Solscore } from '../target/types/solscore';
// import { expect } from 'chai';
import { PublicKey, Keypair, LAMPORTS_PER_SOL, Transaction } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';

describe('solscore', () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solscore as Program<Solscore>;
  
  // Create test keypairs
  const admin = Keypair.generate();
  const user = Keypair.generate();
  
  // Define PDAs
  let fplGlobalStatePDA: PublicKey;
  let fplUserPDA: PublicKey;
  let stakeConfigPDA: PublicKey;
  let stakeVaultPDA: PublicKey;
  let treasuryPDA: PublicKey;
  let rewardConfigPDA: PublicKey;
  let rewardPoolPDA: PublicKey;
  
  // User stake accounts
  let userStakeAccountPDA: PublicKey;
  
  // Reward token
  let rewardMint: PublicKey;
  let userRewardAccount: PublicKey;
  let rewardTokenAccount: PublicKey;
  
  // Lock period in seconds (1 week)
  const lockPeriod = new anchor.BN(86400 * 7);
  
  before(async () => {
    // Airdrop SOL to test accounts
    const adminAirdrop = await provider.connection.requestAirdrop(
      admin.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(adminAirdrop);
    
    const userAirdrop = await provider.connection.requestAirdrop(
      user.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(userAirdrop);
    
    // Find PDA addresses
    [fplGlobalStatePDA] = await PublicKey.findProgramAddress(
      [Buffer.from("fpl-global")],
      program.programId
    );
    
    [fplUserPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("fpl-user"), user.publicKey.toBuffer()],
      program.programId
    );
    
    [stakeConfigPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("stake-config")],
      program.programId
    );
    
    [stakeVaultPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("stake-vault")],
      program.programId
    );
    
    [treasuryPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("treasury")],
      program.programId
    );
    
    [rewardConfigPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("reward-config")],
      program.programId
    );
    
    [rewardPoolPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("reward-pool")],
      program.programId
    );
    
    [userStakeAccountPDA] = await PublicKey.findProgramAddress(
      [Buffer.from("user-stake"), user.publicKey.toBuffer()],
      program.programId
    );
    
    // Create reward token mint
    rewardMint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      9
    );
    
    // Create token accounts
    const userRewardTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      rewardMint,
      user.publicKey
    );
    userRewardAccount = userRewardTokenAccount.address;
    
    const rewardTokenPoolAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      rewardMint,
      rewardPoolPDA,
      true
    );
    rewardTokenAccount = rewardTokenPoolAccount.address;
    
    // Mint tokens to reward pool
    await mintTo(
      provider.connection,
      admin,
      rewardMint,
      rewardTokenAccount,
      admin.publicKey,
      1_000_000 * 10**9 // 1 million tokens
    );
  });

  // Initialize the FPL global state
  it('Initializes FPL Global State', async () => {
    const currentTimestamp = Math.floor(Date.now() / 1000);
    
    await program.methods
      .initializeFplGlobal({
        currentGameweek: 1,
        seasonStart: new anchor.BN(currentTimestamp),
        seasonEnd: new anchor.BN(currentTimestamp + 86400 * 300), // 300 days
        apiUrl: "https://fantasy.premierleague.com/api/",
      })
      .accounts({
        admin: admin.publicKey,
        globalState: fplGlobalStatePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();
      
    // Fetch and verify the account data
    const globalState = await program.account.fplGlobalState.fetch(fplGlobalStatePDA);
    expect(globalState.admin.toString()).to.equal(admin.publicKey.toString());
    expect(globalState.currentGameweek).to.equal(1);
  });

  // Initialize stake config
  it('Initializes Stake Config', async () => {
    await program.methods
      .initializeStakeConfig({
        minStakeAmount: new anchor.BN(0.1 * LAMPORTS_PER_SOL),
        maxStakeAmount: new anchor.BN(100 * LAMPORTS_PER_SOL),
        earlyWithdrawalFee: 10, // 10%
        lockOptions: [
          new anchor.BN(86400 * 7),  // 1 week
          new anchor.BN(86400 * 30), // 1 month
          new anchor.BN(86400 * 90), // 3 months
        ],
      })
      .accounts({
        admin: admin.publicKey,
        stakeConfig: stakeConfigPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();
      
    // Fetch and verify the account data
    const stakeConfig = await program.account.stakeConfig.fetch(stakeConfigPDA);
    expect(stakeConfig.admin.toString()).to.equal(admin.publicKey.toString());
    expect(stakeConfig.minStakeAmount.toNumber()).to.equal(0.1 * LAMPORTS_PER_SOL);
  });

  // Initialize reward config
  it('Initializes Reward Config', async () => {
    await program.methods
      .initializeRewardConfig({
        rewardToken: rewardMint,
        baseScoreMultiplier: new anchor.BN(100), // 1x multiplier = 100
        minScoreForReward: new anchor.BN(50),
        maxScoreForReward: new anchor.BN(100),
        minRewardAmount: new anchor.BN(10 * 10**9), // 10 tokens
        maxRewardAmount: new anchor.BN(100 * 10**9), // 100 tokens
      })
      .accounts({
        admin: admin.publicKey,
        rewardConfig: rewardConfigPDA,
        rewardMint: rewardMint,
        rewardPool: rewardPoolPDA,
        rewardTokenAccount: rewardTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();
      
    // Fetch and verify the account data
    const rewardConfig = await program.account.rewardConfig.fetch(rewardConfigPDA);
    expect(rewardConfig.admin.toString()).to.equal(admin.publicKey.toString());
    expect(rewardConfig.rewardToken.toString()).to.equal(rewardMint.toString());
  });

  // Register FPL user
  it('Registers an FPL User', async () => {
    const fplId = "12345678";
    
    await program.methods
      .registerFplUser(fplId)
      .accounts({
        user: user.publicKey,
        fplUser: fplUserPDA,
        globalState: fplGlobalStatePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      
    // Fetch and verify the account data
    const fplUser = await program.account.fplUser.fetch(fplUserPDA);
    expect(fplUser.authority.toString()).to.equal(user.publicKey.toString());
    expect(fplUser.fplId).to.equal(fplId);
  });

  // Stake SOL
  it('Stakes SOL', async () => {
    const stakeAmount = new anchor.BN(1 * LAMPORTS_PER_SOL); // 1 SOL
    
    const userBalanceBefore = await provider.connection.getBalance(user.publicKey);
    
    await program.methods
      .stakeTokens(stakeAmount, lockPeriod)
      .accounts({
        user: user.publicKey,
        userStakeAccount: userStakeAccountPDA,
        stakeConfig: stakeConfigPDA,
        stakeVault: stakeVaultPDA,
        fplUser: fplUserPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      
    // Fetch and verify the account data
    const userStakeAccount = await program.account.userStakeAccount.fetch(userStakeAccountPDA);
    expect(userStakeAccount.owner.toString()).to.equal(user.publicKey.toString());
    expect(userStakeAccount.amountStaked.toNumber()).to.equal(stakeAmount.toNumber());
    
    // Check user's SOL balance decreased
    const userBalanceAfter = await provider.connection.getBalance(user.publicKey);
    expect(userBalanceAfter).to.be.lessThan(userBalanceBefore - stakeAmount.toNumber());
    
    // Check stake vault balance increased
    const vaultBalance = await provider.connection.getBalance(stakeVaultPDA);
    expect(vaultBalance).to.equal(stakeAmount.toNumber());
  });

  // Update gameweek scores
  it('Updates Gameweek Scores', async () => {
    // Mock gameweek data
    const gameweek = 1;
    const userScores = [
      { 
        fplId: "12345678", 
        gameweekScore: 75,
        totalScore: 75,
        rank: 100000
      }
    ];
    
    await program.methods
      .updateGameweekScores(gameweek, userScores)
      .accounts({
        admin: admin.publicKey,
        globalState: fplGlobalStatePDA,
      })
      .signers([admin])
      .rpc();
      
    // Fetch and verify the user data has been updated
    const fplUser = await program.account.fplUser.fetch(fplUserPDA);
    expect(fplUser.lastUpdatedGameweek).to.equal(gameweek);
    expect(fplUser.currentScore).to.equal(75);
    expect(fplUser.totalScore).to.equal(75);
  });
  
  // Claim rewards
  it('Claims Rewards', async () => {
    await program.methods
      .claimRewards()
      .accounts({
        user: user.publicKey,
        fplUser: fplUserPDA,
        userRewardAccount: userRewardAccount,
        rewardConfig: rewardConfigPDA,
        rewardPool: rewardPoolPDA,
        rewardTokenAccount: rewardTokenAccount,
        globalState: fplGlobalStatePDA,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();
      
    // Check if rewards were transferred to user's token account
    const userTokenBalance = await provider.connection.getTokenAccountBalance(userRewardAccount);
    expect(parseInt(userTokenBalance.value.amount)).to.be.greaterThan(0);
    
    // Check that the user's claimable rewards were reset
    const fplUser = await program.account.fplUser.fetch(fplUserPDA);
    expect(fplUser.pendingRewards.toNumber()).to.equal(0);
    expect(fplUser.lastRewardClaim.toNumber()).to.be.greaterThan(0);
  });
  
  // Try to unstake early (should apply early withdrawal fee)
  it('Applies Early Withdrawal Fee', async () => {
    const vaultBalanceBefore = await provider.connection.getBalance(stakeVaultPDA);
    const treasuryBalanceBefore = await provider.connection.getBalance(treasuryPDA);
    const userBalanceBefore = await provider.connection.getBalance(user.publicKey);
    
    await program.methods
      .unstakeTokens()
      .accounts({
        user: user.publicKey,
        userStakeAccount: userStakeAccountPDA,
        stakeConfig: stakeConfigPDA,
        stakeVault: stakeVaultPDA,
        treasury: treasuryPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      
    // Check that stake account was closed
    try {
      await program.account.userStakeAccount.fetch(userStakeAccountPDA);
      expect.fail("Stake account should be closed");
    } catch (error) {
      // Expected error - account not found
    }
    
    // Check that early withdrawal fee was applied
    const vaultBalanceAfter = await provider.connection.getBalance(stakeVaultPDA);
    expect(vaultBalanceAfter).to.equal(0); // All funds should be withdrawn
    
    const treasuryBalanceAfter = await provider.connection.getBalance(treasuryPDA);
    const fee = vaultBalanceBefore * 0.1; // 10% fee
    expect(treasuryBalanceAfter - treasuryBalanceBefore).to.be.approximately(fee, 100);
    
    const userBalanceAfter = await provider.connection.getBalance(user.publicKey);
    expect(userBalanceAfter - userBalanceBefore).to.be.approximately(vaultBalanceBefore - fee, 10000);
  });
  
  // Test staking again with full lock period expiration
  it('Stakes SOL for Full Lock Period', async () => {
    const stakeAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL); // 0.5 SOL
    
    // Create new stake account
    await program.methods
      .stakeTokens(stakeAmount, lockPeriod)
      .accounts({
        user: user.publicKey,
        userStakeAccount: userStakeAccountPDA,
        stakeConfig: stakeConfigPDA,
        stakeVault: stakeVaultPDA,
        fplUser: fplUserPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      
    // Get stake account
    const userStakeAccount = await program.account.userStakeAccount.fetch(userStakeAccountPDA);
    
    // Simulate time passing - modify the lock end timestamp
    // This would normally be done by waiting for the lock period to expire
    await program.methods
      .updateLockTimestampForTesting(new anchor.BN(Math.floor(Date.now() / 1000) - 1))
      .accounts({
        admin: admin.publicKey,
        userStakeAccount: userStakeAccountPDA,
      })
      .signers([admin])
      .rpc();
  });
  
  // Unstake without early withdrawal fee after lock period expires
  it('Unstakes SOL After Lock Period', async () => {
    const vaultBalanceBefore = await provider.connection.getBalance(stakeVaultPDA);
    const userBalanceBefore = await provider.connection.getBalance(user.publicKey);
    
    await program.methods
      .unstakeTokens()
      .accounts({
        user: user.publicKey,
        userStakeAccount: userStakeAccountPDA,
        stakeConfig: stakeConfigPDA,
        stakeVault: stakeVaultPDA,
        treasury: treasuryPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      
    // Check that stake account was closed
    try {
      await program.account.userStakeAccount.fetch(userStakeAccountPDA);
      expect.fail("Stake account should be closed");
    } catch (error) {
      // Expected error - account not found
    }
    
    // Check that no fee was applied (user gets full amount back)
    const vaultBalanceAfter = await provider.connection.getBalance(stakeVaultPDA);
    expect(vaultBalanceAfter).to.equal(0);
    
    const userBalanceAfter = await provider.connection.getBalance(user.publicKey);
    expect(userBalanceAfter - userBalanceBefore).to.be.approximately(vaultBalanceBefore, 10000);
  });
  
  // Update gameweek
  it('Updates Current Gameweek', async () => {
    const newGameweek = 2;
    
    await program.methods
      .updateCurrentGameweek(newGameweek)
      .accounts({
        admin: admin.publicKey,
        globalState: fplGlobalStatePDA,
      })
      .signers([admin])
      .rpc();
      
    // Verify gameweek was updated
    const globalState = await program.account.fplGlobalState.fetch(fplGlobalStatePDA);
    expect(globalState.currentGameweek).to.equal(newGameweek);
  });
  
  // Test emergency withdraw
  it('Performs Emergency Withdraw', async () => {
    // First, stake some tokens again
    const stakeAmount = new anchor.BN(0.3 * LAMPORTS_PER_SOL);
    
    await program.methods
      .stakeTokens(stakeAmount, lockPeriod)
      .accounts({
        user: user.publicKey,
        userStakeAccount: userStakeAccountPDA,
        stakeConfig: stakeConfigPDA,
        stakeVault: stakeVaultPDA,
        fplUser: fplUserPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    
    // Emergency withdraw by admin
    const adminBalanceBefore = await provider.connection.getBalance(admin.publicKey);
    
    await program.methods
      .emergencyWithdraw()
      .accounts({
        admin: admin.publicKey,
        stakeConfig: stakeConfigPDA,
        stakeVault: stakeVaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();
      
    // Verify vault is empty
    const vaultBalanceAfter = await provider.connection.getBalance(stakeVaultPDA);
    expect(vaultBalanceAfter).to.equal(0);
    
    // Verify admin received the funds
    const adminBalanceAfter = await provider.connection.getBalance(admin.publicKey);
    expect(adminBalanceAfter).to.be.greaterThan(adminBalanceBefore);
  });
});
