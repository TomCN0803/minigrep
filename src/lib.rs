use std::{
    env::{self, Args},
    error::Error,
    fs,
};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get a text"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let lines = search(&config.query, &content);

    for line in lines.iter() {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use self::super::search;

    #[test]
    fn test_search() {
        let res = search("hi", "asdasd\nhiasda\nasdhi");

        for r in res.iter() {
            println!("{}", r);
        }
    }
}
