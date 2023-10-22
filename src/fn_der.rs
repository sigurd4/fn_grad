use crate::*;

pub trait FnDerOnce<Arg>: FnOnce<(Arg,)>
{
    type Derivative: FnOnce<(Arg,)>;

    fn into_derivative(self) -> Self::Derivative;
}

pub trait FnDerMut<Arg>: FnDerOnce<Arg> + FnMut<(Arg,)>
where
    Self::Derivative: FnMut<(Arg,)>
{
    fn as_derivate_mut(&mut self) -> &mut Self::Derivative;
}

pub trait FnDer<Arg>: FnDerMut<Arg> + Fn<(Arg,)>
where
    Self::Derivative: Fn<(Arg,)>
{
    fn as_derivate(&self) -> &Self::Derivative;
}

// IMPLEMENTATION

impl<F, Arg> FnGradOnce<(Arg,)> for F
where
    F: FnDerOnce<Arg>
{
    type Gradient = <Self as FnDerOnce<Arg>>::Derivative;

    fn into_gradient(self) -> Self::Gradient
    {
        self.into_derivative()
    }
}

impl<F, Arg> FnGradMut<(Arg,)> for F
where
    F: FnDerMut<Arg>,
    Self::Gradient: FnMut<(Arg,)>
{
    fn as_gradient_mut(&mut self) -> &mut Self::Gradient
    {
        self.as_derivate_mut()
    }
}

impl<F, Arg> FnGrad<(Arg,)> for F
where
    F: FnDer<Arg>,
    Self::Gradient: Fn<(Arg,)>
{
    fn as_gradient(&self) -> &Self::Gradient
    {
        self.as_derivate()
    }
}