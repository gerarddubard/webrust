# 🤝 Contributing to WebRust

Thank you for your interest in contributing to **WebRust**!  
This guide explains how to get started, follow the workflow, and keep your contributions aligned with the project's goals.

---

## 🚀 Getting Started

### 🧩 Prerequisites
- **Rust** 1.70 or later
- **Git** (for version control)
- **A modern browser** (for GUI testing)

### 🎯 Project Philosophy

WebRust bridges Python's simplicity, SQL's analytical power, and Rust's performance.  
When contributing, please ensure that your work reflects these guiding principles:

- **Ergonomics, analytics, and performance can coexist**
- **Simple interfaces for complex systems**
- **Developer experience matters**
- **Rust's safety and zero-cost abstractions remain non-negotiable**
- **Data analytics should be first-class**

> 💡 *Every PR should make WebRust more accessible, expressive, analytical, and joyful to use.*

---

## 🛠️ Setting Up Your Environment

1. **Fork** the repository on GitHub
2. **Clone** your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/webrust.git
   ```
3. **Enter the directory:**
   ```bash
   cd webrust
   ```
4. **Run tests:**
   ```bash
   cargo test
   ```
5. **Try examples:**
   ```bash
   cargo run --example py_chart
   cargo run --example py_sql  # 🆕 NEW: SQL analytics demo
   ```

---

## 💡 How to Contribute

### 🐞 Reporting Bugs
- Use [GitHub Issues](https://github.com/gerarddubard/webrust/issues)
- Provide a clear **title** and **description**
- Include:
    - Steps to reproduce
    - Expected vs. actual behavior
    - Your OS and Rust version
    - Minimal reproducible code sample (if possible)
    - For SQL issues: Include the query and error message

### 🌱 Suggesting Features
- Check for existing issues before opening a new one
- Clearly describe:
    - The problem or motivation
    - The proposed feature
    - Expected benefits or use cases
- Optionally, outline a basic implementation idea

---

## 🧰 Code Contributions

### 🔍 Before You Start
- Review open issues & PRs to avoid duplication
- For large changes, discuss first via an issue
- Align with WebRust's philosophy (ergonomic + analytical + safe + modern)

### ⚙️ Development Workflow
1. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Implement your feature or fix
3. Add or update **tests**
4. Run:
   ```bash
   cargo test
   cargo run --example py_chart
   cargo run --example py_sql  # Test SQL functionality
   cargo fmt
   cargo clippy
   ```
5. Commit with a clear message
6. Push to your fork and open a **Pull Request**

---

## 🧩 Code Style & Standards

### 🦀 Rust Guidelines
- Format with `cargo fmt`
- Lint with `cargo clippy`
- Prefer expressive, concise code over cleverness
- Use meaningful variable names
- Keep functions focused and composable
- Document all public APIs with `///` comments

### 📝 Documentation
- Update docs for all API changes
- Add examples demonstrating new behavior
- Ensure doc tests compile (`cargo test --doc`)
- Update `README.md` and `CHANGELOG.md` when relevant
- Follow the comment style in `examples/py_sql.rs` for new examples

### 🧪 Testing
- Add **unit tests** for new functionality
- Cover **edge cases and errors**
- Ensure examples compile and run
- Test across major platforms (Linux, macOS, Windows) if applicable
- For SQL features: Test with various queries and data types

---

## 🧭 Areas for Contribution

### 🔥 High Priority
- Bug fixes and performance improvements
- Documentation enhancements
- New chart types and visualization tools
- **SQL functionality improvements** (new DuckDB features, query optimization)
- **Arrow integration enhancements** (new data types, streaming improvements)
- Improved table formatting and LaTeX rendering
- Cross-platform stability

### ⚡ Medium Priority
- Styling & theming options
- Component system and reusable UI widgets
- Example dashboards and tutorials
- **SQL query builder utilities**
- **Database connectors** (PostgreSQL, MySQL, SQLite)

### 🧩 Low Priority (but appreciated)
- Code cleanup and refactoring
- Extra test coverage
- Translation of documentation
- Community utilities or scripts
- Performance benchmarks

---

## 🗄️ Contributing to SQL Module

The SQL module (`webrust::db::sql`) is a key feature in 1.3.0. When contributing:

**Testing SQL Features**:
- Test queries with different data types (integers, floats, strings, booleans, nulls)
- Verify streaming behavior with large result sets (100K+ rows)
- Check error handling for invalid SQL
- Test edge cases: empty results, single row, single column

**Performance Considerations**:
- Use `cargo bench` for SQL query benchmarks
- Profile memory usage with large datasets
- Ensure thread-local buffers are properly managed
- Verify SIMD optimizations are applied

**Documentation**:
- Document SQL syntax limitations
- Provide examples for complex queries
- Explain performance characteristics
- Note DuckDB-specific extensions

---

## 🔄 Pull Request Process

1. Ensure your code follows formatting & lint rules
2. Add or update documentation
3. Include relevant tests
4. Update `CHANGELOG.md` for user-facing changes
5. Verify all tests pass locally
6. Write a clear PR description:
    - What was changed
    - Why it was changed
    - How to test it

> ✅ PRs that include updated docs and examples are always reviewed faster.

---

## 🌍 Community Guidelines

- Be respectful and inclusive
- Provide constructive feedback
- Keep discussions on-topic
- Help newcomers learn
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

---

## 💬 Questions & Support

If you're unsure about something:
- Check existing issues and docs first
- Open a *"question"* issue if necessary
- Be patient — maintainers will respond as soon as possible

---

## 📜 License

By contributing to WebRust, you agree that your contributions are licensed under the **MIT License**, the same as the main project.

---

## 🌟 Recognition

Your name will appear in the **project documentation** and release notes.  
Major contributors may be invited to join the **WebRust Maintainers Team**.

---

*Thank you for helping make WebRust a bridge between Python simplicity, SQL power, Rust safety, and web modernity — and for keeping programming joyful.* ❤️