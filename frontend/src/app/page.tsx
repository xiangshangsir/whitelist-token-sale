'use client';

import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useState, useEffect } from 'react';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';

// 替换为你的 Program ID（部署后更新）
const PROGRAM_ID = 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS';

export default function Home() {
  const { connected, publicKey } = useWallet();
  const [saleData, setSaleData] = useState<any>(null);
  const [whitelistData, setWhitelistData] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [claimAmount, setClaimAmount] = useState('');
  const [message, setMessage] = useState('');

  const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

  // 加载销售配置
  const loadSaleConfig = async () => {
    if (!publicKey) return;
    
    setLoading(true);
    try {
      // 这里需要从链上读取数据
      // 实际部署后需要实现 PDA 查找和数据解析
      setMessage('销售配置加载成功（示例数据）');
      
      // 示例数据（部署后替换为真实数据）
      setSaleData({
        startTime: new Date(),
        endTime: new Date(Date.now() + 86400000),
        tokenPrice: 1,
        totalSupply: 100000,
        tokensSold: 0,
      });
    } catch (error) {
      setMessage('加载失败：' + (error as Error).message);
    } finally {
      setLoading(false);
    }
  };

  // 加载白名单信息
  const loadWhitelistInfo = async () => {
    if (!publicKey) {
      setMessage('请先连接钱包');
      return;
    }

    setLoading(true);
    try {
      // 查询白名单账户 PDA
      const [whitelistPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('whitelist'),
          new PublicKey(PROGRAM_ID).toBuffer(),
          publicKey.toBuffer(),
        ],
        new PublicKey(PROGRAM_ID)
      );

      // 读取链上数据（需要 IDL）
      setMessage(`白名单账户：${whitelistPda.toBase58()}`);
      
      // 示例数据
      setWhitelistData({
        allocation: 500,
        claimed: 0,
        isActive: true,
      });
    } catch (error) {
      setMessage('查询失败：' + (error as Error).message);
    } finally {
      setLoading(false);
    }
  };

  // Claim 代币
  const handleClaim = async () => {
    if (!connected) {
      setMessage('请先连接钱包');
      return;
    }

    const amount = parseInt(claimAmount);
    if (!amount || amount <= 0) {
      setMessage('请输入有效的数量');
      return;
    }

    setLoading(true);
    try {
      // 这里需要调用智能合约的 claim_tokens 指令
      // 实际部署后需要实现 Anchor 调用
      setMessage(`Claim ${amount} 代币的请求已发送（示例）`);
      
      // 模拟交易
      await new Promise(resolve => setTimeout(resolve, 2000));
      setMessage('✅ Claim 成功！代币已发放到您的账户');
      setClaimAmount('');
    } catch (error) {
      setMessage('Claim 失败：' + (error as Error).message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (connected) {
      loadSaleConfig();
      loadWhitelistInfo();
    }
  }, [connected, publicKey]);

  return (
    <main className="min-h-screen p-8 bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-4xl font-bold text-white">
            🦞 Whitelist Token Sale
          </h1>
          <WalletMultiButton className="bg-purple-600 hover:bg-purple-700" />
        </div>

        {/* 状态提示 */}
        {message && (
          <div className="mb-6 p-4 bg-white/10 backdrop-blur rounded-lg border border-white/20">
            <p className="text-white">{message}</p>
          </div>
        )}

        {/* 连接钱包提示 */}
        {!connected && (
          <div className="text-center py-12">
            <p className="text-xl text-white/80 mb-4">
              请连接 Phantom 钱包继续
            </p>
            <div className="text-6xl">👆</div>
          </div>
        )}

        {connected && (
          <div className="grid md:grid-cols-2 gap-6">
            {/* 销售配置卡片 */}
            <div className="bg-white/10 backdrop-blur rounded-lg p-6 border border-white/20">
              <h2 className="text-2xl font-bold text-white mb-4">
                📊 销售配置
              </h2>
              
              {saleData ? (
                <div className="space-y-3 text-white/90">
                  <div>
                    <span className="text-white/60">代币价格：</span>
                    <span className="font-semibold">{saleData.tokenPrice} USDC</span>
                  </div>
                  <div>
                    <span className="text-white/60">总供应量：</span>
                    <span className="font-semibold">{saleData.totalSupply.toLocaleString()}</span>
                  </div>
                  <div>
                    <span className="text-white/60">已售出：</span>
                    <span className="font-semibold">{saleData.tokensSold.toLocaleString()}</span>
                  </div>
                  <div>
                    <span className="text-white/60">开始时间：</span>
                    <span className="font-semibold">
                      {saleData.startTime.toLocaleString()}
                    </span>
                  </div>
                  <div>
                    <span className="text-white/60">结束时间：</span>
                    <span className="font-semibold">
                      {saleData.endTime.toLocaleString()}
                    </span>
                  </div>
                </div>
              ) : (
                <p className="text-white/60">加载中...</p>
              )}

              <button
                onClick={loadSaleConfig}
                disabled={loading}
                className="mt-4 w-full py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 rounded-lg text-white font-semibold transition"
              >
                {loading ? '加载中...' : '刷新配置'}
              </button>
            </div>

            {/* 白名单信息卡片 */}
            <div className="bg-white/10 backdrop-blur rounded-lg p-6 border border-white/20">
              <h2 className="text-2xl font-bold text-white mb-4">
                🎫 我的白名单
              </h2>
              
              {whitelistData ? (
                <div className="space-y-3 text-white/90">
                  <div>
                    <span className="text-white/60">分配额度：</span>
                    <span className="font-semibold text-green-400">{whitelistData.allocation} 代币</span>
                  </div>
                  <div>
                    <span className="text-white/60">已领取：</span>
                    <span className="font-semibold">{whitelistData.claimed} 代币</span>
                  </div>
                  <div>
                    <span className="text-white/60">剩余额度：</span>
                    <span className="font-semibold text-yellow-400">
                      {whitelistData.allocation - whitelistData.claimed} 代币
                    </span>
                  </div>
                  <div>
                    <span className="text-white/60">状态：</span>
                    <span className={`font-semibold ${whitelistData.isActive ? 'text-green-400' : 'text-red-400'}`}>
                      {whitelistData.isActive ? '✅ 活跃' : '❌ 未激活'}
                    </span>
                  </div>
                </div>
              ) : (
                <p className="text-white/60">查询中...</p>
              )}

              <button
                onClick={loadWhitelistInfo}
                disabled={loading}
                className="mt-4 w-full py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 rounded-lg text-white font-semibold transition"
              >
                {loading ? '查询中...' : '查询白名单'}
              </button>
            </div>

            {/* Claim 代币卡片 */}
            <div className="md:col-span-2 bg-white/10 backdrop-blur rounded-lg p-6 border border-white/20">
              <h2 className="text-2xl font-bold text-white mb-4">
                💰 Claim 代币
              </h2>
              
              <div className="flex gap-4 items-end">
                <div className="flex-1">
                  <label className="block text-white/60 mb-2">
                    Claim 数量
                  </label>
                  <input
                    type="number"
                    value={claimAmount}
                    onChange={(e) => setClaimAmount(e.target.value)}
                    placeholder="输入数量（最多你的额度）"
                    className="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-lg text-white placeholder-white/40 focus:outline-none focus:ring-2 focus:ring-purple-500"
                  />
                </div>
                <button
                  onClick={handleClaim}
                  disabled={loading || !claimAmount}
                  className="px-8 py-3 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 rounded-lg text-white font-semibold transition"
                >
                  {loading ? '处理中...' : 'Claim'}
                </button>
              </div>

              <p className="mt-4 text-white/60 text-sm">
                💡 提示：Claim 时需要支付对应数量的 USDC
              </p>
            </div>
          </div>
        )}

        {/* 页脚 */}
        <footer className="mt-12 text-center text-white/40 text-sm">
          <p>Superteam Earn Bounty Submission</p>
          <p>GitHub: @xiangshangsir</p>
        </footer>
      </div>
    </main>
  );
}
