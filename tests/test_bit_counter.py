import numpy as np
from bit_counter import count_ones

def test_bit_count():
    np.random.seed(42)
    arr = np.packbits(np.random.choice([True, False], 1000))

    count = count_ones(arr)

    assert count == 490


def test_bit_count_of_view():
    np.random.seed(42)
    arr = np.packbits(np.random.choice([True, False], 100000000))

    count = count_ones(arr.view(np.uint64))

    assert count == 50005072

def test_bit_counter_equivalent_to_numpy_solution():
    np.random.seed(42)
    arr = np.packbits(np.random.choice([True, False], 1000))

    bit_counter_count = count_ones(arr)
    numpy_count = np.bitwise_count(arr).sum()

    assert bit_counter_count == numpy_count

def test_bit_counter_equivalent_to_numpy_solution_with_views():
    np.random.seed(42)
    arr = np.packbits(np.random.choice([True, False], 100000000))

    bit_counter_count = count_ones(arr.view(np.uint64))
    numpy_count = np.bitwise_count(arr.view(np.uint64)).sum()

    assert bit_counter_count == numpy_count
