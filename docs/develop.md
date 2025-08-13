# 开发流程

开发过程说的是，一个新功能从想法到落地的完整过程。

1. 在 `/docs` 创建 `yymm-dd-task-desc.md` 文件，说明任务内容。
2. 切 `feat/yymm-dd-task-desc` 分支并创建 task-init commit，格式为：

   ```markdown
   task-init: task description.
   ```

3. 在功能实现过程中，实时在文件中更新想法、逻辑和问题。
4. 功能落地后，将该任务文件移动到 `/docs/done` 目录中，并创建 `task-done` commit。
