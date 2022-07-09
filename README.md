# bit-counter

Package for counting the number of one bits in a numpy array of uint8 values.
Implemented as a Python module using Rust, providing high performance counting.


## Building

To build this package an installation of Rust and Python with the [maturin](https://maturin.rs/)
package is required. The Maturin documentation on [maturin local development](https://maturin.rs/develop.html) is a useful reference for more details.

When building it is best to set the RUSTFLAGS environment variable to specify
targeting CPUs with the [POPCNT](https://en.wikipedia.org/wiki/SSE4#POPCNT_and_LZCNT) instruction. i.e. `RUSTFLAGS='-C target-feature=+popcnt' maturin build -r`.


## Example usage

    import numpy as np
    from bit_counter import count_ones

    arr = np.packbits(np.random.choice([True, False], 1000000))

    count_of_true_values = count_ones(arr)

