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


## 2. 如何使用
您可以拷贝一份`rtt_rust_example_app`程序到`Applications`目录，接着使用`scons --rust`命令来构建程序。这将生成一个静态库（`librust.a`）在`rtt_rust`目录下。请保证该静态库参与了最终的链接。

## 3. 联系方式
* 维护：陈泓霖
* 邮箱：chenhonglin@aliyun.com
