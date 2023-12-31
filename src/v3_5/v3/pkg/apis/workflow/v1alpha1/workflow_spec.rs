use serde::{Deserialize, Serialize};
use k8s_openapi::api::{core, policy};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSpec {
    /// Optional duration in seconds relative to the workflow start time which
    /// the workflow is allowed to run before the controller terminates
    /// the io.argoproj.workflow.v1alpha1.
    /// A value of zero is used to terminate a Running workflow
    #[serde(rename = "activeDeadlineSeconds", skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i32>,

    #[serde(rename = "affinity", skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Box<core::v1::Affinity>>,
    
    /// ArchiveLogs indicates if the container logs should be archived
    #[serde(rename = "archiveLogs", skip_serializing_if = "Option::is_none")]
    pub archive_logs: Option<bool>,

    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Box<super::Arguments>>,
    
    #[serde(rename = "artifactGC", skip_serializing_if = "Option::is_none")]
    pub artifact_gc: Option<Box<super::WorkflowLevelArtifactGC>>,

    #[serde(rename = "artifactRepositoryRef", skip_serializing_if = "Option::is_none")]
    pub artifact_repository_ref: Option<Box<super::ArtifactRepositoryRef>>,

    /// AutomountServiceAccountToken indicates whether a service account token
    /// should be automatically mounted in pods. ServiceAccountName of 
    /// ExecutorConfig must be specified if this value is false.
    #[serde(rename = "automountServiceAccountToken", skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,

    #[serde(rename = "dnsConfig", skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<Box<core::v1::PodDNSConfig>>,

    /// Set DNS policy for the pod. Defaults to \"ClusterFirst\". Valid values are 
    /// 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. 
    /// DNS parameters given in DNSConfig will be merged with the policy 
    /// selected with DNSPolicy. To have DNS options set along with hostNetwork, 
    /// you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    #[serde(rename = "dnsPolicy", skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,

    /// Entrypoint is a template reference to the starting point of the io.argoproj.workflow.v1alpha1.
    #[serde(rename = "entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,

    #[serde(rename = "executor", skip_serializing_if = "Option::is_none")]
    pub executor: Option<Box<super::ExecutorConfig>>,

    /// Hooks holds the lifecycle hook which is invoked at lifecycle of step, 
    /// irrespective of the success, failure, or error status of the primary step.
    #[serde(rename = "hooks", skip_serializing_if = "Option::is_none")]
    pub hooks: Option<::std::collections::HashMap<String, super::LifecycleHook>>,

    #[serde(rename = "hostAliases", skip_serializing_if = "Option::is_none")]
    pub host_aliases: Option<Vec<core::v1::HostAlias>>,

    /// Host networking requested for this workflow pod. Default to false.
    #[serde(rename = "hostNetwork", skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    
    /// ImagePullSecrets is a list of references to secrets in the same namespace 
    /// to use for pulling any images in pods that reference this ServiceAccount. 
    /// ImagePullSecrets are distinct from Secrets because Secrets can be mounted in the pod, 
    /// but ImagePullSecrets are only accessed by the kubelet. More info: 
    /// https://kubernetes.io/docs/concepts/containers/images/#specifying-imagepullsecrets-on-a-pod
    #[serde(rename = "imagePullSecrets", skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<Vec<core::v1::LocalObjectReference>>,

    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Box<super::Metrics>>,

    /// NodeSelector is a selector which will result in all pods of the workflow
    /// to be scheduled on the selected node(s). This is able to be overridden
    /// by a nodeSelector specified in the template.
    #[serde(rename = "nodeSelector", skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<::std::collections::HashMap<String, String>>,
    
    /// OnExit is a template reference which is invoked at the end of the workflow, irrespective 
    /// of the success, failure, or error of the primary io.argoproj.workflow.v1alpha1.
    #[serde(rename = "onExit", skip_serializing_if = "Option::is_none")]
    pub on_exit: Option<String>,

    /// Parallelism limits the max total parallel pods that can execute at the same time in a workflow
    #[serde(rename = "parallelism", skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,

    #[serde(rename = "podDisruptionBudget", skip_serializing_if = "Option::is_none")]
    pub pod_disruption_budget: Option<Box<policy::v1::PodDisruptionBudgetSpec>>,
    
    #[serde(rename = "podGC", skip_serializing_if = "Option::is_none")]
    pub pod_gc: Option<Box<super::PodGC>>,
    
    #[serde(rename = "podMetadata", skip_serializing_if = "Option::is_none")]
    pub pod_metadata: Option<Box<super::Metadata>>,

    /// Priority to apply to workflow pods. DEPRECATED: Use PodPriorityClassName instead.
    #[serde(rename = "podPriority", skip_serializing_if = "Option::is_none")]
    pub pod_priority: Option<i32>,

    /// PriorityClassName to apply to workflow pods.
    #[serde(rename = "podPriorityClassName", skip_serializing_if = "Option::is_none")]
    pub pod_priority_class_name: Option<String>,

    /// PodSpecPatch holds strategic merge patch to apply against the pod spec.
    /// Allows parameterization of container fields which are not strings (e.g. resource limits).
    #[serde(rename = "podSpecPatch", skip_serializing_if = "Option::is_none")]
    pub pod_spec_patch: Option<String>,

    /// Priority is used if controller is configured to process limited number
    /// of workflows in parallel. Workflows with higher priority are processed first.
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,

    #[serde(rename = "retryStrategy", skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<Box<super::RetryStrategy>>,
    
    /// Set scheduler name for all pods. Will be overridden if container/script
    /// template's scheduler name is set. Default scheduler will be used if neither specified.
    #[serde(rename = "schedulerName", skip_serializing_if = "Option::is_none")]
    pub scheduler_name: Option<String>,

    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    pub security_context: Option<Box<core::v1::PodSecurityContext>>,

    /// ServiceAccountName is the name of the ServiceAccount to run all pods of the workflow as.
    #[serde(rename = "serviceAccountName", skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,

    /// Shutdown will shutdown the workflow according to its ShutdownStrategy
    #[serde(rename = "shutdown", skip_serializing_if = "Option::is_none")]
    pub shutdown: Option<String>,
    
    /// Suspend will suspend the workflow and prevent execution of any future steps in the workflow
    #[serde(rename = "suspend", skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,

    #[serde(rename = "synchronization", skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<Box<super::Synchronization>>,

    #[serde(rename = "templateDefaults", skip_serializing_if = "Option::is_none")]
    pub template_defaults: Option<Box<super::Template>>,

    /// Templates is a list of workflow templates used in a workflow
    #[serde(rename = "templates", skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<super::Template>>,

    /// Tolerations to apply to workflow pods.
    #[serde(rename = "tolerations", skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<core::v1::Toleration>>,

    #[serde(rename = "ttlStrategy", skip_serializing_if = "Option::is_none")]
    pub ttl_strategy: Option<Box<super::TTLStrategy>>,

    #[serde(rename = "volumeClaimGC", skip_serializing_if = "Option::is_none")]
    pub volume_claim_gc: Option<Box<super::VolumeClaimGC>>,
    
    /// VolumeClaimTemplates is a list of claims that containers are allowed to reference.
    /// The Workflow controller will create the claims at the beginning of the workflow
    /// and delete the claims upon completion of the workflow.
    #[serde(rename = "volumeClaimTemplates", skip_serializing_if = "Option::is_none")]
    pub volume_claim_templates: Option<Vec<core::v1::PersistentVolumeClaim>>,

    /// Volumes is a list of volumes that can be mounted by containers
    /// in a io.argoproj.workflow.v1alpha1.
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<core::v1::Volume>>,

    #[serde(rename = "workflowMetadata", skip_serializing_if = "Option::is_none")]
    pub workflow_metadata: Option<Box<super::WorkflowMetadata>>,

    #[serde(rename = "workflowTemplateRef", skip_serializing_if = "Option::is_none")]
    pub workflow_template_ref: Option<Box<super::WorkflowTemplateRef>>,
}

impl WorkflowSpec {
    pub fn new() -> WorkflowSpec {
        WorkflowSpec {
            active_deadline_seconds: None,
            affinity: None,
            archive_logs: None,
            arguments: None,
            artifact_gc: None,
            artifact_repository_ref: None,
            automount_service_account_token: None,
            dns_config: None,
            dns_policy: None,
            entrypoint: None,
            executor: None,
            hooks: None,
            host_aliases: None,
            host_network: None,
            image_pull_secrets: None,
            metrics: None,
            node_selector: None,
            on_exit: None,
            parallelism: None,
            pod_disruption_budget: None,
            pod_gc: None,
            pod_metadata: None,
            pod_priority: None,
            pod_priority_class_name: None,
            pod_spec_patch: None,
            priority: None,
            retry_strategy: None,
            scheduler_name: None,
            security_context: None,
            service_account_name: None,
            shutdown: None,
            suspend: None,
            synchronization: None,
            template_defaults: None,
            templates: None,
            tolerations: None,
            ttl_strategy: None,
            volume_claim_gc: None,
            volume_claim_templates: None,
            volumes: None,
            workflow_metadata: None,
            workflow_template_ref: None,
        }
    }
}
