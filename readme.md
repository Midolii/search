# 简单的搜索文件小工具

## 配置

```yml
key: template # 搜索的关键字
entry: ./ # 搜索入口文件夹
output: ./search-output.json # 输出结果文件
include_file: # 匹配关键字，功能待开发
exclude_file: # 不匹配的关键字
  - .git
  - target
exclude_extension: # 不匹配的文件后缀
```
