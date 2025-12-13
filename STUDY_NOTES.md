# Trae 结对编程学习笔记 (2025-12-13)

## 1. 编辑器基础：GNU nano
**如何保存并退出：**
1.  按 `Ctrl + O` (Write Out) -> 回车 (确认文件名) -> 保存。
2.  按 `Ctrl + X` (Exit) -> 退出。
3.  **快捷方式**：直接按 `Ctrl + X` -> 输入 `Y` (Yes) -> 回车。

## 2. Git 多远程仓库管理
当你有一个项目同时属于 `origin` (自己的仓库) 和 `template` (模板仓库) 时：

*   **查看远程仓库**：
    ```bash
    git remote -v
    ```
*   **添加新远程**：
    ```bash
    git remote add template <URL>
    ```
*   **推送到指定仓库**：
    ```bash
    git push origin main    # 推送到 rcli
    git push template main  # 推送到 template
    ```
*   **修改默认推送目标**：
    ```bash
    git branch -u origin/main  # 设置当前分支默认追踪 origin/main
    ```

## 3. CI/CD 与 GitHub Actions
**文件位置**：`.github/workflows/build.yml`

*   **CI (持续集成)**：自动运行测试 (`cargo check`, `cargo nextest`)、格式检查 (`cargo fmt`) 和 Lint (`cargo clippy`)。
*   **CD (持续交付)**：打标签 (`v*`) 时自动生成 Changelog 并发布 Release。

**遇到的问题与修复**：
1.  **触发分支**：将 `master` 改为 `main`。
2.  **Action 版本过旧**：将 `orhun/git-cliff-action@v2` 升级到 `@v4` (解决 Docker/Node 兼容性问题)。
3.  **无测试报错**：`cargo nextest` 默认无测试会报错，添加 `--no-tests=pass` 参数修复。

## 4. Git 核心概念
*   **Tag (标签)**：给某个 Commit 贴的永久便利贴，常用于标记版本号 (如 `v1.0.0`)。
*   **Diff (差异)**：
    *   `git diff`：看工作区 vs 暂存区。
    *   `git diff --staged`：看暂存区 vs 上次提交。
*   **Branch (分支)**：
    *   `main` vs `master`：本质没区别，`main` 是现在的政治正确默认名。
    *   `git checkout -b <name>`：创建并切换分支。
*   **Pull Request (PR)**：
    *   将一个分支的代码合并到另一个分支的请求。
    *   流程：新建分支 -> 修改提交 -> Push -> 在 GitHub 提 PR -> 审核通过 -> Merge -> 删除临时分支。
*   **Git Config Pull**：
    *   `git config pull.rebase false`：设置拉取策略为合并 (Merge)，保留分叉历史，适合新手。

## 5. GitHub 安全设置
**Branch Protection (分支保护)**：
*   位置：Settings -> Branches -> Rulesets。
*   作用：防止 `main` 分支被强制推送 (`force push`) 或误删除。
*   建议：开启 "Require status checks to pass" (要求 CI 通过才能合并)。

## 6. 其他小知识
*   `git add .`：将当前目录下所有修改加入暂存区。
*   `git status`：Git 操作前的“仪表盘”，随时检查状态。
*   `Conventional Commits`：约定式提交规范 (如 `fix(ci): ...`)，用于自动生成 Changelog。
