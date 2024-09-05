use maud::{html, Markup, DOCTYPE};

pub struct Header<'a> {
    pub title: &'a str,
}

pub fn generate_header(options: Header) -> Markup {
    let markup = html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        // link href="/style.css" rel="stylesheet" type="text/css";
        title { (options.title) };
        link rel="icon" type="image/x-icon" href="https://forum.mafiascum.net/ext/mafiascum/isos/images/favicon.ico";
    };

    markup
}
