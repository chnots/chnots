# 2508-13 Codemirror 内的 MDWT(Markdown With Toent) 高亮

## MDWT (Markdown With Toent) 是什么

这是 _Chnots_ 定义的一种 markdown 拓展标记

- hashtag  
  标签，_Chnots_ 中对文章进行分类的工具，同时可以用来模拟文件夹的功能。

  ```markdown
  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. #hastag #hash-tag #hash/tag
  ```

- backlink  
  双链，用来指向其他 chnots 中的块。

  ```markdown
  Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. [[backlink-id]]
  ```

- Toent  
  Toent (**To**do and Ev**ent**) 特有标记内容，用来进行事件和任务的标记。由 org-mode 启发而来。

  ```
  # [TODO !A] release plan
  ; event: 2025-08-13 08:00

  - [TODO] release new version
    ; event: 2025-09-13 08:00
  ```

- Props  
  用来对 markdown 中的**块**进行标记。块是指 heading 和 ListItem。

  在目前的设计中 `props` 主要包括两类内容。
  - 块 `id`: 用来供其他 chnot 进行索引
  - 任务状态转移：记录任务状态及变化时间

  ```
  # [TODO] item
  ; ID: item-id
  ; EVENT: 2025-12-02 12:00:00 ,12d **12d =2025-12-30
  ; STATE: TODO @ 2025-05-05 12:00:00 +8:00
  ;; NOTE: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
  ; STATE: DOING @ 2025-05-05 12:00:00 +8:00
  ; STATE: DONE @ 2025-05-05 15:00:00 +8:00
  ```

## 前端实现

本应用采用 Codemirror 作为 markdown 编辑器。如果需要实现上面的功能需要实现 `@lezer/markdown` 拓展，并将对应的标记进行渲染高亮。

这部分参考了 https://github.com/erykwalder/lezer-markdown-obsidian 中的很多代码，在此鸣谢。
