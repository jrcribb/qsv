use crate::workdir::Workdir;

fn data(headers: bool) -> String {
    if headers {
        String::from("name,age,city\nJohn,30,New York\nJane,25,Boston\n")
    } else {
        String::from("John,30,New York\nJane,25,Boston\n")
    }
}

#[test]
fn template_basic() {
    let wrk = Workdir::new("template_basic");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Hello {{name}} from {{city}}!\n\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Hello John from New York!\nHello Jane from Boston!";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_no_headers() {
    let wrk = Workdir::new("template_no_headers");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Name: {{_c1}}, Age: {{_c2}}\n\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv")
        .arg("--no-headers");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Name: name, Age: age\nName: John, Age: 30\nName: Jane, Age: 25";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_string() {
    let wrk = Workdir::new("template_string");
    wrk.create_from_string("data.csv", &data(true));

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}} is {{age}} years old\n\n")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John is 30 years old\nJane is 25 years old";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_custom_delimiter() {
    let wrk = Workdir::new("template_custom_delimiter");
    wrk.create_from_string(
        "data.csv",
        "name;age;city\nJohn;30;New York\nJane;25;Boston\n",
    );
    wrk.create_from_string("template.txt", "Name: {{ name }}, Age: {{age}}\n\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv")
        .args(["--delimiter", ";"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Name: John, Age: 30\nName: Jane, Age: 25";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_with_filters() {
    let wrk = Workdir::new("template_filters");
    wrk.create_from_string("data.csv", "name,amount\nJohn,1234.5678\nJane,9876.54321\n");
    wrk.create_from_string(
        "template.txt",
        "{{ name }}: ${{ amount | float | round(2) }}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John: $1234.57\nJane: $9876.54";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_with_conditionals() {
    let wrk = Workdir::new("template_conditionals");
    wrk.create_from_string("data.csv", "name,age\nJohn,17\nJane,21\n");
    wrk.create_from_string(
        "template.txt",
        "{{ name }} is {% if age | int >= 18 %}an adult{% else %}a minor{% endif %}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John is a minor\nJane is an adult";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_missing_field() {
    let wrk = Workdir::new("template_missing_field");
    wrk.create_from_string("data.csv", "name,age\nJohn,30\nJane,25\n");
    wrk.create_from_string(
        "template.txt",
        "{{ name }} ({{ missing_field | default('N/A') }})\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John (N/A)\nJane (N/A)";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_empty_input() {
    let wrk = Workdir::new("template_empty");
    wrk.create_from_string("data.csv", "name,age\n");
    wrk.create_from_string("template.txt", "Hello {{name}}!\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_with_loops() {
    let wrk = Workdir::new("template_loops");
    wrk.create_from_string(
        "data.csv",
        "name,hobbies\nJohn,\"reading,gaming,cooking\"\nJane,\"hiking,painting\"\n",
    );
    wrk.create_from_string(
        "template.txt",
        "{{ name }}'s hobbies: {% for hobby in hobbies | split(',') %}{{ hobby | trim }}{% if not \
         loop.last %}, {% endif %}{% endfor %}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John's hobbies: reading, gaming, cooking, \nJane's hobbies: hiking, painting, ";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_error_invalid_syntax() {
    let wrk = Workdir::new("template_invalid_syntax");
    wrk.create_from_string("data.csv", "name,age\nJohn,30\n");
    wrk.create_from_string("template.txt", "{{ name } }}\n"); // Invalid syntax

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn template_error_missing_template() {
    let wrk = Workdir::new("template_missing_template");
    wrk.create_from_string("data.csv", "name,age\nJohn,30\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("nonexistent.txt")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn template_with_whitespace_control() {
    let wrk = Workdir::new("template_whitespace");
    wrk.create_from_string("data.csv", "name,items\nJohn,\"a,b,c\"\n");
    wrk.create_from_string(
        "template.txt",
        "Items:{%- for item in items | split(',') %}\n  - {{ item }}{%- if not loop.last %}{%- \
         endif %}{%- endfor %}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Items:\n  - a\n  - b\n  - c";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_output_file() {
    let wrk = Workdir::new("template_output");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "{{name}},{{city}}\n\n");

    let output_file = "output.txt";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("--output")
        .arg(output_file)
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(output_file);
    let expected = "John,New York\nJane,Boston\n";
    assert_eq!(got, expected);
}

#[test]
fn template_output_directory() {
    let wrk = Workdir::new("template_output_dir");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Hello {{name}} from {{city}}!\n");

    let outdir = "output_dir";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv")
        .arg(outdir);

    wrk.assert_success(&mut cmd);

    // Check that files were created with default ROWNO naming
    let file1 = wrk.read_to_string(&format!("{outdir}/1.txt"));
    let file2 = wrk.read_to_string(&format!("{outdir}/2.txt"));

    assert_eq!(file1, "Hello John from New York!");
    assert_eq!(file2, "Hello Jane from Boston!");
}

#[test]
fn template_output_custom_filename() {
    let wrk = Workdir::new("template_custom_filename");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Greetings from {{city}}!\n");

    let outdir = "custom_output";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("--outfilename")
        .arg("{{name}}_greeting-{{ QSV_ROWNO }}.txt")
        .arg("data.csv")
        .arg(outdir);

    wrk.assert_success(&mut cmd);

    // Check that files were created with custom naming
    let file1 = wrk.read_to_string(&format!("{outdir}/John_greeting-1.txt"));
    let file2 = wrk.read_to_string(&format!("{outdir}/Jane_greeting-2.txt"));

    assert_eq!(file1, "Greetings from New York!");
    assert_eq!(file2, "Greetings from Boston!");
}

#[test]
fn template_output_directory_no_headers() {
    let wrk = Workdir::new("template_output_dir_no_headers");
    wrk.create_from_string("data.csv", &data(false));
    wrk.create_from_string("template.txt", "Record: {{_c1}} - {{_c3}}\n");

    let outdir = "no_headers_output";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("--no-headers")
        .arg("data.csv")
        .arg(outdir);

    wrk.assert_success(&mut cmd);

    // Check files with row numbers
    let file1 = wrk.read_to_string(&format!("{outdir}/1.txt"));
    let file2 = wrk.read_to_string(&format!("{outdir}/2.txt"));

    assert_eq!(file1, "Record: John - New York");
    assert_eq!(file2, "Record: Jane - Boston");
}

#[test]
fn template_custom_filters() {
    let wrk = Workdir::new("template_custom_filters");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "amount", "bytes", "score", "active"],
            svec!["John", "1234567", "1048576", "3.14159", "yes"],
            svec!["Jane", "7654321.04", "1073741824", "2.71828", "no"],
        ],
    );

    // Test all custom filters
    wrk.create_from_string(
        "template.txt",
        "Name: {{ name|substr(0,2) }}\nAmount: {{ amount|human_count }}\nBytes: {{ \
         bytes|float|filesizeformat }} {{bytes|float|filesizeformat(true) }}\nScore (2 decimals): \
         {{ score|format_float(2) }}\nScore (rounded): {{ score|round_num(1) }}\nActive: {{ \
         active|str_to_bool }}\nFloat with commas: {{ amount|human_float_count }}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Name: Jo
Amount: 1,234,567
Bytes: 1.0 MB 1.0 MiB
Score (2 decimals): 3.14
Score (rounded): 3.1
Active: true
Float with commas: 1,234,567
Name: Ja
Amount: <FILTER_ERROR>
Bytes: 1.1 GB 1.0 GiB
Score (2 decimals): 2.72
Score (rounded): 2.7
Active: false
Float with commas: 7,654,321.04"#;
    assert_eq!(got, expected);
}

#[test]
fn template_inline() {
    let wrk = Workdir::new("template_inline");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "25"],
            svec!["Bob", "30"],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("Hello {{name}}, you are {{age}} years old!\n\n")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
Hello Alice, you are 25 years old!
Hello Bob, you are 30 years old!";
    assert_eq!(got, expected);
}

#[test]
fn template_conditional() {
    let wrk = Workdir::new("template_conditional");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "17"],
            svec!["Bob", "21"],
        ],
    );

    wrk.create_from_string(
        "template.txt",
        "{{ name }} is {% if age|round_num(0) >= '18' %}an adult{% else %}a minor{% endif %}.\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
Alice is a minor.
Bob is an adult.";
    assert_eq!(got, expected);
}

#[test]
fn template_render_error() {
    let wrk = Workdir::new("template_render_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "25"],
            svec!["Bob", "30"],
        ],
    );

    // Test invalid template syntax with default error message
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("Hello {{name}, invalid syntax!")
        .arg("data.csv");

    wrk.assert_err(&mut *&mut cmd);
    let got: String = wrk.output_stderr(&mut cmd);
    let expected =
        "syntax error: unexpected `}}`, expected end of variable block (in template:1)\n";
    assert_eq!(got, expected);
}

#[test]
fn template_filter_error() {
    let wrk = Workdir::new("template_filter_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "amount"],
            svec!["Alice", "not_a_number"],
            svec!["Bob", "123.45"],
        ],
    );

    // Test filter error with default error message
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}}: {{amount|format_float(2)}}\n\n")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Alice: <FILTER_ERROR>\nBob: 123.45";
    assert_eq!(got, expected);

    // Test custom filter error message
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}}: {{amount|format_float(2)}}\n\n")
        .arg("--customfilter-error")
        .arg("INVALID NUMBER")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Alice: INVALID NUMBER\nBob: 123.45";
    assert_eq!(got, expected);

    // Test empty string as filter error
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}}: {{amount|format_float(2)}}\n\n")
        .arg("--customfilter-error")
        .arg("<empty string>")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Alice: \nBob: 123.45";
    assert_eq!(got, expected);
}

#[test]
fn template_contrib_filters() {
    let wrk = Workdir::new("template_contrib_filters");
    wrk.create(
        "data.csv",
        vec![
            svec!["text", "num", "datalist", "url"],
            svec![
                "hello WORLD",
                "12345.6789",
                "a,b,c",
                "https://example.com/path?q=test&lang=en"
            ],
            svec![
                "Testing 123",
                "-98765.4321",
                "1,2,3",
                "http://localhost:8080/api"
            ],
        ],
    );

    // Test various minijinja_contrib filters
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            // String filters
            "capitalize: {{text|capitalize}}\n",
            "title: {{text|title}}\n",
            "upper: {{text|upper}}\n",
            "lower: {{text|lower}}\n",
            // URL encode
            "urlencode: {{text|urlencode}}\n",
            // List filters
            "split: {{datalist|split(',')|join('|')}}\n",
            "first: {{datalist|split(',')|first}}\n",
            "last: {{datalist|split(',')|last}}\n",
            // Add newline between records
            "\n"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "capitalize: Hello world\n",
        "title: Hello World\n",
        "upper: HELLO WORLD\n",
        "lower: hello world\n",
        "urlencode: hello%20WORLD\n",
        "split: a|b|c\n",
        "first: a\n",
        "last: c\n",
        "capitalize: Testing 123\n",
        "title: Testing 123\n",
        "upper: TESTING 123\n",
        "lower: testing 123\n",
        "urlencode: Testing%20123\n",
        "split: 1|2|3\n",
        "first: 1\n",
        "last: 3",
    );
    assert_eq!(got, expected);
}

