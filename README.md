# rcompute
Distributed computing in rust


## Project Roadmap

- [x] **Phase 1** — Rust, Cargo and CI setup
- [x] **Phase 2** — Minimal orchestrator and workers
- [x] **Phase 3** — Orchestrator and worker setup: configuration, availability, threshold, ...
- [x] **Phase 4** — Worker discovery and lifecycle
- [x] **Phase 5** — Refactoring as an actors like system for orchestrator and workers (TBC)
- [X] **Phase 6** — Observability: as feature and for proper testing as well
- [ ] **Phase 7** — Timeouts and deadlines
- [ ] **Phase 8** — Task management and lifecycle
- [ ] **Phase 9** — ......
- [ ] **Phase 10** — ......

## Local Development

### Using cargo

```bash
cargo test  # test only 
cargo build # compile and bundle to /target
cargo run
```


## More details

[Specification](specs/DistributedComputing.md)