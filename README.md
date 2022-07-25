# Simplest Steps to get started
Just run from VSCode (no need to pre-start OpenOCD)
![image](https://user-images.githubusercontent.com/7802334/180878631-a17a8889-f641-4066-9d69-58813f02e050.png)


# Simple Manual Steps to get started

1. `openocd`
2. `cargo run` (from a second command prompt as the openocd blocks the first one)

# More detailed explanation
OpenOCD should automatically use the `openocd.cfg` file.
![image](https://user-images.githubusercontent.com/7802334/180642878-bf306ea5-b4c9-4491-bd0e-0a7ba8b8b39a.png)

The cargo runner is set up to build --> connect to openocd --> flash and run.
![image](https://user-images.githubusercontent.com/7802334/180642930-ee3dae4e-3601-4b0e-9606-663997119ac6.png)

