# xmly-rs

rust语言写的喜马拉雅签到做任务脚本。

## 使用

1. `git clone https://github.com/ClassmateLin/xmly-rs.git && cd xmly-rs;`或者下载二进制文件。
2. `vim Settings.toml`, 填写token_list（token可通过抓包app的cookies找到`1&_token`）, 例如:

```toml
token_list = [
    "407821822&007FB240340C674176BD0ACF96FE912E8D8154860BE57FE3D834"
]
```
3. cargo run 

