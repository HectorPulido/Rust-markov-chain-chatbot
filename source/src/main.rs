mod lib;

use lib::markov_models::MarkovModel;
use std::collections::HashMap;

fn main() {
    let mut model = MarkovModel {
        model: HashMap::new(),
        model_path: String::from("data/p&w_model.txt"),
    };

    // model.train_with_dataset(String::from("data/p&w.txt"));
    model.load_model();

    loop {
        println!("> ");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();

        let bot_sentence = model.predict(&input);

        println!("> {}", bot_sentence);

        if input == String::from("quit") {
            break;
        }
    }
}
