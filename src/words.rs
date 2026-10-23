// словари для тренировки печати :d
use std::fs;
use std::path::Path;

pub const WORDS_EN: &[&str] = &[
    "the", "be", "to", "of", "and", "a", "in", "that", "have", "i", "it", "for", "not", "on", "with",
    "he", "as", "you", "do", "at", "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
    "or", "an", "will", "my", "one", "all", "would", "there", "their", "what", "so", "up", "out", "if",
    "about", "who", "get", "which", "go", "me", "when", "make", "can", "like", "time", "no", "just", "him",
    "know", "take", "people", "into", "year", "your", "good", "some", "could", "them", "see", "other",
    "than", "then", "now", "look", "only", "come", "its", "over", "think", "also", "back", "after", "use",
    "two", "how", "our", "work", "first", "well", "way", "even", "new", "want", "because", "any", "these",
    "give", "day", "most", "us", "write", "type", "paper", "press", "key", "letter", "ink", "ribbon",
    "story", "mind", "quick", "brown", "fox", "jumps", "lazy", "dog", "vintage", "classic", "sound",
    "space", "shift", "return", "page", "line", "word", "sentence", "fast", "speed", "test", "focus",
    "rhythm", "steady", "clean", "light", "dark", "heart", "soul", "sharp", "simple", "craft", "metal",
    "world", "system", "program", "silent", "shadow", "future", "vision", "memory", "stream", "buffer",
    "source", "engine", "signal", "vector", "screen", "pixel", "matrix", "hacker", "device", "random",
    "cyber", "ghost", "neural", "logic", "syntax", "thread", "packet", "socket", "daemon"
];

// код и сниппеты >_<
pub const CODE_WORDS: &[&str] = &[
    "fn", "let", "mut", "pub", "struct", "impl", "match", "enum", "trait", "return", "const", "static",
    "type", "where", "async", "await", "for", "while", "loop", "if", "else", "break", "continue", "use",
    "mod", "crate", "self", "super", "as", "ref", "true", "false", "Option<T>", "Result<T, E>", "Ok(val)",
    "Err(e)", "Some(x)", "None", "Vec::new()", "String::from(\"\")", "unwrap()", "expect()", "clone()",
    "into()", "as_ref()", "push(item)", "len()", "is_empty()", "iter().map()", "filter(|x| x > 0)",
    "format!(\"{}\")", "println!(\"ok\")", "vec![1, 2, 3]", "Default::default()", "Rc<RefCell<T>>",
    "Arc<Mutex<T>>", "HashMap::new()", "spawn(move || {})", "channel()", "Box::new(val)", "unsafe",
    "extern", "macro_rules!", "derive(Debug)", "serde(rename)", "#[inline]", "cargo run", "cargo test",
    "std::sync::Arc", "tokio::main", "eprintln!(\"err\")", "assert_eq!(a, b)", "todo!()", "unreachable!()"
];

// режим цитат :d
pub const QUOTES: &[&str] = &[
    "Talk is cheap. Show me the code.",
    "Stay hungry, stay foolish.",
    "There are only two hard things in Computer Science: cache invalidation and naming things.",
    "Simplicity is prerequisite for reliability.",
    "Make it work, make it right, make it fast.",
    "First, solve the problem. Then, write the code.",
    "In the middle of difficulty lies opportunity.",
    "Do, or do not. There is no try.",
    "Every keystroke leaves an echo in eternity."
];

// Читаем слова из скачанного файла словаря
pub fn load_dict_words(path: &Path) -> Option<Vec<String>> {
    if let Ok(content) = fs::read_to_string(path) {
        let words: Vec<String> = content
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        if !words.is_empty() {
            return Some(words);
        }
    }
    None
}
