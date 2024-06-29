# GenUI Ignore

在`.gen_ignore`中是GenUI项目需要忽略监视的文件或目录

- 使用相对路径，相对于当前的GenUI项目目录而言
- 精确到某个文件，请不要使用类似`**/*.xxx`的方式进行忽略
- 使用换行进行分割
- 当你不添加时默认使用下方默认忽略内容

默认的忽略文件如下所示:

```
Cargo.toml
src/main.rs
.gitignore
Cargo.lock
target
.gen_cache
.gen_ignore
```