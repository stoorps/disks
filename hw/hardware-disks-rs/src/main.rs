use hardware_common::bytes_to_pretty;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");


   let blk_devices = disks_rs::BlockDevice::discover()?;

    for device in blk_devices {
         println!("Device: {} - {} - {}", device.name(), device.device().to_str().unwrap(), bytes_to_pretty(&device.size(), true));
         // Here you can call other methods on the device, like connect, mount, etc.
         // Example: device.connect().await.unwrap();
    }


    Ok(())

}