// examples/py_utils.rs
use webrust::prelude::*;

#[gui(Times 14px black !white)]
fn main() {
    println("<blue b>🎯 webrust Range and Iterator Examples");
    println("<gray i>Demonstrating Python-like comprehensions with when/then syntax:\n");

    println("\n<green b>=== Numeric Tests (Horizontal Tables) ===");

    println("<magenta i>0.to(5):\n");
    for i in 0.to(5) {
        print("<white !purple w25 m1>{i}");
    }

    println("<magenta i>4.5.to(0.0):\n");
    for i in 4.5.to(0.0) {
        print("<white !darkred w30 m1>{i}");
    }

    println("<magenta i>20.to(0).by(-2):\n");
    for i in 20.to(0).by(-2) {
        print("<white !navy w20 m1>{i}");
    }

    println("<magenta i>0.0.to(4.0).by(0.25):\n");
    for x in 0.0.to(4.0).by(0.25) {
        print("<white !darkgreen w25 m1>{x}");
    }

    println("\n<green b>=== Character Tests (Horizontal Tables) ===");

    println("<magenta i>'a'.to('z'):\n");
    for c in 'a'.to('z') {
        print("<white !darkorange w10 m1>{c}");
    }

    println("<magenta i>'Z'.to('A').by(-2):\n");
    for c in 'Z'.to('A').by(-2) {
        print("<white !darkmagenta w25 m1>{c}");
    }

    println("<magenta i>enumerate(0.to(5)):\n");
    let e1: Vec<(usize, i32)> = enumerate(0.to(5)).then(|(i, v)| (i, v));
    table(&e1).header(["Index", "Value"]);

    let names = vec!["Bob", "Alice", "Guido"];
    println("<magenta i>enumerate(&names):\n");
    let e2: Vec<(usize, &str)> = enumerate(&names).then(|(i, &v)| (i, v));
    table(&e2).header(["Index", "Name"]);

    println("<magenta i>enumerate(&names) + start (10):\n");
    let start = 10usize;
    let e3: Vec<(usize, &str)> = enumerate(&names).then(|(i, &v)| (i + start, v));
    table(&e3).header(["Index", "Name"]);

    println("<magenta i>enumerate('a'.to('e')) + start (50):\n");
    let start2 = 50usize;
    let e4: Vec<(usize, char)> = enumerate('a'.to('e')).then(|(i, c)| (i + start2, c));
    table(&e4).header(["Index", "Character"]);

    println("\n<purple b>=== Advanced Tests (Real Tables) ===");

    println("<magenta i>Multiples of 3 up to 30:\n");
    let mult3: Vec<(usize, i32)> = enumerate(0.to(11)).then(|(idx, i)| (idx, i * 3));
    table(&mult3).header(["Index", "Multiple"]);

    let fruits = vec!["apple", "banana", "cherry", "date"];
    println("<magenta i>Fruits with length:\n");
    let fruits_tbl: Vec<(usize, &str, usize)> =
        enumerate(&fruits).then(|(i, &f)| (i + 1, f, f.len()));
    table(&fruits_tbl).header(["Index", "Fruit", "Len"]);

    println("<magenta i>Letters with ASCII codes:\n");
    let letters_ascii: Vec<(usize, char, u8)> =
        enumerate('A'.to('F')).then(|(i, c)| (i, c, c as u8));
    table(&letters_ascii).header(["Index", "Letter", "ASCII"]);

    println("<magenta i>Square roots (0.0 to 2.0):\n");
    let sqrt_tbl: Vec<(usize, f64)> = enumerate(0.to(5)).then(|(i, v)| {
        let r = ((v as f64).sqrt() * 1000.0).round() / 1000.0;
        (i, r)
    });
    table(&sqrt_tbl).header(["Index", "Square Root"]);

    let ages = vec![15, 25, 35, 45, 55];
    println("<magenta i>Age categories:\n");
    let ages_tbl: Vec<(usize, &str, i32)> = enumerate(ages).then(|(i, age)| {
        let cat = if age < 18 {
            "minor"
        } else if age < 65 {
            "adult"
        } else {
            "senior"
        };
        (i + 1, cat, age)
    });
    table(&ages_tbl).header(["Index", "Category", "Age"]);

    println("\n<red b>=== Range + Enumerate Combinations (Tables) ===");
    println("<magenta i>Even numbers with index:\n");

    println("<mb0>");
    print("<SteelBlue !lightskyblue w80 t1 |navy p2>Index");
    print("<SteelBlue !lightskyblue w120 t1 |navy p2>Even Number");
    for (index, number) in enumerate(0.to(20).by(2)) {
        println("<mb0>");
        print("<SteelBlue !White w80 t1 |navy p2>{index}");
        print("<SteelBlue !White w120 t1 |navy p2>{number}");
    }

    println("<magenta i>Vowels with position:\n");
    println("<mb0>");
    print("<SteelBlue !lightskyblue w80 t1 |navy p2>Index");
    print("<SteelBlue !lightskyblue w80 t1 |navy p2>Vowel");
    for (index, vowel) in enumerate(['a', 'e', 'i', 'o', 'u']) {
        println("<mb0>");
        print("<SteelBlue w80 t1 |navy p2>{index + 1}");
        print("<SteelBlue w80 t1 |navy p2>{vowel}");
    }

    println("<magenta i>Cubes of numbers 1-5:\n");
    println("<mb0>");
    print("<SteelBlue !lightskyblue  w80 t1 |navy p2>Index");
    print("<SteelBlue !lightskyblue  w100 t1 |navy p2>Number & Cube");
    for (index, number) in enumerate(1.to(6)) {
        println("<mb0>");
        let cube = number * number * number;
        print("<SteelBlue w80 t1 |navy p2>{index}");
        print("<SteelBlue w100 t1 |navy p2>${number}^3 = {cube}$");
    }

    println("<bright_magenta i b>\n🎉 Demonstration completed with real tables!");

    println("\n<darkmagenta b>=== List and Dictionary Comprehensions - NEW SYNTAX ===");
    println("<gray i>Speaking syntax: when = filter, then = map + collect automatically!");

    println("\n<green b>📋 Basic examples");

    let squares: Vec<i32> = 0.to(10).then(|x| x * x);
    println("<darkblue> 🐍: [x**2 for x in range(10)]");
    println("<darkgreen> 🦀: 0.to(10).then(|x| x * x)");
    println("<blue b>📊 Result: {squares:c}");

    let stepped: Vec<i32> = 0.to(20).by(3).then(|x| x);
    println("\n<darkblue> 🐍: [x for x in range(0, 20, 3)]");
    println("<darkgreen> 🦀: 0.to(20).by(3).then(|x| x)");
    println("<blue b>📊 Result: {stepped:c}");

    let letters: Vec<char> = (0..5).then(|i| (b'a' + i as u8) as char);
    println("\n<darkblue> 🐍: [chr(ord('a') + i) for i in range(5)]");
    println("<darkgreen> 🦀: (0..5).then(|i| (b'a' + i as u8) as char)");
    println("<blue b>📊 Result: {letters:c}");

    println("\n<blue b>📚 Dictionaries (automatic inference!)");

    println("<darkblue>🐍: {{x: x**2 for x in range(5)}}");
    println("<darkgreen> 🦀: 0.to(5).then(|x| (x, x * x))");
    let squares_dict: Vec<(i32, i32)> = 0.to(5).then(|x| (x, x * x));
    table(&squares_dict).header(["Number", "Square"]);

    println("<darkblue>\n 🐍: {{c: ord(c) for c in 'abcde'}}");
    println("<darkgreen> 🦀: 'a'.to('f').then(|c| (c, c as u8))");
    let char_codes: Vec<(char, u8)> = 'a'.to('f').then(|c| (c, c as u8));
    table(&char_codes).header(["Char", "ASCII"]);

    println("\n<purple b>🔍 Filtering with when()");

    let evens: Vec<i32> = 0.to(20).when(|&x| x % 2 == 0).then(|x| x);
    println("<darkblue> 🐍: [x for x in range(20) if x % 2 == 0]");
    println("<darkgreen> 🦀: 0.to(20).when(|&x| x % 2 == 0).then(|x| x)");
    println("<blue b>📊 Result: {evens:c}");

    let filtered_squares: Vec<i32> = 0.to(20).when(|&x| x % 2 == 0 && x % 3 == 0).then(|x| x * x);
    println("\n<magenta b>Multiple conditions:");
    println("<darkblue> 🐍: [x**2 for x in range(20) if x % 2 == 0 and x % 3 == 0]");
    println("<darkgreen> 🦀: 0.to(20).when(|&x| x % 2 == 0 && x % 3 == 0).then(|x| x * x)");
    println("<blue b>📊 Result: {filtered_squares:c} ");

    let filtered_chain1: Vec<i32> = 0.to(20).when(|&x| x % 2 == 0 && x % 3 == 0).then(|x| x * x);
    let filtered_chain2: Vec<i32> = (0..20)
        .when(|&x| x % 2 == 0)
        .when(|&x| x % 3 == 0)
        .then(|x| x * x);
    println("\n<magenta b>Chained filters (webrust advantage):");
    println("<darkblue># Python doesn't chain filters, but equivalent:");
    println("<darkgreen> 🦀: 0.to(20).when(|&x| x % 2 == 0 && x % 3 == 0).then(|x| x * x)");
    println("<darkgreen> 🦀: 0.to(20).when(|&x| x % 2 == 0).when(|&x| x % 3 == 0).then(|x| x * x)");
    println("<blue b>📊 Result: {&filtered_chain1:c}");

    println("\n<blue b>🌍 Real examples with new syntax");

    let fib_count: i32 = 7;
    let mut fib = vec![0i64, 1i64];
    for i in 2..fib_count as usize {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }

    let fib_pairs: Vec<(usize, i64)> = enumerate(&fib).then(|(i, &v)| (i, v));
    table(&fib_pairs).header(["Index", "Fibonacci"]);

    let ratios: Vec<f64> = (1..fib.len())
        .when(|&i| fib[i - 1] != 0)
        .then(|i| fib[i] as f64 / fib[i - 1] as f64);
    println("\n<yellow b>Fibonacci ratios:");
    for (i, ratio) in enumerate(ratios) {
        let next_idx = i + 2;
        let curr_idx = i + 1;
        println("F{next_idx}/F{curr_idx} = {ratio:.6}");
    }

    let words = vec!["hello", "world", "python", "rust", "comprehension"];
    let long_words: Vec<String> = words.when(|word| word.len() > 4).then(|word| word.upper());
    println("\n<orange b>Text processing:");
    println("<darkblue> 🐍: [word.upper() for word in words if len(word) > 4]");
    println("<darkgreen> 🦀: words.when(|word| word.len() > 4).then(|word| word.upper())");
    println("<blue b>📊 Words > 4 characters: {long_words:c}");

    println("\n<bright_green b>✅ Perfect Balance Achieved!");
    println("<gray>✨ Speaking syntax: when/then vs filter/map");
    println("<gray>🚀 Auto collect(): no manual .collect() needed");
    println("<gray>⚡ Native performance: compiles to identical iterator chains");
    println("<gray>🎯 Type inference: Vec/HashMap automatically detected");
    println("<gray>🔗 Chainable: .when().when().then() flows naturally");
    println("<gray>🌍 Universal: works with webrust ranges, std ranges, vectors, arrays!");
}
