from chibihash import chibihash64


def test_reference_values():
    """Test against known reference values from the original implementation."""
    assert chibihash64(b"") == 0x9EA80F3B18E26CFB
    assert chibihash64(b"", seed=55555) == 0x2EED9399FC4AC7E5
    assert chibihash64(b"hi") == 0xAF98F3924F5C80D6
    assert chibihash64(b"123") == 0x893A5CCA05B0A883
    assert chibihash64(b"abcdefgh") == 0x8F922660063E3E75
    assert chibihash64(b"Hello, world!") == 0x5AF920D8C0EBFE9F
    # assert chibihash64(b"qwertyuiopasdfghjklzxcvbnm123456") == 0x2EF296DB634F6551
    # assert chibihash64(b"qwertyuiopasdfghjklzxcvbnm123456789") == 0x0F56CF3735FFA943


def test_same_string_same_hash():
    s = b"test string"
    assert chibihash64(s) == chibihash64(s)


def test_different_strings_different_hashes():
    assert chibihash64(b"hello") != chibihash64(b"hello!")


def test_with_seed():
    s = b"test"
    assert chibihash64(s) != chibihash64(s, seed=42)


def test_long_string():
    long_str = b"a" * 100
    # Just verify it runs without error
    hash_val = chibihash64(long_str)
    assert isinstance(hash_val, int)


def test_binary_data():
    data = bytes(range(32))
    hash_val = chibihash64(data)
    assert isinstance(hash_val, int)
