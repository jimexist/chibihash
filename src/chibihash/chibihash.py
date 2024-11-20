def load64le(p: bytes, offset: int = 0) -> int:
    """Load 8 bytes as a 64-bit little endian integer."""
    return (
        p[offset] << 0
        | p[offset + 1] << 8
        | p[offset + 2] << 16
        | p[offset + 3] << 24
        | p[offset + 4] << 32
        | p[offset + 5] << 40
        | p[offset + 6] << 48
        | p[offset + 7] << 56
    )


def chibihash64(key: bytes, seed: int = 0) -> int:
    """64-bit hash function.

    Args:
        key: Input bytes to hash
        seed: Optional seed value (default 0)

    Returns:
        64-bit hash value
    """
    k = key
    length = len(k)

    P1 = 0x2B7E151628AED2A5
    P2 = 0x9E3793492EEDC3F7
    P3 = 0x3243F6A8885A308D
    MASK64 = 0xFFFFFFFFFFFFFFFF

    h = [P1, P2, P3, seed]

    offset = 0
    while length >= 32:
        for i in range(4):
            lane = load64le(k, offset + i * 8)
            h[i] ^= lane
            h[i] = (h[i] * P1) & MASK64
            h[(i + 1) & 3] ^= ((lane << 40) | (lane >> 24)) & MASK64
        length -= 32
        offset += 32

    h[0] = (h[0] + (((length << 32) | (length >> 32)) & MASK64)) & MASK64

    if length & 1:
        h[0] ^= k[offset]
        length -= 1
        offset += 1
    h[0] = (h[0] * P2) & MASK64
    h[0] ^= h[0] >> 31

    i = 1
    while length >= 8:
        h[i] ^= load64le(k, offset)
        h[i] = (h[i] * P2) & MASK64
        h[i] ^= h[i] >> 31
        length -= 8
        offset += 8
        i += 1

    i = 0
    while length > 0:
        if length >= 2:
            h[i] ^= k[offset] | (k[offset + 1] << 8)
        else:
            h[i] ^= k[offset]
        h[i] = (h[i] * P3) & MASK64
        h[i] ^= h[i] >> 31
        length -= 2
        offset += 2
        i += 1

    x = seed
    x ^= (h[0] * ((h[2] >> 32) | 1)) & MASK64
    x ^= (h[1] * ((h[3] >> 32) | 1)) & MASK64
    x ^= (h[2] * ((h[0] >> 32) | 1)) & MASK64
    x ^= (h[3] * ((h[1] >> 32) | 1)) & MASK64

    # moremur mixing
    x ^= x >> 27
    x = (x * 0x3C79AC492BA7B653) & MASK64
    x ^= x >> 33
    x = (x * 0x1C69B3F74AC4AE35) & MASK64
    x ^= x >> 27

    return x
