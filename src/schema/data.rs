use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Mediawiki {
    pub version: String,
    #[serde(rename = "lang")]
    pub xml_lang: String,
    pub xmlns: String,
    #[serde(rename = "xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub siteinfo: Siteinfo,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Siteinfo {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub base: String,
    pub case: String,
    pub dbname: String,
    pub generator: String,
    pub namespaces: Namespaces,
    pub sitename: String,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Namespaces {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub namespace: Vec<Namespace>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Namespace {
    pub case: String,
    pub key: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}
