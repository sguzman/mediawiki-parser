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
    pub page: Vec<Page>,
    pub siteinfo: Siteinfo,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Page {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub id: String,
    pub restrictions: Option<String>,
    pub revision: Revision,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Revision {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub comment: Option<String>,
    pub contributor: Contributor,
    pub id: String,
    pub minor: Option<Minor>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Contributor {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub id: Option<String>,
    pub ip: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Minor {}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Text {
    #[serde(rename = "space")]
    pub xml_space: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Siteinfo {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub base: String,
    pub case: String,
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
    pub key: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}
