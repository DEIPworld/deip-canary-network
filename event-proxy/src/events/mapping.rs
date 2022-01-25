#![allow(dead_code)]

use super::{Deip, Assets, DeipDao, DeipProposal, DomainEventData};

use node_template_runtime::{Event, Runtime};

///
/// Code in this module is not intended to be called.
/// This is just for compile-time check that every Event in
/// the node Runtime has corresponding entities in `frame/{mod_name}.rs`
/// and corresponding variants in the enum DomainEventData.
///
/// The common recipe is the following:
/// 1. add corresponding entity to `frame/{mod_name}.rs`
/// 2. if the entity uses new types then register them in `types.rs`.
///     You have to set them in RuntimeT too; compiler will help
/// 3. add corresponding variant to DomainEventData
/// 4. add corresponding arm to match in the `known_domain_events`
/// 5. edit this file to settle the compile failure.
///

fn match_event<T>(e: &Event) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipDao + Assets,
{
    match e {
        Event::DeipDao(deip_dao_event) => match_event_deip_dao(deip_dao_event),

        Event::Deip(deip_event) => match_event_deip(deip_event),

        Event::DeipProposal(deip_proposal_event) => {
            match_event_deip_proposal(deip_proposal_event)
        }

        Event::ParityTechAssets(assets_event) => match_event_deip_assets(assets_event),
        
        Event::Session(..) | Event::ImOnline(..)
        
        | Event::OctopusAppchain(..) | Event::OctopusLpos(..) | Event::OctopusUpwardMessages(..) 

        | Event::System(_)
        | Event::Utility(_)
        | Event::Grandpa(_)
        | Event::Balances(_)
        | Event::Sudo(_)
        // | Event::template(_)
        | Event::DeipVesting(_)
        | Event::Multisig(_) => unreachable!(),
    }
}

fn match_event_deip_dao<T>(e: &pallet_deip_dao::Event<Runtime>) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipDao + Assets,
{
    use pallet_deip_dao::Event::*;

    match e {
        DaoCreate(_) => {
            /* deip_dao::DaoCreateEvent */
            unimplemented!()
        }
        DaoAlterAuthority(_) => {
            /* deip_dao::DaoTransferOwnershipEvent */
            unimplemented!()
        }
        DaoMetadataUpdated(_) => {
            /* deip_dao::DaoMetadataUpdatedEvent */
            unimplemented!()
        }
        __Ignore(..) => unreachable!(),
    }
}

fn match_event_deip_proposal<T>(e: &pallet_deip_proposal::Event<Runtime>) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipDao + Assets,
{
    use pallet_deip_proposal::Event::*;

    match e {
        Proposed { .. } => {
            /* deip_proposal::ProposedEvent */
            unimplemented!()
        }
        Approved { .. } => {
            /* deip_proposal::ApprovedEvent */
            unimplemented!()
        }
        RevokedApproval { .. } => {
            /* deip_proposal::RevokedApprovalEvent */
            unimplemented!()
        }
        Resolved { .. } => {
            /* deip_proposal::ResolvedEvent */
            unimplemented!()
        }
        Expired { .. } => {
            /* deip_proposal::ExpiredEvent */
            unimplemented!()
        }
        __Ignore(..) => unreachable!(),
    }
}

