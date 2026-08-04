# Dense Matrix Multiplication

## Overview

Dense matrix multiplication is the operation of multiplying two matrices where most or all entries contain non-zero values.

Given:

- Matrix A with dimensions `m × n`
- Matrix B with dimensions `n × p`

Their product is:

```text
C = A × B
```

where C has dimensions:

```text
m × p
```

Matrix multiplication is only defined when:

```text
number of columns in A = number of rows in B
```

---

## How Matrix Multiplication Works

Each element of the resulting matrix is computed as the dot product of:

- one row from matrix A
- one column from matrix B

### Formula

The value at row `i` and column `j` of the result matrix is:

```text
C[i][j] = Σ(A[i][k] × B[k][j])
          for k = 0 to n-1
```

This means:

1. Select row `i` from A
2. Select column `j` from B
3. Multiply corresponding elements
4. Sum the products

---

## Example

Given:

```text
A =

| 1 2 |
| 3 4 |
```

and

```text
B =

| 5 6 |
| 7 8 |
```

The resulting matrix is:

```text
C = A × B
```

### Compute C[0][0]

Row 0 of A:

```text
[1, 2]
```

Column 0 of B:

```text
[5, 7]
```

Dot product:

```text
1 × 5 + 2 × 7 = 19
```

### Compute C[0][1]

```text
1 × 6 + 2 × 8 = 22
```

### Compute C[1][0]

```text
3 × 5 + 4 × 7 = 43
```

### Compute C[1][1]

```text
3 × 6 + 4 × 8 = 50
```

Result:

```text
| 19 22 |
| 43 50 |
```

---

## Dot Product

Matrix multiplication is fundamentally built from dot products.

Given two vectors:

```text
a = [a1, a2, ..., an]
b = [b1, b2, ..., bn]
```

their dot product is:

```text
a · b = a1×b1 + a2×b2 + ... + an×bn
```

Example:

```text
[1,2,3] · [4,5,6]

= 1×4 + 2×5 + 3×6

= 32
```

Every element of the output matrix is one dot product.

---

## Naive Algorithm

The classical implementation uses three nested loops.

```text
for i = 0 .. m-1
    for j = 0 .. p-1
        C[i][j] = 0

        for k = 0 .. n-1
            C[i][j] += A[i][k] * B[k][j]
```

The outer loops iterate over every output element.

The inner loop computes the dot product for that element.

---

## Computational Complexity

### General Case

For:

```text
A: m × n
B: n × p
```

the algorithm performs approximately:

```text
m × n × p
```

multiply-add operations.

Time complexity:

```text
O(m × n × p)
```

### Square Matrix Case

If both matrices are:

```text
n × n
```

then:

```text
O(n³)
```

Examples:

| Matrix Size | Approximate Operations |
|-------------|----------------------:|
| 100 × 100 | 1 million |
| 1,000 × 1,000 | 1 billion |
| 10,000 × 10,000 | 1 trillion |

---

## Space Complexity

The result matrix requires:

```text
m × p
```

elements.

Additional storage required:

```text
O(m × p)
```

For square matrices:

```text
O(n²)
```

Note that the input matrices themselves are typically not counted when analyzing the additional memory required by the algorithm.

---

## Blocking (Tiled Multiplication)

High-performance implementations usually divide matrices into blocks (tiles).

Instead of operating on individual elements, they operate on submatrices:

```text
A = | A00 A01 |
    | A10 A11 |

B = | B00 B01 |
    | B10 B11 |
```

The computation becomes:

```text
C00 = A00×B00 + A01×B10
C01 = A00×B01 + A01×B11
C10 = A10×B00 + A11×B10
C11 = A10×B01 + A11×B11
```

Benefits:

- Better CPU cache utilization
- Better memory locality
- Better vectorization (SIMD)
- Better distributed scalability

The computational complexity remains:

```text
O(n³)
```

but practical performance improves significantly because the processor spends less time waiting for data from memory.

---

## Dense vs Sparse Matrices

### Dense Matrix

Most entries are non-zero.

Example:

```text
1 2 3
4 5 6
7 8 9
```

### Sparse Matrix

Most entries are zero.

Example:

```text
1 0 0
0 0 5
0 0 0
```

Dense matrix multiplication processes all values.

Sparse matrix multiplication uses specialized algorithms and storage formats to avoid processing zero values whenever possible.

---

## Summary

Dense matrix multiplication computes:

```text
C = A × B
```

where:

```text
C[i][j] = Σ(A[i][k] × B[k][j])
```

Key properties:

- Input dimensions: `m × n` and `n × p`
- Output dimensions: `m × p`
- Each output element is the dot product of a row of A and a column of B
- Classical time complexity: `O(m × n × p)`
- Square matrix complexity: `O(n³)`
- Output space complexity: `O(m × p)`
- High-performance implementations use blocked (tiled) multiplication to improve cache efficiency and scalability