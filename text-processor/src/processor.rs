pub struct DocumentSource<'a> {
    pub name: &'a str,
    pub content: &'a str,
}

impl<'a> DocumentSource<'a> {
    pub fn new(name: &'a str, content: &'a str) -> Self {
        DocumentSource { name, content }
    }
}

// Multiple lifetime parameters required for distinct reference sources
pub struct TextProcessor<'primary, 'secondary> {
    primary_source: &'primary DocumentSource<'primary>,
    secondary_sources: Vec<&'secondary DocumentSource<'secondary>>,
}

impl<'primary, 'secondary> TextProcessor<'primary, 'secondary> {
    pub fn new(primary: &'primary DocumentSource<'primary>) -> Self {
        TextProcessor {
            primary_source: primary,
            secondary_sources: Vec::new(),
        }
    }

    pub fn add_secondary_source(&mut self, secondary: &'secondary DocumentSource<'secondary>) {
        self.secondary_sources.push(secondary);
    }

    // Lifetime elision Rule 3: &self lifetime assigned to output
    pub fn longest_line(&self) -> &str {
        let mut longest = self.primary_source.content
            .lines()
            .max_by_key(|line| line.len())
            .unwrap_or("");

        for source in &self.secondary_sources {
            let source_longest = source.content
                .lines()
                .max_by_key(|line| line.len())
                .unwrap_or("");
            if source_longest.len() > longest.len() {
                longest = source_longest;
            }
        }
        
        longest
    }

    // Returns references tied to the lifetime of the sources within &self.
    // This works because the string literals used in main ("file.txt", "database")
    // have the 'static lifetime, which outlives any temporary scope.
    pub fn get_all_sources(&self) -> Vec<&str> {
        let mut sources = vec![self.primary_source.name];
        for source in &self.secondary_sources {
            sources.push(source.name);
        }
        sources
    }
}