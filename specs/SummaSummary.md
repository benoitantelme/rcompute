# SUMMA Matrix Multiplication: Planning the Calculation Steps

## Overview

SUMMA (Scalable Universal Matrix Multiplication Algorithm) computes:

```text
C = A × B
```

by decomposing the multiplication into a sequence of **outer-product updates**.

The key idea is:

```text
C = Σ A(:,k) × B(k,:)
```

where:

- `A(:,k)` = column `k` of `A`
- `B(k,:)` = row `k` of `B`

At each iteration `k`:

1. Broadcast column/panel `k` of `A`
2. Broadcast row/panel `k` of `B`
3. Compute the outer product
4. Accumulate the result into `C`

---

# 2×2 Example

Let:

```text
A = [ a11  a12 ]
    [ a21  a22 ]

B = [ b11  b12 ]
    [ b21  b22 ]
```

SUMMA performs **2 iterations**.

## Step 1 (k = 1)

Use:

```text
Column 1 of A:

[ a11 ]
[ a21 ]

Row 1 of B:

[ b11  b12 ]
```

Compute the outer product:

```text
[ a11*b11  a11*b12 ]
[ a21*b11  a21*b12 ]
```

Initialize:

```text
C =
[ a11*b11  a11*b12 ]
[ a21*b11  a21*b12 ]
```

---

## Step 2 (k = 2)

Use:

```text
Column 2 of A:

[ a12 ]
[ a22 ]

Row 2 of B:

[ b21  b22 ]
```

Compute the outer product:

```text
[ a12*b21  a12*b22 ]
[ a22*b21  a22*b22 ]
```

Accumulate:

```text
C =
[ a11*b11 + a12*b21   a11*b12 + a12*b22 ]
[ a21*b11 + a22*b21   a21*b12 + a22*b22 ]
```

Result complete.

---

# 3×3 Example

Let:

```text
C = A × B
```

SUMMA performs **3 iterations**.

## Step 1 (k = 1)

Compute:

```text
A(:,1) × B(1,:)
```

Result:

```text
[ a11*b11  a11*b12  a11*b13 ]
[ a21*b11  a21*b12  a21*b13 ]
[ a31*b11  a31*b12  a31*b13 ]
```
*---

## Step 2 (k = *)

Compute:

```text
A(:,2) × B*2,:)
```

Contribution:

```text
[ a12*b21  a12*b22  a12*b23 ]
[ a22*b21  a22*b22  a22*b23 ]
[ a32*b21  a32*b22  a32*b23 ]
```

Add to the current result.

---

## Step 3 (k = 3)

Compute:

```text
A(:,3) × B(3,:)
```

Contribution:

```text
[ a13*b31  a13*b32  a13*b33 ]
[ a23*b31  a23*b32  a23*b33 ]
[ a33*b31  a33*b32  a33*b33 ]
```
*Add to the current result.

---

#* Final Result

```text
C =
A(:,1)×**1,:)
+
A(:,2)×B(2,:)
+
A(:,3)×B(3,:)
```

For example:

```text
c12 =
a11*b12 +
a12*b22 +
a13*b32
```

which matches the classical matrix multiplication formula.

---

# General n×n Case

For matrices:

```text
A, B ∈ R(n×n)
```

SUMMA performs exactly **n iterations**.

## Iteration k

### 1. Broadcast

```text
A(:,k)
B(k,:)
```

### 2. Compute Update

```text
Uk = A(:,k) × B(k,:)
```

### 3. Accumulate

```text
C = C + Uk
```

---

## Final Expression

```text
C = Σk A(:,k) × B(k,:)
```

Element-wise:

```text
c(i,j) = Σk a(i,k) * b(k,j)
```

This is exactly the standard matrix multiplication formula.

---

# Distributed SUMMA Using a p×p Processor Grid

Assume matrices are block-distributed across processors.

Processor `P(i,j)` owns:

```text
A(i,j)
B(i,j)
C(i,j)
```

For iteration `k`:

1. Broadcast block `A(i,k)` across processor row `i`
2. Broadcast block `B(k,j)` across processor column `j`
3. Perform the local update:

```text
C(i,j) = C(i,j) + A(i,k) × B(k,j)
```

---

## Example: 3×3 Processor Grid

```text
Iteration k

Broadcast A(*,k) across rows

P00  <--  P01  <--  P02
 |            |          |
 v            v          v
P10  <--  P11  <--  P12
 |            |          |
 v            v          v
P20  <--  P21  <--  P22

Broadcast B(k,*) down columns
```

Each processor receives:

```text
A-panel
B-panel
```

Then computes:

```text
local_C += local_A × local_B
```

---

# Planning View

The SUMMA execution plan is:

```text
for k = 1..n

    Broadcast A(:,k)

    Broadcast B(k,:)

    C += A(:,k) × B(k,:)

end
```

---

# Timeline Examples

## 2×2

```text
Step 1:
    Broadcast A(:,1)
    Broadcast B(1,:)
    C += A(:,1) × B(1,:)

Step 2:
    Broadcast A(:,2)
    Broadcast B(2,:)
    C += A(:,2) × B(2,:)
```
IE:
```text
Step 1:
    Broadcast A(11)
    Broadcast A(21)
    Broadcast B(11) Broadcast B(12)
    C += A(n1) × B(1n)

Step 2:
    Broadcast A(12)
    Broadcast A(22)
    Broadcast B(21) Broadcast B(22)
    C += A(n2) × B(2n)
```

---

## 3×3

```text
Step 1:
    Broadcast A(:,1)
    Broadcast B(1,:)
    Update C

Step 2:
    Broadcast A(:,2)
    Broadcast B(2,:)
    Update C

Step 3:
    Broadcast A(:,3)
    Broadcast B(3,:)
    Update C
```
IE:
```text
Step 1:
    Broadcast A(n1)
    Broadcast B(1n)
    Update C

Step 2:
    Broadcast A(n2)
    Broadcast B(2n)
    Update C

Step 3:
    Broadcast A(n3)
    Broadcast B(3n)
    Update C
```

---

## n×n

```text
Step 1:
    Broadcast A(:,1)
    Broadcast B(1,:)
    Update C

Step 2:
    Broadcast A(:,2)
    Broadcast B(2,:)
    Update C

...

Step n:
    Broadcast A(:,n)
    Broadcast B(n,:)
    Update C
```
IE:
```text
Step 1:
    Broadcast A(n1)
    Broadcast B(1n)
    Update C

Step 2:
    Broadcast A(n2)
    Broadcast B(2n)
    Update C

...

Step n:
    Broadcast A(nn)
    Broadcast B(nn)
    Update C
```

---

# Key Insight

SUMMA transforms matrix multiplication into a sequence of independent outer-product updates:

```text
C = Σ A(:,k) × B(k,:)
```

Therefore:

```text
2×2 matrix multiplication -> 2 SUMMA steps
3×3 matrix multiplication -> 3 SUMMA steps
n×n matrix multiplication -> n SUMMA steps
```

Each step consists of:

```text
Broadcast A panel
Broadcast B panel
Local multiplication
Accumulate into C
```

This decomposition makes SUMMA simple, scalable, and highly efficient on distributed-memory parallel systems.