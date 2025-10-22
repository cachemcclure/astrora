"""Basic import tests"""

import pytest


def test_import_poliastro():
    """Test that poliastro can be imported"""
    import poliastro
    assert hasattr(poliastro, "__version__")


def test_version():
    """Test version string format"""
    import poliastro
    version = poliastro.__version__
    assert isinstance(version, str)
    assert len(version.split(".")) >= 2  # At least major.minor
