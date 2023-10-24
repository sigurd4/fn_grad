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
    type GradientOutput;

    fn gradient_once(self, args: Args) -> Self::GradientOutput;
}

#[const_trait]
pub trait FnGradMut<Args>: FnGradOnce<Args> + FnMut<Args>
where
    Args: Tuple
{
    fn gradient_mut(&mut self, args: Args) -> Self::GradientOutput;
}

#[const_trait]
pub trait FnGrad<Args>: FnGradMut<Args> + Fn<Args>
where
    Args: Tuple
{
    fn gradient(&self, args: Args) -> Self::GradientOutput;
}