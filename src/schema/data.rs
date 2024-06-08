use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "elementFormDefault")]
    pub element_form_default: String,
    #[serde(rename = "targetNamespace")]
    pub target_namespace: String,
    pub xmlns: String,
    #[serde(rename = "xmlns:mw")]
    pub xmlns_mw: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub annotation: Annotation,
    #[serde(rename = "complexType")]
    pub complex_type: Vec<ComplexType>,
    pub element: SchemaElement,
    pub import: Import,
    #[serde(rename = "simpleType")]
    pub simple_type: SimpleType,
}

#[derive(Serialize, Deserialize)]
pub struct Annotation {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub documentation: Documentation,
}

#[derive(Serialize, Deserialize)]
pub struct Documentation {
    #[serde(rename = "lang")]
    pub xml_lang: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ComplexType {
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub attribute: Vec<ComplexTypeAttribute>,
    pub sequence: Option<Sequence>,
    #[serde(rename = "simpleContent")]
    pub simple_content: Option<SimpleContent>,
}

#[derive(Serialize, Deserialize)]
pub struct ComplexTypeAttribute {
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub attribute_ref: Option<String>,
    #[serde(rename = "type")]
    pub attribute_type: Option<String>,
    #[serde(rename = "use")]
    pub attribute_use: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sequence {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub choice: Option<Choice>,
    pub element: Vec<SequenceElement>,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    #[serde(rename = "maxOccurs")]
    pub max_occurs: String,
    #[serde(rename = "minOccurs")]
    pub min_occurs: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub element: Vec<ChoiceElement>,
}

#[derive(Serialize, Deserialize)]
pub struct ChoiceElement {
    pub name: String,
    #[serde(rename = "type")]
    pub element_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SequenceElement {
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<String>,
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub element_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleContent {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub extension: Extension,
}

#[derive(Serialize, Deserialize)]
pub struct Extension {
    pub base: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub attribute: ExtensionAttribute,
}

#[derive(Serialize, Deserialize)]
pub struct ExtensionAttribute {
    pub default: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "ref")]
    pub attribute_ref: Option<String>,
    #[serde(rename = "type")]
    pub attribute_type: Option<String>,
    #[serde(rename = "use")]
    pub attribute_use: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SchemaElement {
    pub name: String,
    #[serde(rename = "type")]
    pub element_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Import {
    pub namespace: String,
    #[serde(rename = "schemaLocation")]
    pub schema_location: String,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleType {
    pub name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub restriction: Restriction,
}

#[derive(Serialize, Deserialize)]
pub struct Restriction {
    pub base: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub enumeration: Vec<Enumeration>,
}

#[derive(Serialize, Deserialize)]
pub struct Enumeration {
    pub value: String,
}


