use webrust::prelude::*;

#[gui(Consolas 12px darkturquoise !darkslategray)]
fn main() {
    println("<silver>Legend: 🐍 Python  |  🧩 WebRust  |  🦀 Rust std");
    println("<darkturquoise,b>🚀 Python-like String Methods in Rust with WebRust");
    println("<limegreen>✨ Now with ultra-simplified splitby() method - Python syntax!");

    println("\n<gold,b>🔧 Trim / LTrim / RTrim:");
    println("<yellow> 🐍: \"  Hello World  \".strip()                    # \"Hello World\"");
    println("<deepskyblue> 🧩: \"  Hello World  \".trim()                    // same as std");
    println("<orange> 🦀: \"  Hello World  \".trim()");
    let r = "  Hello World  ".trim();
    println("<darkturquoise,b>📊 Result: '{r}'");

    println("<yellow> 🐍: \"###hello\".lstrip('#')");
    println("<deepskyblue> 🧩: \"###hello\".ltrim(\"#\")                   // WebRust alias");
    println("<orange> 🦀: \"###hello\".trim_start_matches('#')");
    let r = "###hello".ltrim("#");
    println("<darkturquoise,b>📊 Result: '{r}'");

    println("<yellow> 🐍: \"world!!!\".rstrip('!')");
    println("<deepskyblue> 🧩: \"world!!!\".rtrim(\"!\")                   // WebRust alias");
    println("<orange> 🦀: \"world!!!\".trim_end_matches('!')");
    let r = "world!!!".rtrim("!");
    println("<darkturquoise,b>📊 Result: '{r}'");

    println("\n<gold,b>🔤 Case Transformation:");
    println("<yellow> 🐍: \"Hello World\".upper()                       # \"HELLO WORLD\"");
    println("<deepskyblue> 🧩: \"Hello World\".upper()                     // WebRust alias");
    println("<orange> 🦀: \"Hello World\".to_uppercase()");
    let upper = "Hello World".upper();
    println("<darkturquoise,b>📊 Result: '{upper}'");

    println("<yellow> 🐍: \"Hello World\".lower()                       # \"hello world\"");
    println("<deepskyblue> 🧩: \"Hello World\".lower()                     // WebRust alias");
    println("<orange> 🦀: \"Hello World\".to_lowercase()");
    let lower = "Hello World".lower();
    println("<darkturquoise,b>📊 Result: '{lower}'");

    println("<yellow> 🐍: \"hello world\".title()                       # \"Hello World\"");
    println("<deepskyblue> 🧩: \"hello world\".title()                     // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let title = "hello world".title();
    println("<darkturquoise,b>📊 Result: '{title}'");

    println("<yellow> 🐍: \"hello world\".capitalize()                  # \"Hello world\"");
    println("<deepskyblue> 🧩: \"hello world\".capitalize()               // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let cap = "hello world".capitalize();
    println("<darkturquoise,b>📊 Result: '{cap}'");

    println("<yellow> 🐍: \"SwapCase\".swapcase()                       # \"sWAPcASE\"");
    println("<deepskyblue> 🧩: \"SwapCase\".swapcase()                     // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let swapped = "SwapCase".swapcase();
    println("<darkturquoise,b>📊 Result: '{swapped}'");

    println("\n<gold,b>🔍 Search & Pattern Tests:");
    println("<yellow> 🐍: \"user@example.com\".startswith('user')        # True");
    println("<deepskyblue> 🧩: \"user@example.com\".startswith(\"user\")    // WebRust alias");
    println("<orange> 🦀: \"user@example.com\".starts_with(\"user\")");
    let starts_user = "user@example.com".startswith("user");
    println("<darkturquoise,b>📊 Result: {starts_user}");

    println("<yellow> 🐍: \"user@example.com\".endswith('.com')         # True");
    println("<deepskyblue> 🧩: \"user@example.com\".endswith(\".com\")     // WebRust alias");
    println("<orange> 🦀: \"user@example.com\".ends_with(\".com\")");
    let ends_com = "user@example.com".endswith(".com");
    println("<darkturquoise,b>📊 Result: {ends_com}");

    println("<yellow> 🐍: 'abcabc'.find('bc')                         # 1");
    println("<deepskyblue> 🧩: \"abcabc\".find(\"bc\")                     // same as std");
    println("<orange> 🦀: \"abcabc\".find(\"bc\")");
    let find_res = "abcabc"
        .find("bc")
        .map(|i| i.to_string())
        .unwrap_or("None".into());
    println("<darkturquoise,b>📊 Result: {find_res}");

    println("<yellow> 🐍: 'banana'.count('an')                        # 2");
    println(
        "<deepskyblue> 🧩: \"banana\".count(\"an\")                    // WebRust Python-like ✨",
    );
    println("<orange> 🦀: \"banana\".matches(\"an\").count()");
    let count_res = "banana".count("an");
    println("<darkturquoise,b>📊 Result: {count_res}");

    println("\n<gold,b>⭐ splitby - The Python-like Revolution:");
    println("<limegreen,b>🎯 ONE method, THREE patterns - Just like Python!");

    println("\n<cyan,b>📋 Pattern 1 - Delimiter splitby:");
    println(
        "<yellow> 🐍: \"python,rust,go\".split(\",\")                # ['python', 'rust', 'go']",
    );
    println(
        "<deepskyblue> 🧩: \"python,rust,go\".splitby(\",\")              // QUASI-IDENTICAL! ✨",
    );
    println("<orange> 🦀: \"python,rust,go\".split(',').collect::<Vec<_>>()");
    let langs = "python,rust,go".splitby(",");
    println("<darkturquoise,b>📊 Result: {langs:c}");

    println("\n<cyan,b>🔤 Pattern 2 - Whitespace splitby:");
    println(
        "<yellow> 🐍: \"hello  world\\ttab\".split()                 # ['hello', 'world', 'tab']",
    );
    println(
        "<deepskyblue> 🧩: \"hello  world\\ttab\".splitby(\"\")             // QUASI-IDENTICAL! ✨",
    );
    println("<orange> 🦀: \"hello  world\\ttab\".split_whitespace().collect::<Vec<_>>()");
    let words = "hello  world\ttab".splitby("");
    println("<darkturquoise,b>📊 Result: {words:c}");

    println("\n<cyan,b>📄 Pattern 3 - Line splitby:");
    println("<yellow> 🐍: \"L1\\nL2\\nL3\".split(\"\\n\")                  # ['L1', 'L2', 'L3']");
    println(
        "<deepskyblue> 🧩: \"L1\\nL2\\nL3\".splitby(\"\\n\")                // QUASI-IDENTICAL! ✨",
    );
    println("<orange> 🦀: \"L1\\nL2\\nL3\".lines().collect::<Vec<_>>()");
    let lines = "L1\nL2\nL3".splitby("\n");
    println("<darkturquoise,b>📊 Result: {lines:c}");

    println("\n<gold,b>🔗 Fluent Chaining - The Magic:");
    println("<yellow> 🐍: \", \".join(\"a,b,c\".split(\",\"))             # \"a, b, c\"");
    println(
        "<deepskyblue> 🧩: \"a,b,c\".splitby(\",\").join(\", \")            // PURE ELEGANCE! ✨",
    );
    let chain1 = "a,b,c".splitby(",").join(" → ");
    println("<darkturquoise,b>📊 Result: '{chain1}'");

    println("<yellow> 🐍: \"-\".join(\"hello world\".split())             # \"hello-world\"");
    println(
        "<deepskyblue> 🧩: \"hello world\".splitby(\"\").join(\"-\")        // PURE ELEGANCE! ✨",
    );
    let chain2 = "hello world".splitby("").join("-");
    println("<darkturquoise,b>📊 Result: '{chain2}'");

    println("<yellow> 🐍: \" | \".join(\"L1\\nL2\".split(\"\\n\"))          # \"L1 | L2\"");
    println(
        "<deepskyblue> 🧩: \"L1\\nL2\".splitby(\"\\n\").join(\" | \")        // PURE ELEGANCE! ✨",
    );
    let chain3 = "L1\nL2\nL3".splitby("\n").join(" ✨ ");
    println("<darkturquoise,b>📊 Result: '{chain3}'");

    println("\n<gold,b>🔄 Replace:");
    println("<yellow> 🐍: \"a-b-c\".replace('-', '_')                  # \"a_b_c\"");
    println("<deepskyblue> 🧩: \"a-b-c\".replace(\"-\", \"_\")             // same as std");
    println("<orange> 🦀: \"a-b-c\".replace(\"-\", \"_\")");
    let rep = "a-b-c".replace("-", "_");
    println("<darkturquoise,b>📊 Result: '{rep}'");

    println("<yellow> 🐍: \"a-b-c\".replace('-', '_', 1)              # \"a_b-c\"");
    println("<deepskyblue> 🧩: \"a-b-c\".replacen(\"-\", \"_\", 1)         // same as std");
    println("<orange> 🦀: \"a-b-c\".replacen(\"-\", \"_\", 1)");
    let rep1 = "a-b-c".replacen("-", "_", 1);
    println("<darkturquoise,b>📊 Result: '{rep1}'");

    println("\n<gold,b>🎨 Padding & Formatting:");
    println("<yellow> 🐍: \"42\".zfill(6)                             # \"000042\"");
    println("<deepskyblue> 🧩: \"42\".zfill(6)                            // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let z = "42".zfill(6);
    println("<darkturquoise,b>📊 Result: '{z}'");

    println("<yellow> 🐍: \"left\".ljust(10, '-')                      # \"left------\"");
    println("<deepskyblue> 🧩: \"left\".ljust(10, '-')                     // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let lj = "left".ljust(10, '-');
    println("<darkturquoise,b>📊 Result: '{lj}'");

    println("<yellow> 🐍: \"right\".rjust(10, '-')                     # \"-----right\"");
    println("<deepskyblue> 🧩: \"right\".rjust(10, '-')                    // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let rj = "right".rjust(10, '-');
    println("<darkturquoise,b>📊 Result: '{rj}'");

    println("<yellow> 🐍: \"rust\".center(10, '*')                     # \"***rust***\"");
    println("<deepskyblue> 🧩: \"rust\".center(10, '*')                    // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let c = "rust".center(10, '*');
    println("<darkturquoise,b>📊 Result: '{c}'");

    println("\n<gold,b>🏷️ Prefix / Suffix:");
    println("<yellow> 🐍: \"prefix_hello\".removeprefix('prefix_')     # \"hello\"");
    println("<deepskyblue> 🧩: \"prefix_hello\".removeprefix(\"prefix_\")  // WebRust alias");
    println("<orange> 🦀: \"prefix_hello\".strip_prefix(\"prefix_\").unwrap_or(\"prefix_hello\")");
    let no_pre = "prefix_hello".removeprefix("prefix_");
    println("<darkturquoise,b>📊 Result: '{no_pre}'");

    println("<yellow> 🐍: \"world!!!\".removesuffix('!')               # \"world!!\"");
    println("<deepskyblue> 🧩: \"world!!!\".removesuffix(\"!\")            // WebRust alias");
    println("<orange> 🦀: \"world!!!\".strip_suffix(\"!\").unwrap_or(\"world!!!\")");
    let no_suf = "world!!!".removesuffix("!");
    println("<darkturquoise,b>📊 Result: '{no_suf}'");

    println("\n<gold,b>✅ Content Validation:");
    println("<yellow> 🐍: \"hello\".isalpha()                        # True");
    println("<deepskyblue> 🧩: \"hello\".isalpha()                       // WebRust alias");
    println("<orange> 🦀: \"hello\".chars().all(|c| c.is_alphabetic())");
    let is_alpha = "hello".isalpha();
    println("<darkturquoise,b>📊 Result: {is_alpha}");

    println("<yellow> 🐍: \"12345\".isdigit()                        # True");
    println("<deepskyblue> 🧩: \"12345\".isdigit()                       // WebRust alias");
    println("<orange> 🦀: \"12345\".chars().all(|c| c.is_ascii_digit())");
    let is_digit = "12345".isdigit();
    println("<darkturquoise,b>📊 Result: {is_digit}");

    println("<yellow> 🐍: \"Rust2025\".isalnum()                      # True");
    println("<deepskyblue> 🧩: \"Rust2025\".isalnum()                     // WebRust alias");
    println("<orange> 🦀: \"Rust2025\".chars().all(|c| c.is_alphanumeric())");
    let is_alnum = "Rust2025".isalnum();
    println("<darkturquoise,b>📊 Result: {is_alnum}");

    println("<yellow> 🐍: \"RUST\".isupper()                         # True");
    println("<deepskyblue> 🧩: \"RUST\".isupper()                        // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let is_up = "RUST".isupper();
    println("<darkturquoise,b>📊 Result: {is_up}");

    println("<yellow> 🐍: \"python\".islower()                       # True");
    println("<deepskyblue> 🧩: \"python\".islower()                      // WebRust custom impl");
    println("<orange> 🦀: (custom implementation)");
    let is_low = "python".islower();
    println("<darkturquoise,b>📊 Result: {is_low}");

    println("\n<gold,b>🌍 Real-World Examples:");

    println("\n<cyan,b>📊 CSV Processing:");
    let csv_data = "name,age,city\nAlice,25,Paris\nBob,30,Lyon";
    let rows = csv_data.splitby("\n");
    let headers = rows[0].splitby(",");
    println("<darkturquoise,b>📋 Headers: {headers:c}");

    for (i, row) in rows.iter().skip(1).enumerate() {
        let fields = row.splitby(",");
        let row_num = i + 1;
        let name = fields[0];
        let age = fields[1];
        let city = fields[2];
        println("<lightblue>   Row {row_num}: {name} (age {age}) from {city}");
    }

    println("\n<cyan,b>🏷️ Tag Processing:");
    let raw_tags = "rust, performance,  safety,   memory ";
    let clean_tags = raw_tags
        .splitby(",")
        .iter()
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty())
        .collect::<Vec<_>>()
        .join(" • ");
    println("<darkturquoise,b>🏷️ Clean tags: {clean_tags}");

    println("\n<cyan,b>📁 Path Processing:");
    let path = "src/viz/string.rs";
    let parts = path.splitby("/");
    println("<darkturquoise,b>📂 Path parts: {parts:c}");
    let file_parts = parts.last().unwrap().splitby(".");
    let file_name = file_parts[0];
    let file_ext = file_parts[1];
    println("<darkturquoise,b>📄 File: name='{file_name}', ext='{file_ext}'");

    println("\n<gold,b>⚡ Performance Showcase:");
    println("<limegreen>🚀 All splitby() methods are #[inline] - Zero overhead!");
    println("<limegreen>🎯 Pattern matching resolved at compile time!");
    println("<limegreen>✨ Python syntax with Rust performance!");

    println("\n<limegreen,b>💬 Interactive Demo:");
    let input_txt: String = input("Enter text with commas, spaces, or newlines:");
    if !input_txt.is_empty() {
        println("\n<cyan,b>🔍 Analysis:");

        let by_comma = input_txt.splitby(",");
        let by_space = input_txt.splitby("");
        let by_newline = input_txt.splitby("\n");

        if by_comma.len() > 1 {
            println("<yellow>📋 CSV-like (comma): {by_comma:c}");
        }
        if by_space.len() > 1 {
            println("<lightblue>🔤 Words (space): {by_space:c}");
        }
        if by_newline.len() > 1 {
            println("<lightgreen>📄 Lines (newline): {by_newline:c}");
        }

        let upper = input_txt.upper();
        let is_alpha = input_txt.isalpha();
        let trimmed = input_txt.trim();

        println("<darkturquoise,b>✨ Transformations:");
        println("<darkturquoise>   Upper: '{upper}'");
        println("<darkturquoise>   Is alphabetic: {is_alpha}");
        println("<darkturquoise>   Trimmed: '{trimmed}'");
    }

    println("\n<gold,b>🎉 WebRust Achievement Unlocked:");
    println("<limegreen,b>✅ Python syntax: splitby(\",\"), splitby(\"\"), splitby(\"\\n\")");
    println("<limegreen,b>✅ Rust performance: Zero-cost abstractions");
    println("<limegreen,b>✅ Fluent chaining: .splitby().join()");
    println("<limegreen,b>✅ Type safety: Compile-time guarantees");
    println("<limegreen,b>✅ Ergonomics: Best of both worlds!");

    println("\n<darkturquoise,b>🚀 WebRust - Where Python meets Rust performance!");
    println(
        "<silver>💡 One trait, infinite possibilities. Welcome to the future of string processing!",
    );
}
