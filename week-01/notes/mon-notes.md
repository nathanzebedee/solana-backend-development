# 10/24/2022

### centralized systems
by closing a system, you can achieve security -- layers of access control to restrict access to the system and/or prevent fraudulent activity

### decentralized systems
in contrast, decentralized systems are open to the public, and therefore anyone can participate in the network. 

in a decentralized system, authority is moved to the end user, rather than a centralized authority.

centralized systems can often be easier to implement and control, however, decentralized systems are often more flexible and resilient.

### components of a blockchain
**gossip network**: 
a decentralized network of nodes that communicate with each other to share information. gossip networks are used to propagate information throughout the network.

**shared public ledger**: 
a distributed database that stores the state of the network. the ledger is shared among all nodes in the network, and is updated by consensus.

**cryptography**: 
cryptographic algorithms are used to secure the network and verify transactions. because we do not necessarily know who we are transacting with over a decentralized network, we need to be able to verify that the transaction is valid using cryptography.

### goals of decentralization
* participation
* diversity
* conflict resolution
* flexibility
* empowerment
* transparency
* censorship resistance

### history of cryptographic systems
1970s:<br>
*How do you send a secure message over an insecure network?*
* **symmetric cryptography**: 
    * single key for encrypting and decrypting data
    * issue: key management. how do you share this key in a safe way, without it falling into the wrong hands?
* **asymmetric cryptography**: 
    * two keys (public & private), one for encrypting (public) and one for decrypting data (private). the two keys are mathematically related, but not the same.
    * improvement: given a private key, you can deterministically derive a public key (but not vice versa). this means that you can share your public key with anyone, without worrying about it falling into the wrong hands.

*How do I know that the message was sent by the person I think it was sent by?*
* **digital signatures**: 
    * a digital signature is a mathematical technique that allows a user to prove that they are the owner of a private key, without revealing the private key itself.
    * encrypt > sign > decrypt > verify: data is encrypted using the private key, and decrypted using the respective public key -- therefore we can make assumptions about who sent this message.
* **hashing**: 
    * a hash function is a mathematical function that takes an arbitrary amount of data and returns a fixed-length, hash value. the hash value is deterministic, meaning that the same input will always produce the same output. the hash value is also one-way, meaning that it is impossible to derive the original input from the hash value.
    * hash functions are used to verify the integrity of data. if the hash value of the data is known, you can verify that the data has not been tampered with by recalculating the hash value of the data and comparing it to the original hash value. for this reason, a hash value can be used as a digital fingerprint for data.
* **consensus**: 
    * a consensus algorithm is a set of rules that nodes in a network must follow to reach agreement on the state of the network. consensus algorithms are used to ensure that all nodes in the network are in agreement on the state of the network, and that the network is operating as expected.
    * consensus algorithms are used to ensure that all nodes in the network are in agreement on the state of the network, and that the network is operating as expected.

1980s:<br>
*How did the aforementioned cryptographic innovations manifest themselves?*
* E-Cash (1983, David Chaum): blind signature for digital transactions
* Combatting Junk Mail (1992, Cynthia Dwork, Moni Naor): proof of work for spam prevention
* Bitcoin (2008, Satoshi Nakamoto): proof of work for digital currency which solved the double spending problem

### verifiable random delay functions
a verifiable random delay function (VRF) is a cryptographic primitive that maps inputs to verifiable pseudorandom outputs. 
### four properties of digital signatures
* authentic
* unfalsifiable
* non-reusable
* tamper-evident

### references
"The Internet of Money" - Andreas Antonopoulos
