# opentrade-newsliquid 技能设计计划书

## 1. 概述

`opentrade-newsliquid` 是 opentrade 技能族中首个面向 **CEX（中心化交易所）** 的技能模块。它通过 Newsliquid 业务网关，为 AI 编程助手提供 CEX 合约/现货交易能力，包括：市场数据查询、账户管理、订单管理、持仓管理、杠杆配置、交易历史、钱包代理等。

与同目录下的 `opentrade-dex-swap`、`opentrade-portfolio` 等 DEX 技能不同，本技能操作的是 CEX 托管账户，交易由服务端执行，无需用户签名或广播交易。同时，所有写操作（下单、平仓、修改杠杆等）均受内置风控引擎保护。

### 核心特点

- **CEX 交易**：现货 + 合约，支持限价/市价/止盈止损等多种订单类型
- **内置风控**：价格偏离检测、仓位限制、频率限制、余额校验四重保护
- **零签名**：所有交易由服务端执行，用户无需管理私钥或签名交易
- **与 DEX 互补**：可结合 DEX 技能实现 CEX↔DEX 套利、对冲等高级策略

---

## 2. API 接口映射

所有接口基于 `https://ai.6551.io` 基础 URL，使用 Bearer Token 鉴权。

路由前缀：`/open/trader/newsliquid/v1/`

### 2.1 市场数据 (Market) — 5 个接口，无需风控

| # | 方法 | 路径 | 说明 |
|---|------|------|------|
| 1 | GET | `/open/trader/newsliquid/v1/market/metadata` | 获取市场元数据（交易对列表、精度、限制等） |
| 2 | GET | `/open/trader/newsliquid/v1/market/ticker` | 获取实时行情（最新价、24h 涨跌幅、成交量） |
| 3 | GET | `/open/trader/newsliquid/v1/market/klines` | 获取 K 线数据（支持多种时间周期） |
| 4 | GET | `/open/trader/newsliquid/v1/market/base-currencies` | 获取基础货币列表（USDT、BTC 等） |
| 5 | GET | `/open/trader/newsliquid/v1/market/time` | 获取服务器时间 |

### 2.2 账户信息 (Account) — 3 个接口，无需风控

| # | 方法 | 路径 | 说明 |
|---|------|------|------|
| 6 | GET | `/open/trader/newsliquid/v1/account/summary` | 账户摘要（余额、杠杆、最大仓位） |
| 7 | GET | `/open/trader/newsliquid/v1/account/spot` | 查询指定现货资产 |
| 8 | GET | `/open/trader/newsliquid/v1/account/spots` | 查询所有现货资产 |

### 2.3 配置管理 (Config) — 2 个接口，无需风控

| # | 方法 | 路径 | 说明 |
|---|------|------|------|
| 9 | GET | `/open/trader/newsliquid/v1/config` | 获取交易配置 |
| 10 | PUT | `/open/trader/newsliquid/v1/config` | 更新交易配置 |

### 2.4 订单管理 (Orders) — 5 个接口，下单/改单需风控

| # | 方法 | 路径 | 风控 | 说明 |
|---|------|------|------|------|
| 11 | POST | `/open/trader/newsliquid/v1/orders` | ✅ | 创建订单（限价/市价/止盈止损） |
| 12 | PUT | `/open/trader/newsliquid/v1/orders/edit` | ✅ | 修改订单 |
| 13 | DELETE | `/open/trader/newsliquid/v1/orders/:orderId` | ✗ | 取消订单 |
| 14 | GET | `/open/trader/newsliquid/v1/orders/open` | ✗ | 查询当前挂单 |
| 15 | GET | `/open/trader/newsliquid/v1/orders/closed` | ✗ | 查询已完成订单 |

### 2.5 持仓管理 (Positions) — 3 个接口，平仓需风控

| # | 方法 | 路径 | 风控 | 说明 |
|---|------|------|------|------|
| 16 | GET | `/open/trader/newsliquid/v1/positions` | ✗ | 查询当前持仓 |
| 17 | GET | `/open/trader/newsliquid/v1/positions/history` | ✗ | 查询历史持仓 |
| 18 | POST | `/open/trader/newsliquid/v1/positions/close` | ✅ | 平仓 |

### 2.6 交易历史 (Trades) — 1 个接口，无需风控

| # | 方法 | 路径 | 说明 |
|---|------|------|------|
| 19 | GET | `/open/trader/newsliquid/v1/trades/history` | 查询成交历史 |

### 2.7 杠杆与保证金 (Leverage) — 6 个接口，修改杠杆需风控

