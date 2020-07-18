use {
    markov::Chain, wasm_bindgen::prelude::*,
};

#[wasm_bindgen]
pub fn markov(input: String, count: usize) -> String {
    let mut chain = Chain::new();

    for line in input.lines() {
        chain.feed_str(line);
    }

    let mut out = String::new();
    out.push_str("<h3>Generated sentences:</h3>");

    for line in chain.iter_for(count) {
        let mut line_out = String::new();
        for word in line {
            line_out.push_str(&format!("{} ", word));
        }
        out.push_str(&format!("<li>{}</li>", line_out))
    }

    out
}
