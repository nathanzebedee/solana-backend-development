# 11/01/2022 - solana development

### accounts
in solana, everything is an *account*. an account is essentially a file with an address (public key) that lives on the solana blockchain. if you ever need to store data on the blockchain, the data will be stored in an account.
* accounts have only one owner
* only the account owner can modify the account's data or make a debit to the account's balance
* you must pay *rent* in order to persist an account on the blockchain

below are the fields which exist on an account:
* key
* isSigner
* isWritable
* lamports
* data
* owner
* executable
* rent_epoch

### owner vs holder
* the **holder** is the entity who owns the private key of the account. they are able to transfer the balance of the account.
* the **owner** of an account has the write to amend the data of the account. by default, the *system program* is the owner of any account (unless specified otherwise).

ex: if you create an account in solana in order to store some SOL, you would be the owner but the system program would be the owner.

### program accounts
there are different types of accounts, and one such type is a *program account*. whereas other accounts store arbitrary data, programs store code (and hold no state or balance). a program is a collection of instructions that can be executed by the solana runtime. programs are written in rust and compiled to a binary blob that is stored on the blockchain. the program is responsible for validating the transaction and modifying the state of the blockchain.

things to know about program accounts:
* they are marked as `executable`
* they store executable code, and do not have a balance or hold any state
* are owned by the `BDF Loader` program
* can be read and interacted with via `instruction` calls
* their address can be referenced via the `program_id` field

**program derived addresses (PDAs)** enable programs to create accounts that only the program can modify.

the **BPF Loader** is a program that can load other programs onto the blockchain. it is the owner of all program accounts. it allows BPF programs to interface with the solana runtime.

**sealevel** is the solana network runtime that enables the parallel execution of instructions and transactions.

**token program**: a kind of program that implements a fungible token, or NFT other than the native SOL token.

**instructions**: what is called in order to execute a function on a program.

**sysvar**: an account which enforces certain variables of the network such as epoch, rent, validator rewards, etc.

**rent**: the solana network charges rent (in SOL) for the data that is held in accounts (since that data is taking up validator network resources). an account can be marked *rent exempt* if it has a balance of at least 2-years worth of rent. if an account is unable to pay rent, it will no longer load.

### native accounts
*special* programs that are required to run the network. below are a few notable native programs:
* **system program**: responsible for creating new accounts, and assigning account ownership, (if an account does not have a specified owner, it is owned by the system program) and for transferring SOL between accounts.
* **bpf loader**: responsible for deployment, upgrades, and instruction execution of solana programs

### data accounts
data accounts are simply accounts which store arbitrary data - like files. there are two kinds of data accounts:
* system owned accounts
* PDA (Program Derived Address) accounts

**program Derived Address (PDA) accounts** are accounts that are created by a program. they are owned by the program, and can only be modified by the program. they are useful for storing data that is only relevant to a program. With PDAs, a program can sign for a transaction without having to use a private key.

### differences from ethereum
**ethereum**:<br>
only smart contracts have storage and they naturally have full control over that storage.

**solana**:<br>
*any* account can store state, but the storage for smart contracts is only used to store immutable byte code -- the actual state is stored in a separate *data account*.

### using modules
you can bring modules into your program with the `use` keyword.

```
use std::env;
```

### error handling
in rust, we have *recoverable* and *unrecoverable* errors. * **recoverable errors** are errors that can be handled by the program. with these, we want to notify the user, but there is a structured way to proceed. for these kinds of errors, we use the `enum Result<T, E> { Ok(T), Err(E) }` type. a good practice is to use `match` to handle these errors.
* **un-recoverable errors** are errors that cannot be handled by the program, and will cause the program to exit.

```
fn main() {
    // recoverable
    let f = File::open("hello.txt"); // returns a Result
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // error handling
        },
    };

    // unrecoverable
    let array = [1, 2, 3];
    println!("{}", array[99]); // panics - out of bounds
}
```

### the `?` operator
the `?` operator is a shortcut for handling errors. it works similarly to the `match` expression, but it is less verbose. it can only be used in functions that return `Result`.

the `?` operator is placed after an expression.
* if the returned value is an `Ok`, the value inside the `Ok` will be returned from the current function.
* if the returned value is an `Err`, the `Err` will be returned from the current function.

```
fn main() {
    let f = File::open("hello.txt")?;
}
```

### panic! macro
the `panic!` macro is used to create *unrecoverable* errors. it is used when there is no way to recover from an error. it is used when there is a bug in the program, or if the program is in a bad state.

```
fn main() {
    panic!("Bug found!");
}
```

### Cargo.toml
the `Cargo.toml` file is used to configure the project. it is used to specify the dependencies of the project, and the version of rust that the project is using.

```
[package]
name = "hello"
version = "0.1.0"   
edition = "2021"

[dependencies]
...
```