| # | 方法 | 路径 | 风控 | 说明 |
|---|------|------|------|------|
| 20 | GET | `/open/trader/newsliquid/v1/leverage` | ✗ | 获取可用杠杆倍数 |
| 21 | GET | `/open/trader/newsliquid/v1/leverage/current` | ✗ | 获取当前杠杆设置 |
| 22 | PUT | `/open/trader/newsliquid/v1/leverage/current` | ✅ | 修改杠杆倍数 |
| 23 | GET | `/open/trader/newsliquid/v1/margin/mode` | ✗ | 获取保证金模式 |
| 24 | GET | `/open/trader/newsliquid/v1/position/mode` | ✗ | 获取持仓模式（单向/双向） |
| 25 | PUT | `/open/trader/newsliquid/v1/position/mode` | ✗ | 修改持仓模式 |

### 2.8 钱包代理 (Wallet Agent) — 4 个接口，无需风控

| # | 方法 | 路径 | 说明 |
|---|------|------|------|
| 26 | POST | `/open/trader/newsliquid/v1/walletagent/create` | 创建钱包代理 |
| 27 | GET | `/open/trader/newsliquid/v1/walletagent/list` | 获取钱包代理列表 |
| 28 | GET | `/open/trader/newsliquid/v1/walletagent/address/:address` | 按地址查询钱包代理 |
| 29 | PUT | `/open/trader/newsliquid/v1/walletagent/authorize` | 授权钱包代理 |

**合计：29 个 API 端点**

---

## 3. 风控引擎说明

所有标记"风控"的写操作接口，在请求转发前会经过四重风控规则链：

| 规则 | 名称 | 说明 |
|------|------|------|
| Rule 1 | 价格偏离检测 (PriceDeviation) | 买入价不得偏离市场现价超过 10%，防止误操作 |
| Rule 2 | 仓位限制 (PositionLimit) | 单笔/总仓位不超过设定比率，控制风险敞口 |
| Rule 3 | 频率限制 (RateLimit) | 1 分钟内 API 调用频率限制，防止刷单 |
| Rule 4 | 余额校验 (BalanceCheck) | 校验可用余额/保证金是否充足 |

风控采用 **fail-open** 策略：当 Redis 不可用或市场数据获取失败时，请求放行（保证可用性）。所有风控结果记录到审计日志。

---

## 4. Workflow 设计

### 4.1 CEX 基础工作流

#### Workflow A：CEX 现货交易

```
1. 查看市场行情 (market/ticker)
2. 查看账户余额 (account/summary 或 account/spots)
3. 获取交易对元数据 (market/metadata) — 确认精度和限制
4. 下单 (orders) — 限价/市价
5. 查看挂单状态 (orders/open)
6. [可选] 修改订单 (orders/edit) 或取消订单 (orders/:orderId)
7. 查看成交历史 (trades/history)
```

#### Workflow B：CEX 合约交易

```
1. 查看市场行情 (market/ticker + market/klines)
2. 查看账户摘要 (account/summary) — 确认保证金余额
3. 查看/设置杠杆 (leverage/current)
4. 查看/设置持仓模式 (position/mode) — 单向或双向
5. 下单 (orders) — 开多/开空
6. 查看持仓 (positions)
7. [可选] 平仓 (positions/close) 或设置止盈止损
8. 查看历史持仓 (positions/history)
```

#### Workflow C：账户管理

```
1. 查看账户摘要 (account/summary)
2. 查看所有现货资产 (account/spots)
3. 查看/修改交易配置 (config)
4. 管理钱包代理 (walletagent/*)
```

### 4.2 CEX + DEX 联动工作流

以下工作流结合 `opentrade-newsliquid`（CEX）与现有 DEX 技能，实现跨场景策略。

#### Workflow D：新闻驱动的 CEX 交易

```
1. [opennews] 搜索加密新闻，获取 AI 评级和交易信号
2. [opentwitter] 查看 KOL 观点和市场情绪
3. [opentrade-newsliquid] 查看 CEX 行情 (market/ticker)
4. [opentrade-newsliquid] 查看账户余额 (account/summary)
5. [opentrade-newsliquid] 在 CEX 下单 (orders)
6. [opentrade-newsliquid] 监控持仓和订单状态
```

#### Workflow E：CEX-DEX 价差套利

```
1. [opentrade-newsliquid] 获取 CEX 价格 (market/ticker)
2. [opentrade-market] 获取 DEX 链上价格 (market/price)
3. 比较价差，判断套利机会
4a. CEX 价格低 → [opentrade-newsliquid] CEX 买入 + [opentrade-dex-swap] DEX 卖出
4b. DEX 价格低 → [opentrade-dex-swap] DEX 买入 + [opentrade-newsliquid] CEX 卖出
5. 确认两端成交，计算实际收益
```

