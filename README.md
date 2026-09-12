# rcompute
Distributed computing in rust

## Description

Rust distributed matrix multiplication calculator, using SUMMA algorithm.\
Assigning workers for the calculation grid of size n*n (no worker queue).

## Project Roadmap

- [x] **Phase One**: Initial setup
  - [x] **1** — Rust, Cargo and CI setup
  - [x] **2** — Minimal orchestrator and workers
  - [x] **3** — Orchestrator and worker setup: configuration, availability, threshold, ...
  - [x] **4** — Worker discovery and lifecycle
  - [x] **5** — Refactoring as an actors like system for orchestrator and workers (TBC)
  - [x] **6** — Observability: as feature and for proper testing as well
  - [x] **7** — Timeouts and deadlines
  - [x] **8** — Task management and lifecycle

- [x] **Phase Two**: Enrichment
  - [x] **1** — Adding more tests
  - [x] **2** — Additional specifications and algorithm
  - [x] **3** — Better configuration and code/apis

- [ ] **Phase Three**: Matrix Multiplication
  - [x] **1** — Specification, algorithm, ''map reduce'', ...
  - [x] **2** — Single thread execution for unit tests setup
  - [x] **3** — SUMMA computation
  - [x] **4** — Cleanup of old code
  - [x] **5** — Update with timeouts
  - [ ] **6** — Failure management
  - [ ] **7** — ......

## Local Development

### Using cargo

```bash
cargo test  # test only 
cargo build # compile and bundle to /target
cargo run
```


## More details

- [Main (old) specification](specs/DistributedComputing.md)
- [Matrix Multiplication Basics](specs/MatrixMultiplicationBasics.md)
- [Scalable Universal Matrix Multiplication Algorithm](specs/ScalableUniversalMatrixMultiplicationAlgorithm.md)
- [SUMMA Summary](specs/SummaSummary.md)
- [SUMMA Algorithm](specs/SummaAlgo.md)
