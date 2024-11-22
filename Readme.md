# Photo Synchronization Tool

## Overview

The Photo Synchronization Tool helps you and your friends compile a complete collection of photos organized by date. Each day's photos are stored in folders identified by the date (e.g., "2024-04-15"). Each photo is uniquely identified by a hash value. The tool identifies which photos are missing from your collection and indicates which friend has them.

## Requirements

1. **Photo Identification**: Each photo is uniquely identified by a hash value. The program simulates this using unique identifiers for each photo.

2. **Incremental Synchronization Algorithm**:

   - Identifies differences between your photo collection and those of your friends.
   - Calculates a hash for each day's collection to quickly determine if synchronization is needed.
   - If hashes differ, it identifies specific photo identifiers that are missing from your collection.

3. **Efficiency Considerations**: Uses hash comparisons to minimize computational overhead by avoiding unnecessary calculations.

4. **Output**: Returns a list of identifiers missing from your collection, along with a reference to which friend's collection they can be found in.

5. **Test Cases**: Includes scenarios with different overlaps between collections to test both hash comparison and identification of missing photos.

## Project Structure

```
photo_sync/
├── src/
│   ├── lib.rs      // Library file containing core functionality
│   ├── main.rs     // Main file for running the program
├── Cargo.toml      // Dependencies and project metadata
└── README.md       // Project documentation`
```

## Implementation Details

### 1. `lib.rs`

In `lib.rs`, we define the main structures and functions:

- **Structs**:
  - `DayPhotos`: Represents the photo collections for a specific day.
- **Functions**:
  - `calculate_photo_hash()`: Computes the SHA-256 hash for a given set of photo identifiers.
  - `calculate_total_hash()`: Computes the SHA-256 hash for a given total set of photo identifiers.
  - `sync_photos()`: Compares the collections and identifies missing photos.

### 2. `main.rs`

In `main.rs`, we import the library functions and use them to process sample data:

- Creates an instance of `DayPhotos` with sample data.
- Calls `sync_photos()` to identify missing photos and prints the results.

### 3. Testing

Tests are included in `lib.rs` to validate the functionality of the synchronization algorithm:

- A test case checks various scenarios to ensure that the program correctly identifies missing photos.

## Dependencies

This project uses the following crates:

- `serde`: For serialization and deserialization (if needed).
- `sha2`: For calculating SHA-256 hashes.
- `std`: For standard library functionalities.

Make sure to include these dependencies in your `Cargo.toml` file:

```toml
[dependencies]
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"] }
```
