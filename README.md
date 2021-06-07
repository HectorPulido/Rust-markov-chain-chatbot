# Markov chain chatbot made with rust

![Test](/img/img.png)<br/>

This is a handmade markov chain based chatbot <br/>

## Example
How to init the markov model
``` Rust
mod lib;

use lib::markov_models::MarkovModel;
use std::collections::HashMap;

fn main() {
    let mut model = MarkovModel {
        model: HashMap::new(),
        model_path: String::from("data/p&w_model.txt"),
    };

    ...
}
```

You can train the model using the function train_with_dataset  
``` Rust
...
model.train_with_dataset(String::from("data/p&w.txt"));
...

```

If you already have a pretrained model, you can load it with the function load_model

``` Rust
...
model.load_model();
...

```

Also you can get prediction with the function predict

``` Rust
...
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
```






<div align="center">
<h3 align="center">Let's connect 😋</h3>
</div>
<p align="center">
<a href="https://www.linkedin.com/in/hector-pulido-17547369/" target="blank">
<img align="center" width="30px" alt="Hector's LinkedIn" src="https://www.vectorlogo.zone/logos/linkedin/linkedin-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://twitter.com/Hector_Pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitter" src="https://www.vectorlogo.zone/logos/twitter/twitter-official.svg"/></a> &nbsp; &nbsp;
<a href="https://www.twitch.tv/hector_pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitch" src="https://www.vectorlogo.zone/logos/twitch/twitch-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw" target="blank">
<img align="center" width="30px" alt="Hector's Youtube" src="https://www.vectorlogo.zone/logos/youtube/youtube-icon.svg"/></a> &nbsp; &nbsp;

</p>