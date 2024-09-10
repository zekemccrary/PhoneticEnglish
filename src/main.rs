mod parse;

use crate::parse::tokenize;

const DICT: &'static str = "./cmu_dict.json";


fn main() {
    let stir = "Hi. T!h!e ostrick-catcher wasn't dead!!";

    let token_vec = tokenize(stir);

    println!("{:?}", token_vec);
}


