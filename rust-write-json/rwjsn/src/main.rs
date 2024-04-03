use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name : String
}

#[derive(Serialize,Deserialize)]
struct Article{
    article : String,
    author : String,
    paragraph : Vec<Paragraph>
}

fn main() {
    let article = Article{
        article : "string1".to_owned(),
        author : "author1".to_owned(),
        paragraph : vec![
            Paragraph{
                name : "name1".to_owned()
            },
            Paragraph{
                name : "name2".to_owned()
            },
            Paragraph{
                name : "name3".to_owned()
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("{}", json);
}