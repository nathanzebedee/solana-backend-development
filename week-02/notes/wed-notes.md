# 11/2/2022 - solana programs

### solana programs overview
* programs process instructions from both end users and other programs
* all programs are *stateless*, as in they interact with data stored in separate accounts
* programs themselves are simply accounts marked `executable` that contain the program code (logic, not data)
* all programs are owned by the **BPF Loader** and executed by the solana runtime (sealevel)
* a solana program can be written in any language that compiles to BPF bytecode - but the solana team recommends Rust
* all programs have a single entry point where instruction processing takes place: `process_instruction`
* instructions passed into a program always contain the following parameters:
    * `program_id: pubkey` - the program's pubkey
    * `accounts: &[AccountInfo]`-  the accounts the instruction is operating on
    * `instruction_data: &[u8]` - the instruction data

### transactions
clients can invoke programs by submitting a transaction to a cluster. a single transaction can include many instructions, and each instruction can invoke a different program. 

when a transaction is submitted, the solana runtime processes each instruction sequentially. if any part of an instruction fails, the entire transaction fails and all state changes are reverted.

### instruction data
transactions contain *instructions* in the form of a byte array `&[u8]`. the contents of the byte array specify which operations the program should perform, and any information the program may need beyond what the accounts contain. 

### program architecture (w/out anchor)
programs are composed of distinct modules:
* `entrypoint.rs` - contains the `entrypoint` function
* `processor.rs` - contains the `process_instruction` function
* `instruction.rs` - contains the `Instruction` enum
* `state.rs` - contains the `State` struct
* `error.rs` - contains the `ProgramError` enum

### programs and state
in order for a program to handle state, we must create other non-executable accounts and set the program as the owner of those accounts. if the program is the owner, it can write to the account's data.

### program id
the instruction's `program_id` specifies the public key of the program being invoked. though programs are stateless, they can inquire about the ownership of a provided account that it is to attempting to interact with.

### generic program flow
1. serialized arguments (accounts, signers, instructions) are received by the `entrypoint!` macro
2. the entrypoint forwards the arguments to the processor module
3. the `processor` invokes the instruction module to decode the instruction data
4. using the decoded data, the processor decides which function to use to process the specific request
5. the processor may use the `state` module to encode the state into or decode the state of an account which had been passed into the entrypoint or can be derived programmatically
6. if an error occurs at any point, execution stops and the program reverts with a general or specific error code

### generic client flow
1. load interface description language (IDL)
2. connect to the network
3. assemble instruction
4. submit instruction (via RPC call)
5. read modified account state (via RPC call)

