use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Netaman" => "Err",
        "Taman" => "Ok",
        "Tekst" => "String",
        "Rechnik" => "HashMap",
        "Standard" => "Default",
        "Greska" => "Error",
        "Mozhda" => "Option",
        "Neshto" => "Some",
        "Nishto" => "None",
        "Rezultat" => "Result",
        "Sebe" => "Self",
        "ispecati" => "println",
        "stoj" => "break",
        "asinhrona" => "async",
        "pricekaj" => "await",
        "vokrug" => "loop",
        "premesti" => "move",
        "kutija" => "crate",
        "nedostigliv_kod" => "unreachable_code",
        "kako" => "as",
        "konstanta" => "const",
        "osobina" => "trait",
        "nesigurno" => "unsafe",
        "vo" => "in",
        "od" => "from",
        "dinamicko" => "dyn",
        "odvitkaj" => "unwrap",
        "standard" => "default",
        "kako_ref" => "as_ref",
        "vi" => "io",
        "nadvoreshna" => "extern",
        "netocno" => "false",
        "funk" => "fn",
        "roditel" => "super",
        "vnesi" => "insert",
        "zemi" => "get",
        "dozvoleno" => "allow",
        "upm" | "panika" | "sranje" | "jbg" => "panic",
        "modul" => "mod",
        "mutabilno" => "mut",
        "nov" => "new",
        "kade" => "where",
        "za_sekoj" => "for",
        "zemi_ili_vnesi_so" => "get_or_insert_with",
        "glavna" => "main",
        "javna" => "pub",
        "ima" => None?,
        "vrati" => "return",
        "implementacija_na" => "impl",
        "ref" => "ref",
        "sovpadni" => "match",
        "ako" => "if",
        "inaku" => "else",
        "sebe" => "self",
        "neka" => "let",
        "staticno" => "static",
        "objekt" => "struct",
        "ako_se_zaebi_ondak" => "expect",
        "dodeka" => "while",
        "koristi" => "use",
        "stavi_vo" => "into",
        "vajstina" => "true",
        "enum" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rgja(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
