# What's the Difference of Building a Ethereum Smart Contract, Substrate Smart Contract, a Parachain & a Parathread?
I will attempt to contrast the development of a smart contract &amp; the development of a parachain/parathread.

## Our contenders

* Ethereum Smart Contracts: Ethereum is a blockchain platform that allows developers to build decentralized applications (dApps) using smart contracts. Smart contracts are self-executing programs that automatically enforce the rules and logic of an agreement between parties. Ethereum's smart contracts are written in Solidity and are executed on the Ethereum Virtual Machine (EVM).

* Substrate Smart Contracts: Substrate is a blockchain development framework that allows developers to build their own custom blockchains. It includes a modular architecture that allows developers to choose which components they want to include in their blockchain. Substrate smart contracts are written in the Rust programming language and are executed using the Ink! smart contract language.

* Parachains and Parathreads: Parachains and parathreads are components of the Polkadot network, which is a multi-chain network that allows for interoperability between different blockchains. Parachains are independent blockchains that are connected to the Polkadot network, while parathreads are lighter-weight blockchains that can run on the Polkadot network without requiring a full parachain slot. Parachains and parathreads can have their own smart contracts, written in any programming language that can be compiled to WebAssembly.

### How do they differe in how they're constructed.

* Ethereum Smart Contracts: Ethereum smart contracts are constructed using the Solidity programming language. Solidity is a high-level language that is similar to JavaScript and is specifically designed for writing smart contracts. Once a smart contract is written, it is compiled into bytecode that can be executed on the Ethereum Virtual Machine (EVM), which is a sandboxed environment that allows the contract to be executed on the Ethereum network.

* Substrate Smart Contracts: Substrate smart contracts are constructed using the Ink! smart contract language, which is a Rust-based language specifically designed for writing smart contracts on the Substrate blockchain development framework. Ink! provides a high-level language that is easy to use and includes features such as automatic memory management, type safety, and built-in testing tools. Once a smart contract is written in Ink!, it is compiled into WebAssembly bytecode that can be executed on the Substrate runtime environment.

* Parachains and Parathreads: Parachains and parathreads are constructed using any programming language that can be compiled into WebAssembly bytecode. This includes languages such as Rust, C++, and AssemblyScript. Once a smart contract is written in one of these languages, it is compiled into WebAssembly bytecode and uploaded to the Polkadot network. The contract can then be executed on the Polkadot network using the WebAssembly runtime environment.

-- In summary, Ethereum smart contracts are constructed using the Solidity programming language and are executed on the Ethereum Virtual Machine. Substrate smart contracts are constructed using the Ink! smart contract language and are executed on the Substrate runtime environment. Parachains and parathreads can be constructed using any programming language that can be compiled into WebAssembly bytecode and are executed on the WebAssembly runtime environment on the Polkadot network.


# Discoveries from experiment

Overall, the choice between developing a smart contract or a parachain will depend on the specific use case and requirements of the project. Smart contracts are more suitable for simple use cases and limited budgets, while parachains are better suited for more complex use cases with greater customization needs and larger budgets.


# Pitfalls & Pain Points





### Disclaimer: 
All code pertaining to substrate based parachains, parathreads were intended for reference of what is both possible and is not an endorsement of any particular software.

### Links to Sources.

Builder's Starter Guide - https://guide.kusama.network/docs/build-build-with-polkadot

Collator - https://guide.kusama.network/docs/learn-collator

Polkadot GitHub - https://github.com/paritytech/polkadot

Connect A Realy and Parachain (Tutorial) - https://docs.substrate.io/tutorials/connect-relay-and-parachains/prepare-a-local-relay-chain/

Pay as You Go (Parathread Explaination) - https://medium.com/polkadot-network/parathreads-pay-as-you-go-parachains-7440d23dde06
