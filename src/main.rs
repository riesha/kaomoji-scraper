extern crate reqwest;
extern crate select;

use select::document::{self, Document};
use select::predicate::{Class, Name, Predicate};

fn main() {
    scrape_links("https://kaomoji-copy.com");
}

fn scrape_links(url: &str) {

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let doc = Document::from_read(resp).unwrap();
    let mut links: Vec<String> = Vec::new();

    for menu in doc.find(Class("menu")) {

          menu.find(Name("a"))
         .filter_map(|n| n.attr("href"))
         .for_each(|x|links.push(x.to_string()));
        
    }

    for link in links {
        let full_url = url.to_string() + &link.clone();
        scrape_kao(full_url);
    }
}

fn scrape_kao(url: String) {
    let link = url.clone();
    let  resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("input"))
        .filter_map(|n| n.attr("value"))
        .for_each(|x| println!("{} -> {}",link,x));
}