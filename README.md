# Mini Blockchain in Rust

## Overview

This project is a simple implementation of a blockchain in Rust. Each block in the blockchain contains the following:

- **Data**: The content stored in the block
- **Hash**: A unique identifier for the block, generated using the `sha2` hashing algorithm.
- **Previous Hash**: A reference to the hash of the previous block, ensuring the integrity of the chain.
- **Timestamp**: The time when the block was created, recorded as a UNIX timestamp.

The blockchain ensures that the data is tamper-proof by linking each block to its predecessor using hashes.

## Features

1. **Block Structure**:

   - Each block contains:

     - `data`: The block's payload.
     - `hash`: A unique SHA-256 hash of the block.
     - `prev_hash`: The hash of the previous block (or `None` for the first block).
     - `timestamp`: The time of creation in seconds since the UNIX epoch.

2. **Blockchain Structure**:

   - Maintains a vector of blocks.
   - Provides methods to add blocks and print the blockchain.

3. **Cryptographic Hashing**:

   - Uses the `sha2` crate for secure SHA-256 hashing.
   - Combines `data`, `prev_hash`, and `timestamp` to compute a block's hash.

4. **Timestamp**:

   - Captures the exact moment a block is created using Rust's `SystemTime`.

## How It Works

1. **Adding a Block**:

   - When a block is added, its hash is calculated based on:
     - The block's data.
     - The previous block's hash.
     - The block's timestamp.
   - This ensures the uniqueness and immutability of each block.

2. **Integrity Check**:

   - Each block contains a reference to the previous block's hash. If any block is modified, the hashes of all subsequent blocks will no longer match, breaking the chain.

3. **Displaying the Blockchain**:

   - Prints all blocks in the chain with their details.

## Example Output

```plaintext
Block {
data: "First Block",
hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
prev_hash: None,
timestamp: 1704028120
}
Block {
data: "Second Block",
hash: "4c9e07b8d2a2918df198510e23e03234cbb2dc926b89a5f3af446837c6e2f5a4",
prev_hash: Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
timestamp: 1704028121
}
Block {
data: "Third Block",
hash: "3f79bb7b435b05321651daefc8db5a8d27462e4f8d07f178b865ef5a9e28df0d",
prev_hash: Some("4c9e07b8d2a2918df198510e23e03234cbb2dc926b89a5f3af446837c6e2f5a4"),
timestamp: 1704028122
}
```

## Prerequisites

- Rust and Cargo must be installed. [Install Rust here](https://www.rust-lang.org/tools/install).

## Running the Program

1. Clone the repository:

```sh
git clone https://github.com/Obcodelab/mini-blockchain.git
```

2. Navigate to the project directory:

```sh
cd mini-blockchain
```

3. Run the program:

```sh
cargo run
```

## Possible Enhancements

- Add a proof-of-work mechanism for mining blocks.
- Implement block validation to ensure the integrity of the chain.
- Allow the blockchain to persist data by saving and loading from a file.
- Add a network layer for peer-to-peer synchronization.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request with suggestions or improvements.
