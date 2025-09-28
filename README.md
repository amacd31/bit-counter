# bit-counter

Package for efficiently counting the number of one bits in a numpy array.
Implemented as a Python module using Rust, providing high performance counting.

## Building

To build this package an installation of Rust and Python with the [maturin](https://maturin.rs/)
package is required. The Maturin documentation on [maturin local development](https://maturin.rs/develop.html) is a useful reference for more details.

This project is configured by default to target CPUs with the [POPCNT](https://en.wikipedia.org/wiki/SSE4#POPCNT_and_LZCNT) instruction.
The builds available on PyPI have been built with this configuration.
If you require a version for an older CPU without `popcnt` support, build with
the RUSTFLAGS environment variable to exclude the `popcnt` target-feature.
i.e. `RUSTFLAGS='-C target-feature=-popcnt' maturin build -r`.

By default the bit counting is done with a parallel map across all available
CPUs through the use of [rayon](https://docs.rs/rayon/latest/rayon/). Number of threads
can be configured with the environment variable `RAYON_NUM_THREADS`.

## Example usage

    import numpy as np
    from bit_counter import count_ones

    arr = np.packbits(np.random.choice([True, False], 1000000))

    count_of_true_values = count_ones(arr)

## Performance

When built to target a CPU with `popcnt` support the `count_ones` method
provided is substantially faster than a naive `np.unpackbits(arr).sum()`.
Note that a faster `bitwise_count` function was added in `numpy`
[1.22.0](https://numpy.org/doc/1.22/release/1.22.0-notes.html#bit-count-to-compute-the-number-of-1-bits-in-an-integer).
The `count_ones` method also doesn't require unpacking the bit packed numpy
array, so doesn't require any addtional memory to do the calculation.

For example with the following test code for 100 million True/False values that
are packed:

```
import numpy as np
from bit_counter import count_ones

np.random.seed(42)
arr = np.packbits(np.random.choice([True, False], 100000000))
```

Using this package on an AMD Ryzen 9 3900X 12-Core Processor yields:

```
%timeit count_ones(arr)

471 µs ± 46.4 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

Better CPU performance can be had by treating the packed array as uint64:

```
%timeit count_ones(arr.view(np.uint64))

198 µs ± 47.2 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```

Compared to the simple unpack and sum case with numpy 2.2.6:

```
%timeit np.unpackbits(arr).sum()

45.8 ms ± 879 µs per loop (mean ± std. dev. of 7 runs, 10 loops each)
```

The faster available numpy solution using `bitwise_count` is:

```
%timeit np.bitwise_count(arr).sum()

7.12 ms ± 44.8 µs per loop (mean ± std. dev. of 7 runs, 100 loops each)
```

Which is faster again when treating the packed bits as `uint64`:

```
%timeit np.bitwise_count(arr.view(np.uint64)).sum()

1.14 ms ± 4.94 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)
```
