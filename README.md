# rcompute
Distributed computing in rust


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
  - [ ] **1** — Adding more tests
  - [ ] **2** — Better configuration and code/apis
  - [ ] **3** — ......

- [ ] **Phase Three**: Matrix Multiplication
  - [x] **1** — Specification, algorithm, ''map reduce'', ...
  - [ ] **2** — Introducing calculations, different sub tasks, sub results, ...
  - [ ] **3** — Calculation lifecycle
  - [ ] **4** — Calculation failure management
  - [ ] **5** — ......

## Local Development

### Using cargo

```bash
cargo test  # test only 
cargo build # compile and bundle to /target
cargo run
```


## More details

- [Main specification](specs/DistributedComputing.md)
- [Matrix Multiplication Basics](specs/MatrixMultiplicationBasics.md)
- [Scalable Universal Matrix Multiplication Algorithm](specs/ScalableUniversalMatrixMultiplicationAlgorithm.md)
