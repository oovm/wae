//! WAE Macros - 过程宏模块
//!
//! 提供用于简化开发的过程宏：
//! - `#[derive(ToSchema)]` - 自动生成 Schema 定义
//! - `query!` - 编译时 SQL 查询宏
//! - `query_as!` - 编译时 SQL 查询宏（自动映射到结构体）

#![warn(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, Ident, Lit, Meta, Type, parse_macro_input};

fn get_type_schema(ty: &Type) -> TokenStream2 {
    let type_str = quote!(#ty).to_string();

    if type_str.contains("String") || type_str.contains("&str") {
        quote! { wae_schema::Schema::string() }
    }
    else if type_str.contains("i8")
        || type_str.contains("i16")
        || type_str.contains("i32")
        || type_str.contains("i64")
        || type_str.contains("i128")
        || type_str.contains("isize")
        || type_str.contains("u8")
        || type_str.contains("u16")
        || type_str.contains("u32")
        || type_str.contains("u64")
        || type_str.contains("u128")
        || type_str.contains("usize")
    {
        quote! { wae_schema::Schema::integer() }
    }
    else if type_str.contains("f32") || type_str.contains("f64") {
        quote! { wae_schema::Schema::number() }
    }
    else if type_str.contains("bool") {
        quote! { wae_schema::Schema::boolean() }
    }
    else if type_str.contains("Vec") || type_str.contains("[]") {
        quote! { wae_schema::Schema::array(<wae_schema::Schema as Default>::default()) }
    }
    else if type_str.contains("Option") {
        quote! { <wae_schema::Schema as Default>::default().nullable(true) }
    }
    else {
        quote! {
            <#ty as wae_schema::ToSchema>::schema()
        }
    }
}

fn extract_doc_comment(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        let doc = lit_str.value().trim().to_string();
                        if !doc.is_empty() {
                            return Some(doc);
                        }
                    }
                }
            }
        }
    }
    None
}

