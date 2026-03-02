# Whitelist Token Sale - Frontend Demo

这是一个简单的前端界面，用于演示 Whitelist-gated Token Sale 智能合约的功能。

## 🚀 快速开始

### 1. 安装依赖
```bash
cd frontend
npm install
```

### 2. 运行开发服务器
```bash
npm run dev
```

访问 http://localhost:3000

### 3. 连接钱包
- 点击 "Select Wallet"
- 选择 Phantom
- 授权连接

## 📋 功能

- ✅ 连接 Phantom 钱包
- ✅ 查看销售配置
- ✅ 查询白名单状态
- ✅ Claim 代币
- ✅ 实时状态更新

## 🔧 配置

在 `src/app/page.tsx` 中修改 Program ID：

```typescript
const PROGRAM_ID = '你的 Program ID（部署后更新）';
```

## 🎨 UI 特点

- 渐变背景
- 毛玻璃效果
- 响应式设计
- 暗色主题
- 钱包适配器

## 📱 页面结构

```
Header
├── 标题
└── 钱包连接按钮

主内容区（连接钱包后显示）
├── 销售配置卡片
│   ├── 代币价格
│   ├── 总供应量
│   ├── 已售出
│   └── 时间信息
├── 我的白名单卡片
│   ├── 分配额度
│   ├── 已领取
│   ├── 剩余额度
│   └── 状态
└── Claim 代币卡片
    ├── 输入数量
    └── Claim 按钮

Footer
└── GitHub 链接
```

## 🎯 部署到 Vercel（可选）

```bash
# 安装 Vercel CLI
npm i -g vercel

# 部署
vercel
```

## 💡 提交 Superteam 时

1. 本地运行展示录屏
2. 或部署到 Vercel 提供在线 Demo
3. 在提交页面附上链接

---

**Made for Superteam Earn Bounty** 🦞
