/*
   Output and output-related code.
*/

use crate::opts::Cli;
use crate::Entry;
use lazy_static::lazy_static;
use regex::Regex;

struct OutputName {
    name: String,
    size: usize,
    quoted: bool,
}

/*
   Output the set of names passed in, simple columns with names only. Assume
   they are already sorted by the order that the configuration specified.
*/
pub fn output_names_simple(entries: &Vec<Entry>, width: u32, config: &Cli) {
    let count: usize = entries.len();
    let names: Vec<OutputName> = entries
        .iter()
        .map(|e| entry_to_output_with_frills(e, config))
        .collect();
    let max_len: usize = names.iter().map(|e| e.size).max().unwrap();
    let any_quoted: bool = names.iter().any(|e| e.quoted);
    let all_quoted: bool = config.quote_name;
    let mut leader: u8 = 0;
    let mut spacer: u8 = 2;
    if any_quoted && !all_quoted {
        leader += 1;
        spacer += 1;
    }
    let mut cols: u8 = 1;
    if !(config.one || config.zero) {
        loop {
            let w = max_len as u8 * cols + spacer * (cols - 1) + leader;
            if w as u32 > width {
                break;
            } else {
                cols += 1;
            }
        }
    }
    // Determine the full-length column size and the remainder (if any) size.
}

fn entry_to_output_with_frills(entry: &Entry, config: &Cli) -> OutputName {
    lazy_static! {
        static ref WS: Regex = Regex::new(r"(\s)").unwrap();
    }
    lazy_static! {
        static ref SP: Regex = Regex::new(r"[/*@=|]").unwrap();
    }
    lazy_static! {
        static ref SQ: Regex = Regex::new(r"(')").unwrap();
    }
    lazy_static! {
        static ref DQ: Regex = Regex::new(r#"(")"#).unwrap();
    }

    let base_name: String = entry.name.clone();
    let has_whitespace: bool = match base_name.find(char::is_whitespace) {
        Some(_) => true,
        None => false,
    };
    let has_special: bool = SP.is_match(&base_name);
    let needs_quote = has_whitespace || has_special;

    let quoted_name: String = if config.quote_name {
        // The CLI options require all names be quoted. Use "" for quoting.
        let mut quoted = DQ.replace_all(&base_name, "\\$1").into_owned();
        quoted.insert(0, '"');
        quoted.push('"');
        quoted
    } else if needs_quote {
        // The name contains whitespace, so quote it with ''.
        let mut quoted = WS.replace_all(&base_name, "\\$1").into_owned();
        quoted = SQ.replace_all(&quoted, "\\$1").into_owned();
        quoted.insert(0, '"');
        quoted.push('"');
        quoted
    } else {
        base_name
    };

    OutputName {
        name: quoted_name.clone(),
        size: quoted_name.len(),
        quoted: needs_quote,
    }
}
