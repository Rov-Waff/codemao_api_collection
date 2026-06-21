//! # 🐱 编程猫API集合 Codemao API Collection
//!
//! 基于 Rust 实现的编程猫开放平台 API 工具包，为社区脚本开发提供异步基础设施。
//!
//! ## ✨ 功能
//!
//! - 🔐 用户账号认证与操作
//! - 💬 社区内容查询与互动
//! - 🎨 作品信息获取
//! - 📝 论坛帖文管理
//!
//! ## 🚀 安装
//!
//! 直接通过 Cargo 添加依赖：
//!
//! ```bash
//! cargo add codemao_api_collection
//! cargo add tokio --feature full
//! ```
//!
//! 或在 `Cargo.toml` 中添加：
//!
//! ```toml
//! codemao_api_collection = "0.1"
//! ```
//!
//! ## 🧩 用法
//!
//! 你需要先创建一个 `Account` 对象，并引入相应的行为 Trait 后即可使用库提供的接口。
//!```rust
//!use codemao_api_collection::Account;
//!use codemao_api_collection::forum::forum_behavior::ForumBehavior;
//!#[tokio::main]
//!async fn main() {
//!    dotenv().ok();
//!    env_logger::init();
//!    let username = env::var("USERNAME").expect("env USERNAME not found");
//!    let password = env::var("PASSWORD").expect("env PASSWORD not found");
//!    let account = Account::new(&username, &password).await.unwrap();
//!    let res = account.search_posts("Rust", None, None).await.unwrap();
//!    info!("{:?}", res);
//!}
//!```
//!
//!
//!
//!
//! ## 📦 模块
//!
//! - `account`：与账号相关的 API
//! - `community`：社区内容与动态相关的 API
//! - `works`：作品信息相关的 API
//! - `forum`：论坛帖子相关的 API
//!
//! ## 🎯 目标
//!
//! 提供一个轻量、异步且易于扩展的编程猫 API 封装，方便脚本与工具快速接入猫站服务。
//!
//! ## 协议
//!
//! 基于MIT Licence开源
//!
//! ## 不稳定！
//!
//! 目前处于测试阶段，不保证完全稳定可靠

pub mod account;
pub mod community;
pub mod forum;
pub mod works;
const BASE_URL: &str = "https://api.codemao.cn/";
pub use account::Account;
pub use account::user_behavior::UserBehaviors;
pub use community::community_behavior::CommunityBehavior;
pub use forum::forum_behavior::ForumBehavior;
pub use works::work_behavior::WorkBehavior;

#[cfg(feature = "python")]
pub mod python_bindings {
    use pyo3::prelude::*;
    use pyo3::conversion::IntoPyObject;
    use pyo3::types::{PyDict, PyDictMethods, PyList};

    /// Convert any `serde::Serialize` type to a Python object (dict/list/primitives).
    /// Requires a `Python<'_>` token (available from pymethod signatures).
    pub fn to_pyobject<'py, T: serde::Serialize>(value: &T, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let json = serde_json::to_value(value)
            .map_err(|e| pyo3::exceptions::PyTypeError::new_err(e.to_string()))?;
        json_to_py(py, &json)
    }

    fn json_to_py<'py>(py: Python<'py>, value: &serde_json::Value) -> PyResult<Bound<'py, PyAny>> {
        match value {
            serde_json::Value::Null => Ok(py.None().into_bound(py)),
            serde_json::Value::Bool(b) => Ok(Bound::clone(&bool::into_pyobject(*b, py).unwrap()).into_any()),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i.into_pyobject(py).unwrap().into_any())
                } else if let Some(f) = n.as_f64() {
                    Ok(f.into_pyobject(py).unwrap().into_any())
                } else {
                    Ok(n.to_string().into_pyobject(py).unwrap().into_any())
                }
            }
            serde_json::Value::String(s) => Ok(s.as_str().into_pyobject(py).unwrap().into_any()),
            serde_json::Value::Array(arr) => {
                let items: PyResult<Vec<Bound<'_, PyAny>>> = arr.iter().map(|item| json_to_py(py, item)).collect();
                let list = PyList::new(py, items?)?;
                Ok(list.into_any())
            }
            serde_json::Value::Object(obj) => {
                let dict = PyDict::new(py);
                for (k, v) in obj {
                    let key = k.as_str().into_pyobject(py).unwrap();
                    let val = json_to_py(py, v)?;
                    dict.set_item(key, val)?;
                }
                Ok(dict.into_any())
            }
        }
    }

    #[pymodule]
    pub fn codemao_api_collection(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<crate::Account>()?;
        Ok(())
    }
}

#[cfg(feature = "python")]
pub use python_bindings::codemao_api_collection;