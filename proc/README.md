# proc

The  proc  filesystem  is  a pseudo-filesystem which provides an interface to the linux kernel data structures.  It is commonly mounted at /proc. We can read this fs and gather useful information such as CPU usage, disk and memory information and many such information.

This project is about writing experimental scripts using the `procfs` crate to gather such information.
