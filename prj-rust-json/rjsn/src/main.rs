use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{
    name : String,
}

#[derive(Serialize,Deserialize)]
struct Article{
    article:String,
    author : String,
    paragraph : Vec<Paragraph>
}

fn read_json_typed(raw_json : &str) -> Article{
    let parsed : Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn main() {
    let json = r#" 
    {
        "article" : "how to work with rust",
        "author" : "sanidhayaa",
        "paragraph" : [
            {
                "name" : "first paragraph"
            },
            {
                "name" : "second paragraph"
            },
            {
                "name" : "third paragraph"
            }
        ]
    }"#;

    let parsed : Article = read_json_typed(json);
    println!("\n\n The name of the first paragraoh is {}", parsed.paragraph[0].name);

}
