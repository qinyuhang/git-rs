use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub timestamp: u64,
    pub timezone: String,
}
pub type Committer = Author;

impl FromStr for Author {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(" ").collect::<Vec<&str>>();
        dbg!(&v);
        let timestamp = u64::from_str(v[3])?;
        Ok(Self {
            name: v[1].to_string(),
            email: v[2].replace("<", "").replace(">", "").to_string(),
            timestamp,
            timezone: v[4].to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::author::Author;

    /// author qinyuhang <qinyuhangxiaoxiang@gmail.com> 1768740725 +0800
    #[test]
    fn can_parse() {
        let case = "author qinyuhang <qinyuhangxiaoxiang@gmail.com> 1768740725 +0800";
        let author = Author::from_str(&case);
        assert!(author.is_ok());
        let author = author.unwrap();
        assert_eq!(author.name, "qinyuhang".to_string());
        assert_eq!(author.email, "qinyuhangxiaoxiang@gmail.com".to_string());
        assert_eq!(author.timestamp, 1768740725);
        assert_eq!(author.timezone, "+0800".to_string());
    }
}
