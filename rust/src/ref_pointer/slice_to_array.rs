use std::array;

#[test]
fn best_slice_to_array() {
    // the fn slice_to_array_unchecked or slice_to_array is the best
    {
        assert_eq!(3, size_of::<[u8; 3]>());
        assert_eq!(16, size_of::<&[u8]>());
    }
    {
        let array1 = [1, 2, 3];
        let ptr_array1 = array1.as_ptr();
        let array2: ArrayRef<'_, i32, 3> = array1[..].try_into().unwrap();
        assert_eq!(ptr_array1, array2.as_ref().as_ptr());

        let array3: &[i32; 3] = unsafe { slice_to_array_unchecked(&array1[..]) };
        assert_eq!(array3.as_ptr(), ptr_array1);
    }
}

#[inline]
unsafe fn slice_to_array_unchecked<T, const N: usize>(slice: &[T]) -> &[T; N] {
    &*slice.as_ptr().cast::<[T; N]>()
}

#[inline]
fn slice_to_array<T, const N: usize>(slice: &[T]) -> Option<&[T; N]> {
    if slice.len() != N {
        None
    } else {
        unsafe { Some(&*slice.as_ptr().cast::<[T; N]>()) }
    }
}

#[derive(Debug)] // For easy printing
pub struct ArrayRef<'a, T, const N: usize>(&'a [T; N]); // 'pub' if you want to use it outside module

impl<'a, T, const N: usize> AsRef<[T; N]> for ArrayRef<'a, T, N> {
    fn as_ref(&self) -> &'a [T; N] {
        self.0
    }
}

impl<'a, T, const N: usize> TryFrom<&'a [T]> for ArrayRef<'a, T, N> {
    type Error = &'static str;

    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        if value.len() != N {
            Err("the slice must contain exactly N elements")
        } else {
            unsafe { Ok(Self(&*(value.as_ptr() as *const [T; N]))) }
        }
    }
}
