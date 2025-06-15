

# 🧠 **Project Overview: E++**

---

## 📛 Name: `E++`

> A Python-compatible compiled language — readable like Python, powerful like C++, and built for native performance.

---

## 🎯 **Project Goal**

To create a **fully Python-compatible language** that:

* Uses Python 3.x **syntax** (ideally 100% compatible)
* Compiles `.eppx` files into **native binaries**
* Does **not** depend on CPython at runtime
* Can run standard Python libraries and modules
* Uses a modern systems language as the backend (Rust or C++)
* Supports a **custom IR**, compiler, runtime, and package manager

---

## 🔍 **Key Principles**

| Feature                   | Description                                                  |
| ------------------------- | ------------------------------------------------------------ |
| ✅ Python syntax           | Keep the language readable and beginner-friendly             |
| ✅ Native compilation      | Target machine code using LLVM or transpile to C++           |
| ✅ Zero CPython dependency | No reliance on the CPython interpreter                       |
| ✅ Package support         | Use `uv` (Rust-based Python package manager)                 |
| ✅ Optional static typing  | Add typing without sacrificing Python's flexibility          |
| ✅ Custom runtime          | Reimplement core Python behaviors (like `list`, `str`, etc.) |

---

## 🗂️ File Extension

```plaintext
.eppx
```

---

## 📁 Project Structure (recommended)

```plaintext
eppx-lang/
├── src/
│   ├── parser/          # Python grammar parser
│   ├── ast/             # Abstract syntax tree
│   ├── typechecker/     # Optional static typing engine
│   ├── ir/              # Intermediate representation
│   ├── codegen/         # Native code generator (LLVM/C++)
│   ├── runtime/         # Memory, GC, stdlib functions
│   └── cli/             # Command-line interface (eppx run/build)
├── stdlib/              # E++-compatible standard lib
├── examples/            # Demo .eppx programs
├── tools/               # uv wrappers, migration tools
├── docs/                # Design notes, reference
├── tests/               # Compiler + runtime tests
├── Cargo.toml / CMakeLists.txt
└── README.md
```

---

## 🛠️ **Tech Stack**

| Component             | Language/Tool                                                        |
| --------------------- | -------------------------------------------------------------------- |
| **Compiler language** | 🦀 Rust (**recommended**) or C++                                     |
| **Parsing**           | `pest` / `lalrpop` (Rust) or ANTLR (C++)                             |
| **IR**                | Custom designed structs/enums                                        |
| **Codegen**           | LLVM (via `inkwell`) or transpile to C++                             |
| **Package manager**   | [`uv`](https://github.com/astral-sh/uv) (Rust-based pip replacement) |
| **Testing**           | `cargo test`, your own `eppx test` command                           |
| **Runtime**           | Custom Python-compatible runtime in Rust/C++                         |

---

## 🧱 Compiler Architecture (High Level)

```plaintext
Source (.eppx)
     ↓
Lexer + Parser
     ↓
AST
     ↓
Type Checker (optional)
     ↓
Intermediate Representation (IR)
     ↓
Code Generation (LLVM / C++ transpile)
     ↓
Native Binary
     ↓
Your Runtime + Optional Python Lib Loader
```

---

## 🧬 Python Compatibility Strategy

| Component    | Approach                                                    |
| ------------ | ----------------------------------------------------------- |
| ✅ Syntax     | Fully mimic Python 3.x grammar                              |
| ✅ Stdlib     | Reimplement or wrap required parts                          |
| ✅ Libs       | Use `uv` to install wheels; provide your own dynamic loader |
| ❌ No CPython | Do **not** embed or require CPython                         |
| 🔄 Optional  | Implement support for `.py` files (if desired) later        |

---

## 📦 Package Manager Plan

* Use [`uv`](https://github.com/astral-sh/uv) for package installation
* Wrap it in `eppx install` / `eppx add` CLI commands
* Automatically manage `.eppx_packages/` directory
* Optionally build a runtime shim to load `.pyd` / `.so` files from Python packages

---

## 🧪 Sample Commands (CLI)

```bash
eppx new myproject         # Init new project
eppx build                 # Compile .eppx to native binary
eppx run main.eppx         # Compile and execute
eppx install numpy         # Use uv under the hood
eppx test                  # Run tests
```

---

## 📘 Roadmap

### ✅ Phase 1: Bootstrap Compiler

* [ ] Language grammar (Python 3.x + your extensions)
* [ ] AST + parser
* [ ] IR definition
* [ ] Basic codegen (LLVM or transpile to C++)
* [ ] CLI: `eppx build`, `eppx run`

### ⚙️ Phase 2: Minimal Runtime

* [ ] `print`, `str`, `int`, `list`
* [ ] Memory management
* [ ] Custom error system

### 📦 Phase 3: Package Manager Integration

* [ ] CLI wrapper around `uv`
* [ ] Dependency resolver and `.eppx_packages`

### 🔁 Phase 4: Python Lib Interop

* [ ] Load `.pyd` / `.so` files via Rust FFI
* [ ] Bind to popular packages (e.g., `numpy`, `requests`)

### 🔥 Phase 5: Optimizations + Tooling

* [ ] Optional type annotations
* [ ] IDE plugin / VSCode support
* [ ] Debugger, profiler

---

## 📣 Tagline Ideas

> **E++**: *Python on steroids — native, modern, and yours.*

> *The language Python wishes it could compile to.*

> *E++: Python’s soul. LLVM’s speed. Your control.*

for example to run examples/hello.eppx
```bash
cargo run -- run examples/hello.eppx
```
```bash
cargo run -- help
```