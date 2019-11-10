# talent-plan
talent-plan works

## 将开发两个作业
第一 模拟shell， ls | sort,将结果保存在file中用json结构

# cross compiler

apt-get install -qq gcc-arm-linux-gnueabi

cat ~/.cargo/config
[target.armv5te-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"

cargo build --target=armv5te-unknown-linux-gnueabi
