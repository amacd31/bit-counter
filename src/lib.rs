use num_traits::PrimInt;
use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use rayon::prelude::*;

fn count_ones_generic<T>(arr: PyReadonlyArray1<T>) -> PyResult<usize>
where
    T: numpy::Element + PrimInt + Send + Sync,
{
    let arr = match arr.as_slice() {
        Ok(arr) => arr,
        Err(error) => return Err(PyTypeError::new_err(error)),
    };

    let bit_count = arr
        .into_par_iter()
        .map(|elem| elem.count_ones() as usize)
        .sum::<usize>();

    Ok(bit_count)
}

// Macro to reduce repetition in type checking
macro_rules! try_extract_and_count {
    ($arr:expr, $($type:ty),+) => {
        $(
            if let Ok(array) = $arr.extract::<PyReadonlyArray1<$type>>() {
                return count_ones_generic(array);
            }
        )+
    };
}

/// count_ones(arr)
/// --
///
/// Count the number of ones in a numpy array of integers.
///
/// Used for counting up the number of one values in integer numpy arrays.
/// Supports uint8, uint16, uint32, uint64, int8, int16, int32, and int64 dtypes.
///
/// Example usage:
///
///     import numpy as np
///     from bit_counter import count_ones
///     
///     np.random.seed(42)
///     arr = np.packbits(np.random.choice([True, False], 100000000))
///     popcount = count_ones(arr.view(np.uint64)) # View as uint64 for better performance
///
/// :param arr: Numpy integer array to count the number of one bits in.
/// :returns: Integer of the number of bits that were one in the array.
#[pyfunction]
fn count_ones(arr: &Bound<'_, PyAny>) -> PyResult<usize> {
    // Try to extract as different numpy array types based on dtype
    try_extract_and_count!(arr, u8, u16, u32, u64, i8, i16, i32, i64);

    Err(PyTypeError::new_err(
        "Unsupported array dtype. Supported dtypes are: uint8, uint16, uint32, uint64, int8, int16, int32, int64"
    ))
}

#[pymodule]
fn bit_counter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_ones, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
