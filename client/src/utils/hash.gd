# Utility functions
const CHUNK_SIZE = 1024
const HASHING_CONTEXT = HashingContext.HASH_SHA256

static func hash_packed_byte_array(data: PackedByteArray) -> PackedByteArray:
    var ctx = HashingContext.new()
    ctx.start(HASHING_CONTEXT)
    var n_chunks = data.size() / CHUNK_SIZE + (0 if data.size() % CHUNK_SIZE == 0 else 1)
    for i in range(n_chunks):
        ctx.update(data.slice(i * CHUNK_SIZE, (i + 1) * CHUNK_SIZE))
    #var res = ctx.finish()
    #printt(res.hex_encode(), Array(res))
    return ctx.finish()
