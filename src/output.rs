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
    // How many total entries will we be displaying:
    let count: usize = entries.len();
    // Convert the Entry structs to OutputName structs:
    let names: Vec<OutputName> = entries
        .iter()
        .map(|e| entry_to_output_with_frills(e, config))
        .collect();
    // After all the names have been "decorated", what is the widest name?
    let widest: usize = names.iter().map(|e| e.size).max().unwrap();
    // Note whether there is at least one quoted name:
    let any_quoted: bool = names.iter().any(|e| e.quoted);
    // Also note whether ALL are quoted due to -Q:
    let all_quoted: bool = config.quote_name;
    // `leader` is how much leading space a name should have if it is NOT
    // quoted:
    let leader: u8 = if any_quoted && !all_quoted { 1 } else { 0 };
    // `spacer` is the amount of space between names. Note that if the next
    // name is not quoted, `leader` will be added to the spacing.
    let mut spacer: u8 = 2;

    // Calculate the max number of columns we can use, based on the width of
    // the widest name.
    let mut cols: u8 = 1;
    if !(config.one || config.zero) {
        loop {
            let w = widest as u8 * cols + spacer * (cols - 1) + leader;
            if w as u32 > width {
                break;
            } else {
                cols += 1;
            }
        }
    }
    // Adjust `spacer` to reflect the spacing needed based on `cols`:

    // Determine the full-length column size and the remainder (if any) size.
}

/*
   Create an OutputName struct from the Entry ref passed in. Determines if the
   name to be displayed needs to be quoted, and/or if a type-indicator should
   be appended to the (possibly-quoted) name.
*/
fn entry_to_output_with_frills(entry: &Entry, config: &Cli) -> OutputName {
    // These macro-blocks set up the regular expressions that are used, in a
    // way that prevents them from being re-computed on every call to this fn.
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

    // Start with the name itself (a clone of it)
    let base_name: String = entry.name.clone();
    // Determine if there is any whitespace in the name:
    let has_whitespace: bool = match base_name.find(char::is_whitespace) {
        Some(_) => true,
        None => false,
    };
    // Look for the special characters that are used for file-type indication:
    let has_special: bool = SP.is_match(&base_name);
    // If either of those were true, it needs to be quoted.
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
        // No quoting is necessary. Just return `base_name` itself.
        base_name
    };

    // Create the struct that is used elsewhere for determining display params.
    // Get the length of `quoted_name` first, since that doesn't borrow the
    // value. Then give it to the struct as `name` after that.
    OutputName {
        size: quoted_name.len(),
        name: quoted_name,
        quoted: needs_quote,
    }
}
