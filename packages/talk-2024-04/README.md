# 基于Rust开发跨平台SDK

## 为什么选择Rust?

> 在获得C++性能的同时, Rust提供了更好的内存安全性和并发性能, 并且不需要GC和运行时.


1. 🐶[Save the planet, code in Rust](https://tweedegolf.nl/en/blog/120/green-rust)
![green rust](https://tweedegolf.nl/images/rust-is-green-main.png)

<!--

该文章依据科学文献，并通过数据中心电力消耗的现状和趋势，强调了采用Rust这样高效的编程语言对减缓电力消耗增长有重要意义，并可以降低计算成本，为保护地球贡献一份力量。

 -->

2. 零成本抽象
   1. 内存安全(所有权)
      - 手动管理 C/C++, 容易出错
      - 智能指针 C++/ObjC/Swift, 性能损失、循环引用问题
      - GC Java/#Net/Erlang 大量内存消耗、不必要的堆内存分配,潜在的[stop-the-world](https://github.com/golang/go/blob/master/src/runtime/mgc.go#L24)
        - ARC(自动引用计数) Objective-C/Swift
      - 所有权 Rust, 范式转移
    2. 并发安全
      - 单进程(最安全的并发是单线程并发) JS, 无法有效利用多核
      - GIL Python/Ruby, 一把大锁牺牲性能换来安全, 锁的粒度太大
      - CSP Go, 消息同步(coroutine -> channel -> coroutine), 额外内存拷贝和堆内存分配
      - Actor model Erlang/Elixir/Akka, 消息同步(actor -> actor),额外内存拷贝和堆内存分配
      - 所有权 + 类型系统 Rust, 类型安全保证并发安全, 无性能损失(可无缝兼容其他并发安全方案, 如actix)
    3. 类型安全
3. 性能
    - 抽象最小化 ASM/C
    - 零成本抽象 Rust/C++
4. 语言表现力
    - 函数式编程 Haskell/Elm
    - Duck typing Python/Ruby/JS

总而言之:
1. 类型、内存、并发安全
2. C/C++级别的性能
3. Python/Elixir级的语言表现力

### Rust的`value`

1. 在作用域中只允许一个所有者, 但离开作用域自动销毁
2. 可以拥有多个不可变引用
3. 只能有1个可变引用
4. 引用的生命周期必须小于所有者的生命周期

<!-- 看示例代码 -->

保证类型安全的traits:
- Copy trait: i32, usize, [T:4]等
- 闭包(Closure) trait: 通过引用捕获作用域的值(也可move), Fn, FnMut, FnOnce
- Deref/DerefMut trait: 允许解引用或持有可变引用, Box/Arc/MutexGuard/RwLockGuard...
- Send/Sync trait: 线程传递安全, Rc
- Sync trait: 数据多线程共享安全, Rc/Cell/RefCell

> trait在rust中类似于interface 本身不包含任何数据, 类似marker指定这个数据结构的特性,


<!--
prompt 1:
我是一个软件工程师, 在准备公司内部的技术分享, 主题是: 使用Rust开发跨平台、多语言SDK. 我想分享一些Rust的特性, 以及如何基于FFI实现这一目标. 我的分享大纲为:
1. 动机
    1.1 为什么要做这个项目?
        1.1.1 困境
    1.2 为什么选择Rust?
2. 理由和替代方案
   2.1 为什么这个设计是所有可能性中最好的?
   2.2 还考虑过哪些? 不选择的理由
3. 实现该项目的现有技术?
4. 如何使用Rust实现?
    3.1 代码设计
    3.2
5. 未解决的问题
6. 未来可能性

需要你帮我看看这个大纲是否合理, 有没有遗漏的地方, 以及有没有需要补充的地方?

prompt 2:
要做的这个SDK名为`imagekit`, 计划支持多个操作系统(window、linux、macOS、iOS、Android、鸿蒙...)与CPU架构(x86、arm、RISC-V...), 支持在多个编程语言中调用(java、python、c/c++、js...), 提供基本的图像处理功能、格式转换、OCR等. 需要你帮我基于以上描述, 为这个分享的`动机`部分, 生成有说服力的内容

prompt 3:
在此分享中, 我需要具体描述使用Rust的优势, 及考虑过的替代方案, 和不选择其他方案的原因, 帮我生成这部分内容

prompt 4:
该项目中主要使用了以下`crates`:
- `image` 提供图片处理功能、格式转换
- `uniffi` 多语言绑定生成器(Kotlin、Swift、Python、Ruby...)
- `prost` 使用`ProtoBuf`协议进行数据交换
- `tonic` 提供gRPC服务
- `salvo` 提供HTTP服务
- `schemars` 提供JSON Schema生成
- `clap` 提供命令行参数解析
- `cxx`
此外, 还使用了`wasm-bindgen`和`js-sys`支持在WebAssembly(WASI)中调用, `quicktype`生成多种编程语言的数据模型

需要你帮我润色下以上内容, 以填充`实现该项目的现有技术`部分

prompt 5:
该项目目录结构为:
```
.
├── Cargo.toml # cargo 项目根配置文件
├── apps
│   └── mobile-tauri # 基于Tauri的桌面端应用
├── crates
│   ├── imagekit    # 核心库
│   ├── imagekit-api   # OpenAPI 服务
│   ├── imagekit-cli    # 命令行工具
│   ├── imagekit-json   # JSON 调用方式绑定
│   ├── imagekit-ohos   # 鸿蒙系统支持
│   ├── imagekit-uniffi  # uniffi 绑定
│   ├── imagekit-wasm   # WebAssembly 支持
│   ├── sdk-schemas     # JSON Schema 生成
│   └── uniffi-bindgen
├── languages
│   ├── kotlin
│   ├── swift
│   └── wasm
├── nx.json
├── package.json
├── pnpm-workspace.yaml
├── rust-toolchain.toml
├── scripts
│   ├── aarch64-unknown-linux-ohos-clang++.sh
│   ├── aarch64-unknown-linux-ohos-clang.sh
│   ├── armv7-unknown-linux-ohos-clang++.sh
│   ├── armv7-unknown-linux-ohos-clang.sh
│   ├── x86_64-unknown-linux-ohos-clang++.sh
│   └── x86_64-unknown-linux-ohos-clang.sh
├── support
│   ├── schemas
│   └── scripts
└── tsconfig.base.json
```
 -->

------

## 1. 动机

### 1.1 为什么要做这个项目?

在当今的软件开发领域，图像处理已成为一个不可或缺的部分。从简单的图片编辑到复杂的机器视觉应用，图像处理技术的应用场景日益广泛。然而，开发者在面对跨平台、多语言的图像处理需求时，往往面临着以下困境：
1. 平台差异性: 不同的操作系统和设备有着不同的API和图像处理库，导致开发者需要为每个平台编写特定的代码。
2. 语言限制: 现有的图像处理库往往只支持特定编程语言，限制了开发者在项目中使用多种语言的能力。
3. 安全性和稳定性: 在图像处理中，内存管理和并发控制不当可能导致安全漏洞或程序崩溃

### 1.2 为什么选择Rust?

- 内存安全：Rust的所有权系统保证了内存安全，减少了内存泄漏和悬挂指针的风险。
- 并发性：Rust的并发模型简单而强大，可以在不牺牲安全性的前提下提高程序的执行效率。
- 跨平台：Rust支持跨平台编译，可以轻松构建适用于多种操作系统和CPU架构的应用程序。
- 性能：Rust的性能接近C和C++，适合对性能有较高要求的图像处理任务。
- FFI（外部函数接口）：Rust的FFI特性使得它可以轻松地与C语言互操作，进而支持在其他编程语言中调用。

选择Rust，它不仅能够提供高效的图像处理功能，而且能够安全、稳定地在多种操作系统和编程语言中运行。

## 2. 理由和替代方案

### `trade off`

在决定使用Rust之前，也考虑了其他几种语言和技术方案：
1. C/C++：虽然C/C++在性能上与Rust相当，但它们缺乏内存安全保障，增加了开发和维护的难度。
2. Java：Java具有良好的跨平台特性和成熟的生态系统，但其性能不如Rust，特别是在处理图像数据时。
3. Go：Go语言在并发编程和网络服务方面表现出色，但在系统级编程和与C语言互操作方面不如Rust灵活。

选择Rust是因为它在性能、安全性、并发性以及跨平台能力方面提供了最佳平衡，同时其FFI特性也满足了我们与多种编程语言互操作的需求。


## 3. 实现该项目的现有技术?

在构建`imagekit` SDK的过程中，精心挑选了一系列Rust生态中的库（crates），它们为项目提供了强大的功能支持和灵活性。以下是此项目中使用的主要技术：
- `image`：提供了丰富的图像处理功能和格式转换能力，是Rust在图像处理领域的事实标准库。
- `uniffi`: 为了生成跨语言的绑定，而采用了uniffi。它允许为Kotlin、Swift、Python、Ruby等多种编程语言生成接口绑定，极大地简化了SDK在不同语言环境中的集成。
- `prost`: 使用ProtoBuf作为数据交换格式，确保了数据结构的一致性和高效序列化，这对于跨语言服务之间的通信至关重要。
- `tonic`: 为了提供gRPC服务，我们选择了tonic。它是一个高性能的gRPC框架，支持Rust的异步特性，为构建分布式系统提供了强大的支持。
- `salvo`: 它是一个快速且灵活的HTTP服务器框架，能够轻松处理各种HTTP请求。
- `schemars`: 使用了schemars来生成`JSON Schema`，帮助定义和验证数据结构，确保数据的一致性和有效性。
- `clap`: 在命令行参数解析方面，clap提供了强大的功能，允许我们轻松构建复杂的命令行界面。
- `cxx`: 为了与C++代码进行互操作，使用了cxx。它是一个用于创建和使用C++绑定的库，使得Rust能够无缝地与现有的C++生态系统集成。

除了这些核心库，还利用了wasm-bindgen和js-sys来支持在WebAssembly环境中调用imagekit。这使得SDK能够在现代Web浏览器、Wasmer等环境中运行，极大地拓展了其应用范围。
此外，还使用了`quicktype`来生成多种编程语言的数据模型.


## 4. 如何使用Rust实现?

### 4.1 目录结构
