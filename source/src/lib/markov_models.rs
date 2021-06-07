use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

pub struct MarkovModel {
    pub model: HashMap<String, Vec<String>>,
    pub model_path: String,
}

#[allow(dead_code)]
impl MarkovModel {
    pub fn read_dataset(&mut self, dataset_path: String) -> Vec<String> {
        let mut file = File::open(dataset_path).expect("File not found");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not read the file");
        let splited_data: Vec<String> = contents
            .to_lowercase()
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        return splited_data;
    }

    pub fn load_model(&mut self) {
        let mut model_str: String = String::new();
        let mut file = File::open(&self.model_path).expect("Could not find the file");
        file.read_to_string(&mut model_str)
            .expect("Could not read the file");
        let model: HashMap<String, Vec<String>> = serde_json::from_str(&model_str).unwrap();
        self.model = model;
    }

    pub fn save_model(&mut self) -> Result<(), String> {
        let follow_string = serde_json::to_string(&self.model).unwrap();
        let mut file = File::create(&self.model_path).expect("Could not create save file");
        file.write_all(follow_string.as_bytes())
            .expect("Could not save the data");
        return Ok(());
    }

    fn remove_repeated(&self, data: &Vec<String>) -> Vec<String> {
        let mut data_to_modify = data.clone();
        let set: HashSet<_> = data_to_modify.drain(..).collect();
        data_to_modify.extend(set.into_iter());
        return data_to_modify;
    }

    fn next_word(&mut self, word: &String) -> String {
        if self.model.contains_key(word) {
            let num = rand::thread_rng().gen_range(0..self.model[word].len());
            return self.model[word][num].clone();
        } else {
            return String::from("the");
        }
    }

    pub fn predict(&mut self, sentence: &String) -> String {
        let splited_sentence: Vec<String> = sentence
            .to_lowercase()
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();

        let num = rand::thread_rng().gen_range(0..splited_sentence.len());
        let mut s = splited_sentence[num].clone();
        let mut response = String::new();
        loop {
            let next_word = self.next_word(&s);
            response = format!("{} {}", response, next_word);
            s = next_word;

            match s.chars().nth(s.len() - 1).unwrap() {
                '.' | ',' | '?' | '!' => break,
                _ => (),
            };
        }
        return response;
    }

    pub fn train_with_dataset(&mut self, dataset_path: String) {
        let text = self.read_dataset(dataset_path);

        let text_without_duplicated = self.remove_repeated(&text);
        let mut follow: HashMap<String, Vec<String>> = HashMap::new();
        for check in text_without_duplicated {
            let mut working: Vec<String> = Vec::new();
            for w in 0..(text.len() - 1) {
                if check == text[w] {
                    let mut chars = text[w].chars();
                    let chars_count = text[w].chars().count();
                    match chars.nth(chars_count - 1).unwrap() {
                        '(' | ')' | '.' | ',' | '?' | '!' => (),
                        _ => working.push(text[w + 1].clone()),
                    }
                }
            }
            follow.insert(check, working);
        }
        self.model = follow;
        let _ = self.save_model();
    }
}
