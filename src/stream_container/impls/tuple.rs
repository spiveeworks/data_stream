
macro_rules! tuple_iter_t
{
    ($T0: tt; $T1: tt) => (tuple_iter_t!($T0; $T1, ));
    ($T0: ty; $T1: ty, $($T:ty),*) => (tuple_iter_t!(iter::chain<$T0, $T1>; $($T),*));
    ($T0: ty; ) => ($T0);
}


macro_rules! tuple_impl_stream_container
{
    ($(
        $Tuple:ident 
        {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => 
    {
        $(
            impl<T, $($T:StreamContainer<T>),+> StreamContainer<T> for ($($T,)+)
            {
                type Iter = tuple_iter_t!(iter::Empty; $($T),+);
                fn fill_with<I> (iter: &mut I) -> Self
                    where I: Iterator,
                          <I as Iterator>::Item == T
                {
                    ($(
                        try_option!(<$T as StreamContainer<T>>::fill_with(iter))
                    ),+)
                }
                fn into_stream (self) -> Self::Iter
                {
                    iter::empty()
                    $(
                        .chain(<$T as StreamContainer<T>>::into_stream(self.$idx))
                    )+
                }

            }
        )+
    }
}

tuple_impl_stream_container! 
{
    Tuple1 
    {
        (0) -> A
    }
    Tuple2 
    {
        (0) -> A
        (1) -> B
    }
    /*
    Tuple3 
    {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 
    {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
    */
}
