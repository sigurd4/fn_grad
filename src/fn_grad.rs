use core::marker::Tuple;

pub trait FnGradOnce<Args>: FnOnce<Args>
where
    Args: Tuple
{
    type Gradient: FnOnce<Args>;

    fn into_gradient(self) -> Self::Gradient;
}

pub trait FnGradMut<Args>: FnGradOnce<Args, Gradient: FnMut<Args>> + FnMut<Args>
where
    Args: Tuple
{
    fn as_gradient_mut(&mut self) -> &mut Self::Gradient;
}

pub trait FnGrad<Args>: FnGradMut<Args, Gradient: Fn<Args>> + Fn<Args>
where
    Args: Tuple
{
    fn as_gradient(&self) -> &Self::Gradient;
}