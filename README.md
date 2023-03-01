# What's the Difference of Building a Smart Contract, a Parachain & a Parathread?
I will attempt to contrast the development of a smart contract &amp; the development of a parachain/parathread.

## What's a Smart Contract?
An Ethereum smart contract is a computer program that runs on the Ethereum blockchain. It executes automatically when certain conditions are met, and it cannot be changed once deployed. Smart contracts are useful for executing financial transactions, voting, auctions, and more, and they enable new forms of decentralized applications and organizations. Overall, smart contracts increase trust, efficiency, and innovation in a variety of industries.


### How are Smart Contracts Constructed?

Constructing a smart contract involves defining the contract terms, choosing a programming language, developing the smart contract code, testing and debugging it, and deploying it on a blockchain network. Smart contracts are self-executing and self-enforcing contracts that are fully automated and transparent. They eliminate the need for intermediaries and can help to reduce the time and costs associated with traditional contractual agreements. The specific programming language and standards used may vary depending on the blockchain network being used.



## What's a Parachain?

A parachain is a separate blockchain that can communicate with other blockchains and shares security with a relay chain. This means that it can benefit from the security of the relay chain and can communicate with other blockchains that are also connected to the relay chain.

Parachains provide more space and flexibility for developers to create custom features and applications. They can be used to create things like custom monetary systems, voting systems, or governance mechanisms that can be tailored to specific needs.

### Parachain Construction

Building a parachain involves defining what it will do, building the logic it will use to process transactions, connecting it to the Kusama Relay Chain, testing it, and maintaining and upgrading it over time. It's a complex process that requires a good understanding of blockchain development, but it allows developers to build custom blockchains with a lot of flexibility and customization options for their applications.




## What's a parathread?
On the other hand, a parathread is also a separate blockchain that can communicate with other blockchains. However, it is designed to be less expensive to use than a parachain. Parathreads only produce a block when necessary, which means they can save on resources and are better suited for applications that do not require frequent updates.

Both parachains and parathreads provide customization options for developers, allowing them to create new types of applications that were not possible before. However, the main difference is that parachains provide more space and flexibility, while parathreads are more cost-effective and suitable for applications that require less frequent updates.


### Parathread Construction

In summary, constructing a parathread involves defining the use case, developing the code, connecting it to the main blockchain network, allocating resources, testing, and launching it. Parathreads provide an additional layer of scalability to blockchain networks, allowing for specialized use cases without impacting the performance of the main blockchain. The specific instructions and protocols for constructing a parathread may vary depending on the blockchain platform being used.

### Disclaimer: 
All code pertaining to substrate based parachains, parathreads were intended for reference of what is both possible and is not an endorsement of any particular software.

### Links to Sources.

Builder's Starter Guide - https://guide.kusama.network/docs/build-build-with-polkadot

Collator - https://guide.kusama.network/docs/learn-collator

Polkadot GitHub - https://github.com/paritytech/polkadot

Connect A Realy and Parachain (Tutorial) - https://docs.substrate.io/tutorials/connect-relay-and-parachains/prepare-a-local-relay-chain/
