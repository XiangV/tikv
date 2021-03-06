// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::coprocessor::codec::data_type::*;
use crate::coprocessor::dag::expr::EvalContext;
use crate::coprocessor::Result;

/// A trait for all summable types.
///
/// This trait is used to implement `AVG()` and `SUM()` by using generics.
pub trait Summable: Evaluable {
    /// Returns the zero value.
    fn zero() -> Self;

    /// Adds assign another value.
    fn add_assign(&mut self, ctx: &mut EvalContext, other: &Self) -> Result<()>;
}

impl Summable for Decimal {
    #[inline]
    fn zero() -> Self {
        Decimal::zero()
    }

    #[inline]
    fn add_assign(&mut self, _ctx: &mut EvalContext, other: &Self) -> Result<()> {
        // TODO: If there is truncate error, should it be a warning instead?
        let r: crate::coprocessor::codec::Result<Decimal> = (self as &Self + other).into();
        *self = r?;
        Ok(())
    }
}

impl Summable for Real {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn add_assign(&mut self, _ctx: &mut EvalContext, other: &Self) -> Result<()> {
        *self += other;
        Ok(())
    }
}
