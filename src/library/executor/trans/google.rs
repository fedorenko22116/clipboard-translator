use super::Translator;
use super::error::TranslationError;
use super::{Translated, Lang};
use std::collections::HashMap;
use reqwest::Url;
use serde::{Deserialize, Serialize};


pub struct GoogleTranslator;

impl Translator for GoogleTranslator {
    fn translate(&self, text: &String) -> Result<Translated, TranslationError> {
        let response = self.request(text)
            .map_err(|err| TranslationError::Connection)?;

        let parsed: Response = serde_json::from_str(&response)
            .map_err(|err| TranslationError::Connection)?;

        let res = parsed.sentences.first().ok_or(TranslationError::Connection)?;

        Ok(Translated::new(&res.trans, &Lang::Custom(parsed.src)))
    }

    fn get_name(&self) -> String {
        "Google".to_string()
    }
}

impl GoogleTranslator {
    fn request(&self, text: &String) -> Result<String, Box<dyn std::error::Error>> {
        let url = Url::parse_with_params(
            "https://translate.googleapis.com/translate_a/single",
            &[
                ("client", "at"),
                ("tl", "ru"),
                ("sl", "auto"),
                ("dt", "t"),
                ("dj", "1"),
                ("q", &text),
            ],
        ).unwrap().to_string();

        let client = reqwest::blocking::Client::new();
        let request = client.get(url.as_str())
            .header(
                "User-Agent",
                "AndroidTranslate/5.3.0.RC02.130475354-53000263 5.1 phone TRANSLATE_OPM5_TEST_1"
                    .to_string()
            );

        Ok(request.send()?.text()?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    sentences: Vec<Trans>,
    src: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Trans {
    trans: String,
    orig: String
}
