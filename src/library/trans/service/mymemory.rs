use super::{error::TranslationError, Lang, Translated, TranslatorService};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MyMemoryTranslator;

impl TranslatorService for MyMemoryTranslator {
    fn translate(
        &self,
        text: &String,
        s_lang: &Lang,
        t_lang: &Lang,
    ) -> Result<Translated, TranslationError> {
        let response = self
            .request(
                text,
                &s_lang.to_owned().to_string(),
                &t_lang.to_owned().to_string(),
            )
            .map_err(|err| TranslationError::Connection)?;

        let parsed: Response =
            serde_json::from_str(&response).map_err(|err| TranslationError::Connection)?;

        let res = parsed.matches.first().ok_or(TranslationError::Connection)?;

        Ok(Translated::new(&res.translation, &s_lang))
    }
}

impl MyMemoryTranslator {
    fn request(
        &self,
        text: &String,
        s_lang: &String,
        t_lang: &String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = Url::parse_with_params(
            "https://api.mymemory.translated.net/get",
            &[
                ("langpair", &format!("{}|{}", s_lang, t_lang)),
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
    matches: Vec<Matches>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Matches {
    translation: String,
}
