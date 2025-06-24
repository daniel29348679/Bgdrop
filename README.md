# `Bgdrop`

**A minimal background dropper for Rust.**
Free memory in background threads to reduce latency spikes in critical paths.

---

## 📦 Installation
* Edit Cargo.toml
```toml
# Cargo.toml
[dependencies]
bgdrop = "0.1"
```
* Or use Cargo
```bash
cargo add bgdrop
```
---


## 🔧 Usage

```rust
use bgdrop::Bgdrop;

fn main() {
    // Create a background dropper with 1 thread
    let dropper = Bgdrop::new();

    // Drop a large Vec in the background
    let large_vec = vec![0u8; 10_000_000];
    dropper.drop(large_vec);

    // Create a background dropper with multiple threads
    let pool = Bgdrop::with_threads(4);
    for _ in 0..10 {
        pool.drop(vec![0u8; 1_000_000]);
    }
}
```
---
## ⚠️ Notice

* Creating a `Bgdrop` instance will **start at least one dedicated background thread** for memory dropping.
* **Avoid creating `Bgdrop` instances frequently.** Use `.clone()` to share the same background thread.
* For large or complex data structures (e.g., `Vec<Vec<_>>`, `HashMap<_, _>`, or trees), offloading the drop to background can **improve performance by 10× or more**, especially in latency-critical code paths.
* For **small types** like tiny `Vec`s, primitive types, or types that **do not implement `Clone`**, `bgdrop` is unnecessary and adds overhead.
* Use `bgdrop` **only when dropping time becomes a measurable bottleneck**.

---

## 💡 Motivation

In performance-sensitive applications like games, real-time systems, or low-latency services, releasing large memory allocations (e.g. `Vec<u8>`, `HashMap`, etc.) may cause noticeable spikes in frame time or latency.

By moving the drop operation to a background thread, `bgdrop` helps smooth the performance curve.

---

## 🔒 Thread Safety

All types submitted to `drop()` must be:

* `Send`
* `'static` lifetime

Internally, values are wrapped in `Box<dyn Send>` and sent via a lock-free `crossbeam::channel` to a background thread that performs the drop.

---

## 🚧 Limitations

* `!Sync` types cannot be used.
* Type erasure using `Box<dyn Send>` incurs a minor allocation cost.
* Dropped objects' destructors cannot observe ordering with respect to other code.

---

## 📜 License

MIT

