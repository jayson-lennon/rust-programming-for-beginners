// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

/// A contact record.
#[derive(Debug)]
struct Record {
    /// Unique ID number for this record.
    id: i64,
    /// The name of the contact.
    name: String,
    /// The email of the contact.
    email: Option<String>,
}

/// Contains all saved records.
#[derive(Debug)]
struct Records {
    inner: HashMap<i64, Record>,
}

impl Records {
    /// Create a new records collection.
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Edit an existing record. Will insert a new record if the id is not found.
    fn edit(&mut self, id: i64, name: &str, email: Option<String>) {
        self.inner.insert(
            id,
            Record {
                id,
                name: name.to_string(),
                email,
            },
        );
    }

    /// Returns the next available record id.
    fn next_id(&self) -> i64 {
        // First we just get all the keys (ids).
        // This vector is just a copy of the ids, so we can throw it
        // away when done.
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
        // "pop" removes the last entry from the vector, so we will have
        // the largest ID currently in use.
        // Adding 1 to the largest ID gives us the next ID, and if none
        // were found, we just start at 1.
        match ids.pop() {
            Some(id) => id + 1,
            None => 1,
        }
    }

    /// Adds a new record to the database.
    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }

    /// Converts this structure into a vector of Record.
    /// This is used when saving the data.
    fn into_vec(mut self) -> Vec<Record> {
        // "drain" moves the records out of the hashmap, so we can then
        // move them into a vector. It also returns t (key, value) pair
        // so we need to access the value with ".1" since it is a tuple.
        let mut records: Vec<_> = self.inner.drain().map(|kv| kv.1).collect();
        // We sort the records in order by id before returning them.
        records.sort_by_key(|rec| rec.id);
        records
    }

    /// Searches for all records containing the supplied name.
    fn search(&self, name: &str) -> Vec<&Record> {
        // We simple filter through the values here and see if there
        // are any matches using the ".contains" method on a string.
        // The search is case-insensitive due to the usage of ".to_lowercase".
        self.inner
            .values()
            .filter(|rec| rec.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }

    fn remove(&mut self, id: i64) -> Option<Record> {
        self.inner.remove(&id)
    }
}

/// Errors that may occur while parsing the data file.
#[derive(Error, Debug)]
enum ParseError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("empty record")]
    EmptyRecord,
    #[error("missing field: {0}")]
    MissingField(String),
}

/// Parses a single record line.
fn parse_record(record: &str) -> Result<Record, ParseError> {
    // We use ".split" on ',' to create a vector of strings.
    // This vector will contain elements of each field in the record
    // without the delimiting commas separating them.
    let fields: Vec<&str> = record.split(',').collect();

    // The id and name fields are required, so a match expression is used
    // in order to extract the data (if possible) and place it into the
    // appropriate variables.

    // Here we try to get the "id" portion of the record, which should be
    // the first entry (which is index 0). We then use "from_str_radix"
    // to parse the string id into a numeric i64 id. We abort the function
    // if this conversion fails, or if we do not find the id.
    let id = match fields.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };

    // Here we try to get the "name" portion of the record, which should
    // be the second entry (which is index 1). We also ensure that a name
    // actually exists by using filter on the name and seeing if it is an
    // empty string (""). The "fields" vector contains &str type, "get"
    // references again (&&str) and then filter references a third time (&&&str).
    // The asterisks (**) remove two of these references so we can compare
    // to the empty string ("") which is a &str. This is not something to
    // worry about because the compiler will tell you if asterisks are needed
    // when you attempt to compile the program.
    let name = match fields.get(1).filter(|name| **name != "") {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };

    // The email field is the third piece of data (index 2). "Get" returns
    // an Option, so all we need to do is simply map the email to a
    // String type with "to_string" and then filter if it is empty.
    // Map and filter will only run if we actually have data to work
    // with, since emails are optional.
    let email = fields
        .get(2)
        .map(|email| email.to_string())
        .filter(|email| email != "");

    Ok(Record { id, name, email })
}

/// Parses the entire record file.
fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    // We use ".split" with '\n' to get each line one at a time.
    // '\n' means "new line". "Enumerate" provides the data and
    // the current enumeration index (starting from 0) and we use
    // this number to report line errors.
    for (num, record) in records.split('\n').enumerate() {
        if record != "" {
            match parse_record(record) {
                Ok(rec) => recs.add(rec),
                Err(e) => {
                    if verbose {
                        println!(
                            "error on line number {}: {}\n  > \"{}\"\n",
                            num + 1,
                            e,
                            record
                        );
                    }
                }
            }
        }
    }
    recs
}

/// Loads the raw records from a file.
fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_records(buffer, verbose))
}

/// Saves the records to disk.
fn save_records(file_name: PathBuf, records: Records) -> std::io::Result<()> {
    // We use OpenOptions to configure how the file should be opened.
    // This is needed so we can get write access to the file. Additionally,
    // we "truncate" the file, which deletes all the contents. This is done
    // because we rewrite the entire contents whenever we save. It is possible
    // to write to a specific section of the file, but rewriting the entire
    // file is the simplest method.
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;

    // First we write the field names.
    file.write(b"id,name,email\n")?;

    // Then we iterate through each record and write it to the file.
    // "Into_iter" creates an iterator that takes ownership of the data
    // during iteration. We do this so we don't have to make additional
    // copies of the data before saving it to disk (we can just work with
    // it directly).
    for record in records.into_vec().into_iter() {
        // When we do not have an email, we just use an empty string ("").
        let email = match record.email {
            Some(email) => email,
            None => "".to_string(),
        };
        // This creates a new string that is properly formatted to CSV.
        let line = format!("{},{},{}\n", record.id, record.name, email);
        // We then write the string to the file. "write" works with bytes,
        // so we just access the bytes of the string with "as_bytes".
        file.write(line.as_bytes())?;
    }
    // "Flushing" the data ensures that everything is written to disk before
    // continuing. Without this line, it is possible for the program to
    // terminate before the system is done writing to the file, and this
    // can result in corrupted data.
    file.flush()?;
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(about = "project 2: contact manager")]
struct Opt {
    #[structopt(short, parse(from_os_str), default_value = "p2_data.csv")]
    data_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    Edit {
        id: i64,
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    List {},
    Remove {
        id: i64,
    },
    Search {
        query: String,
    },
}

/// Runs the program. This is so we can utilize the question mark operator.
fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        Command::Add { name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let next_id = recs.next_id();
            recs.add(Record {
                id: next_id,
                name,
                email,
            });
            save_records(opt.data_file, recs)?;
        }
        Command::Edit { id, name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            recs.edit(id, &name, email);
            save_records(opt.data_file, recs)?;
        }
        Command::List { .. } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            for record in recs.into_vec() {
                println!("{:?}", record);
            }
        }
        Command::Remove { id } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            if recs.remove(id).is_some() {
                save_records(opt.data_file, recs)?;
                println!("record deleted");
            } else {
                println!("record not found");
            }
        }
        Command::Search { query } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            let results = recs.search(&query);
            if results.is_empty() {
                println!("no records found");
            } else {
                for rec in results {
                    println!("{:?}", rec);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        println!("an error occurred: {}", e);
    }
}
