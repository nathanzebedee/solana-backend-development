# token program

### token program
the token program is part of the *solana program library* which is a collection of on-chain programs targeting the sealevel parallel runtime, covering a number of areas such as:

* tokens
* governance
* name service
* token swaps
* lending

the solana token program is used to create tokens with functionality similar to ERC20 and ERC721 (NFT) standards.

### comparison to ethereum
ethereum does not have any built in support for tokens, they rely on:

* a standard to be followed
* libraries to be created by third parties to provide implementations

if you want to create a token on ethereum, you wil need to write and deploy a contract.

### creating a token
1. we must create a *token mint account* which contains details about the token and who can do administrative functions. think of this as the "blueprint" for the token.
2. we must create a *token account* which acts like the purse for the token. the token account points to the token mint account, and holds instances of those tokens.
3. after having created the *token mint account* (blueprint) and the *token account* (purse), we can now mint tokens into the token account and then transfer the tokens to other users.

### transferring tokens
when we transfer tokens, we invoke the `process_tranfer` function. it takes three parameters:

* source account
* destination account
* amount

the function also does some checks:

* neither source or destination accounts are frozen
* the source account's mint and destination account's mint match
* the transferred `amount` is no more than the source account's balance

this function transfers a certain amount of tokens from a source account to a destination account.

note: the source and destination accounts can be the same.

### associated token account program
in order for a token transfer to succeed, the recipient must already have a token account for the mint of token being sent. The Associated Token Account program allows the sender to create the associated token account for the receiver, so the token transfer just works.

