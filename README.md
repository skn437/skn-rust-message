# SKN Rust Message Library

<img width="150px" src="https://firebasestorage.googleapis.com/v0/b/skn-ultimate-project-la437.appspot.com/o/GitHub%20Library%2F06-Rust-SRM.svg?alt=media&token=31a6d76a-e240-458b-ae2a-9b90ec19a775" alt="rust" />

> Rust

[![Crates IO](https://img.shields.io/crates/v/best_skn_message)](gh) [![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/mit)

&nbsp;

## **_RustDocs:_**

### Read the Rustdoc for the main Module

- [Message Module](gh)

&nbsp;

## **_Introduction:_**

### This is a simple Rust Library for notification message on console

### I made this library so that I can use it in all of my rust projects without writing the same codes over and over again

### The main Module of this library is `Message` which holds 3 functions to return colored String & 3 functions to output notification messages

&nbsp;

## **_Details:_**

### **`Message` Module:**

- It has 3 functions which return colored Strings as output

  - error (takes 1 argument & returns a red colored String with cross mark)
  - success (takes 1 argument & returns a green colored String with tick mark)
  - info (takes 1 argument & returns a blue colored String with book info mark)

- It has 3 functions which outputs notifications

  - action_failure (takes 1 argument as 'action name' & outputs an action failure message)
  - action_complete (takes 1 argument as 'action name' & outputs an action complete message)
  - action_notify (takes 2 arguments as 'action name', 'notification info' & outputs an action info message)

&nbsp;

## **_Use Case:_**

- Rust

&nbsp;

## **_Requirements:_**

- 💀 Minimum Rust Version: `1.78.0`

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
> let error: String = message::error("Error occured!");
> let success: String = message::success("Process completed!");
> let info: String = message::info("Process is safe to pause!");
>
> message::action_failure("Copy Files"); // Action name as argument
> message::action_complete("Read Config File"); // Action name as argument
> message::action_notify("Run Shell Scripts", "Safe to use without error"); // Action name & notification info message as arguments
> ```

&nbsp;

## **_Dedicated To:_**

- 👩‍🎨`Prodipta Das Logno` & 🧛‍♀️`Atoshi Sarker Prithula`: The two most special ladies of my life. I can't thank them enough for always treasuring me a lot. I am lucky that I met with these two amazing ladies. They have two special places in my heart and no other girl can ever replace them.
- 💯`My Father & Mother`: The greatest treasures of my life ever.
