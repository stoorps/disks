use anyhow::Result;

pub trait Partition {
    fn pretty_name(&self) -> String;

    fn connect(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;


    fn mount(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    fn unmount(&self) -> impl std::future::Future<Output = Result<()>> + Send;


    fn delete(&self) -> impl std::future::Future<Output = Result<()>> + Send;


    fn format(&self, name: String, erase: bool, partion_type: String) -> impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement
    fn edit_partition(&self, partition_type: String, name: String, flags: u64) -> impl std::future::Future<Output = Result<()>> + Send;



   //TODO: implement
    fn edit_filesystem_label(&self, label: String) -> impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement
    fn change_passphrase(&self) ->impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement
    fn resize(&self, new_size_bytes: u64) ->impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement
    fn check_filesystem(&self) ->impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement
    fn repair_filesystem(&self) ->impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement
    fn take_ownership(&self, recursive: bool) ->impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement. See how edit mount options -> User session defaults works in gnome-disks.
    fn default_mount_options(&self) -> impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement. Look at gnome-disks -> partition -> edit mount options. Likely make all params optional.
    fn edit_mount_options(&self, mount_at_startup: bool, show_in_ui: bool, requre_auth: bool,display_name: Option<String>,
                                   icon_name: Option<String>, symbolic_icon_name: Option<String>,  options: String,
                                   mount_point: String, identify_as: String, file_system_type: String) ->impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement
    fn edit_encrytion_options(&self) ->impl std::future::Future<Output = Result<()>> + Send;


   //TODO: implement. creates a *.img of self.
    fn create_image(&self, output_path: String) ->impl std::future::Future<Output = Result<()>> + Send;

}