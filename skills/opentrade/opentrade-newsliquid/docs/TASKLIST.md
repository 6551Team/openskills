# opentrade-newsliquid 开发任务清单

> 状态标记：⬜ 待开始 | 🔄 进行中 | ✅ 已完成 | ❌ 已取消

---

## Phase 1：SKILL.md 编写

核心交付物，AI 编程助手通过此文件理解和调用 newsliquid 技能。

| # | 任务 | 状态 | 说明 |
|---|------|------|------|
| 1.1 | 编写 SKILL.md 元数据头 | ✅ | name、description（触发关键词）、version、license |
| 1.2 | 编写 Pre-flight Checks 段 | ✅ | 环境变量配置、Token 获取、安全警告 |
| 1.3 | 编写 Skill Routing 段 | ✅ | 与其他 opentrade 技能的路由分工说明 |
| 1.4 | 编写 Quickstart 段 | ✅ | 6 个最常用接口的 curl 示例 |
| 1.5 | 编写 Command Index 段 | ✅ | 29 个 API 端点的索引表（按 8 个分类） |
| 1.6 | 编写 Market 接口文档 | ✅ | 5 个市场数据接口的详细参数、示例、响应格式 |
| 1.7 | 编写 Account 接口文档 | ✅ | 3 个账户信息接口 |
| 1.8 | 编写 Config 接口文档 | ✅ | 2 个配置管理接口 |
| 1.9 | 编写 Orders 接口文档 | ✅ | 5 个订单管理接口，含风控标记说明和多种订单类型示例 |
| 1.10 | 编写 Positions 接口文档 | ✅ | 3 个持仓管理接口 |
| 1.11 | 编写 Trades 接口文档 | ✅ | 1 个交易历史接口 |
| 1.12 | 编写 Leverage 接口文档 | ✅ | 6 个杠杆与保证金接口 |
| 1.13 | 编写 Wallet Agent 接口文档 | ✅ | 4 个钱包代理接口 |
| 1.14 | 编写 Workflow 段 | ✅ | 3 个 CEX 基础工作流 + 4 个 CEX-DEX 联动工作流 |
| 1.15 | 编写 Edge Cases 段 | ✅ | 风控拦截、余额不足、频率限制等 10 个异常场景 |
| 1.16 | 编写 Amount Display Rules 段 | ✅ | CEX 标准单位 vs DEX 最小单位说明 |
| 1.17 | 编写 Global Notes 段 | ✅ | 鉴权、风控、金额单位等通用注意事项 |

---

## Phase 2：项目集成

将新技能集成到 opentrade 技能族中。

| # | 任务 | 状态 | 说明 |
|---|------|------|------|
| 2.1 | 更新 QUICK_REFERENCE.md | ✅ | 技能总览表 + CEX Trading 工作流示例 |
| 2.2 | 更新 README.md | ✅ | 技能数量 6→7，添加 newsliquid 详细说明和示例 |
| 2.3 | 更新 CLAUDE.md | ✅ | Project Structure、Skills Overview、Common Workflows 三处更新 |

---

## Phase 3：CLI 支持（可选）

如需通过 opentrade CLI 调用 newsliquid 接口。

| # | 任务 | 状态 | 说明 |
|---|------|------|------|
| 3.1 | 评估 CLI 集成方案 | ⬜ | 确定是否需要在 Rust CLI 中添加 newsliquid 子命令 |
| 3.2 | 添加 newsliquid 命令模块 | ⬜ | 在 cli/src/commands/ 下新增 newsliquid.rs |
| 3.3 | 实现市场数据命令 | ⬜ | ticker、klines、metadata 等 |
| 3.4 | 实现账户命令 | ⬜ | summary、spot、spots |
| 3.5 | 实现订单命令 | ⬜ | create、edit、cancel、list |
| 3.6 | 实现持仓命令 | ⬜ | positions、close、history |
| 3.7 | 实现杠杆命令 | ⬜ | leverage get/set、margin mode、position mode |
| 3.8 | 实现钱包代理命令 | ⬜ | create、list、address、authorize |

---

## Phase 4：测试与验证

| # | 任务 | 状态 | 说明 |
|---|------|------|------|
| 4.1 | 验证所有 curl 示例可执行 | ⬜ | 逐一测试 SKILL.md 中的 curl 命令 |
| 4.2 | 验证风控接口行为 | ⬜ | 测试下单/改单/平仓的风控拦截场景 |
| 4.3 | 验证 Skill Routing 准确性 | ⬜ | 确认 AI 助手能正确路由到 newsliquid |
| 4.4 | 验证 CEX-DEX 联动工作流 | ⬜ | 端到端测试套利、对冲等联动场景 |
| 4.5 | 验证错误处理和边界情况 | ⬜ | 测试 Edge Cases 中列出的所有异常场景 |

---

## 优先级说明

- **P0（必须）**：Phase 1（SKILL.md 是技能的核心交付物） ✅ 已完成
- **P1（重要）**：Phase 2（集成到现有文档体系） ✅ 已完成
- **P2（可选）**：Phase 3（CLI 支持，视需求决定）+ Phase 4（测试验证）

---

## 依赖关系

```
Phase 1 (SKILL.md) ✅ ──→ Phase 2 (集成) ✅ ──→ Phase 4 (测试) ⬜
                                                    ↑
Phase 3 (CLI, 可选) ⬜ ────────────────────────────┘
```

Phase 1 和 Phase 2 已完成。Phase 3 为可选项。Phase 4 待人工验证。
