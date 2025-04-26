#![allow(dead_code)]

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Person {
    name: String,
    country: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PersonResponse {
    status: String,
    code: u16,
    total: u64,
    data: Vec<Person>,
}

trait DataFetcher: std::fmt::Debug {
    fn get_person(&self) -> Result<PersonResponse>;
}

impl DataFetcher for RequestFetcher {
    fn get_person(&self) -> Result<PersonResponse> {

        let response: PersonResponse = self
            .client
            .get("https://fakerapi.it/api/v1/custom?_quantity=1&name=name&country=country")
            .send()?
            .json()?;

        Ok(response)
    }
}

#[derive(Default, Debug)]
struct RequestFetcher {
    client: reqwest::blocking::Client,
}


#[derive(Debug)]
struct App {
    client: Box<dyn DataFetcher>,
}
impl Default for App {
    fn default() -> Self {
        Self{client: Box::new(RequestFetcher::default())}
    }
}
impl App {
    pub fn fetch_person(&self) -> Result<PersonResponse> {
        self.client.get_person()
    }
}

fn main() -> Result<()> {
    let app = App::default();
    let person = app.fetch_person();
    dbg!(&person);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug)]
    struct FakeFetcher;
    impl DataFetcher for FakeFetcher{
        fn get_person(&self) -> Result<PersonResponse> {
           Ok(
              PersonResponse{
                  status: "OK".to_string(),
                  code: 200,
                  total: 1,
                  data: vec![Person{
                      name: "test name".to_string(),
                      country: "test_country".to_string()
                      
                  }],
              } 
           ) 
        }
    }
    
    #[test]
    fn fetches_data(){
       let app = App {
           client: Box::new(FakeFetcher)
       };
        let person = app.fetch_person();
        
        assert!(person.is_ok());
    }
}