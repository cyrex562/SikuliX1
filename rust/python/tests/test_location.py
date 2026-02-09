"""Tests for Location and Offset classes."""

import pytest
from sikulix import Location, Offset


class TestLocation:
    """Test Location class."""

    def test_create_location(self):
        """Test creating a location."""
        loc = Location(100, 200)
        assert loc.x == 100
        assert loc.y == 200

    def test_location_offset(self):
        """Test offsetting a location."""
        loc = Location(100, 200)
        offset = Offset(50, -30)
        new_loc = loc.offset(offset)
        assert new_loc.x == 150
        assert new_loc.y == 170

    def test_location_distance(self):
        """Test calculating distance between locations."""
        loc1 = Location(0, 0)
        loc2 = Location(3, 4)
        assert loc1.distance_to(loc2) == 5.0

    def test_location_repr(self):
        """Test location string representation."""
        loc = Location(100, 200)
        assert "100" in repr(loc)
        assert "200" in repr(loc)


class TestOffset:
    """Test Offset class."""

    def test_create_offset(self):
        """Test creating an offset."""
        offset = Offset(10, -20)
        assert offset.dx == 10
        assert offset.dy == -20

    def test_offset_repr(self):
        """Test offset string representation."""
        offset = Offset(10, -20)
        assert "10" in repr(offset)
        assert "-20" in repr(offset)
