"""Basic import tests"""


def test_import_astrora():
    """Test that astrora can be imported"""
    import astrora

    assert hasattr(astrora, "__version__")


def test_version():
    """Test version string format"""
    import astrora

    version = astrora.__version__
    assert isinstance(version, str)
    assert len(version.split(".")) >= 2  # At least major.minor
