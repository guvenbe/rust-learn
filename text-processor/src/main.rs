mod processor;
mod config;
mod analyzer;

use processor::{TextProcessor, DocumentSource};
// Note: ProcessingOptions is defined but not used in this example's main function.
// It's imported here to acknowledge its definition.
use config::{ConfigManager /*, ProcessingOptions */}; 
use analyzer::{TextAnalyzer, WordCounter, FrequencyAnalyzer};

fn main() {
    // Create document sources with different lifetimes
    let file_content = "Hello world from file\nThis is line two";
    let database_content = "Database record one\nDatabase record two\nExtra database line";
    let user_input = "User provided text input";
    
    let file_source = DocumentSource::new("file.txt", file_content);
    let db_source = DocumentSource::new("database", database_content);
    
    // Create processor with multiple lifetime sources
    let mut processor = TextProcessor::new(&file_source);
    processor.add_secondary_source(&db_source);
    
    // Process documents
    let result = processor.longest_line();
    println!("Longest line: {}", result);
    
    // Configuration with complex lifetimes
    let global_config = "global_settings.toml";
    let local_config = "local_override.conf";
    let mut config_manager = ConfigManager::new(global_config, local_config);
    config_manager.add_override("max_lines", "100");
    config_manager.add_override("encoding", "utf-8");
    
    println!("Config status: {}", config_manager.get_status());
    
    // Text analysis with lifetime-bound traits
    let counter = WordCounter;
    let (first_word, count) = counter.analyze(user_input);
    // Note: "User provided text input" has 4 words
    println!("First word: '{}' Count: {}", first_word, count);
    
    let frequency = FrequencyAnalyzer;
    let frequent_words = frequency.get_top_words(user_input, 3);
    // Note: HashMap iteration order is not guaranteed. Output may vary.
    println!("Top words (up to 3): {:?}", frequent_words);
    
    // Demonstrate multiple lifetime parameters
    let system_log = "System started successfully";
    let user_log = "User login attempt";
    let merged = merge_logs(system_log, user_log);
    println!("Merged logs:\n{}", merged);
    
    let context = "Processing context";
    let data = "Actual data to process";
    let (ctx, processed_data) = process_with_context(context, data);
    println!("Context: {} Data: {}", ctx, processed_data);
}

// Functions demonstrating multiple lifetime parameters (Rules 1 & 2 don't apply)
fn merge_logs<'a, 'b>(system_log: &'a str, user_log: &'b str) -> String {
    format!("System: {}\nUser: {}", system_log, user_log)
}

fn process_with_context<'ctx, 'data>(
    context: &'ctx str,
    data: &'data str
) -> (&'ctx str, &'data str) {
    (context, data)
}