# solana theory

### cryptography
public and private keys are derived from an elliptic curve. Solana uses the EdDSA algorithm, which is a variant of the Ed25519 algorithm. Elliptic curves have a shorter key length for the same amount of security.

in elliptic curve cryptography, the following are derived from each other:
private key > public key > address

### blockchain as a state machine
* messages are in the form of transactions, which represent state transitions.
* a blockchain is a state machine that processes transactions according to the consensus rules
* the blockchain -- chain of blocks -- is secured through cryptography and acts like a public journal of all recorded transactions

assorting transactions in a block might just be a convenient, although arbitrary way of gathering information so that it may be passed between nodes for consensus.

blocks are bundles of transactions; blocks are linked together in a chian via cryptography (hashing). each block references the former block by pointing to its hash value in the block header.

### solana blocks
although solana still uses the "block" terminology to refer to a bundle of transactions, solana blocks are not actually blocks in the traditional sense. 

solana groups nodes in "clusters", creating neighborhoods of validators which submit transactions to the network. 

each block solana block is 10MB; blocks are proposed every 800ms.

### consensus - byzantine fault tolerance
* byzantine fault tolerance (BFT) is a consensus algorithm that can tolerate a certain number of faulty nodes in the network.
* consensus works to solve the "double-spend" problem, which is an issue in digital cash systems where a single digital token can be spent multiple times (this is possible because a digital token consists of a digital file which can be copied or falsified).
* synchronization: a standard of time is difficult to achieve in a decentralized network.

solana's biggest innovation was allowing a decentralized system to order events in a verifiable way -- proof of history.

note: BFT systems tend not to scale well as number of nodes in a network increases - this is because the number of messages between nodes increases exponentially with the number of nodes. this ultimately leads to centralization in systems; if you try to add more nodes to make a network decentralized, you inevitably suffer performance issues which lead to centralization. solana's PoH algorithm solves this problem.

### proof of history
proof of history works in two parts:
* choosing a block producer / leader
* agreeing on which blocks form the canonical chain

*how do we make this selection process fair?*
solana uses proof of stake (PoS) to select block producers. VRFs are used to assign nodes a time slot in which they can propose a block.

issue: **liveness** - what do we do when someone is chosen for a slot, but nothing is produced? what if no producer is chosen? does the blockchain stall? 
* we can have a timeout period. if nothing is produced within this time period, then you can simply move on to the next slot. however, this is only possible if there is a clock that is synchronized across the network (PoH).

we do not need to send transactions to every node such as in a gossip protocol. we can simply send the transactions to the predesignated leader (for a given slot) and the leader can broadcast the block to the rest of the network.

### solana vs hedera
hedera relies upon a medium measure for their idea of time. solana' is much more decentralized. on solana, any node can run its own clock without relying on a central clock. this "clock" is a hashing algorithm - a SHA256 recursive hash: the output from one step forming the input of the next step. 

this is useful because we can always recreate this process by verifying the hash of the previous step. futhermore, you can associate an account with this hash timeline, giving us a way to verify the history of an account.

because you cannot parallelize the hashing process, and can only step through the events sequentially, we can say that the hash values count as a sequence of time and effectively time-stamp events. however, once we have the steps, we can verify the sequence of events in parallel. 

### deterministic leader selection
for every given slot, nodes within the network know whether they are going to be a leader, client, or validator. this allows validators to execute transactions ahead of time and send them to the leader; as validators keep track of time, they can stamp each incoming transaction.

thus, we do not have to wait for blocks to be gossiped between nodes. we can execute transactions in parallel, and then send them to the leader, making for a faster network.

2/3 of validators must vote on a signature of the state for the PoH hash to be canonicalized, and cannot be rolled back.

### solana history
1. Anatoly Yakovenki published the whitepaper in 2017 specifying PoH.
2. the codebase was moved to Rust (from C++) and was named Loom.
3. in Feb 2018, a throughput of 10k transactions per second was achieved.
4. in March 2018, the project was named Solana
5. in July 2018, a testnet of 50 nodes was built and managed up to 250k tps.
6. in december 2018, the testnet was increased to 150 nodes, and the throughput averaged 200k tps, peaking at 500k.

### network outages
on sept 14, 2021, a cohort of bots flooded the solana network. solana engineers were unable to stabilize the network, and as a result the validator community opted to coordinate a restart of the network.

the solution seems to be setting a limit on how much can be forwarded when a node gets behind.

one of the biggest benefits of a blockchain is that even in complete liveness failure, the validators are individually responsible for recovering the state and continuing the chain without relying on a trusted third party.

### crypto economics of an attack
if SOL fees are $0.000005 then 400k transactions ~$2.

essentially, what is has shown is that a cost of ~$7,000 an hour is required to cause an outage on the network.

compare this to the cost of a 51% attack on PoW chains.

that being said, transaction fees can have an effect on the network's ability to be overwhelmed. the outages have resulted because of the network's low-cost transaction fees.

## mempool
the mempool is a queue of transactions that are waiting to be processed. the mempool is a temporary storage for transactions that are waiting to be processed.

### crypto economics

*in a system, what do we want to encourage?*
* trusted execution
* open access
* fast finality
* decentralized control
* inexpensiveness

*things we want to discourage?*
* safety failure
* censorship
* slow finality
* expensiveness

*why do incentives matter?*
* an incentive is any design element of a system that influences the behavior of the system participants by changing the relative costs and benefits of choices those participants may make.
    * rewards
        * block rewards, transaction fees
        * privileges within the system
    * punishments
        * loss of deposits, loss of privileges (PoS)

## governance
"the greatest challenge that new blockchains must solve is not speed or scaling, it is governance." - Kai Sedgwick

governance is the question: "how will our blockchain as a system progress?" (coming to agreements on updating the system)

solana has the feature proposal program (FPP) which allows for the community to propose changes to the system. the FPP is a formal process for proposing, discussing, and implementing changes to the Solana protocol.
