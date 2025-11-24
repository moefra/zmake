# ZMake 的设计哲学 (The Philosophy of ZMake)

> "打碎旧世界，创立新世界。"
>
> ZMake 不仅仅是一个构建工具，它是一个**分布式、去中心化、可编程**的构建生态系统。

---

## 一、 核心价值观 (Core Values)

### 1. 极致的去中心化 (Radical Decentralization)
**拒绝“官方特权”，拥抱“联邦宇宙”。**

*   **没有 `std` 或 `core` 保留字**：系统核心库只是众多包中的一个 (`moe.kawayi:zmake`)。
*   **基于域名的命名空间**：采用类似 Java/Maven 的反向域名表示法（如 `moe.kawayi:rules_cc`）。利用 DNS 的所有权机制天然解决命名冲突和身份验证问题。
*   **平等的生态位**：任何组织（如 Google, Microsoft）或个人开发者发布的构建规则，在解析优先级和功能上与官方规则完全平等。

### 2. 绝对的封闭性与可复现性 (Hermeticity & Reproducibility)
**“在我的机器上能跑”是不够的，必须在任何机器、任何时间都能跑出同样的结果。**

*   **沙箱化执行 (Sandboxing)**：
    *   构建脚本（`BUILD.ts`）和动作执行（Action Execution）在严格隔离的环境中运行。
    *   严禁未声明的文件系统访问、网络请求和环境变量读取。
*   **输入即定义 (Input Addressable)**：
    *   构建的输出仅取决于输入文件的哈希和配置（Configuration）。
    *   时间戳、用户路径、随机数生成器等非确定性因素被严格剔除。
*   **锁机制 (Locking)**：
    *   所有外部依赖（npm 包、工具链、远程规则）必须通过 `WORKSPACE` 解析并锁定哈希（Lockfile），确保时间维度的可复现性。

### 3. 显式优于隐式 (Explicit over Implicit)
**宁可繁琐，也要精确。**

*   **全限定标识符 (Fully Qualified ID)**：
    *   拒绝模糊的短名称（如 `test`），强制使用结构化 ID (`group:artifact@ver#type::path`)。
    *   消灭“魔法字符串”，通过类型系统和 ID 解析器保证引用的唯一性和正确性。
*   **配置显式传递**：
    *   工具链的选择、编译选项的变更，必须通过 `Configuration` 对象显式传递和继承，拒绝隐式的全局状态污染。

---

## 二、 技术架构原则 (Architecture Principles)

### 4. Vanilla TypeScript 赋能 (Powered by Vanilla TS)
**利用世界上最流行的脚本语言，但不改变其语法。**

*   **零方言 (No Dialects)**：直接使用标准的 TypeScript 编译器和语法，享受现有的 IDE（VS Code）、Linter（ESLint）和格式化工具（Prettier）生态。
*   **拥抱标准**：支持 ESM (ECMAScript Modules)，允许循环依赖（由 V8 引擎处理），支持 Top-Level Await。
*   **分层运行时 (Layered Runtime)**：
    *   **定义层 (`BUILD.ts`)**：纯声明式，无 IO 权限。
    *   **逻辑层 (`*.zmake.ts`)**：纯计算，用于生成构建图。
    *   **任务层 (`scripts/*.ts`)**：全功能 Deno 环境，用于运维和胶水代码。

### 5. 现代化的 Rust 内核 (Modern Rust Kernel)
**高性能、内存安全、并发友好。**

*   **库优先 (Library-First)**：
    *   `zmake` 首先是一个库 (`zmake_lib`)，其次才是一个命令行工具 (`zmake_cli`)。这意味着它可以被集成到 IDE、CI Runner 甚至作为 Web 服务运行。
*   **类型驱动开发**：利用 Rust 强大的类型系统（如 `NeutralPath`, `ConfiguredId`）在编译期消除逻辑错误。
*   **异步与并行**：基于 `Tokio` 和 `Actor` 模型设计，充分利用多核性能处理庞大的构建图。

### 6. 内容寻址与虚拟化 (CAS & Virtualization)
**文件不仅仅是路径，更是内容。**

*   **CAS (Content-Addressable Storage)**：所有源文件、中间产物、工具链均通过哈希（SHA256/xxHash）索引。去重、缓存和完整性校验是系统的原生能力。
*   **虚拟文件系统 (Virtual FS)**：构建过程中操作的是 `VirtualFile` 和 `Digest`，而非物理路径。这使得远程构建（Remote Execution）成为可能，因为 Worker 不需要拥有完整的文件系统，只需按需拉取数据块。

---

## 三、 生态与互操作 (Ecosystem & Interoperability)

### 7. 拥抱现有生态 (Ecosystem Reuse)
**不造孤岛，做连接者。**

*   **包管理器集成**：
    *   原生支持 `jsr` (JavaScript Registry) 和 `npm` 依赖。构建脚本可以直接引用现有的 JS 库（如 `lodash`, `semver`）来辅助逻辑编写。
*   **IDE 友好**：
    *   计划支持生成 `.sln` (Visual Studio), `compile_commands.json` (Clang/LLVM), `.idea` (IntelliJ) 等项目文件。
    *   通过 BSP (Build Server Protocol) 与编辑器深度集成。

### 8. 协议标准化 (Protocol Standardization)
**使用通用的语言与世界对话。**

*   **数据交换**：使用 JSON 和 Protobuf 定义接口。
*   **远程执行 API (REAPI)**：兼容或参考 Bazel Remote Execution API，使得 `zmake` 可以对接现有的构建农场（Build Farm）和缓存后端。

### 9. 原生可扩展 (Native Extensibility)
**从单机到集群的平滑过渡。**

*   **远程缓存**：支持 HTTP/S3/Redis 等多种后端存储构建产物。
*   **远程执行**：支持通过 gRPC/QUIC 将繁重的编译任务分发到远程 Worker 集群，实现大规模并行构建。
*   **插件化工具链**：通过 `ToolProvider` 和 `ToolType` 接口，用户可以轻松接入非标准的编译器或自定义工具，而无需修改内核。

---

**总结：**
ZMake 致力于成为一个**严谨的工程师工具**。它牺牲了一定的“上手随意性”，换取了**大规模软件工程**所需的稳定性、速度和可维护性。
