# 🚀 部署和使用指南

## 前置准备

### 1. 安装依赖
```bash
# 安装 Node.js 依赖
npm install

# 安装 Rust 依赖（如果还没装）
cargo build-bpf
```

### 2. 配置 Solana CLI
```bash
# 配置到 Devnet
solana config set --url devnet

# 生成或导入钱包（如果还没做）
solana-keygen new
# 或
solana-keygen import

# 获取 Devnet SOL（测试用）
solana airdrop 2
```

### 3. 安装 Anchor
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install latest
avm use latest
```

---

## 📦 构建项目

```bash
cd whitelist-token-sale
anchor build
```

构建成功后会在 `target/deploy/` 生成：
- `whitelist_token_sale.so` (程序二进制)
- `whitelist_token_sale-keypair.json` (程序密钥)

---

## 🧪 本地测试

```bash
# 启动本地验证器（新终端）
solana-test-validator

# 运行测试（原终端）
anchor test
```

---

## 🌐 部署到 Devnet

### 1. 部署程序
```bash
anchor deploy --provider.cluster devnet
```

部署成功后记录 **Program ID**（重要！）

### 2. 创建测试用 Token（可选）
```bash
# 创建自己的 Token 用于测试
spl-token create-token

# 创建 Token Account
spl-token create-account <TOKEN_MINT_ADDRESS>

# 铸造代币
spl-token mint <TOKEN_MINT_ADDRESS> 1000000
```

### 3. 初始化销售
```bash
# 使用 anchor 调用指令
anchor run initialize-sale \
  --start-time <UNIX_TIMESTAMP> \
  --end-time <UNIX_TIMESTAMP> \
  --max-allocation 1000 \
  --token-price 1 \
  --total-supply 100000
```

或者用 Solana CLI:
```bash
solana program deploy target/deploy/whitelist_token_sale.so
```

---

## 🔧 调用合约

### 添加用户到白名单
```bash
anchor run add-to-whitelist \
  --user <USER_PUBKEY> \
  --allocation 500
```

### 用户 Claim 代币
```bash
anchor run claim-tokens \
  --amount 100
```

### 提取未售出代币
```bash
anchor run withdraw-unsold
```

---

## 📊 验证部署

### 查看程序信息
```bash
solana program show <PROGRAM_ID>
```

### 查看账户数据
```bash
solana account <ACCOUNT_PUBKEY>
```

### 在 Solscan 上查看
```
https://solscan.io/account/<ACCOUNT_PUBKEY>?cluster=devnet
```

---

## 🎯 Superteam Earn 提交流程

### 1. 准备提交材料
- ✅ GitHub 仓库链接（公开）
- ✅ Devnet 部署地址
- ✅ 测试视频/GIF（可选但推荐）
- ✅ 实现说明文档

### 2. 在 Superteam Earn 页面
1. 访问：https://superteam.fun/earn
2. 找到 "Whitelist-gated Token Sale" 任务
3. 点击 "Submit Work"
4. 填写：
   - GitHub 仓库：你的 repo 链接
   - Devnet Demo：可交互的测试地址
   - 说明文档：README.md
5. 提交！

### 3. 等待评审
- 评审周期：通常 1-2 周
- 结果通知：Email + Twitter DM
- 奖金发放：USDC 直接打到你的钱包

---

## 💡 加分项（提高获胜几率）

### 前端 Demo（强烈推荐）
创建一个简单的前端页面：
```bash
# 使用 Next.js + Anchor
npx create-next-app frontend
cd frontend
npm install @coral-xyz/anchor @solana/wallet-adapter-react
```

功能包括：
- 连接 Phantom 钱包
- 查看销售配置
- 管理员添加白名单
- 用户 Claim 代币
- 实时显示进度

### 完整测试
```bash
# 增加测试覆盖率
anchor test --coverage

# 目标：80%+ 覆盖率
```

### 文档完善
- 详细的 README
- API 文档
- 部署视频教程

---

## 🔐 安全检查清单

部署前确认：
- [ ] 所有算术运算使用 checked/overflow
- [ ] 权限验证正确（admin only 功能）
- [ ] 时间窗口验证逻辑正确
- [ ] PDA 种子正确（防止重放攻击）
- [ ] 事件发射完整（便于索引）
- [ ] 测试覆盖所有错误情况

---

## 📞 需要帮助？

- Anchor 文档：https://www.anchor-lang.com
- Solana Cookbook：https://solanacookbook.com
- Solana Discord：https://discord.com/invite/solana
- Superteam Discord：https://discord.gg/superteam

---

## 🎉 获胜后

1. **接收 USDC**：奖金会打到你的 Phantom 钱包
2. **提现**：转到交易所（币安/OKX/Coinbase）
3. **继续下一个 Bounty**：重复流程赚更多！

Good Luck! 🦞💰
