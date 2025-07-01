use anyhow::Result;

use crate::CreatePartitionInfo;

pub trait Drive {
    fn pretty_name(&self) -> String;

    fn eject(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    fn power_off(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    fn create_partition(
        &self,
        info: CreatePartitionInfo,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    //async fn get_drive_paths(connection: &Connection) -> Result<Vec<DriveBlockPair>>;

    //async fn get_drives() -> Result<Vec<DriveModel>>;
}
