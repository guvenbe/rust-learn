// Multiple lifetime parameters for different configuration scopes
pub struct ConfigManager<'global, 'local> {
    global_config: &'global str,
    local_config: &'local str,
    overrides: Vec<(&'local str, &'local str)>,
}

impl<'global, 'local> ConfigManager<'global, 'local> {
    pub fn new(global: &'global str, local: &'local str) -> Self {
        ConfigManager {
            global_config: global,
            local_config: local,
            overrides: Vec::new(),
        }
    }

    // Only 'local references are added as overrides
    pub fn add_override(&mut self, key: &'local str, value: &'local str) {
        self.overrides.push((key, value));
    }

    pub fn get_status(&self) -> String {
        if !self.overrides.is_empty() {
            format!("Local config '{}' with {} overrides applied to global '{}'", 
                   self.local_config, 
                   self.overrides.len(), 
                   self.global_config)
        } else {
            format!("Using global config: {}", self.global_config)
        }
    }

    pub fn get_override(&self, key: &str) -> Option<&str> {
        self.overrides.iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
    }
}

// Lifetime parameter for the encoding reference
pub struct ProcessingOptions<'a> {
    pub max_lines: Option<usize>,
    pub encoding: Option<&'a str>,
    pub ignore_case: bool,
}

impl<'a> ProcessingOptions<'a> {
    pub fn new() -> Self {
        ProcessingOptions {
            max_lines: None,
            encoding: None,
            ignore_case: false,
        }
    }

    pub fn with_encoding(mut self, encoding: &'a str) -> Self {
        self.encoding = Some(encoding);
        self
    }

    pub fn with_max_lines(mut self, lines: usize) -> Self {
        self.max_lines = Some(lines);
        self
    }
}