---
theme: default
layout: center
highlighter: shiki
css: unocss
colorSchema: light
transition: fade-out
mdc: true
growX: 50
growY: 130
growSize: 1.5
favicon: /trip.ico
title: '使用Rust开发跨平台SDK'
titleTemplate: '%s - Slides'
---

<h1 flex="~ col">
<div text-2xl op80 font-montserrat>使用Rust开发跨平台SDK<devicon-rust ml-1 /></div>
</h1>

<div uppercase text-sm tracking-widest op50>
UGC前端组 冯伟
</div>

<div abs-br mx-35 my-11 flex="~ col gap-4 items-end" text-left>
  <img src="/trip-group.png" h-7 alt="Trip.com Group">
  <div text-xs opacity-75 mt--4>2024/04/24</div>
</div>

---
layout: intro
---

## 背景

<div mt-4 />

在当今的软件开发领域，图像处理已成为一个不可或缺的部分。从简单的图片编辑到复杂的机器视觉应用，图像处理技术的应用场景日益广泛。然而，开发者在面对图像处理需求时，往往面临着以下困境：
1. 平台差异性: 不同的操作系统和设备有着不同的API和图像处理库，导致开发者需要为每个平台编写特定的代码。
2. 语言限制: 现有的图像处理库往往只支持特定编程语言，限制了开发者在项目中使用多种语言的能力。
3. 安全性和稳定性: 在图像处理中，内存管理和并发控制不当可能导致安全漏洞或程序崩溃

---
layout: intro
growX: 80
growY: -10
---

## 为什么选择Rust?

<div mt-4 />

<v-clicks>

