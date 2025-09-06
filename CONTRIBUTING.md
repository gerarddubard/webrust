# Contributing to WebRust

Thank you for your interest in contributing to WebRust! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites
- Rust 1.70 or later
- Git
- A web browser for testing GUI features

### Project Philosophy

WebRust exists to make Rust more accessible to developers from all backgrounds, particularly those coming from Python. We believe that:

- Ergonomics and performance should coexist
- Complex systems can have simple interfaces
- Developer experience matters as much as runtime performance
- Experimentation with language ergonomics benefits the entire ecosystem

When contributing, consider how your changes advance these goals while maintaining Rust's core principles of safety and performance.

### Setting up the development environment
1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/webrust.git`
3. Navigate to the directory: `cd webrust`
4. Run tests: `cargo test`
5. Try the examples: `cargo run --example py_chart`

## How to Contribute

### Reporting Bugs
- Use the GitHub issue tracker
- Describe the bug with a clear title and description
- Include steps to reproduce the issue
- Mention your operating system and Rust version
- If possible, provide a minimal code example

### Suggesting Features
- Check existing issues to avoid duplicates
- Create a new issue with a clear description of the proposed feature
- Explain the use case and benefits
- Consider providing a basic implementation outline

### Code Contributions

#### Before you start
- Check existing issues and pull requests to avoid duplicate work
- For major changes, create an issue first to discuss the approach
- Make sure your changes align with the project's goals

#### Development workflow
1. Create a feature branch: `git checkout -b feature/your-feature-name`
2. Make your changes
3. Add or update tests as needed
4. Run the test suite: `cargo test`
5. Test examples: `cargo run --example py_chart`
6. Format code: `cargo fmt`
7. Run linter: `cargo clippy`
8. Commit with clear messages
9. Push to your fork and create a pull request

#### Code style
- Follow standard Rust formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add documentation for public APIs
- Keep functions focused and reasonably sized
- Follow existing patterns in the codebase

#### Documentation
- Update relevant documentation for API changes
- Add examples for new features
- Ensure code examples in documentation are tested and working
- Update README.md if needed

#### Testing
- Add unit tests for new functionality
- Test edge cases and error conditions
- Ensure examples work correctly
- Test on multiple platforms when possible

## Areas for Contribution

### High Priority
- Bug fixes and stability improvements
- Performance optimizations
- Documentation improvements
- Additional chart types and visualization features
- Extended string manipulation methods
- Cross-platform compatibility

### Medium Priority
- New styling options and themes
- Additional table formatting features
- Enhanced LaTeX support
- Example applications and tutorials

### Low Priority (but welcome!)
- Code refactoring and cleanup
- Additional tests
- Translation of documentation
- Community tools and utilities

## Pull Request Process

1. Ensure your code follows the project's coding standards
2. Update documentation as needed
3. Add tests for new functionality
4. Ensure all tests pass
5. Update CHANGELOG.md if your changes are user-facing
6. Write a clear pull request description explaining:
    - What changes you made
    - Why you made them
    - How to test them

## Community Guidelines

- Be respectful and inclusive
- Help others learn and grow
- Focus on constructive feedback
- Keep discussions on-topic
- Follow the Rust Code of Conduct

## Questions and Support

If you have questions about contributing:
- Check existing issues and documentation first
- Create a GitHub issue for technical questions
- Join discussions in existing issues
- Be patient - maintainers will respond when available

## License

By contributing to WebRust, you agree that your contributions will be licensed under the same license as the project (MIT License).

## Recognition

Contributors will be acknowledged in the project documentation. Significant contributors may be invited to become project maintainers.

Thank you for helping make WebRust better!