// mod parser;
use parser::Parser;

fn main() {
    let doc = r#"
        <top label="Top">
            <semi-bottom label="Bottom"/>
            <middle>
                <bottom label="Another bottom"/>
            </middle>
        </top>"#;

    println!("{:?}", parser::element().parse(doc));
}