- 内存安全：Rust的所有权系统保证了内存安全，减少了内存泄漏和悬挂指针的风险。
- 并发性：Rust的并发模型简单而强大，可以在不牺牲安全性的前提下提高程序的执行效率。
- 跨平台：Rust支持跨平台编译，可以轻松构建适用于多种操作系统和CPU架构的应用程序。
- 性能：Rust的性能接近C和C++，适合对性能有较高要求的图像处理任务。
- FFI（外部函数接口）：Rust的FFI特性使得它可以轻松地与C语言互操作，进而支持在其他编程语言中调用。
- [Save the planet, code in Rust 🐶](https://tweedegolf.nl/en/blog/120/green-rust)
![green rust](https://tweedegolf.nl/images/rust-is-green-main.png){.w-120}

</v-clicks>
<div mt-8 />
<v-click>

> 选择Rust，它不仅能够提供高效的图像处理功能，而且能够安全、稳定地在多种操作系统和编程语言中运行。

</v-click>

<!--

内存安全: rust被设计成在编译时强制执行内存安全检查的一种语言. 意味着在开发时, 开发人员就能规避常见的内存安全问题, 如空指针引用、内存泄漏、缓冲区溢出等. 这一切也得益于其所有权和借用检查器
并发安全: rust的并发安全建立在所有权系统、借用检查和生命周期检查的基础上, 使得开发人员可以在不牺牲安全性的前提下实现高效的并发处理. rust 内置了线程安全特性, 如原子类型、锁、条件变量等, rust通过Future和async/await等异步编程模式来支持非阻塞等IO操作
跨平台: Rust的跨平台能力来自其编译器和语言特性. rust编译器基于LLVM项目 本身就支持多种架构和操作系统. rust提供了许多跨平台API, 这些功能可以在不同平台使用, 并且可以通过条件编译进行适配, 实现跨平台编译

有很多优点.
rust 已经成为国内外诸多大厂主力开发语言.

该文章依据科学文献，并通过数据中心电力消耗的现状和趋势，强调了采用Rust这样高效的编程语言对减缓电力消耗增长有重要意义，并可以降低计算成本，为保护地球贡献一份力量。

 -->

---
layout: intro
---

## Trade off

<div mt-8></div>

选择Rust意味着:
- 范式转移 (*Paradigm Shift*)
- 大量计算机体系结构、操作系统知识扑面而来

![rust-difficulty-curve](/rust-difficulty-curve.png){.h-80.mx-auto}

<!--

rust的学习曲线是陡峭的, 但是一旦掌握, 会发现rust是一种非常强大的语言, 可以帮助开发人员避免许多常见的错误, 并提高代码的质量和性能
学习Rust, 首先需要了解:
- 所有权系统和借用检查器
- 零成本抽象
  - 在Rust中使用抽象(trait、范型、闭包等) 不会带来运行时代价, rust在编译时对抽象进行优化并转化为高效的机器码
  - 使用抽象提高代码的可读性和可维护性, 而无需担心性能问题
- 并发和异步编程
- 宏编程
- unsafe代码, 绕过编译时的安全检查, 但需要手动确保内存安全

-->

---
layout: center
growX: 0
growY: 70
---

## Alternatives


<div mt-8></div>

1. **C/C++**：虽然`C/C++`在性能上与`Rust`相当，但它们缺乏内存安全保障，增加了开发和维护的难度。
2. **Java**：`Java`具有良好的跨平台特性和成熟的生态系统，但其性能不如Rust，特别是在处理图像数据时。
3. **Go**：`Go`语言在并发编程和网络服务方面表现出色，但在系统级编程和与C语言互操作方面不如Rust灵活。
4. **WASI**: WebAssembly System Interface, 为WebAssembly定义了一套系统调用接口, 但其生态尚不完善. [Towards a modern Web Stack.](https://docs.google.com/document/d/1peUSMsvFGvqD5yKh3GprskLC3KVdAlLGOsK6gFoEOD0/edit?resourcekey=0-bPajpoo9IBZpG__-uCBE6w#heading=h.34a91yqebirw)

<v-clicks>

**选择Rust是因为它在性能、安全性、并发性以及跨平台能力方面提供了最佳平衡，同时其FFI特性也满足了与多种编程语言互操作的需求。**

</v-clicks>

<!--

Ian Hixie(前HTML规范编辑、现Flutter团队成员)发表的一项简短宣言: Towards a modern Web Stack. 他认为, 网络和浏览器太多了, 建议重新开始, 在WebAssembly(用于代码)、WebGPU(用于图形)、WebHID(用于输入)和ARIA(可访问性)之上重建Web
wasm是浏览器支持的第二种语言,支持动态分发(运行时添加逻辑, 应用程序的大部分都是动态的并来自网络)


 -->

---
layout: intro
---

## 从C调用Rust

```rs
#[repr(C)]
pub struct User {
    pub name: String,
    age: u8,
}

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}
```

```c
extern void hello_from_rust();

int main() {
    hello_from_rust();
    return 0;
}
```

```sh
$ rustc ./extlib.rs --crate-type=cdylib -C link-args="-s" -o libextlib.so
$ gcc ./main.c -o main -L . -lextlib
$ ./main
Hello from Rust!
```

<!--

结构体加上`#[repr(C)]`使用C的内存布局
函数调用, 使用`#[no_mangle]`避免函数名被混淆, `extern "C"`声明遵循C的函数调用约定

首先使用rustc编译Rust代码, 然后使用gcc编译C代码, 最后链接生成的二进制文件

-->



---
layout: default
---

### 从Rust中调用C函数

```rs
#[link(name = "extlib")]
extern {
    fn hello_from_c();
}

fn main() {
    unsafe {
        hello_from_c();
    }
}
```

```c
#include <stdio.h>
#include <stdint.h>

void hello_from_c() {
    printf("Hello from C!\n");
}
```

```sh
$ gcc -shared -o libextlib.so extlib.c
$  rustc ./main.rs -l extlib -L .
$ ./main
Hello from C!
```

<!--

`#[link(name = "extlib")]` 用于指示rustc链接名为`extlib`的库, 以便在运行时找到对应的符号
先用gcc编译C代码为动态链接库, 然后用rustc编译Rust代码, 并指定链接C库
-->

---
layout: center
---

## 如何实现?

<div mt-4 />

> FFI: Rust的FFI特性使得它可以轻松地与C语言互操作，进而支持在其他编程语言中调用。

[gitlab/imagekit]([vscode:///Users/fengwei/Projects/ctrip/libs/imagekit/](https://git.dev.sh.ctripcorp.com/feng.w/imagekit))

---
layout: center
growX: 0
growY: 70
---

## 如何实现?

<div mt-4 />

在构建`imagekit` SDK的过程中，精心挑选了一系列Rust生态中的库（crates），它们为项目提供了强大的功能支持和灵活性。以下是此项目中使用的主要技术：
- `image`：提供了丰富的图像处理功能和格式转换能力，Rust在图像处理领域的事实标准库。
- `uniffi`: 为Kotlin、Swift、Python、Ruby等多种编程语言生成接口绑定。
- `prost`: ProtoBuf编解码。
- `tonic`: 高性能的gRPC框架，支持Rust的异步特性，为构建分布式系统提供了强大的支持。
- `salvo`: 快速且灵活的HTTP服务器框架，能够轻松处理各种HTTP请求。
- `schemars`: 生成`JSON Schema`，帮助定义和验证数据结构，确保数据的一致性和有效性。
- `clap`: 在命令行参数解析方面，clap提供了强大的功能，允许轻松构建复杂的命令行界面。
- `cxx`: 用于创建和使用C++绑定的库，使得Rust能够无缝地与现有的C++生态系统集成。
- `napi-ohos`: 使用Rust开发HarmonyOS的`Node-API`扩展

除以上核心库，还利用了`wasm-bindgen`和`js-sys`来支持在WebAssembly环境中调用imagekit。这使得SDK能够在现代Web浏览器、Wasmer等环境中运行，极大地拓展了其应用范围。
此外，还使用了`quicktype`来生成多种编程语言的数据模型

---
layout: center
growX: 0
growY: 70
---

## 未来展望

<div mt-4 />

- 通过[Dioxus](https://dioxuslabs.com/)/[TAURI](https://tauri.app/)/[egui](https://docs.rs/egui/latest/egui/)开发跨多端应用(Desktop、Mobile、Web) [Are we GUI Yet?](https://areweguiyet.com/)
- 跨平台业务逻辑复用
  - [crux](https://redbadger.github.io/crux/)
  - [xstate-rs](https://github.com/statelyai/xstate/discussions/3836)
- 构建跨平台组件
  - [ndk](https://crates.io/crates/ndk): Rust bindings for the Android NDK
  - [cocao](https://crates.io/crates/cacao): Provides safe bindings for `AppKit` and `UIKit`
  - [HarmonyOS Native XComponent](https://developer.huawei.com/consumer/cn/doc/harmonyos-guides-V5/napi-xcomponent-guidelines-0000001885919493-V5)

---
layout: center
growX: 40
growY: 90
---

## 参考

- [Firefox Application Services](https://github.com/mozilla/application-services)
- [Bitwarden SDK](https://github.com/bitwarden/sdk)
- [使用Node-API实现跨语言交互](https://developer.huawei.com/consumer/cn/doc/harmonyos-guides-V5/node-api_u5b9e_u73b0_u8de8_u8bed_u8a00_u4ea4_u4e92-0000001774120790-V5)
- [FeatureProbe](https://github.com/FeatureProbe/FeatureProbe#readme)

---
layout: end
---