#[test]
fn template_contrib_functions() {
    let wrk = Workdir::new("template_contrib_functions");
    wrk.create(
        "data.csv",
        vec![
            svec!["num_messages", "date_col"],
            svec!["1", "2023-06-24T16:37:22+00:00"],
            svec!["2", "1999-12-24T16:37:22+12:00"],
        ],
    );

    // Test various minijinja_contrib functions
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "pluralize: You have {{ num_messages }} message{{ num_messages|int|pluralize }}\n",
            "now: {{now()|datetimeformat|length > 2}}\n", // Just verify we get a non-empty string
            "dtformat: {{date_col|datetimeformat(format=\"long\", tz=\"EST\")}}\n",
            "\n\n"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "pluralize: You have 1 message\n",
        "now: true\n",
        "dtformat: June 24 2023 11:37:22\n",
        "\n",
        "pluralize: You have 2 messages\n",
        "now: true\n",
        "dtformat: December 23 1999 23:37:22",
    );
    assert_eq!(got, expected);
}

#[test]
fn template_pycompat_filters() {
    let wrk = Workdir::new("template_pycompat_filters");
    wrk.create(
        "data.csv",
        vec![
            svec!["text", "num", "mixed"],
            svec!["Hello World!", "123", "ABC123xyz  "],
            svec!["TESTING", "abc", "  Hello  "],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            // Test string methods from Python compatibility
            "isascii: {{text.isascii()}}\n",
            "isdigit: {{num.isdigit()}}\n",
            "startswith: {{text.startswith('Hello')}}\n",
            "isnumeric: {{num.isnumeric()}}\n",
            "isupper: {{text.isupper()}}\n",
            "replace: {{mixed.replace('ABC', 'XYZ')}}\n",
            "rfind: {{mixed.rfind('xyz')}}\n",
            "rstrip: {{mixed.rstrip()}}\n",
            "\n"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "isascii: true\n",
        "isdigit: true\n",
        "startswith: true\n",
        "isnumeric: true\n",
        "isupper: false\n",
        "replace: XYZ123xyz  \n",
        "rfind: 6\n",
        "rstrip: ABC123xyz\n",
        "isascii: true\n",
        "isdigit: false\n",
        "startswith: false\n",
        "isnumeric: false\n",
        "isupper: true\n",
        "replace:   Hello  \n",
        "rfind: -1\n",
        "rstrip:   Hello",
    );
    assert_eq!(got, expected);
}
