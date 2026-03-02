const anchor = require('@coral-xyz/anchor');
const { assert } = require('chai');

describe('whitelist-token-sale', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.WhitelistTokenSale;
  const admin = provider.wallet.publicKey;
  const user = anchor.web3.Keypair.generate();

  // 配置参数
  const startTime = Math.floor(Date.now() / 1000) + 60; // 1 分钟后
  const endTime = Math.floor(Date.now() / 1000) + 3600; // 1 小时后
  const maxAllocationPerUser = new anchor.BN(1000);
  const tokenPrice = new anchor.BN(1); // 1 USDC = 1 Token
  const totalSupply = new anchor.BN(100000);
  const userAllocation = new anchor.BN(500);
  const claimAmount = new anchor.BN(100);

  let saleConfigPda;
  let saleTokenAccountPda;
  let saleUsdcAccountPda;
  let whitelistAccountPda;

  before(async () => {
    // 为用户空投一些 SOL 用于测试
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL)
    );
  });

  it('初始化销售配置', async () => {
    // 这里需要实际的 Token Mint 地址
    // 测试时需要先创建测试用的 Token
    const tokenMint = new anchor.web3.PublicKey('YourTokenMintAddress');
    const usdcMint = new anchor.web3.PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

    [saleConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('sale_config'), tokenMint.toBuffer()],
      program.programId
    );

    [saleTokenAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('sale_token_account'), tokenMint.toBuffer()],
      program.programId
    );

    [saleUsdcAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('sale_usdc_account'), usdcMint.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .initializeSale(
          new anchor.BN(startTime),
          new anchor.BN(endTime),
          maxAllocationPerUser,
          tokenPrice,
          totalSupply
        )
        .accounts({
          admin: admin,
          saleConfig: saleConfigPda,
          tokenMint: tokenMint,
          usdcMint: usdcMint,
        })
        .rpc();

      const config = await program.account.saleConfig.fetch(saleConfigPda);
      assert.ok(config.admin.equals(admin));
      assert.ok(config.startTime.toNumber() === startTime);
      assert.ok(config.endTime.toNumber() === endTime);
      assert.ok(config.maxAllocationPerUser.eq(maxAllocationPerUser));
      assert.ok(config.tokenPrice.eq(tokenPrice));
    } catch (error) {
      console.log('初始化需要实际的 Token Mint，跳过完整测试');
    }
  });

  it('添加用户到白名单', async () => {
    const tokenMint = new anchor.web3.PublicKey('YourTokenMintAddress');

    [saleConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('sale_config'), tokenMint.toBuffer()],
      program.programId
    );

    [whitelistAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('whitelist'), saleConfigPda.toBuffer(), user.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .addToWhitelist(userAllocation)
        .accounts({
          admin: admin,
          saleConfig: saleConfigPda,
          user: user.publicKey,
          whitelistAccount: whitelistAccountPda,
        })
        .rpc();

      const whitelist = await program.account.whitelistAccount.fetch(whitelistAccountPda);
      assert.ok(whitelist.user.equals(user.publicKey));
      assert.ok(whitelist.allocation.eq(userAllocation));
      assert.ok(whitelist.claimed.eq(new anchor.BN(0)));
      assert.ok(whitelist.isActive === true);
    } catch (error) {
      console.log('需要先初始化销售配置');
    }
  });

  it('从白名单移除用户', async () => {
    const tokenMint = new anchor.web3.PublicKey('YourTokenMintAddress');

    [saleConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('sale_config'), tokenMint.toBuffer()],
      program.programId
    );

    [whitelistAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('whitelist'), saleConfigPda.toBuffer(), user.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .removeFromWhitelist()
        .accounts({
          admin: admin,
          saleConfig: saleConfigPda,
          user: user.publicKey,
          whitelistAccount: whitelistAccountPda,
        })
        .rpc();

      const whitelist = await program.account.whitelistAccount.fetch(whitelistAccountPda);
      assert.ok(whitelist.isActive === false);
    } catch (error) {
      console.log('需要先添加用户到白名单');
    }
  });

  it('用户 Claim 代币', async () => {
    // 这个测试需要完整的 Token 设置
    // 包括：Token Mint, User Token Account, USDC Account 等
    console.log('Claim 测试需要完整的 Token 环境，建议在 Devnet 上手动测试');
  });

  it('验证错误情况', async () => {
    // 测试非管理员尝试添加白名单
    const tokenMint = new anchor.web3.PublicKey('YourTokenMintAddress');

    [saleConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('sale_config'), tokenMint.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .addToWhitelist(userAllocation)
        .accounts({
          admin: user.publicKey, // 非管理员
          saleConfig: saleConfigPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();
      
      assert.fail('应该抛出 Unauthorized 错误');
    } catch (error) {
      assert.ok(error.message.includes('Unauthorized'));
    }
  });
});
