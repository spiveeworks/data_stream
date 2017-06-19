use stream_container::{StreamContainer};

use std::iter;


impl<T, A, B> StreamContainer<T> for (A, B)
    where A: StreamContainer<T>,
          B: StreamContainer<T>,
{
    type Iter = iter::Chain<<A as StreamContainer<T>>::Iter, <B as StreamContainer<T>>::Iter>;
    fn fill_with<I: Iterator<Item=T>> (stream: &mut I) -> Option<Self>
    {
        if let Some(a) = StreamContainer::<T>::fill_with(stream)
        {
            if let Some(b) = StreamContainer::<T>::fill_with(stream)
              { Some((a, b)) }
            else
              { None }
        }
        else
          { None }
    }
    fn into_stream(self) -> Self::Iter
    {
        self.0.into_stream().chain(self.1.into_stream())
    }
}


