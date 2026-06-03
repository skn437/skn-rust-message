# SKN Rust Message Library

<img width="150px" src="./public/rust.svg" alt="rust" />

> Rust

[![Crates IO](https://img.shields.io/crates/v/best_skn_message)](https://crates.io/crates/best_skn_message) [![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/mit)

&nbsp;

## **_RustDocs:_**

### Read the Rustdoc for the main Module

- [message](https://docs.rs/best_skn_message/1.3.0/best_skn_message/message/index.html)

&nbsp;

## **_Introduction:_**

### This is a simple Rust Library for notification message on console

### I made this library so that I can use it in all of my rust projects without writing the same codes over and over again

### The main Module of this library is `message` which holds 3 functions to return colored Strings & 3 functions to output formatted static String notification messages

&nbsp;

## **_Details:_**

### **`message` Module:**

- It has 3 functions which return colored Strings as output
  - error (takes 1 argument & returns a red colored String with cross mark)
  - success (takes 1 argument & returns a green colored String with tick mark)
  - info (takes 1 argument & returns a blue colored String with book info mark)

- It has 3 functions which outputs formatted static String notifications
  - action_failure (takes 1 argument as 'action name' & outputs an action failure message)
  - action_complete (takes 1 argument as 'action name' & outputs an action complete message)
  - action_notify (takes 2 arguments as 'action name', 'notification info' & outputs an action info message)

- The String returned by each function, doesn't contain new line character i.e. '\n'

&nbsp;

## **_Use Case:_**

- Rust

&nbsp;

## **_Requirements:_**

- 💀 Minimum Rust Version: `1.93.0`

&nbsp;

## **_Usage:_**

### To install the package, type the following in console

> ```zsh
> cargo add best_skn_message
> ```

### Inside your Rust Code, import the package like this

> ```rust
> use best_skn_message::message;
> ```

### For `Message` Module, use like the following (Just an example)

> ```rust
> let error: String = message::error("Error occurred!");
> panic!("{} \n", error);
>
> let success: String = message::success("Process completed!");
> println!("{}", success);
>
> let info: String = message::info("Process is safe to pause!");
> println!("{}", info);
>
> let failure: String = message::action_failure("Copy Files"); // Action name as argument
> panic!("{} \n", failure);
>
> let complete: String = message::action_complete("Read Config File"); // Action name as argument
> println!("{}", complete);
>
> let notify: String = message::action_notify("Run Shell Scripts", "Safe to use without error!"); // Action name & notification info message as arguments
> println!("{}", notify);
> ```

&nbsp;

## **_Dedicated To:_**

- 👩‍🎨`Logno` & 🧛‍♀️`Atoshi`: The two most special ladies of my life. My best wishes will always be with you two.
- 💯`My Parents`: The greatest treasures of my life ever.
- 👩`Trina`: The lady who resides deep inside my heart. Our paths got separated, perhaps it was fate and I accepted it. But you are always in each and every moments of my life. May you be healthy and happy always.

&nbsp;

## **_License:_**

Copyright (C) 2024 SKN Shukhan

Licensed under the MIT License
