# bit-counter

Package for counting the number of one bits in a numpy array of uint8 values.
Implemented as a Python module using Rust, providing high performance counting.

## Example usage

    import numpy as np
    from bit_counter import count_ones

    arr = np.packbits(np.random.choice([True, False], 1000000))

    count_of_true_values = count_ones(arr)

