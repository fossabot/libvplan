#[derive(Debug, Deserialize)]
pub struct Value {
    #[serde(rename = "$value")]
    pub value: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Vplan {
    #[serde(rename = "kopf")]
    pub header: Header,
    #[serde(rename = "freietage")]
    pub days_off: DaysOff,
    #[serde(rename = "haupt")]
    pub main: Main,
    #[serde(rename = "fuss")]
    pub footer: Footer
}

#[derive(Debug, Deserialize)]
pub struct Header {
    #[serde(rename = "datei")]
    pub file: Value,
    #[serde(rename = "titel")]
    pub title: Value,
    #[serde(rename = "schulname")]
    pub schoolname: Value,
    #[serde(rename = "datum")]
    pub date: Value,
    #[serde(rename = "kopfinfo")]
    pub info: HeaderInfo
}

#[derive(Debug, Deserialize)]
pub struct HeaderInfo {
    #[serde(rename = "abwesendk")]
    pub naclasses: Value,
    #[serde(rename = "abwesendr")]
    pub narooms: Value,
    #[serde(rename = "aenderungk")]
    pub changed: Value
}

#[derive(Debug, Deserialize)]
pub struct DaysOff {
    #[serde(rename = "ft")]
    pub items: Vec<DayOff>
}

#[derive(Debug, Deserialize)]
pub struct DayOff {
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct Main {
    #[serde(rename = "aktion")]
    pub items: Vec<Action>
}

#[derive(Debug, Deserialize)]
pub struct Action {
    #[serde(rename = "klasse")]
    pub class: ActionInner,
    #[serde(rename = "stunde")]
    pub lesson: ActionInner,
    #[serde(rename = "fach")]
    pub subject: ActionInner,
    #[serde(rename = "lehrer")]
    pub teacher: ActionInner,
    #[serde(rename = "raum")]
    pub room: ActionInner,
    pub info: ActionInner
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ActionInner {
    fageaendert: Option<String>,
    legeaendert: Option<String>,
    rageaendert: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Footer {
    #[serde(rename = "fusszeile")]
    pub items: Vec<FooterLine>
}

#[derive(Debug, Deserialize)]
pub struct FooterLine {
    #[serde(rename = "fussinfo")]
    pub inner: Value
}
