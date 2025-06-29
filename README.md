# Disks
A Disk utilty application for the Cosmic Desktop.

At the moment, I'm using usdisks2-rs and zbus as a basis for this application, but have my eye on disks-rs, and may decide to start using it/contributing to it in the near future.

The code (which is currently in a rough prototyping phase) is available here.  The UI is essentially a clone of gnome disks, but I have some plans to improve this in the future. 

The goal of this project is to ship as a part of the [Cosmic Utilities](https://github.com/cosmic-utils) organisation once it's ready.

![Screenshot of cosmos-disks](https://github.com/stoorps/cosmos-apps/blob/main/screenshots/cosmos-disks.png)


## disks-dbus
This project is an abstraction layer for dbus interfaces. The idea here is to provide models that can easily be swapped out at a later date, as better suited rust crates become available for achieving the same functionality.
