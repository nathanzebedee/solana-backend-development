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

### program architecture (w/out anchor)
programs are composed of distinct modules:
* `entrypoint.rs` - contains the `entrypoint` function
* `processor.rs` - contains the `process_instruction` function
* `instruction.rs` - contains the `Instruction` enum
* `state.rs` - contains the `State` struct
* `error.rs` - contains the `ProgramError` enum

