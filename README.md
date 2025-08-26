## Summary

There's a critical buffer management bug in the CBOR encoding implementation that occurs when encoding parameters with a very specific size and complexity. The bug manifests as RangeError: Offset is outside the bounds of the DataView during parameter serialization.

## Core Problem

This bug is triggered by a buffer boundary condition where:

1. The initial CBOR buffer is allocated with a fixed size (2048 bytes)
2. Complex nested parameter structures fill the buffer to exactly its capacity
3. When the encoder attempts to write additional header bytes, it exceeds the buffer bounds
4. The buffer expansion logic fails to trigger at this specific boundary

## Reproduction Difficulty

This bug is extremely sensitive to parameter structure and size:

-   Any reduction in parameter complexity → buffer usage drops below the boundary → works fine
-   Any addition to parameter complexity → buffer usage exceeds boundary, triggering proper expansion → works fine
-   Only this exact configuration → hits the precise boundary where expansion fails

## Minimal Reproduction Case

This repo is a reproduction case of the error and contains:

-   A Rust canister with the exact type signature that triggers the bug
-   TypeScript test that calls the canister with the problematic arguments
-   The same setup in Rust CDK and Azle cause the same problem, confirming this is an @dfinity/agent issue

## Technical Details

-   Problematic payload size: ~1968 bytes of argument data
-   Buffer size: 2048 bytes (exactly filled including CBOR overhead)
-   Failure point: Attempt to write at offset 2048 in a 0-2047 buffer
-   Root cause: Buffer expansion logic doesn't seem to handle this specific boundary condition
