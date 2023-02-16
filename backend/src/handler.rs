#![allow(non_snake_case)]

// inspired by
// https://github.com/actix/actix-web/blob/web-v3.3.2/src/handler.rs
// https://github.com/actix/actix-web/blob/master/actix-web/src/handler.rs

use futures_util::Future;

pub trait Handler<Args>: Send + Sync + Clone {
  type Output;
  type Future: Future<Output = Self::Output>;
  fn call(&self, args: Args) -> Self::Future;
}

macro_rules! factory ({ $($param:ident)* } => {
    impl<Func, Fut, Out, $($param,)*> Handler<($($param,)*)> for Func
    where
    Func: Fn($($param,)*) -> Fut + Send + Sync + Clone,
    Fut: Future<Output = Out>,
    {
        type Output = Out;
        type Future = Fut;

        fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
            self($($param,)*)
        }
    }
});

factory! {}
factory! { A }
factory! { A B }
factory! { A B C }
factory! { A B C D }
factory! { A B C D E }
factory! { A B C D E F }
factory! { A B C D E F G }
factory! { A B C D E F G H }
factory! { A B C D E F G H I }
factory! { A B C D E F G H I J }
factory! { A B C D E F G H I J K }
factory! { A B C D E F G H I J K L }
factory! { A B C D E F G H I J K L M }
factory! { A B C D E F G H I J K L M N }
factory! { A B C D E F G H I J K L M N O }
factory! { A B C D E F G H I J K L M N O P }
factory! { A B C D E F G H I J K L M N O P Q }
factory! { A B C D E F G H I J K L M N O P Q R }
factory! { A B C D E F G H I J K L M N O P Q R S }
factory! { A B C D E F G H I J K L M N O P Q R S T}
