# 🚧 Disks 🚧

> [!CAUTION]
> This project is currently in an early prototyping phase. *DO NOT* use this for perfoming operations on disks yet!
---
A Disk utility application for the Cosmic Desktop.


### Current plans

#### Decoupling from DBus
At the moment, Disks is tightly coupled to DBus in ways that I don't think make sense. Also, the hardware interop code is a bit of a mess. 
I plan to use [disks-rs](https://github.com/AerynOS/disks-rs) for as much functionality as possible, which should enable cleaner and more direct access to disks/partitions.
There will be some functionality that is not currently supported by `disks-rs`, and so Dbus will fill in where functionality is currently lacking.

#### Better UI/UX
The UI of disks is essentially a clone of Gnome Disks at the moment. There are plans to focus on this and improve it once the lower-level functionality is somewhat complete.

![Screenshot of cosmos-disks](https://github.com/stoorps/cosmos-apps/blob/main/screenshots/cosmos-disks.png)


### Project structure

#### disks-ui
The application.

#### disks-dbus
This project is an abstraction layer for dbus interfaces. The idea here is to provide models that can easily be swapped out at a later date, as better suited rust crates become available for achieving the same functionality.
