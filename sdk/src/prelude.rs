//! Prelude for easy importing of required types when defining workflow and activity definitions

/// Registry prelude
#[allow(unused_imports)]
pub mod registry {
    pub use crate::workflow::into_workflow;
}

/// Activity prelude
#[allow(unused_imports)]
pub mod activity {
    pub use crate::{ActContext, ActExitValue, ActivityError};
    pub use serde::{Deserialize, Serialize};
    pub use temporal_sdk_core_protos::temporal::api::failure::v1::Failure;
}

/// Workflow prelude
#[allow(unused_imports)]
pub mod workflow {
    pub use crate::workflow::AsyncFn;
    pub use crate::workflow::{execute_activity, execute_child_workflow, sleep};
    pub use crate::{
        ActivityOptions, ChildWorkflowOptions, LocalActivityOptions, Signal, SignalData,
        SignalWorkflowOptions, WfContext, WfExitValue, WorkflowResult,
    };
    pub use futures::FutureExt;
    pub use serde::{Deserialize, Serialize};
    pub use std::{
        fmt::{Debug, Display},
        future::Future,
        time::Duration,
    };
    pub use temporal_sdk_core_protos::{
        coresdk::{
            AsJsonPayloadExt, FromPayloadsExt,
            activity_result::{self, activity_resolution},
            child_workflow::{Failure, Success, child_workflow_result},
            workflow_commands::ActivityCancellationType,
        },
        temporal::api::common::v1::Payload,
    };
}

/// Worker prelude
#[allow(unused_imports)]
pub mod worker {
    pub use crate::{Worker, sdk_client_options};
    pub use temporal_sdk_core::{CoreRuntime, Url, init_worker};
    pub use temporal_sdk_core_api::{
        errors::WorkflowErrorType,
        telemetry::TelemetryOptionsBuilder,
        worker::{
            PollerBehavior, WorkerConfigBuilder, WorkerDeploymentOptions, WorkerDeploymentVersion,
            WorkerTuner, WorkerVersioningStrategy,
        },
    };
}

/// Client prelude
#[allow(unused_imports)]
pub mod client {
    pub use crate::sdk_client_options;
    pub use temporal_client::{
        Client, RetryClient, WfClientExt, WorkflowClientTrait, WorkflowOptions,
    };
}
