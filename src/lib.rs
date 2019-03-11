
#[derive(Default,Debug,Eq,PartialEq)]
pub struct NilIter<T>(std::marker::PhantomData<Option<T>>);

impl<T> Iterator for NilIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        None
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn silly() {
        assert_eq!(NilIter::<u8>::default(), NilIter::<u8>::default());
    }
}
