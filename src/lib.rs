#![no_std]

#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(associated_type_bounds)]
#![feature(const_trait_impl)]

use core::marker::Tuple;

#[const_trait]
pub trait FnGradOnce<Args>: FnOnce<Args>
where
    Args: Tuple
{
    type Gradient: FnOnce<Args> + ?Sized;

    fn into_gradient(self) -> Self::Gradient
    where
        Self: Sized,
        Self::Gradient: Sized;
}

#[const_trait]
pub trait FnGradMut<Args>: FnGradOnce<Args, Gradient: FnMut<Args>> + FnMut<Args>
where
    Args: Tuple
{
    fn as_gradient_mut(&mut self) -> &mut Self::Gradient;
}

#[const_trait]
pub trait FnGrad<Args>: FnGradMut<Args, Gradient: Fn<Args>> + Fn<Args>
where
    Args: Tuple
{
    fn as_gradient(&self) -> &Self::Gradient;
}