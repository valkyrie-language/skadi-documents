use std::collections::BTreeMap;
use crate::api_statistics::HomeStatistics;
use aide::axum::IntoApiResponse;
use axum::Json;
use chrono::{DateTime, Local};
use schemars::{JsonSchema, SchemaGenerator};
use schemars::schema::{Schema, SchemaObject, SubschemaValidation};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
    module_path: Option<String>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DocumentInfo {
    Module(ModuleInfo),
    Class(ClassInfo),
    Trait(TraitInfo),
}

impl JsonSchema for DocumentInfo {
    fn schema_name() -> String {
        "DocumentInfo".to_string()
    }

    fn json_schema(g: &mut SchemaGenerator) -> Schema {
        let discriminator = json!({
            "propertyName": "type",
            "mapping": {
                "module": "#/components/schemas/ModuleInfo",
                "class": "#/components/schemas/ClassInfo",
                "trait": "#/components/schemas/TraitInfo",
            }
        });

        let subschemas = SubschemaValidation {
            one_of: Some(vec![
                g.subschema_for::<ModuleInfo>(),
                g.subschema_for::<ClassInfo>(),
                g.subschema_for::<TraitInfo>(),
            ]),
            ..Default::default()
        };

        let schema_object = SchemaObject {
            subschemas: Some(Box::new(subschemas)),
            extensions: BTreeMap::from_iter(vec![("discriminator".to_owned(), discriminator)]),
            ..Default::default()
        };
        Schema::Object(schema_object)
    }
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
    match query.0.module_path {
        Some(s) if s.ends_with("class") => Json(DocumentInfo::Class(ClassInfo { name: "测试类".to_string() })),
        Some(s) if s.ends_with("trait") => Json(DocumentInfo::Trait(TraitInfo { name: "测试特质".to_string() })),
        _ => Json(DocumentInfo::Module(ModuleInfo {
            name: "测试名".to_string(),
            summary: "测试总结".to_string(),
            documentation: "测试文档 $2$ \n```ts\nclass Test {\n    constructor() {\n        console.log('Hello World');\n    }\n}\n```".to_string(),
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
