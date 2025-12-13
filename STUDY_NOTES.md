# Trae 结对编程实战记录 (2025-12-13)

这份文档记录了我们从零开始配置项目、解决 CI/CD 问题到 Git 高级操作的完整全过程。

---

## 1. 启程：编辑器与 Git 基础

### 1.1 Nano 编辑器操作
**场景**：在终端执行 `git commit` 时意外进入了 GNU nano 编辑器。
**操作指南**：
*   **保存**：`Ctrl + O` -> `Enter`。
*   **退出**：`Ctrl + X`。
*   **直接退出**：`Ctrl + X` -> `Y` (保存) -> `Enter`。

### 1.2 Git 多远程仓库 (Multi-Remote)
**场景**：需要将本地代码同时推送到自己的仓库 (`origin`) 和模板仓库 (`template`)。
**关键命令**：
```bash
# 查看远程仓库
git remote -v

# 添加新的远程仓库
git remote add template <URL>

# 推送到不同仓库
git push origin main    # 推送到自己的仓库
git push template main  # 推送到模板仓库

# 修改默认上游 (Upstream)
git branch -u origin/main  # 设置 git push 默认推给 origin
```

---

## 2. 挑战：CI/CD 修复实战 (GitHub Actions)

我们在配置 `.github/workflows/build.yml` 时遇到了一系列问题，并逐一解决。

### 问题 1：Workflow 没有触发
*   **现象**：推送到 `main` 后，GitHub Actions 页面没有反应。
*   **原因**：配置文件里写的是 `branches: [master]`，但你的主分支叫 `main`。
*   **解决**：将 `master` 修改为 `main`。

### 问题 2：Docker 构建失败 (git-cliff-action)
*   **现象**：`Build container for action use... Error: Docker build failed with exit code 1`。
*   **原因**：`orhun/git-cliff-action@v2` 版本过旧，其 Docker 镜像与现在的 GitHub Actions 运行环境 (Node 20) 不兼容。
*   **解决**：升级 Action 版本到 **v4**。
    ```yaml
    - uses: orhun/git-cliff-action@v4  # 从 v2 升级到 v4
    ```

### 问题 3：测试报错 (No tests to run)
*   **现象**：`cargo nextest` 报错 `Error: no tests to run` (exit code 4)。
*   **原因**：项目目前没有测试代码，`nextest` 默认视为失败。
*   **解决**：添加参数允许空测试通过。
    ```yaml
    run: cargo nextest run --all-features --no-tests=pass
    ```

---

## 3. 进阶：Git 核心概念与操作

### 3.1 核心概念释义
*   **Tag (标签)**：给 Commit 贴的永久标记，常用于版本发布 (如 `v1.0.0`)。
*   **Diff (差异)**：`git diff` 查看工作区修改，`git diff --staged` 查看暂存区修改。
*   **Main vs Master**：本质相同，`main` 是目前业界推荐的默认分支名。
*   **git add .**：将当前目录下所有修改加入暂存区。
*   **git status**：操作前的“仪表盘”，确认分支状态和文件状态。

### 3.2 分支与 Pull Request (PR) 流程
**场景**：为了修复配置，我们走了一遍标准的开发流程。
1.  **新建分支**：`git checkout -b chore/fix-cliff-config`。
2.  **提交修改**：`git commit ...`
3.  **推送分支**：`git push origin chore/fix-cliff-config`。
4.  **创建 PR**：在 GitHub 页面点击 "Compare & pull request"。
5.  **合并 PR**：点击 "Merge pull request"，随后删除远程分支。

### 3.3 解决分支分叉 (Divergent Branches)
**场景**：远程合并了 PR，本地又有了新提交，导致 `git push` 被拒绝。
**解决步骤**：
1.  **设置合并策略**：`git config pull.rebase false` (使用 Merge 策略)。
2.  **拉取远程代码**：`git pull origin main` (自动合并远程变更)。
3.  **再次推送**：`git push origin main`。

---

## 4. GitHub 安全配置
**Branch Protection (分支保护)**
*   **位置**：Settings -> Branches -> Rulesets。
*   **目的**：保护 `main` 分支不被强制推送或误删。
*   **建议配置**：
    *   Lock branch: Disabled
    *   Restrict deletions: Enabled (禁止删除)
    *   Restrict force pushes: Enabled (禁止强推)
    *   Require status checks to pass: Enabled (必须等 CI 绿了才能合)

---

## 5. 对话中产生的其他知识点
*   **Changelog 生成位置**：在 GitHub Actions 运行时的虚拟环境中生成，直接发布到 Release，不污染代码库。
*   **Conventional Commits**：一种提交规范 (如 `fix(ci): ...`)，`git-cliff` 依赖它来自动分类生成日志。
*   **Trae 对话保存**：Trae 自动保存历史，但建议手动将重要内容整理成文档 (如本文档) 并提交到仓库存档。
