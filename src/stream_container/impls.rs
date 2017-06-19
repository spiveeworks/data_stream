use stream_container::{StreamContainer};

use std::vec;

impl StreamContainer<u8> for [u8; 8]
{
    type Iter = vec::IntoIter<u8>;
    fn fill_with<I: Iterator<Item = u8>> (stream: &mut I) -> Option<Self>
    {
        let mut out = [0; 8];
        for x in &mut out
        {
            if let Some(thing) = stream.next()
            {
                *x = thing;
            }
            else
            {
                return None;
            }
        }
        Some(out)
    }
    fn into_stream(self) -> Self::Iter
      {self.to_vec().into_iter()}
}


