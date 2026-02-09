"""Tests for Region class."""

import pytest
from sikulix import Region, Location


class TestRegion:
    """Test Region class."""

    def test_create_region(self):
        """Test creating a region."""
        region = Region(10, 20, 100, 50)
        assert region.x == 10
        assert region.y == 20
        assert region.w == 100
        assert region.h == 50

    def test_region_center(self):
        """Test getting region center."""
        region = Region(0, 0, 100, 100)
        center = region.center()
        assert center.x == 50
        assert center.y == 50

    def test_region_contains(self):
        """Test checking if region contains a location."""
        region = Region(0, 0, 100, 100)
        assert region.contains(Location(50, 50))
        assert region.contains(Location(0, 0))
        assert not region.contains(Location(100, 100))
        assert not region.contains(Location(-1, 50))

    def test_region_area(self):
        """Test calculating region area."""
        region = Region(0, 0, 100, 50)
        assert region.area() == 5000

    def test_region_repr(self):
        """Test region string representation."""
        region = Region(10, 20, 100, 50)
        repr_str = repr(region)
        assert "10" in repr_str
        assert "20" in repr_str
        assert "100" in repr_str
        assert "50" in repr_str
