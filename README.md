# xmly-rs

rust语言写的喜马拉雅签到做任务脚本。

## 使用

### 源码方式
1. `git clone https://github.com/ClassmateLin/xmly-rs.git && cd xmly-rs;`。
2. `vim Settings.toml`, 填写token_list（token可通过抓包app的cookies找到`1&_token`）, 例如:

```toml
token_list = [
    "407821822&007FB240340C674176BD0ACF96FE912E8D8154860BE57FE3D834"
]
```
3. cargo run 

### 二进制

1.[点击下载二进制包](https://github.com/ClassmateLin/xmly-rs/releases), 在Settings.toml目录填token。
2. ./xmly执行
