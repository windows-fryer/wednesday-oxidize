use proc_macro::{
    Delimiter::Brace,
    TokenStream,
    TokenTree::{self, Group, Ident, Literal},
};

fn parse_vm_request(vm: String, tree: TokenStream) -> TokenStream {
    let mut base_tokens = TokenStream::new();
    let mut group_stream = TokenStream::new();

    tree.clone().into_iter().for_each(|token| match token {
        Group(ref group) => {
            if group.delimiter() == Brace {
                let start_fn = format!("unsafe {{ VM_{}_START(); }}", vm.to_uppercase());
                let end_fn = format!("unsafe {{ VM_{}_END(); }}", vm.to_uppercase());

                let start_ast: TokenStream = start_fn.parse().unwrap();
                let end_ast: TokenStream = end_fn.parse().unwrap();

                let mut tokens = TokenStream::new();

                tokens.extend(start_ast);

                let last_return = group.stream().into_iter().enumerate().find_map(|token| {
                    if let (idx, Ident(ref ident)) = token {
                        if ident.to_string() == "return" {
                            return Some(idx);
                        }
                    }

                    None
                });

                let last_semicolon = group.stream().into_iter().enumerate().find_map(|token| {
                    if let (idx, TokenTree::Punct(ref punct)) = token {
                        if punct.as_char() == ';' {
                            return Some(idx);
                        }
                    }

                    None
                });

                if let Some(idx) = last_return {
                    let mut group_vec = group.stream().into_iter().collect::<Vec<_>>();

                    let ret_tokens = group_vec.split_off(idx);

                    tokens.extend(group_vec);
                    tokens.extend(end_ast);
                    tokens.extend(ret_tokens);
                } else if let Some(idx) = last_semicolon {
                    let mut group_vec = group.stream().into_iter().collect::<Vec<_>>();

                    let ret_tokens = group_vec.split_off(idx + 1);

                    tokens.extend(group_vec);
                    tokens.extend(end_ast);
                    tokens.extend(ret_tokens);
                } else {
                    tokens.extend(group.stream());
                    tokens.extend(end_ast);
                }

                let filtered_tokens: TokenStream = tree
                    .clone()
                    .into_iter()
                    .filter_map(|token| match token {
                        Group(ref group) => {
                            if group.delimiter() == Brace {
                                None
                            } else {
                                Some(token)
                            }
                        }

                        _ => Some(token),
                    })
                    .collect();

                base_tokens.extend(filtered_tokens);

                group_stream.extend(tokens);
            }
        }

        _ => (),
    });

    let new_group = proc_macro::Group::new(Brace, group_stream);

    base_tokens.extend(std::iter::once(TokenTree::Group(new_group)));

    base_tokens
}

#[proc_macro_attribute]
pub fn themida(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut token_stream: TokenStream = item.clone();

    attr.clone()
        .into_iter()
        .enumerate()
        .for_each(|arg| match arg.1 {
            Ident(ref ident) => {
                if ident.to_string() == "vm" {
                    token_stream = parse_vm_request(
                        attr.clone()
                            .into_iter()
                            .skip(arg.0)
                            .filter_map(|arg| -> Option<String> {
                                match arg {
                                    Literal(ref lit) => Some(lit.to_string().replace("\"", "")),

                                    _ => None,
                                }
                            })
                            .collect(),
                        item.clone(),
                    );
                }
            }

            _ => (),
        });

    token_stream
}
