#![allow(dead_code)]

use std::marker;

trait StreamContainer<T>
{
    type Iter: Iterator<Item = T>;
    fn fill_with<I: Iterator<Item = T>> (stream: &mut I) -> Option<Self>
        where Self: marker::Sized;
    fn into_stream (self) -> Self::Iter;
}

impl<T,C> StreamContainer<T> for C
    where C: StreamCast<T>
{
    type Iter = <<C as StreamCast<T>>::Base as StreamContainer<T>>::Iter;
    fn fill_with<I: Iterator<Item = T>> (stream: &mut I) -> Option<Self>
    {
        let container = C::Base::fill_with(stream);
        container.map(|base| C::from_base(base))
    }
    fn into_stream (self) -> Self::Iter
      {self.into_base().into_stream()}
}

trait StreamCast<T>
{
    type Base: StreamContainer<T>;
    fn into_base(self) -> Self::Base;
    fn from_base(Self::Base) -> Self;
}



/*
unsafe impl TransmuteStream<u8> for Test
{
    type UseBase = [u8;5];
}

unsafe trait TransmuteStream<T> 
  where Self: Sized,
        T: Sized,
{
    type UseBase: StreamContainer<T>;
}
impl<C,T> StreamCast<T> for C
    where C: TransmuteStream<T>
{
    type Base = C::UseBase;
    fn into_base(self) -> Self::Base
      {unsafe{std::mem::transmute::<Self,Self::Base>(self)}}
    fn from_base(base: Self::Base) -> Self
      {unsafe{std::mem::transmute::<Self::Base,Self>(base)}}
}
*/

