use super::{error::TranslationError, Lang, Translated, TranslatorService};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GoogleTranslator;

impl TranslatorService for GoogleTranslator {
    fn translate(
        &self,
        text: &String,
        s_lang: &Lang,
        t_lang: &Lang,
    ) -> Result<Translated, TranslationError> {
        let response = self
            .request(text, &t_lang.to_owned().to_string())
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
                ("client", "gtx"),
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
        let request = client.get(url.as_str()).header("User-Agent", self.get_ua());

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
