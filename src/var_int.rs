use std::iter;
use std::vec;


struct VarInt {as_u64: u64}

impl<T> From<T> for VarInt
    where T: Into<u64>
{
    fn from(x: T) -> Self { VarInt{ as_u64: Into::into(x) } }
}

impl StreamContainer<u8> for VarInt
{
    fn fill_with<I: Iterator<Item=u8>> (stream: &mut I) -> Option<VarInt>
    {
        match stream.next()
        { 
            Some(255) => u64::fill_with(stream).map(From::from),
            Some(254) => u32::fill_with(stream).map(From::from),
            Some(253) => u16::fill_with(stream).map(From::from),
            maybe_x   =>                maybe_x.map(From::from),
        }
    }

    type Iter = iter::Chain<iter::Once<u8>, vec::IntoIter<u8>>;
    fn into_stream(self) -> Self::Iter 
    {
        let (head, tail) = match self.as_u64
        {
            x @ 0 ... 252      => (x as u8, vec::Vec::new()),
            x @ 0 ... u16::MAX => (253, vec::Vec::from_iter((x as u16).into_stream())),
            x @ 0 ... u32::MAX => (254, vec::Vec::from_iter((x as u32).into_stream())),
            x                  => (255, vec::Vec::from_iter( x        .into_stream())),
        };
        iter::once(head).chain(tail.into_iter())
    }
}
