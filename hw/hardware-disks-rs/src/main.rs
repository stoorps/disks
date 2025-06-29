
use disks_rs_partitioning::{blkpg, gpt::partition_types::{Type, LINUX_FS}, writer::DiskWriter, FilesystemExt, GptAttributes, PartitionAttributes, TableAttributes};
use disks_rs_superblock::ext4::Ext4;
use disks_rs_types::Filesystem;
use hardware_common::bytes_to_pretty;
use tracing_subscriber::filter;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    tracing_subscriber::fmt()
        .with_max_level(filter::LevelFilter::DEBUG)
        .init();


   let blk_devices = disks_rs::BlockDevice::discover()?;

    for device in &blk_devices {
         println!("Device: {} - {} - {}", device.name(), device.device().to_str().unwrap(), bytes_to_pretty(&device.size(), true));
         for partition in device.partitions() {
             println!("  Partition: {} - {} - {}", partition.name, partition.device.to_str().unwrap(), bytes_to_pretty(&partition.size, true));
         }


         // Here you can call other methods on the device, like connect, mount, etc.
         // Example: device.connect().await.unwrap();
    }

    let device = blk_devices.iter().find(|d| d.name().contains("nvme0n1")).unwrap();

    println!("Selected Device: {} - {}", device.name(), bytes_to_pretty(&device.size(), true));

    let mut planner = disks_rs_partitioning::planner::Planner::new(device);
                               // .with_start_offset(0)
                              //  .with_end_offset(device.size() -1);


    let _ = planner.plan_initialize_disk();

    let usable_size = planner.usable_size() - 4096_000_000;
    println!("Usable size: {}", bytes_to_pretty(&usable_size, true));

    planner.plan_add_partition_with_attributes(0, usable_size -1,
        Some(PartitionAttributes {
            table: TableAttributes::Gpt(GptAttributes {
                name: Some("Test partition".to_owned()),
                uuid: None,
                type_guid: LINUX_FS,
            }),
            role: None, // Replace `todo!()` with the actual `PartitionRole` value
            filesystem: Some(Filesystem::Standard{
                filesystem_type: disks_rs_types::StandardFilesystemType::Ext4,
                label: Some("Test Filesystem".to_owned()),
                uuid: None,
            })
        })
    )?;

    let writer = DiskWriter::new(device, &planner);



    if let Err(e) = writer.simulate()
    {
        eprintln!("Simulation failed: {}", e);
        return Err(e.into());
    }
    println!("Simulation successful, writing to disk...");

    if let Err(e) = writer.write()
    {
        eprintln!("Write failed: {}", e);
        return Err(e.into());
    }
    println!("Write successful!");

    if let Err(e) = blkpg::sync_gpt_partitions(device.device().to_path_buf().as_path()){
        eprintln!("Failed to sync GPT partitions: {}", e);
        return Err(e.into());
    }







    Ok(())

}