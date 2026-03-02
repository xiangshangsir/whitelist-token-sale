# Whitelist-gated Token Sale 💰

**Superteam Earn Bounty Submission**

奖励：**1,000 USDC**

---

## 📋 功能概述

实现一个白名单控制的代币销售智能合约：

- ✅ 白名单管理（添加/删除地址）
- ✅ 代币销售（限时/限额）
- ✅ 代币 Claim（白名单用户领取）
- ✅ 管理员权限控制
- ✅ 事件发射（前端监听）
- ✅ 完整测试用例

---

## 🚀 快速开始

### 环境要求
```bash
# Rust
rustc --version  # 1.70+

# Anchor
avm --version  # 0.29+

# Solana CLI
solana --version  # 1.16+
```

### 安装依赖
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install latest
```

### 本地开发
```bash
# 启动本地验证器
solana-test-validator

# 运行测试
anchor test

# 部署到 Devnet
anchor deploy --provider.cluster devnet
```

---

## 📦 合约结构

```
programs/
└── whitelist-token-sale/
    └── src/
        └── lib.rs          # 主合约逻辑
accounts/
├── whitelist_account.rs    # 白名单账户
├── sale_config.rs          # 销售配置
└── user_allocation.rs      # 用户分配记录
```

---

## 🔧 核心功能

### 1. 管理员初始化销售
```rust
initialize_sale(
    start_time: i64,
    end_time: i64,
    max_allocation_per_user: u64,
    token_price: u64,
)
```

### 2. 管理白名单
```rust
add_to_whitelist(user: Pubkey, allocation: u64)
remove_from_whitelist(user: Pubkey)
```

### 3. 用户 Claim 代币
```rust
claim_tokens(amount: u64)
```

### 4. 管理员提取未售出代币
```rust
withdraw_unsold_tokens()
```

---

## 📊 账户模型

| 账户 | 用途 |
|------|------|
| `SaleConfig` | 存储销售配置（时间、价格、限额） |
| `WhitelistAccount` | 存储白名单用户及分配额度 |
| `UserAllocation` | 记录用户已 Claim 数量 |

---

## ✅ 测试覆盖

- [x] 初始化销售配置
- [x] 添加/删除白名单
- [x] 白名单用户 Claim
- [x] 非白名单用户 Claim 失败
- [x] 超额 Claim 失败
- [x] 时间窗口验证
- [x] 管理员权限验证

---

## 📝 部署说明

### Devnet 部署
```bash
anchor deploy --provider.cluster devnet
```

### 验证部署
```bash
solana program show <PROGRAM_ID>
```

---

## 🔐 安全考虑

1. **权限控制**：只有管理员可以修改配置
2. **重入保护**：使用 Anchor 的账户验证
3. **溢出检查**：所有算术运算使用 Checked/Overflow
4. **时间验证**：Claim 前检查销售时间窗口
5. **额度限制**：用户 Claim 不超过分配额度

---

## 📞 联系方式

- GitHub: <your-github>
- Twitter: <your-twitter>
- Email: <your-email>

---

## 📄 许可证

MIT
