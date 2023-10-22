#![no_std]

#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(associated_type_bounds)]

moddef::moddef!{
    flat(pub) mod {
        fn_grad,
        fn_der
    }
}