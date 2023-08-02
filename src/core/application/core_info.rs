use super::{allocations::ApplicationAllocations, lifecycle::ApplicationLifecycle};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApplicationInfo {
    pub core_information: ApplicationCoreInfo,
    pub application_lifecycle: ApplicationLifecycle,
    pub datacap_allocations: ApplicationAllocations,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApplicationCoreInfo {
    pub data_owner_name: String,
    pub data_owner_region: String,
    pub data_owner_industry: String,
    pub website: String,
    pub social_media: String,
}

impl ApplicationCoreInfo {
    pub fn new(
        data_owner_name: String,
        data_owner_region: String,
        data_owner_industry: String,
        website: String,
        social_media: String,
    ) -> Self {
        ApplicationCoreInfo {
            data_owner_name,
            data_owner_region,
            data_owner_industry,
            website,
            social_media,
        }
    }
}

impl ApplicationInfo {
    pub fn new(
        core_information: ApplicationCoreInfo,
        application_lifecycle: ApplicationLifecycle,
        datacap_allocations: ApplicationAllocations,
    ) -> Self {
        ApplicationInfo {
            core_information,
            application_lifecycle,
            datacap_allocations,
        }
    }
}
