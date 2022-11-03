# 11/01/2022

### accounts
in solana, everything is an *account*. an account is essentially a file with an address (public key) that lives on the solana blockchain. if you ever need to store data on the blockchain, the data will be stored in an account.
* accounts have only one owner
* only the account owner can modify the account's data or make a debit to the account's balance
* you must pay *rent* in order to persist an account on the blockchain

below are the fields which exist on an account:
* key
* isSigner
*isWritable
* lamports
* data
* owner
* executable
* rent_epoch

### owner vs holder
* the **holder** is the entity who owns the private key of the account. they are able to transfer the balance of the account.
* the **owner** of an account has the write to amend the data of the account. by default, the *system program* is the owner of any account (unless specified otherwise).

ex: if you create an account in solana in order to store some SOL, you would be the owner but the system program would be the owner.

### programs
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

### native programs
*special* programs that are required to run the network. below are a few notable native programs:
* **system program**: responsible for creating new accounts, and assigning account ownership. (if an account does not have a specified owner, it is owned by the system program)
* **bpf loader**: responsible for deployment, upgrades, and instruction execution of solana programs

### differences from ethereum
**ethereum**:<br>
only smart contracts have storage and they naturally have full control over that storage.

**solana**:<br>
*any* account can store state, but the storage for smart contracts is only used to store immutable byte code -- the actual state is stored in a separate *data account*.