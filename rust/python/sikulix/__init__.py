"""
SikuliX - Modern Computer Vision Automation
============================================

SikuliX automates GUI interactions using image recognition powered by OpenCV.
This is the Python API for SikuliX 3.0, built on Rust for performance.

Basic Usage:
    from sikulix import Region, Screen, click, wait

    # Find and click a button
    screen = Screen()
    if screen.exists("button.png"):
        screen.click("button.png")

    # Wait for an image to appear
    screen.wait("login_prompt.png", timeout=5)
    screen.type("username")
"""

from sikulix._native import Location, Offset, Region

__version__ = "3.0.0"
__all__ = [
    "Location",
    "Offset",
    "Region",
    # Screen, Match, Pattern, Image will be added in later phases
]


def get_version() -> str:
    """Get the SikuliX version string."""
    return __version__
