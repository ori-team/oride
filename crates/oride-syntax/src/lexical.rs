//! Highlight lexical contido para linguagens sem grammar tree-sitter empacotada.

use crate::{HighlightKind, HighlightSpan, LanguageId};

pub(crate) fn collect_lexical_spans(
    language: LanguageId,
    source: &str,
    base: usize,
) -> Vec<HighlightSpan> {
    let Some(profile) = profile(language) else {
        return Vec::new();
    };
    let bytes = source.as_bytes();
    let mut spans = Vec::new();
    let mut cursor = 0usize;
    let mut after_fn_decl = false;

    while cursor < bytes.len() {
        if let Some((open, close)) = profile.block_comment {
            if starts_with(bytes, cursor, open) {
                let content_start = cursor + open.len();
                let end = source[content_start..]
                    .find(close)
                    .map_or(bytes.len(), |offset| content_start + offset + close.len());
                push_span(&mut spans, base, cursor, end, HighlightKind::Comment);
                cursor = end;
                continue;
            }
        }
        if starts_with(bytes, cursor, profile.line_comment) {
            let end = source[cursor..]
                .find('\n')
                .map_or(bytes.len(), |offset| cursor + offset);
            push_span(&mut spans, base, cursor, end, HighlightKind::Comment);
            cursor = end;
            continue;
        }

        if matches!(bytes[cursor], b'\'' | b'"' | b'`') {
            after_fn_decl = false;
            let quote = bytes[cursor];
            let start = cursor;
            cursor += 1;
            while cursor < bytes.len() {
                if bytes[cursor] == b'\\' {
                    cursor = (cursor + 2).min(bytes.len());
                } else if bytes[cursor] == quote {
                    cursor += 1;
                    break;
                } else {
                    cursor += 1;
                }
            }
            push_span(&mut spans, base, start, cursor, HighlightKind::String);
            continue;
        }

        if bytes[cursor].is_ascii_digit() {
            after_fn_decl = false;
            let start = cursor;
            cursor += 1;
            while cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphanumeric() || matches!(bytes[cursor], b'_' | b'.'))
            {
                cursor += 1;
            }
            push_span(&mut spans, base, start, cursor, HighlightKind::Number);
            continue;
        }

        if bytes[cursor].is_ascii_alphabetic() || bytes[cursor] == b'_' {
            let start = cursor;
            cursor += 1;
            while cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
            {
                cursor += 1;
            }
            let word = &source[start..cursor];
            let is_fn_def = after_fn_decl;
            after_fn_decl = matches!(word, "def" | "fn" | "func" | "proc" | "function");

            let kind = if is_fn_def {
                Some(HighlightKind::Function)
            } else if profile.keywords.contains(&word) {
                Some(HighlightKind::Keyword)
            } else if profile.types.contains(&word) {
                Some(HighlightKind::Type)
            } else if source[cursor..].trim_start().starts_with('(') {
                Some(HighlightKind::Function)
            } else {
                None
            };
            if let Some(kind) = kind {
                push_span(&mut spans, base, start, cursor, kind);
            }
            continue;
        }

        if matches!(bytes[cursor], b'\n' | b';') {
            after_fn_decl = false;
        }

        cursor += source[cursor..].chars().next().map_or(1, char::len_utf8);
    }

    spans
}

fn starts_with(bytes: &[u8], cursor: usize, token: &str) -> bool {
    !token.is_empty() && bytes[cursor..].starts_with(token.as_bytes())
}

fn push_span(
    spans: &mut Vec<HighlightSpan>,
    base: usize,
    start: usize,
    end: usize,
    kind: HighlightKind,
) {
    if start < end {
        spans.push(HighlightSpan {
            start: base + start,
            end: base + end,
            kind,
        });
    }
}

struct LexicalProfile {
    line_comment: &'static str,
    block_comment: Option<(&'static str, &'static str)>,
    keywords: &'static [&'static str],
    types: &'static [&'static str],
}

