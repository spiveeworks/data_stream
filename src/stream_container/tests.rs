#![cfg(test)]

use super::*;

#[derive(PartialEq, Eq, Clone, Default)]
struct Test(u8, u32);


impl StreamCast<u8> for Test
{
    type Base = [u8; 8];
    fn into_base(self) -> Self::Base
    {
        unsafe
        {
            std::mem::transmute::<Self,Self::Base>(self)
        }
    }
    fn from_base(base: Self::Base) -> Self
      {unsafe{std::mem::transmute::<Self::Base,Self>(base)}}
}


#[test]
fn struct_streaming() 
{
    let x: const u8 = 5;
    let y: const u32 = 1000000;
    let a = Test(x,y);
    let b = a.clone();

    let a_bytes = a.into_stream();
    let a = StreamContainer<u8>::fill_with(a_bytes);

    assert_eq(a, b);
}

