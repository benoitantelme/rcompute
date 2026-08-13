# SUMMA Matrix Multiplication: Algorithm

## Orchestrator

```text
Input:
    A[n,n]
    B[n,n]

Initialize n × n calculation nodes

for each node(i,j):
    send n
    send coordinates (i,j)

for k = 0 to n-1:

    broadcast column A(:,k)

    broadcast row B(k,:)

wait until all nodes return their C(i,j)

assemble matrix C from all results
```

## Node

```text
receive n
receive coordinates (i,j)

c = 0

for k = 0 to n-1:

    receive a = A(i,k)

    receive b = B(k,j)

    c = c + a * b

send c back to orchestrator
```