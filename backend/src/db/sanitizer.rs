pub fn sanitizer(string: &str) -> String {
    let mut s = String::with_capacity(string.len());

    for c in string.chars() {
        if let ',' | '.' | '<' | '>' | '{' | '}' | '[' | ']' | '"' | '\'' | ':' | ';' | '!' | '\\'
        | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '+' | '=' | '~' = c
        {
            s.push('\\');
        }
        s.push(c)
    }

    s
}
