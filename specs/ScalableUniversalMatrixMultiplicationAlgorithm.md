# SUMMA (Scalable Universal Matrix Multiplication Algorithm)

## Purpose

SUMMA is a distributed matrix multiplication algorithm used to compute:

```text
C = A × B
```

when the matrices are too large to fit on a single machine.

The algorithm distributes the matrices across a grid of processors and computes the result collaboratively while minimizing communication overhead.

---

## Processor Layout

Processors are arranged in a 2D grid.

Example: 4 processors in a 2×2 grid.

```text
P00 P01
P10 P11
```

Each processor owns a block of matrices A and B.

```text
A = | A00 A01 |
    | A10 A11 |

B = | B00 B01 |
    | B10 B11 |
```

Processor ownership:

```text
P00 owns A00, B00
P01 owns A01, B01
P10 owns A10, B10
P11 owns A11, B11
```

---

## Core Idea

Each block of C is computed as a sum of block products.

For example:

```text
C00 = A00 × B00 + A01 × B10
C01 = A00 × B01 + A01 × B11
C10 = A10 × B00 + A11 × B10
C11 = A10 × B01 + A11 × B11
```

Rather than moving entire matrices around, SUMMA computes one term of these sums at a time.

---

## Iterative Computation

SUMMA proceeds through the shared dimension one block at a time.

For each iteration k:

1. Broadcast the A blocks from block-column k along each processor row.
2. Broadcast the B blocks from block-row k along each processor column.
3. Each processor multiplies the received blocks and accumulates the result into its local C block.

Pseudocode:

```text
for k = 0 .. numBlockColumns-1

    broadcast A[i][k] across processor row i

    broadcast B[k][j] across processor column j

    C[i][j] += A[i][k] × B[k][j]

end
```

---

## Example: 2×2 Processor Grid

### Iteration k = 0

Broadcast:

```text
A00 -> P00, P01
A10 -> P10, P11

B00 -> P00, P10
B01 -> P01, P11
```

Compute:

```text
P00: C00 += A00 × B00
P01: C01 += A00 × B01
P10: C10 += A10 × B00
P11: C11 += A10 × B01
```

### Iteration k = 1

Broadcast:

```text
A01 -> P00, P01
A11 -> P10, P11

B10 -> P00, P10
B11 -> P01, P11
```

Compute:

```text
P00: C00 += A01 × B10
P01: C01 += A01 × B11
P10: C10 += A11 × B10
P11: C11 += A11 × B11
```

After the second iteration, all blocks of C are complete.

---

## Why the k Iterations Exist

A matrix multiplication is fundamentally a sum of products.

For a single result block:

```text
Cij = Ai0×B0j + Ai1×B1j + Ai2×B2j + ...
```

Each iteration computes one term of that sum.

For example:

```text
Iteration 0:
Cij += Ai0 × B0j

Iteration 1:
Cij += Ai1 × B1j

Iteration 2:
Cij += Ai2 × B2j
```

After all iterations, every required product has been accumulated.

---

## Result Assembly

One important property of SUMMA is that no final reduction is required.

Each processor owns exactly one block of C and computes all contributions for that block.

Example:

```text
P00 computes C00
P01 computes C01
P10 computes C10
P11 computes C11
```

At the end:

```text
P00 contains the final C00
P01 contains the final C01
P10 contains the final C10
P11 contains the final C11
```

The complete matrix is simply the collection of these blocks:

```text
C = | C00 C01 |
    | C10 C11 |
```

No processor needs to merge partial results from other processors.

---

## Communication Pattern

At each iteration:

- A blocks are broadcast horizontally across processor rows.
- B blocks are broadcast vertically across processor columns.

```text
Row broadcast:

Aik ----> ----> ---->

Column broadcast:

 |
 v
 |
 v
Bkj
```

This communication pattern is regular, predictable, and scales well.

---

## Advantages

### Simplicity

The algorithm is easy to understand and implement.

```text
broadcast A
broadcast B
local multiply
accumulate
repeat
```

### Scalability

Computation and memory are distributed across processors.

As more processors are added, larger matrices can be handled efficiently.

### No Final Reduction

Every processor fully owns its result block.

This avoids a costly merge phase at the end of the computation.

### Flexible Processor Grids

SUMMA works with rectangular and square processor grids.

For example:

```text
2×2
4×8
8×16
```

---

## Summary

SUMMA distributes matrices across a 2D processor grid and computes matrix multiplication one block-product term at a time.

For each iteration:

1. Broadcast the required A blocks across rows.
2. Broadcast the required B blocks across columns.
3. Compute:
   ```text
   Cij += Aik × Bkj
   ```
4. Repeat for all k.

Because each processor owns and updates its own result block throughout the computation, the final matrix is already assembled when the last iteration completes.