# Hello Anchor Program

This Solana program uses Anchor and the SPL token metadata library to create and manage NFTs. It provides functionalities for minting NFTs, locking them in a vault, and transferring rental fees. This README file provides an overview of the program and how to use it.

## Table of Contents

- [Installation]
- [Usage]
  - [Minting NFTs]
  - [Swapping SOL for NFTs]
  - [Locking NFTs in vault]
  - [Transferring Rental Fees]


## Installation

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor](https://www.anchor-lang.com/docs/installation)

### Clone the repository

Clone the repository:

```sh
   git clone https://github.com/donjne/metaplexnft_program.git
   cd hello-anchor
```

### Build the Project

Build the project using anchor:

```shell
anchor build
```

### Deploy the project

Build the project using anchor:

```shell
anchor deploy
```

## Usage

### Minting NFTs

To mint an NFT, call the init_nft instruction with the required parameters: name, symbol, and uri.

```rust
pub fn init_nft(
    ctx: Context<MintNFT>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()>
```

### Locking NFTs

To lock an NFT in the vault, call the lock_nft instruction with the required parameter: nft_a.

```rust
pub fn lock_nft(ctx: Context<LockNFT>, nft_a: u64) -> Result<()>
```

### Transferring Rental Fees

To transfer rental fees, call the transfer_rental_fee instruction with the required parameter: rental_fee.

```rust
pub fn transfer_rental_fee(ctx: Context<LockNFT>, rental_fee: u64) -> Result<()>
```
