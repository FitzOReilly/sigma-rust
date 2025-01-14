use crate::serialization::op_code::OpCode;
use crate::types::stype::SType;

use super::expr::Expr;
use super::expr::InvalidArgumentError;
use super::unary_op::UnaryOp;
use super::unary_op::UnaryOpTryBuild;
use crate::has_opcode::HasStaticOpCode;

/// Box value
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExtractAmount {
    /// Box (SBox type)
    pub input: Box<Expr>,
}

impl ExtractAmount {
    /// Type
    pub fn tpe(&self) -> SType {
        SType::SLong
    }
}

impl HasStaticOpCode for ExtractAmount {
    const OP_CODE: OpCode = OpCode::EXTRACT_AMOUNT;
}

impl UnaryOp for ExtractAmount {
    fn input(&self) -> &Expr {
        &self.input
    }
}

impl UnaryOpTryBuild for ExtractAmount {
    fn try_build(input: Expr) -> Result<Self, InvalidArgumentError> {
        input.check_post_eval_tpe(&SType::SBox)?;
        Ok(ExtractAmount {
            input: input.into(),
        })
    }
}

#[cfg(test)]
#[cfg(feature = "arbitrary")]
mod tests {
    use crate::mir::global_vars::GlobalVars;
    use crate::serialization::sigma_serialize_roundtrip;

    use super::*;

    #[test]
    fn ser_roundtrip() {
        let e: Expr = ExtractAmount {
            input: Box::new(GlobalVars::SelfBox.into()),
        }
        .into();
        assert_eq![sigma_serialize_roundtrip(&e), e];
    }
}
