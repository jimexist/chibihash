from chibihash._core import chibihash64 as _chibihash64


def chibihash64(key: bytes, seed: int = 0) -> int:
    return _chibihash64(key, seed)


__all__ = ["chibihash64"]
