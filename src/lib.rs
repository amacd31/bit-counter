use bitvec::prelude::*;
use numpy::PyReadonlyArrayDyn;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

/// count_ones(arr)
/// --
///
/// Count the number of ones in a numpy array of uint8.
///
/// Used for counting up the number of one values in a `uint8`
/// `np.array`. The input to this method would typically be the
/// the output of `np.packbits`.
///
/// :param arr: Numpy uint8 array to count the number of one bits in.
/// :returns: Integer of the number of bits that were one in the array.
#[pyfunction]
fn count_ones(arr: PyReadonlyArrayDyn<u8>) -> PyResult<usize> {
    let arr = match arr.as_slice() {
        Ok(arr) => arr,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let bits = BitSlice::<_, Lsb0>::from_slice(&arr);
    Ok(bits.count_ones())
}

#[pymodule]
fn bit_counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_ones, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
