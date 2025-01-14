//! Unsigned (without proofs) transaction

use super::input::{Input, UnsignedInput};
#[cfg(feature = "json")]
use super::json;
use super::prover_result::ProverResult;
use super::DataInput;
use super::{
    super::{digest32::blake2b256_hash, ergo_box::ErgoBoxCandidate},
    Transaction, TxId,
};
use ergotree_interpreter::sigma_protocol::prover::ProofBytes;
use ergotree_ir::serialization::SigmaSerializationError;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "json")]
use std::convert::TryFrom;

/// Unsigned (inputs without proofs) transaction
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "json",
    serde(
        try_from = "json::transaction::UnsignedTransactionJson",
        into = "json::transaction::UnsignedTransactionJson"
    )
)]
#[derive(PartialEq, Debug, Clone)]
pub struct UnsignedTransaction {
    tx_id: TxId,
    /// unsigned inputs, that will be spent by this transaction.
    pub inputs: Vec<UnsignedInput>,
    /// inputs, that are not going to be spent by transaction, but will be reachable from inputs
    /// scripts. `dataInputs` scripts will not be executed, thus their scripts costs are not
    /// included in transaction cost and they do not contain spending proofs.
    pub data_inputs: Vec<DataInput>,
    /// box candidates to be created by this transaction
    pub output_candidates: Vec<ErgoBoxCandidate>,
}

impl UnsignedTransaction {
    /// Creates new transaction
    pub fn new(
        inputs: Vec<UnsignedInput>,
        data_inputs: Vec<DataInput>,
        output_candidates: Vec<ErgoBoxCandidate>,
    ) -> Result<UnsignedTransaction, SigmaSerializationError> {
        let tx_to_sign = UnsignedTransaction {
            tx_id: TxId::zero(),
            inputs,
            data_inputs,
            output_candidates,
        };
        let tx_id = tx_to_sign.calc_tx_id()?;
        Ok(UnsignedTransaction {
            tx_id,
            ..tx_to_sign
        })
    }

    fn calc_tx_id(&self) -> Result<TxId, SigmaSerializationError> {
        let bytes = self.bytes_to_sign()?;
        Ok(TxId(blake2b256_hash(&bytes)))
    }

    /// Get transaction id
    pub fn id(&self) -> TxId {
        self.tx_id.clone()
    }

    /// message to be signed by the [`ergotree_interpreter::sigma_protocol::prover::Prover`] (serialized tx)
    pub fn bytes_to_sign(&self) -> Result<Vec<u8>, SigmaSerializationError> {
        let empty_proofs_input = self
            .inputs
            .iter()
            .map(|ui| {
                Input::new(
                    ui.box_id.clone(),
                    ProverResult {
                        proof: ProofBytes::Empty,
                        extension: ui.extension.clone(),
                    },
                )
            })
            .collect();
        let tx = Transaction::new(
            empty_proofs_input,
            self.data_inputs.clone(),
            self.output_candidates.clone(),
        )?;
        tx.bytes_to_sign()
    }
}

#[cfg(feature = "json")]
impl From<UnsignedTransaction> for json::transaction::UnsignedTransactionJson {
    fn from(v: UnsignedTransaction) -> Self {
        json::transaction::UnsignedTransactionJson {
            inputs: v.inputs,
            data_inputs: v.data_inputs,
            outputs: v.output_candidates,
        }
    }
}

#[cfg(feature = "json")]
impl TryFrom<json::transaction::UnsignedTransactionJson> for UnsignedTransaction {
    // We never return this type but () fails to compile (can't format) and ! is experimental
    type Error = String;
    fn try_from(tx_json: json::transaction::UnsignedTransactionJson) -> Result<Self, Self::Error> {
        UnsignedTransaction::new(tx_json.inputs, tx_json.data_inputs, tx_json.outputs)
            .map_err(|e| format!("unsigned tx serialization failed: {0}", e))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use proptest::prelude::*;
    use proptest::{arbitrary::Arbitrary, collection::vec};

    impl Arbitrary for UnsignedTransaction {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                vec(any::<UnsignedInput>(), 1..10),
                vec(any::<DataInput>(), 0..10),
                vec(any::<ErgoBoxCandidate>(), 1..10),
            )
                .prop_map(|(inputs, data_inputs, outputs)| {
                    Self::new(inputs, data_inputs, outputs).unwrap()
                })
                .boxed()
        }
        type Strategy = BoxedStrategy<Self>;
    }

    proptest! {

        #![proptest_config(ProptestConfig::with_cases(16))]

        #[test]
        fn test_unsigned_tx_bytes_to_sign(v in any::<UnsignedTransaction>()) {
            prop_assert!(!v.bytes_to_sign().unwrap().is_empty());
        }

    }
}
