//! WAE LSP Server - 标准 Language Server Protocol 实现

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct WaeLanguageServer {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for WaeLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["~".to_string(), ".".to_string()]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "WAE LSP Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "# WAE Language Server\n\nThis is a standard LSP server implementation for WAE files.".to_string(),
            }),
            range: None,
        }))
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem {
                label: "model".to_string(),
                kind: Some(CompletionItemKind::CLASS),
                detail: Some("Define a data model".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "field".to_string(),
                kind: Some(CompletionItemKind::FIELD),
                detail: Some("Define a field in a model".to_string()),
                ..Default::default()
            },
        ])))
    }

    async fn definition(&self, _: DefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| WaeLanguageServer { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}