#### Workflow F：CEX 合约对冲 + DEX 现货持仓

```
1. [opentrade-portfolio] 查看 DEX 链上持仓
2. [opentrade-newsliquid] 在 CEX 开反向合约仓位 (orders) — 对冲风险
3. [opentrade-newsliquid] 设置止盈止损
4. 持续监控两端持仓价值
5. 市场稳定后平仓 (positions/close)
```

#### Workflow G：DEX 发现 + CEX 执行

```
1. [opentrade-token] 搜索热门代币 / 发现新币
2. [opentrade-market] 查看链上交易活跃度和智能钱包信号
3. [opentrade-newsliquid] 确认该代币在 CEX 是否有交易对 (market/metadata)
4. [opentrade-newsliquid] 如果 CEX 有上线 → 在 CEX 交易（更低手续费、更好流动性）
5. [opentrade-dex-swap] 如果 CEX 未上线 → 在 DEX 交易
```

#### Workflow H：跨场景资金管理

```
1. [opentrade-newsliquid] 查看 CEX 账户余额 (account/summary + account/spots)
2. [opentrade-portfolio] 查看 DEX 链上钱包余额
3. 汇总全部资产，生成综合资产报告
4. [opentrade-newsliquid] 通过钱包代理 (walletagent) 管理 CEX↔链上资金流转
```

---

## 5. 技能路由设计

当用户请求涉及以下关键词时，应路由到 `opentrade-newsliquid`：

- CEX 交易、中心化交易所、合约交易、期货交易
- 下单、挂单、限价单、市价单、止盈、止损
- 开多、开空、平仓、持仓
- 杠杆、保证金、仓位模式
- CEX 行情、K 线、交易对
- newsliquid

**不应路由到本技能的场景**（转发到其他技能）：

| 场景 | 应使用的技能 |
|------|-------------|
| DEX swap / 链上交易 | opentrade-dex-swap |
| 链上钱包余额查询 | opentrade-portfolio |
| 链上市场数据 / 智能钱包信号 | opentrade-market |
| 代币搜索 / 持有者分析 | opentrade-token |
| 托管钱包（BSC/Solana） | opentrade-wallet |
| 交易广播 / Gas 费 | opentrade-gateway |

---

## 6. SKILL.md 设计要点

### 接口调用方式

本技能使用 **curl 直接调用 API**（与 opentrade-wallet 一致），不依赖 opentrade CLI 工具。原因：

1. Newsliquid 是独立的 CEX 网关，路由结构与 DEX 不同
2. 接口路径固定（`/open/trader/newsliquid/v1/...`），无需 router discovery
3. 保持与 opentrade-wallet 的一致性

### 文档结构

```
SKILL.md
├── Frontmatter (name, description, metadata)
├── Pre-flight Checks (token 配置)
├── Skill Routing (与其他技能的分工)
├── Quickstart (核心操作速览)
├── Command Index (29 个接口索引表)
├── API Reference (按分类详细文档)
│   ├── Market (5)
│   ├── Account (3)
│   ├── Config (2)
│   ├── Orders (5)
│   ├── Positions (3)
│   ├── Trades (1)
│   ├── Leverage (6)
│   └── Wallet Agent (4)
├── Workflows (CEX 独立 + CEX-DEX 联动)
├── Risk Engine Notes (风控说明)
├── Edge Cases
├── Amount Display Rules
└── Global Notes
```

---

## 7. 与现有技能的关系

```
                    ┌─────────────────────────────────┐
                    │         opentrade 技能族          │
                    ├────────────────┬────────────────┤
                    │   DEX 技能      │   CEX 技能      │
                    ├────────────────┼────────────────┤
                    │ dex-swap       │ newsliquid ◄NEW│
                    │ portfolio      │                │
                    │ market         │                │
                    │ token          │                │
                    │ gateway        │                │
                    │ wallet         │                │
                    └────────────────┴────────────────┘
                              ▲            ▲
                              │  联动工作流  │
                              └─────┬──────┘
                                    │
                         CEX-DEX 套利/对冲/发现
```

---

## 8. 注意事项

1. **鉴权一致**：使用与其他 opentrade 技能相同的 `OPEN_TOKEN`，无需额外配置
2. **风控透明**：在文档中说明风控规则，让用户理解为什么某些操作可能被拒绝
3. **错误处理**：区域限制错误（50125/80001）需友好提示，不暴露原始错误码
4. **金额单位**：CEX 接口使用标准单位（如 `0.1 BTC`），与 DEX 的最小单位（wei/lamports）不同，需在文档中明确说明
5. **无需 CLI**：本技能直接使用 curl 调用 API，不依赖 opentrade CLI 二进制
