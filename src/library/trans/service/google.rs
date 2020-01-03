use super::{error::TranslationError, Lang, Translated, TranslatorService};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            "AndroidTranslate/5.3.0.RC02.130475354-53000263 5.1 phone TRANSLATE_OPM5_TEST_1"
                .to_string(),
        );

        Ok(request.send()?.text()?)
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
