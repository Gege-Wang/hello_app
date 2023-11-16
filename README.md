
## 使用方法
1. 编写测试程序
在src目录下新建一个app[x]的文件夹，编写自己的main函数
2. 编译程序
```bash
cargo build --release --target riscv64gc-unknown-none-elf --bin app[x]
```
3. 运行脚本
```bash
./deploy.sh
```
4. 到arceos下查看程序是否加载成功

## pflash.img 镜像格式
apps_nums:usize ----->二进制程序的个数    
app_size:usize  ----->二进制程序的大小    
...    
[此处存放所有二进制程序的大小]     
app		---->二进制程序     
...    
[此处存放所有二进制程序]    
