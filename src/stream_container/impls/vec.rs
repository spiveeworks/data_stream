use super::{StreamContainer};

use std::iter;
use std::vec;

impl<T> StreamContainer<T> for Vec<T>
  where usize: StreamContainer<T>
{
    type Iter = iter::Chain<<usize as StreamContainer<T>>::Iter, vec::IntoIter::<T>>;
    fn fill_with<I: Iterator<Item=T>> (stream: &mut I) -> Self
    {
        let len: usize = StreamContainer::fill_with(stream);
        let result = vec::Vec::with_capacity(len);
        for _ in 0..len
        {
            if let Some(x) = stream.next()
            {
                result.push(x);
            }
            else
            {
                return None;
            }
        }
        result
    }
    
    fn into_stream (self) -> Self::Iter
      { self.capacity().into_stream().chain(self.into_iter()) }
}
