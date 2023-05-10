# Casper Contract Factory

is an example smart contract factory built for the Casper Network.

By [deploying the factory contract]() and [invoking the generate entry point](), you can create a new contract without compiling it to bytecode.

This factory is currently very limited in its functionality, not supporting the inclusion of custom-written functions in the newly generated contracts.

## Prereqs

Follow the [development prerequisites](https://docs.casper.network/developers/prerequisites) and [getting started with rust](https://docs.casper.network/developers/writing-onchain-code/getting-started/).

## Install

First clone the repository:

```bash
git clone https://github.com/dylanireland/casper-contract-factory.git
cd casper-contract-factory/
```

## Generate Keypair

```bash
casper-client keygen keys
```

## Compile Factory Contract

```bash
cd factory/
make prepare
make build-contract
```

The compiled factory contract will be compiled in *./target/wasm32-unknown-unknown/release/contract.wasm*

## Deploy the Factory

```bash
chmod +x deploy.sh
./deploy.sh
```

(Adjust the node address and network to your liking)

## Generate from Factory

Edit *generate.sh* to use your factory's smart contract hash, then run:

```bash
chmod +x generate.sh
./generate.sh
```

