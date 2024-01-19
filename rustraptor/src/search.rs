pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
      if line.contains(query) {
          results.push(line);
      }
  }

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
      if line.to_lowercase().contains(&query) {
          results.push(line);
      }
  }

  results
}