fn profile(language: LanguageId) -> Option<LexicalProfile> {
    match language {
        LanguageId::Ori => Some(LexicalProfile {
            line_comment: "--",
            block_comment: Some(("--|", "|--")),
            keywords: &[
                "alias",
                "and",
                "any",
                "apply",
                "as",
                "attr",
                "break",
                "case",
                "check",
                "const",
                "continue",
                "do",
                "elif",
                "else",
                "end",
                "enum",
                "extern",
                "false",
                "for",
                "func",
                "handle",
                "if",
                "implement",
                "import",
                "imports",
                "in",
                "is",
                "lazy",
                "loop",
                "map",
                "match",
                "module",
                "mut",
                "newtype",
                "none",
                "not",
                "optional",
                "or",
                "public",
                "range",
                "repeat",
                "result",
                "return",
                "self",
                "set",
                "some",
                "struct",
                "then",
                "trait",
                "true",
                "tuple",
                "use",
                "using",
                "var",
                "void",
                "where",
                "while",
                "with",
            ],
            types: &[
                "bool", "bytes", "float", "float32", "float64", "int", "int8", "int16", "int32",
                "int64", "list", "string", "u8", "u16", "u32", "u64",
            ],
        }),
        LanguageId::Nim => Some(LexicalProfile {
            line_comment: "#",
            block_comment: Some(("#[", "]#")),
            keywords: &[
                "addr",
                "and",
                "as",
                "asm",
                "bind",
                "block",
                "break",
                "case",
                "cast",
                "concept",
                "const",
                "continue",
                "converter",
                "defer",
                "discard",
                "distinct",
                "div",
                "do",
                "elif",
                "else",
                "end",
                "enum",
                "except",
                "export",
                "finally",
                "for",
                "from",
                "func",
                "if",
                "import",
                "in",
                "include",
                "interface",
                "is",
                "isnot",
                "iterator",
                "let",
                "macro",
                "method",
                "mixin",
                "mod",
                "nil",
                "not",
                "notin",
                "object",
                "of",
                "or",
                "out",
                "proc",
                "ptr",
                "raise",
                "ref",
                "return",
                "shl",
                "shr",
                "static",
                "template",
                "try",
                "tuple",
                "type",
                "using",
                "var",
                "when",
                "while",
                "with",
                "without",
                "xor",
                "yield",
            ],
            types: &[
                "bool", "char", "cstring", "float", "float32", "float64", "int", "int8", "int16",
                "int32", "int64", "string", "uint", "uint8", "uint16", "uint32", "uint64",
            ],
        }),
        LanguageId::D => Some(LexicalProfile {
            line_comment: "//",
            block_comment: Some(("/*", "*/")),
            keywords: &[
                "abstract",
                "alias",
                "align",
                "asm",
                "assert",
                "auto",
                "body",
                "break",
                "case",
                "cast",
                "catch",
                "class",
                "const",
                "continue",
                "debug",
                "default",
                "delegate",
                "deprecated",
                "do",
                "else",
                "enum",
                "export",
                "extern",
                "false",
                "final",
                "finally",
                "for",
                "foreach",
                "foreach_reverse",
                "function",
                "goto",
                "if",
                "immutable",
                "import",
                "in",
                "inout",
                "interface",
                "invariant",
                "is",
                "lazy",
                "mixin",
                "module",
                "new",
                "nothrow",
                "null",
                "out",
                "override",
                "package",
                "pragma",
                "private",
                "protected",
                "public",
                "pure",
                "ref",
                "return",
                "scope",
                "shared",
                "static",
                "struct",
                "super",
                "switch",
                "synchronized",
                "template",
                "this",
                "throw",
                "true",
                "try",
                "typeof",
                "union",
                "unittest",
                "version",
                "while",
                "with",
            ],
            types: &[
                "bool",
                "byte",
                "cdouble",
                "cent",
                "cfloat",
                "char",
                "creal",
                "dchar",
                "double",
                "dstring",
                "float",
                "idouble",
                "ifloat",
                "int",
                "ireal",
                "long",
                "ptrdiff_t",
                "real",
                "short",
                "size_t",
                "string",
                "ubyte",
                "ucent",
                "uint",
                "ulong",
                "ushort",
                "void",
                "wchar",
                "wstring",
            ],
        }),
        LanguageId::Lua => Some(LexicalProfile {
            line_comment: "--",
            block_comment: Some(("--[[", "]]")),
            keywords: &[
                "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "goto",
                "if", "in", "local", "nil", "not", "or", "repeat", "return", "then", "true",
                "until", "while",
            ],
            types: &[
                "any", "boolean", "function", "nil", "number", "string", "table", "thread",
                "userdata",
            ],
        }),
        LanguageId::Python => Some(LexicalProfile {
            line_comment: "#",
            block_comment: None,
            keywords: &[
                "and", "as", "assert", "async", "await", "break", "class", "continue", "def",
                "del", "elif", "else", "except", "finally", "for", "from", "global", "if",
                "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return",
                "try", "while", "with", "yield", "True", "False", "None",
            ],
            types: &[
                "int", "float", "str", "bool", "list", "dict", "set", "tuple", "bytes", "object",
                "type", "Any", "Optional", "Union",
            ],
        }),
        LanguageId::JavaScript => Some(LexicalProfile {
            line_comment: "//",
            block_comment: Some(("/*", "*/")),
            keywords: &[
                "async",
                "await",
                "break",
                "case",
                "catch",
                "class",
                "const",
                "continue",
                "debugger",
                "default",
                "delete",
                "do",
                "else",
                "export",
                "extends",
                "finally",
                "for",
                "function",
                "if",
                "import",
                "in",
                "instanceof",
                "let",
                "new",
                "return",
                "super",
                "switch",
                "this",
                "throw",
                "try",
                "typeof",
                "var",
                "void",
                "while",
                "with",
                "yield",
                "true",
                "false",
                "null",
                "undefined",
            ],
            types: &[
                "Array", "Boolean", "Date", "Error", "Function", "Map", "Number", "Object",
                "Promise", "RegExp", "Set", "String", "Symbol",
            ],
        }),
        LanguageId::TypeScript | LanguageId::Tsx => Some(LexicalProfile {
            line_comment: "//",
            block_comment: Some(("/*", "*/")),
            keywords: &[
                "abstract",
                "as",
                "async",
                "await",
                "break",
                "case",
                "catch",
                "class",
                "const",
                "continue",
                "debugger",
                "declare",
                "default",
                "delete",
                "do",
                "else",
                "enum",
                "export",
                "extends",
                "finally",
                "for",
                "function",
                "if",
                "implements",
                "import",
                "in",
                "infer",
                "instanceof",
                "interface",
                "is",
                "keyof",
                "let",
                "namespace",
                "never",
                "new",
                "readonly",
                "return",
                "super",
                "switch",
                "this",
                "throw",
                "try",
                "type",
                "typeof",
                "unknown",
                "var",
                "void",
                "while",
                "with",
                "yield",
                "true",
                "false",
                "null",
                "undefined",
            ],
            types: &[
                "any", "bigint", "boolean", "never", "number", "string", "symbol", "unknown",
                "void", "Array", "Boolean", "Date", "Error", "Function", "Map", "Number", "Object",
                "Partial", "Promise", "Record", "Set", "String",
            ],
        }),
        LanguageId::Ruby => Some(LexicalProfile {
            line_comment: "#",
            block_comment: Some(("=begin", "=end")),
            keywords: &[
                "alias", "and", "begin", "break", "case", "class", "def", "defined?", "do", "else",
                "elsif", "end", "ensure", "false", "for", "if", "in", "module", "next", "nil",
                "not", "or", "redo", "rescue", "retry", "return", "self", "super", "then", "true",
                "undef", "unless", "until", "when", "while", "yield",
            ],
            types: &[
                "Array", "Class", "Float", "Hash", "Integer", "Module", "Numeric", "Object",
                "String", "Symbol",
            ],
        }),
        LanguageId::Html => Some(LexicalProfile {
            line_comment: "<!--",
            block_comment: Some(("<!--", "-->")),
            keywords: &[
                "html", "head", "body", "div", "span", "a", "p", "script", "style", "link", "meta",
                "title", "table", "tr", "td", "th", "ul", "ol", "li", "h1", "h2", "h3", "h4", "h5",
                "h6", "input", "button", "form", "section", "header", "footer", "main",
            ],
            types: &[
                "class", "id", "src", "href", "type", "rel", "name", "value", "alt", "style",
            ],
        }),
        LanguageId::Css => Some(LexicalProfile {
            line_comment: "/*",
            block_comment: Some(("/*", "*/")),
            keywords: &[
                "display",
                "position",
                "margin",
                "padding",
                "border",
                "color",
                "background",
                "font",
                "width",
                "height",
                "flex",
                "grid",
                "top",
                "left",
                "right",
                "bottom",
                "opacity",
                "z-index",
                "overflow",
                "cursor",
                "transition",
                "transform",
            ],
            types: &[
                "px", "em", "rem", "vh", "vw", "%", "auto", "none", "block", "inline", "flex",
                "solid", "relative", "absolute", "fixed",
            ],
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_ori_and_nim_without_external_grammars() {
        let ori = collect_lexical_spans(
            LanguageId::Ori,
            "module demo\nconst value: int = 42 -- note\n",
            0,
        );
        let nim = collect_lexical_spans(LanguageId::Nim, "proc main() = echo \"hi\" # note", 0);

        assert!(ori.iter().any(|span| span.kind == HighlightKind::Keyword));
        assert!(ori.iter().any(|span| span.kind == HighlightKind::Comment));
        assert!(nim.iter().any(|span| span.kind == HighlightKind::String));
        assert!(nim.iter().any(|span| span.kind == HighlightKind::Function));
    }

    #[test]
    fn highlights_dlang_and_lua_lexically() {
        let d_spans = collect_lexical_spans(
            LanguageId::D,
            "import std.stdio;\nvoid main() {\n    string s = \"hello D\"; // print\n    writeln(s);\n}\n",
            0,
        );
        let lua_spans = collect_lexical_spans(
            LanguageId::Lua,
            "local name: string = \"Oride\"\n--[[ block ]]\nprint(name)\n",
            0,
        );

        assert!(d_spans
            .iter()
            .any(|span| span.kind == HighlightKind::Keyword));
        assert!(d_spans.iter().any(|span| span.kind == HighlightKind::Type));
        assert!(d_spans
            .iter()
            .any(|span| span.kind == HighlightKind::Comment));
        assert!(d_spans
            .iter()
            .any(|span| span.kind == HighlightKind::String));
        assert!(d_spans
            .iter()
            .any(|span| span.kind == HighlightKind::Function));

        assert!(lua_spans
            .iter()
            .any(|span| span.kind == HighlightKind::Keyword));
        assert!(lua_spans
            .iter()
            .any(|span| span.kind == HighlightKind::Type));
        assert!(lua_spans
            .iter()
            .any(|span| span.kind == HighlightKind::Comment));
        assert!(lua_spans
            .iter()
            .any(|span| span.kind == HighlightKind::String));
        assert!(lua_spans
            .iter()
            .any(|span| span.kind == HighlightKind::Function));
    }

    #[test]
    fn nim_block_comment_takes_priority_over_line_comment() {
        let source = "#[ first\nsecond ]#\nlet value = 1";
        let spans = collect_lexical_spans(LanguageId::Nim, source, 0);
        assert!(spans.iter().any(|span| {
            span.kind == HighlightKind::Comment
                && &source[span.start..span.end] == "#[ first\nsecond ]#"
        }));
    }
}
