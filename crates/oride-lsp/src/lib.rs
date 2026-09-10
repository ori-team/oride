//! Cliente LSP mínimo e desacoplado (stdio, Content-Length framing).
//!
//! Conecta-se a language servers no `$PATH` (`rust-analyzer`, `clangd`, etc.):
//! initialize, sync de documentos, diagnostics, hover, completion, definition e formatting.

mod client;
mod protocol;
mod types;

pub use client::{LspClient, LspError, LspEvent};
pub use types::{
    character_column, utf16_column, CompletionItem, Diagnostic, DiagnosticSeverity, HoverInfo,
    Location, Position, Range,
};
