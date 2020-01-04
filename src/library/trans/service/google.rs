use super::{error::TranslationError, Lang, Translated, TranslatorService};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use fake_useragent::{UserAgentsBuilder, Browsers};

pub struct GoogleTranslator;

impl TranslatorService for GoogleTranslator {
    fn translate(&self, text: &String, lang: &Lang) -> Result<Translated, TranslationError> {
        let response = self
            .request(text, &lang.to_owned().to_string())
            .map_err(|err| TranslationError::Connection)?;

        let parsed: Response =
            serde_json::from_str(&response).map_err(|err| TranslationError::Connection)?;

        let res = parsed
            .sentences
            .first()
            .ok_or(TranslationError::Connection)?;

        Ok(Translated::new(&res.trans, &Lang::Custom(parsed.src)))
    }
}

impl GoogleTranslator {
    fn request(&self, text: &String, lang: &String) -> Result<String, Box<dyn std::error::Error>> {
        let url = Url::parse_with_params(
            "https://translate.googleapis.com/translate_a/single",
            &[
                ("client", "at"),
                ("tl", lang),
                ("sl", "auto"),
                ("dt", "t"),
                ("dj", "1"),
                ("q", &text),
            ],
        )
        .unwrap()
        .to_string();

        let client = reqwest::blocking::Client::new();
        let request = client.get(url.as_str()).header(
            "User-Agent",
            self.get_ua(),
        );

        Ok(request.send()?.text()?)
    }

    fn get_ua(&self) -> String {
        UserAgentsBuilder::new()
            .cache(false)
            .dir("/tmp")
            .thread(20)
            .set_browsers(
                Browsers::new()
                    .set_chrome()
                    .set_firefox()
                    .set_safari()
            )
            .build()
            .random()
            .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    sentences: Vec<Trans>,
    src: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Trans {
    trans: String,
    orig: String,
}
