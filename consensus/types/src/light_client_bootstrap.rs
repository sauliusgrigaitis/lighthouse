use super::{BeaconBlockHeader, BeaconState, EthSpec, FixedVector, Hash256, SyncCommittee};
use crate::{light_client_update::*, test_utils::TestRandom};
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use std::sync::Arc;
use test_random_derive::TestRandom;
use tree_hash::TreeHash;

/// A LightClientBootstrap is the initializer we send over to lightclient nodes
/// that are trying to generate their basic storage when booting up.
#[cfg_attr(feature = "arbitrary-fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode, TestRandom)]
#[serde(bound = "T: EthSpec")]
pub struct LightClientBootstrap<T: EthSpec> {
    /// Requested beacon block header.
    pub header: BeaconBlockHeader,
    /// The `SyncCommittee` used in the requested period.
    pub current_sync_committee: Arc<SyncCommittee<T>>,
    /// Merkle proof for sync committee
    pub current_sync_committee_branch: FixedVector<Hash256, CurrentSyncCommitteeProofLen>,
}

impl<T: EthSpec> LightClientBootstrap<T> {
    pub fn from_beacon_state(beacon_state: BeaconState<T>) -> Result<Self, Error> {
        let mut header = beacon_state.latest_block_header().clone();
        header.state_root = beacon_state.tree_hash_root();
        Ok(LightClientBootstrap {
            header,
            current_sync_committee: beacon_state.current_sync_committee()?.clone(),
            /// TODO(Giulio2002): Generate Merkle Proof, this is just empty hashes
            current_sync_committee_branch: FixedVector::new(vec![
                Hash256::zero();
                CURRENT_SYNC_COMMITTEE_PROOF_LEN
            ])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MainnetEthSpec;

    ssz_tests!(LightClientBootstrap<MainnetEthSpec>);
}