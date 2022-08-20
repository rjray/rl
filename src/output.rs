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
   Output the set of names passed in. Assume they are already sorted by the
   order that the configuration specified.
*/
pub fn output_names(entries: &Vec<Entry>, width: u32, config: &Cli) {
    let names: Vec<OutputName> = entries
        .into_iter()
        .map(|e| entry_to_output_with_frills(e, config))
        .collect();
    let max_len: usize = names.iter().map(|e| e.size).max().unwrap();
    let quoted: bool = names.iter().any(|e| e.quoted);
    let mut leader: u8 = 0;
    let mut spacer: u8 = 2;
    if quoted {
        leader += 1;
        spacer += 1;
    }
    let mut cols: u8 = 1;
    loop {
        let w = max_len as u8 * cols + spacer * (cols - 1) + leader;
        if w as u32 > width {
            break;
        } else {
            cols += 1;
        }
    }
}

fn entry_to_output_with_frills(entry: &Entry, config: &Cli) -> OutputName {
    lazy_static! {
        static ref WS: Regex = Regex::new(r"(\s)").unwrap();
    }
    lazy_static! {
        static ref SQ: Regex = Regex::new(r"(')").unwrap();
    }
    lazy_static! {
        static ref DQ: Regex = Regex::new(r#"(")"#).unwrap();
    }

    let base_name: String = entry.name.clone();
    let needs_quote: bool = match base_name.find(char::is_whitespace) {
        Some(_) => true,
        None => false,
    };
    let quoted_name: String = if config.quote_name {
        // The CLI options require all names be quoted. Use "" for quoting.
        let mut quoted = WS.replace_all(&base_name, "\\$1").into_owned();
        quoted = DQ.replace_all(&quoted, "\\$1").into_owned();
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
        quoted: config.quote_name || needs_quote,
    }
}
