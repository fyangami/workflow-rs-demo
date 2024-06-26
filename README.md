![kill all humans](https://github.com/fyangami/workflow-rs-demo/blob/main/bender.png?raw=true)

# 为什么要容器化Devops🦾
- 统一开发、测试以及部署环境，避免因为环境的差异而导致的问题
- 降低协作门槛，使得新人能够更快的加入项目（入职第一天不用装环境了，直接开始干活🤭）
- 可解释以及较低的迁移成本
- 自动化！DO NOT REPEAT!

# 关于本仓库
本仓库提供一套关于 **Rust** 项目开发的 **容器化Devops** 流程演示，包括以下内容:
1. 基于 [devcontainer](https://containers.dev) + [VS Code](https://code.visualstudio.com/docs/remote/remote-overview) 的开发环境配置
2. 基于 [Semantic Release](https://github.com/semantic-release/semantic-release) 的自动版本管理机制
3. 基于 **容器化(Docker)** 的测试以及打包配置
4. 基于 [Github Actions](https://github.com/features/actions) 的 **CI\CD** 方案。

项目成果： 
- 从开发 -> 测试 -> 部署全流程的容器化，以及 CI\CD（Github Actions） 集成。
- 基于 [axum](https://github.com/tokio-rs/axum) 的 **HTTP** 服务，以及一个演示接口：`curl http://{container_name}/{path_variable}`。

注：
- **Rust** 代码无需了解。旨在演示现代 `Devops` 实践过程。 
- 由于本项目的所有 **Rust** 代码仅意图演示使用，为了保证代码的简洁，多处使用了`unwrap`，实际项目请完善异常处理。
- 本项目的 **CI\CD** 流程尚不够健壮，但我想应该是一个不错的学习和理解 **Devops** 以及 **容器化** 核心理念的起点😉

# 实验环境依赖
- **Vs Code** 编辑器且安装有 [Remote Development](https://github.com/Microsoft/vscode-remote-release) 插件
- 安装有 **Docker** 和 **Docker Compose**
- **Git**

# 参考文档
- [devcontainer](https://containers.dev) 规范和 [VS Code Remote Development](https://code.visualstudio.com/docs/remote/remote-overview)
- [Semantic Release](https://github.com/semantic-release/semantic-release)
- [Github Actions](https://github.com/features/actions)
- [Docker Docs](https://docs.docker.com/)
