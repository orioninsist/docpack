use crate::token::TokenCounter;

pub fn split_by_tokens(content: &str, max_tokens: usize, counter: &TokenCounter) -> Vec<String> {
    let tokens = counter.encode(content);

    if tokens.len() <= max_tokens {
        return vec![content.to_string()];
    }

    tokens
        .chunks(max_tokens)
        .map(|chunk| counter.decode(chunk))
        .collect()
}
