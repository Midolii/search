# 简单的搜索文件小工具

## 配置说明

```yml
key: template # 搜索的关键字
entry: ./ # 搜索入口文件夹，可以为绝对路径，也可以为相对路径
output: ./search-output.json # 执行后默认输出搜索结果，目前不支持不填写
include_file: # 匹配关键字，功能待开发
exclude_file: # 忽略地址中包含此关键字的文件夹和文件
  - .git
  - target
exclude_extension: # 忽略包含此关键字的文件
```

## 执行方法

> 如果没有 cargo 的话可以参考如下链接进行安装 <https://rustwiki.org/zh-CN/cargo/getting-started/installation.html>

1. 根据上方配置说明配置 `application.yml`
2. 执行 `cargo run`

## 后期施工

1. 丰富配置参数，例如支持不向文件中进行写入结果
2. 增加 `json` 格式的配置以及 `args` 配置的能力
3. 增加适当的报错信息提示

## 注意

1. 执行后会在当前目录生成一个名为 `search-output.json`，此文件名称路径可修改，运行时会自动忽略该文件