fn match_event_deip_assets<T>(
    e: &pallet_deip_assets::pallet_assets::Event<Runtime>,
) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipDao + Assets,
{
    use pallet_deip_assets::pallet_assets::Event::*;

    match e {
        Created(..) => {
            /* deip_assets::CreatedEvent */
            unimplemented!()
        }
        Issued(..) => {
            /* deip_assets::IssuedEvent */
            unimplemented!()
        }
        Transferred(..) => {
            /* deip_assets::TransferredEvent */
            unimplemented!()
        }
        Burned(..) => {
            /* deip_assets::BurnedEvent */
            unimplemented!()
        }
        TeamChanged(..) => {
            /* deip_assets::TeamChangedEvent */
            unimplemented!()
        }
        OwnerChanged(..) => {
            /* deip_assets::OwnerChangedEvent */
            unimplemented!()
        }
        #[cfg(not(feature = "octopus"))]
        ForceTransferred(..) => {
            /* deip_assets::ForceTransferredEvent */
            unimplemented!()
        }
        Frozen(..) => {
            /* deip_assets::FrozenEvent */
            unimplemented!()
        }
        Thawed(..) => {
            /* deip_assets::ThawedEvent */
            unimplemented!()
        }
        AssetFrozen(..) => {
            /* deip_assets::AssetFrozenEvent */
            unimplemented!()
        }
        AssetThawed(..) => {
            /* deip_assets::AssetThawedEvent */
            unimplemented!()
        }
        Destroyed(..) => {
            /* deip_assets::DestroyedEvent */
            unimplemented!()
        }
        ForceCreated(..) => {
            /* deip_assets::ForceCreatedEvent */
            unimplemented!()
        }
        #[cfg(not(feature = "octopus"))]
        MaxZombiesChanged(..) => {
            /* deip_assets::MaxZombiesChangedEvent */
            unimplemented!()
        }
        MetadataSet(..) => {
            /* deip_assets::MetadataSetEvent */
            unimplemented!()
        }
        #[cfg(feature = "octopus")]
        MetadataCleared(..) => unimplemented!(),
        #[cfg(feature = "octopus")]
        ApprovedTransfer(..) => unimplemented!(),
        #[cfg(feature = "octopus")]
        ApprovalCancelled(..) => unimplemented!(),
        #[cfg(feature = "octopus")]
        TransferredApproved(..) => unimplemented!(),
        #[cfg(feature = "octopus")]
        AssetStatusChanged(..) => unimplemented!(),
        
        __Ignore(..) => unreachable!(),
    }
}

fn match_event_deip<T>(e: &pallet_deip::Event<Runtime>) -> DomainEventData<T>
where
    T: DeipProposal + Deip + DeipDao + Assets,
{
    use pallet_deip::RawEvent::*;

    match e {
        ProjectCreated(..) => {
            /* deip::ProjectCreatedEvent */
            unimplemented!()
        }
        ProjectRemoved(..) => {
            /* deip::ProjectRemovedEvent */
            unimplemented!()
        }
        ProjectUpdated(..) => {
            /* deip::ProjectUpdatedEvent */
            unimplemented!()
        }
        ProjectContnetCreated(..) => {
            /* deip::ProjectContentCreatedEvent */
            unimplemented!()
        }
        NdaCreated(..) => {
            /* deip::NdaCreatedEvent */
            unimplemented!()
        }
        NdaAccessRequestCreated(..) => {
            /* deip::NdaAccessRequestCreatedEvent */
            unimplemented!()
        }
        NdaAccessRequestFulfilled(..) => {
            /* deip::NdaAccessRequestFulfilledEvent */
            unimplemented!()
        }
        NdaAccessRequestRejected(..) => {
            /* deip::NdaAccessRequestRejectedEvent */
            unimplemented!()
        }
        DomainAdded(..) => {
            /* deip::DomainAddedEvent */
            unimplemented!()
        }
        ReviewCreated(..) => {
            /* deip::ReviewCreatedEvent */
            unimplemented!()
        }
        ReviewUpvoted(..) => {
            /* deip::ReviewUpvotedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingCreated(..) => {
            /* deip::SimpleCrowdfundingCreatedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingActivated(..) => {
            /* deip::SimpleCrowdfundingActivatedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingFinished(..) => {
            /* deip::SimpleCrowdfundingFinishedEvent */
            unimplemented!()
        }
        SimpleCrowdfundingExpired(..) => {
            /* deip::SimpleCrowdfundingExpiredEvent */
            unimplemented!()
        }
        Invested(..) => {
            /* deip::InvestedEvent */
            unimplemented!()
        }
        ContractAgreementCreated(..) => {
            /* deip::ContractAgreementCreatedEvent */
            unimplemented!()
        }
        ContractAgreementAccepted(..) => {
            /* deip::ContractAgreementAcceptedEvent */
            unimplemented!()
        }
        ContractAgreementFinalized(..) => {
            /* deip::ContractAgreementFinalizedEvent */
            unimplemented!()
        }
        ContractAgreementRejected(..) => {
            /* deip::ContractAgreementRejectedEvent */
            unimplemented!()
        }
    }
}
