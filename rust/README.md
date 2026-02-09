# SikuliX 3.0 - Rust/Python Edition

[![CI](https://github.com/RaiMan/SikuliX1/workflows/CI/badge.svg)](https://github.com/RaiMan/SikuliX1/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Modern computer vision automation framework built with Rust and Python**

SikuliX 3.0 is a complete modernization of the SikuliX automation framework, transitioning from Java/Jython to Rust/CPython for improved performance, maintainability, and modern tooling.

## 🚀 Features

- **Computer Vision**: Template matching and image recognition powered by OpenCV
- **Input Automation**: Mouse and keyboard control across Windows and Linux
- **OCR Support**: Text recognition using Tesseract
- **Serial Automation**: Serial port communication for hardware automation *(NEW)*
- **SSH Automation**: Remote command execution and file transfer *(NEW)*
- **CLI-First**: Modern command-line interface with Python scripting
- **Fast & Efficient**: Built with Rust for native performance

## 📦 Installation

```bash
# Install from PyPI (coming soon)
pip install sikulix

# Or install from source
git clone https://github.com/RaiMan/SikuliX1
cd SikuliX1/rust
pip install maturin
maturin develop
```

## 🎯 Quick Start

```python
from sikulix import Screen

# Create a screen instance
screen = Screen()

# Find and click an image
if screen.exists("button.png"):
    screen.click("button.png")

# Wait for an image with timeout
screen.wait("login_prompt.png", timeout=5)
screen.type("username")
```

### Serial Automation Example *(Phase 4)*

```python
from sikulix import Serial

with Serial("/dev/ttyUSB0", baudrate=115200) as ser:
    ser.expect("login:")
    ser.send("admin\n")
    ser.expect("Password:")
    ser.send("secret\n")
```

### SSH Automation Example *(Phase 4)*

```python
from sikulix import SSH

with SSH("192.168.1.100", username="admin", key_file="~/.ssh/id_rsa") as ssh:
    result = ssh.exec("uname -a")
    print(result.stdout)
```

## 🏗️ Project Structure

```
rust/
├── crates/
│   ├── sikulix-core/       # Core types (Region, Location, Pattern, Match)
│   ├── sikulix-vision/     # Computer vision (OpenCV template matching)
│   ├── sikulix-platform/   # OS-specific screen capture & input
│   ├── sikulix-remote/     # Serial & SSH automation
│   ├── sikulix-python/     # PyO3 Python bindings
│   └── sikulix-gui/        # Screenshot selector tool (optional)
├── python/
│   └── sikulix/            # Pure Python wrapper
├── Cargo.toml              # Workspace configuration
└── pyproject.toml          # Python package configuration
```

## 🛠️ Development

### Prerequisites

- **Rust**: 1.75+ ([Install Rust](https://rustup.rs/))
- **Python**: 3.8+
- **OpenCV**: System installation (for development)
  - Windows: `vcpkg install opencv4`
  - Linux: `sudo apt-get install libopencv-dev`

### Building from Source

```bash
# Clone the repository
git clone https://github.com/RaiMan/SikuliX1
cd SikuliX1/rust

# Run Rust tests
cargo test --workspace

# Build Python package (development mode)
pip install maturin
maturin develop

# Run Python tests
pytest python/tests
```

### Running Tests

```bash
# Rust tests
cargo test -p sikulix-core

# Python tests
pytest python/tests -v

# With coverage
pytest --cov=sikulix python/tests
```

## 📋 Development Roadmap

### ✅ Phase 0: Foundation (CURRENT)
- [x] Cargo workspace structure
- [x] Core types (Location, Offset, Region, Pattern, Match, Image)
- [x] PyO3 bindings skeleton
- [x] Python package structure with maturin
- [x] 17 passing unit tests for core types

### 🚧 Phase 1: Computer Vision Core (Next)
- [ ] OpenCV template matching implementation
- [ ] Image loading and processing utilities
- [ ] Pattern matching with similarity thresholds
- [ ] Python bindings for Finder class
- [ ] Performance benchmarks

### 📅 Phase 2: Screen Capture & Input
- [ ] Platform-specific screen capture (Windows, Linux)
- [ ] Mouse control (move, click, drag, wheel)
- [ ] Keyboard input with key combinations
- [ ] Multi-monitor support

### 📅 Phase 3: Python Integration & CLI
- [ ] Complete Python API
- [ ] CLI tool (`sikulix run`, `sikulix test`, `sikulix doctor`)
- [ ] Migration guide from SikuliX 2.x
- [ ] Example scripts

### 📅 Phase 4: Serial & SSH Automation
- [ ] Serial port communication
- [ ] SSH session management
- [ ] Expect-like pattern matching
- [ ] Combined automation workflows

### 📅 Phase 5: OCR & Advanced Vision
- [ ] Tesseract OCR integration
- [ ] Text recognition API
- [ ] Change detection
- [ ] Multiple match modes

### 📅 Phase 6: GUI Tools (Optional)
- [ ] Screenshot selector (egui)
- [ ] Region capture tool
- [ ] Pattern editor

### 📅 Phase 7: Release
- [ ] Comprehensive documentation
- [ ] Performance optimization
- [ ] CI/CD with pre-built wheels
- [ ] PyPI publication

## 🎯 Goals vs Java/Jython Version

| Metric | Java 2.x | Rust 3.0 (Target) |
|--------|----------|-------------------|
| Template matching | Baseline | 20-30% faster |
| Screen capture | ~30 FPS | 60+ FPS |
| Memory usage | ~200MB (JVM) | <100MB |
| Startup time | ~2-3s | <500ms |
| Package size | ~100MB | <50MB |
| Python version | Jython 2.7 | CPython 3.8+ |

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📝 License

MIT License - see [LICENSE](../LICENSE) for details.

## 🙏 Acknowledgments

- Original SikuliX by Raimund Hocke (RaiMan)
- OpenCV community for computer vision capabilities
- PyO3 team for excellent Rust-Python bindings

## 📚 Documentation

- [API Documentation](https://docs.sikulix.com) *(coming soon)*
- [Migration Guide](docs/migration-guide.md) - Migrating from SikuliX 2.x
- [Architecture Decision Records](docs/adr/) - Design decisions

## 🔗 Links

- [GitHub Repository](https://github.com/RaiMan/SikuliX1)
- [Issue Tracker](https://github.com/RaiMan/SikuliX1/issues)
- [Original SikuliX](https://sikulix.com)

---

**Status**: 🚧 Active Development - Phase 0 Complete (Foundation)