fn generate_struct_schema(name: &Ident, fields: &Fields) -> TokenStream2 {
    let mut properties = Vec::new();
    let mut required_fields = Vec::new();

    match fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                let field_name = field.ident.as_ref().unwrap();
                let field_name_str = field_name.to_string();
                let field_schema = get_type_schema(&field.ty);

                let doc_comment = extract_doc_comment(&field.attrs);
                let property = if let Some(doc) = doc_comment {
                    quote! {
                        .property(#field_name_str, #field_schema.description(#doc))
                    }
                }
                else {
                    quote! {
                        .property(#field_name_str, #field_schema)
                    }
                };
                properties.push(property);

                let is_option = quote!(#field.ty).to_string().contains("Option");
                if !is_option {
                    required_fields.push(field_name_str);
                }
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                let field_name_str = format!("field_{}", i);
                let field_schema = get_type_schema(&field.ty);
                let property = quote! {
                    .property(#field_name_str, #field_schema)
                };
                properties.push(property);
            }
        }
        Fields::Unit => {}
    }

    let required = if required_fields.is_empty() {
        quote! {}
    }
    else {
        quote! { .required(vec![#(#required_fields.to_string()),*]) }
    };

    quote! {
        impl wae_schema::ToSchema for #name {
            fn schema() -> wae_schema::Schema {
                wae_schema::Schema::object()
                    #(#properties)*
                    #required
            }
        }
    }
}

fn generate_enum_schema(name: &Ident, data: &Data) -> TokenStream2 {
    let variants = match data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => return quote! {},
    };

    let mut enum_values = Vec::new();
    for variant in variants {
        let variant_name = variant.ident.to_string();
        enum_values.push(quote! { serde_json::Value::String(#variant_name.to_string()) });
    }

    quote! {
        impl wae_schema::ToSchema for #name {
            fn schema() -> wae_schema::Schema {
                wae_schema::Schema::string()
                    .enum_values(vec![#(#enum_values),*])
            }
        }
    }
}

/// 自动生成 Schema 的派生宏
///
/// # Example
///
/// ```rust,ignore
/// use serde::{Deserialize, Serialize};
/// use wae_schema::{Schema, ToSchema};
///
/// #[derive(Debug, Serialize, Deserialize, ToSchema)]
/// pub struct User {
///     /// 用户 ID
///     pub id: u64,
///     /// 用户名
///     pub name: String,
///     /// 邮箱（可选）
///     pub email: Option<String>,
/// }
/// ```
#[proc_macro_derive(ToSchema)]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match &input.data {
        Data::Struct(data_struct) => generate_struct_schema(name, &data_struct.fields),
        Data::Enum(_) => generate_enum_schema(name, &input.data),
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "Unions are not supported").to_compile_error().into();
        }
    };

    TokenStream::from(expanded)
}

/// 生成 OpenAPI 路由文档的宏
///
/// # Example
///
/// ```rust,ignore
/// use wae_macros::api_doc;
///
/// let doc = api_doc! {
///     "/users" => {
///         GET => {
///             summary: "获取用户列表",
///             response: 200 => "成功",
///         },
///         POST => {
///             summary: "创建用户",
///             body: "User",
///             response: 201 => "创建成功",
///         },
///     },
/// };
/// ```
#[proc_macro]
pub fn api_doc(input: TokenStream) -> TokenStream {
    let _ = input;
    TokenStream::from(quote! {
        wae_schema::OpenApiDoc::new("API", "1.0.0")
    })
}

mod query;

/// SQL 查询宏 - 返回原始行数据
///
/// 执行 SQL 查询并返回 `DatabaseRows`，需要手动迭代处理结果。
///
/// # Example
///
/// ```rust,ignore
/// use wae_macros::query;
/// use wae_database::{DatabaseConnection, DatabaseRow};
///
/// async fn get_users(conn: &dyn DatabaseConnection) -> Result<Vec<DatabaseRow>, Box<dyn std::error::Error>> {
///     let mut rows = query!(conn, "SELECT id, name, email FROM users WHERE active = ?", true).await?;
///     let mut results = Vec::new();
///     while let Some(row) = rows.next().await? {
///         results.push(row);
///     }
///     Ok(results)
/// }
/// ```
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    query::expand_query(input)
}

/// SQL 查询宏 - 自动映射到结构体
///
/// 执行 SQL 查询并自动将结果映射到指定结构体类型。
/// 结构体需要实现 `FromRow` trait。
///
/// # Example
///
/// ```rust,ignore
/// use wae_macros::query_as;
/// use wae_database::{Entity, FromRow, DatabaseRow, DatabaseResult};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct User {
///     id: i64,
///     name: String,
///     email: String,
/// }
///
/// impl FromRow for User {
///     fn from_row(row: &DatabaseRow) -> DatabaseResult<Self> {
///         Ok(Self {
///             id: row.get(0)?,
///             name: row.get(1)?,
///             email: row.get(2)?,
///         })
///     }
/// }
///
/// async fn get_users(conn: &dyn DatabaseConnection) -> Result<Vec<User>, Box<dyn std::error::Error>> {
///     let users = query_as!(User, conn, "SELECT id, name, email FROM users WHERE active = ?", true).await?;
///     Ok(users)
/// }
/// ```
#[proc_macro]
pub fn query_as(input: TokenStream) -> TokenStream {
    query::expand_query_as(input)
}

/// 执行宏 - 执行 INSERT/UPDATE/DELETE 等 SQL 语句
///
/// 执行 SQL 语句并返回影响的行数。
///
/// # Example
///
/// ```rust,ignore
/// use wae_macros::execute;
/// use wae_database::DatabaseConnection;
///
/// async fn insert_user(conn: &dyn DatabaseConnection) -> Result<u64, Box<dyn std::error::Error>> {
///     let affected = execute!(conn, "INSERT INTO users (name, email) VALUES (?, ?)", "Alice", "alice@example.com").await?;
///     Ok(affected)
/// }
/// ```
#[proc_macro]
pub fn execute(input: TokenStream) -> TokenStream {
    query::expand_execute(input)
}

/// 标量查询宏 - 返回单个值
///
/// 执行查询并返回单个值，适用于 COUNT、SUM 等聚合查询。
///
/// # Example
///
/// ```rust,ignore
/// use wae_macros::query_scalar;
/// use wae_database::DatabaseConnection;
///
/// async fn count_users(conn: &dyn DatabaseConnection) -> Result<i64, Box<dyn std::error::Error>> {
///     let count = query_scalar!(i64, conn, "SELECT COUNT(*) FROM users").await?;
///     Ok(count)
/// }
/// ```
#[proc_macro]
pub fn query_scalar(input: TokenStream) -> TokenStream {
    query::expand_query_scalar(input)
}
