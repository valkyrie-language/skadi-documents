use crate::api_statistics::HomeStatistics;
use aide::axum::IntoApiResponse;
use axum::Json;
use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
pub struct DocumentQueryByPath {
    /// The owner of the package.
    organization: String,
    /// The name of the package.
    library: String,
    /// The specific version of the package to query.
    #[serde(default)]
    version: Option<String>,
    /// The path of the document to query.
    #[serde(default)]
    path: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DocumentInfo {
    Module(ModuleInfo),
    Class(ClassInfo),
    Trait(TraitInfo),
}

#[derive(Serialize, JsonSchema)]
pub struct ModuleInfo {
    pub name: String,
    pub summary: String,
    pub documentation: String,
    pub items: Vec<ModuleItem>,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ModuleType {
    Module,
    Class,
    Trait,
    Constant,
}

#[derive(Serialize, JsonSchema)]
pub struct ClassInfo {
    pub name: String,
}

#[derive(Serialize, JsonSchema)]
pub struct TraitInfo {
    pub name: String,
}

#[derive(Serialize, JsonSchema)]
pub struct ModuleItem {
    r#type: ModuleType,
    name: String,
    summary: String,
}

pub async fn list_document(query: Json<DocumentQueryByPath>) -> impl IntoApiResponse {
    match query.0.path.last() {
        Some(s) if s.eq("class") => Json(DocumentInfo::Class(ClassInfo { name: "测试类".to_string() })),
        Some(s) if s.eq("trait") => Json(DocumentInfo::Trait(TraitInfo { name: "测试特质".to_string() })),
        _ => Json(DocumentInfo::Module(ModuleInfo {
            name: "测试名".to_string(),
            summary: "测试总结".to_string(),
            documentation: "测试文档".to_string(),
            items: vec![
                ModuleItem {
                    r#type: ModuleType::Module, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
                ModuleItem {
                    r#type: ModuleType::Class, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
                ModuleItem {
                    r#type: ModuleType::Module, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
                ModuleItem {
                    r#type: ModuleType::Constant, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
                ModuleItem {
                    r#type: ModuleType::Module, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
                ModuleItem {
                    r#type: ModuleType::Trait, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
                ModuleItem {
                    r#type: ModuleType::Module, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
                ModuleItem {
                    r#type: ModuleType::Constant, name: "测试模块".to_string(), summary: "测试模块总结".to_string()
                },
            ],
        })),
    }
}
