#![cfg(test)]

use super::*;

use std::mem;


#[derive(PartialEq, Eq, Clone, Default, Debug)]
struct Test(u8, u32);


impl StreamCast<u8> for Test
{
    type Base = [u8; 8];
    fn into_base(self) -> Self::Base
    {
        unsafe
        {
            mem::transmute::<Self,Self::Base>(self)
        }
    }
    fn from_base(base: Self::Base) -> Self
      {unsafe{mem::transmute::<Self::Base,Self>(base)}}
}


#[test]
fn struct_streaming() 
{
    const X: u8 = 5;
    const Y: u32 = 1000000;
    let a = Test(X,Y);
    let b = a.clone();

    let mut a_bytes = a.into_stream();
    let maybe_a = StreamContainer::<u8>::fill_with(&mut a_bytes);
    let a: Test = maybe_a.expect("Ran out of bytes when reconstructing Test");

    assert_eq!(a, b);
}

