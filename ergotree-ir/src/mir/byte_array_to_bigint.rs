//! Convert byte array to SBigInt
use crate::serialization::op_code::OpCode;
use crate::types::stype::SType;

use super::expr::Expr;
use super::expr::InvalidArgumentError;
use super::unary_op::UnaryOp;
use super::unary_op::UnaryOpTryBuild;

/// Convert byte array to SBigInt
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ByteArrayToBigInt {
    /// Byte array with SColl(SByte) expr type
    pub input: Box<Expr>,
}

impl ByteArrayToBigInt {
    pub(crate) const OP_CODE: OpCode = OpCode::BYTE_ARRAY_TO_BIGINT;

    /// Type
    pub fn tpe(&self) -> SType {
        SType::SBigInt
    }

    pub(crate) fn op_code(&self) -> OpCode {
        Self::OP_CODE
    }
}

impl UnaryOp for ByteArrayToBigInt {
    fn input(&self) -> &Expr {
        &self.input
    }
}

impl UnaryOpTryBuild for ByteArrayToBigInt {
    fn try_build(input: Expr) -> Result<Self, InvalidArgumentError> {
        input.check_post_eval_tpe(SType::SColl(Box::new(SType::SByte)))?;
        Ok(ByteArrayToBigInt {
            input: Box::new(input),
        })
    }
}

#[cfg(feature = "arbitrary")]
/// Arbitrary impl
mod arbitrary {
    use crate::mir::expr::arbitrary::ArbExprParams;

    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for ByteArrayToBigInt {
        type Strategy = BoxedStrategy<Self>;
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            any_with::<Expr>(ArbExprParams {
                tpe: SType::SColl(SType::SByte.into()),
                depth: 0,
            })
            .prop_map(|input| Self {
                input: input.into(),
            })
            .boxed()
        }
    }
}

#[cfg(test)]
#[cfg(feature = "arbitrary")]
mod tests {
    use super::*;
    use crate::serialization::sigma_serialize_roundtrip;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn ser_roundtrip(v in any::<ByteArrayToBigInt>()) {
            let expr: Expr = v.into();
            prop_assert_eq![sigma_serialize_roundtrip(&expr), expr];
        }
    }
}
