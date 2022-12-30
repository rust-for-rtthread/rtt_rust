# RUST support for rt-thread

## 1. 介绍
  这是一个在`rt-thread`上制作的`rust`支持层

### 1.1 目录结构
| 名称 | 说明 |
|---|---|
| docs | 文档目录 |
| rtt_rs2 | 支持层的核心代码 |
| rtt_main | APP启动宏 |
| rtt_rust_example_app | 一个简单的示例程序 |

### 1.2 许可证

### 1.3 依赖
* RT-Thread 3.0+
* RUST toolchains +nightly
* rust-src
- toml (可以使用 pip 安装)


## 2. 如何使用
请在`applications`目录下面添加你的程序
你可以使用如下命令生成：`cargo new --lib xxx`
然后增加基本的库的依赖如下
```toml
# file Cargo.toml
# 请注意修改版本号
[dependencies]
rtt_main = {path = "../../packages/rtt_rust-v1.0.0/rtt_main/"}
rtt_rs2 = {path = "../../packages/rtt_rust-v1.0.0/rtt_rs2/"}
```
然后添加一个最小程序
```rust
// file src/lib.rs
#![no_std]

extern crate alloc;

use alloc::string::String;
use rtt_main::rtt_main;
use rtt_rs2::param::Param;
use rtt_rs2::println;

// appname: 应用的名字，在命令中将被使用
// run: 是否使用rt-thread操作系统的自动执行功能
// cmd: 是否添加app到命令行
// desc: 命令行程序的描述
// 最简版本：#[rtt_main(appname="demo")]
//  请自行调用函数 __demo_main_func
#[rtt_main(appname="demo", run=true, cmd=true, desc="demo app.")]
fn main(param: Param) {
    for i in param {
        println!("{}", String::from_utf8_lossy(&*i))
    }
}
```
`APPS`作为一个标准的rust库，你可是使用任何支持的IDE来辅助你的开发。但是`rtt_rs2`在`build.rs`里面应用了一个环境变量 `RTT_PATH`,请你正确设置这个变量到你的`scons --dist`之后的主路径（因为这将使用rt-thread中的头文件来进行bindgen）。

## 3. 支持情况

- [x] APP自启动
- [x] APP添加到MSH
- [x] 线程基本操作
- [x] 系统延时函数
- [x] 系统信号量
- [x] 系统互斥量
- [x] 系统消息队列

## 4. 联系方式
* 维护：陈泓霖
* 邮箱：chenhonglinchl@aliyun.com
