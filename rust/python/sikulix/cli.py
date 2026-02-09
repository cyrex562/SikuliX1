"""Command-line interface for SikuliX."""

import sys


def main() -> int:
    """Main entry point for the sikulix CLI."""
    print("SikuliX 3.0 - Command Line Interface")
    print("Not yet implemented - Phase 3")
    print()
    print("Planned commands:")
    print("  sikulix run script.py    - Execute an automation script")
    print("  sikulix test tests/      - Run test suite")
    print("  sikulix capture          - Screenshot selector tool")
    print("  sikulix doctor           - Verify installation")
    return 0


if __name__ == "__main__":
    sys.exit(main())
