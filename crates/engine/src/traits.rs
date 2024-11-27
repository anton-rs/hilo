//! Contains the engine api trait.

use alloy_eips::eip1898::BlockNumberOrTag;
use alloy_primitives::B256;
use alloy_rpc_types_engine::{
    ExecutionPayloadV3, ForkchoiceState, ForkchoiceUpdated, PayloadId, PayloadStatus,
};
use async_trait::async_trait;
use op_alloy_protocol::L2BlockInfo;
use op_alloy_rpc_types_engine::{OpExecutionPayloadEnvelopeV3, OpPayloadAttributes};

/// Engine trait specifies the interface between the hilo-engine and the engine-api.
///
/// See: <https://github.com/ethereum-optimism/optimism/blob/develop/op-node/rollup/engine/engine_controller.go#L39C1-L44C2>
#[async_trait]
pub trait Engine {
    type Error: core::fmt::Debug;

    /// Gets a payload for the given payload id.
    async fn get_payload(
        &self,
        payload_id: PayloadId,
    ) -> Result<OpExecutionPayloadEnvelopeV3, Self::Error>;

    /// Updates the forkchoice state with the given payload attributes.
    async fn forkchoice_update(
        &self,
        state: ForkchoiceState,
        attr: OpPayloadAttributes,
    ) -> Result<ForkchoiceUpdated, Self::Error>;

    /// Creates a new payload with the given payload and parent beacon block root.
    async fn new_payload(
        &self,
        payload: ExecutionPayloadV3,
        parent_beacon_block_root: B256,
    ) -> Result<PayloadStatus, Self::Error>;

    /// Returns the [L2BlockInfo] for the given label.
    async fn l2_block_ref_by_label(
        &self,
        label: BlockNumberOrTag,
    ) -> Result<L2BlockInfo, Self::Error>;
}
