//! 第16章 无谓并发
//! 并发编程（Concurrent programming），代表程序的不同部分相互独立地执行，
//! 并行编程（parallel programming）代表程序不同部分同时执行
//! 本章将要涉及到的内容：
//!
//! 如何创建线程来同时运行多段代码。
//! 消息传递（Message passing）并发，其中信道（channel）被用来在线程间传递消息。
//! 共享状态（Shared state）并发，其中多个线程可以访问同一片数据。
//! Sync 和 Send trait，将 Rust 的并发保证扩展到用户定义的以及标准库提供的类型中。

/// 使用线程同时运行代码
mod ch16_01_threads;
pub mod ch16_02_message_passing;
pub mod ch16_03_shared_state;
mod ch16_04_extensible_concurrency_sync_and_send;