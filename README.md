# Pinocchio NFT Minter

A simple Solana NFT minter built with Anchor and a web frontend.

## Features
- Mint NFTs on the Solana blockchain
- Anchor-based smart contract
- Web frontend for easy interaction

## Prerequisites
- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://book.anchor-lang.com/chapter_1/installation.html)

## Setup

1. Clone the repository:
   ```bash
   git clone <repo-url>
   cd nf-minter
   ```
2. Install dependencies:
   ```bash
   yarn install
   # or
   npm install
   ```
3. Build the Solana program:
   ```bash
   cd programs/nf-minter
   anchor build
   cd ../../
   ```
4. Start the local Solana validator (optional for local testing):
   ```bash
   solana-test-validator
   ```
5. Deploy the program:
   ```bash
   anchor deploy
   ```
6. Run the frontend (if available):
   ```bash
   cd app
   yarn install
   yarn dev
   ```

## Usage
- Use the web app to mint NFTs, or interact with the program using Anchor CLI/scripts.

## License
MIT