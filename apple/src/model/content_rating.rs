#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContentRating<N> {
    pub name: N,
    pub rank: u32,
    pub value: u32,
    pub system: Option<ContentRatingSystem>,
    pub advisories: Option<Vec<ContentRatingAdvisory>>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContentRatingsBySystem {
    #[serde(rename = "appsApple")]
    pub apps_apple: ContentRating<AppleName>,
    #[serde(rename = "appsFrance")]
    pub apps_france: Option<ContentRating<FranceName>>,
    #[serde(rename = "appsAustralia")]
    pub apps_australia: Option<ContentRating<AustraliaName>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContentRatingSystem {
    Games,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum AppleName {
    #[serde(rename = "4+")]
    FourPlus,
    #[serde(rename = "9+")]
    NinePlus,
    #[serde(rename = "12+")]
    TwelvePlus,
    #[serde(rename = "17+")]
    SeventeenPlus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum FranceName {
    #[serde(rename = "France 18")]
    France18,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum AustraliaName {
    #[serde(rename = "Australia 18")]
    Australia18,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "strict", derive(Copy))]
#[serde(deny_unknown_fields)]
pub enum ContentRatingAdvisory {
    #[serde(
        rename = "Frequent/Intense Alcohol, Tobacco, or Drug Use or References",
        // Australia doesn't use the Oxford comma.
        alias = "Frequent/Intense Alcohol, Tobacco or Drug Use or References",
        alias = "Häufig/stark ausgeprägt: Gebrauch von Alkohol, Tabak oder Drogen bzw. Verweise hierzu",
        alias = "Consumo de alcohol, tabaco o drogas, o referencias al mismo frecuentes/intensos"
    )]
    FrequentIntenseAlcoholTobaccoOrDrugUseOrReferences,
    #[serde(
        rename = "Frequent/Intense Cartoon or Fantasy Violence",
        alias = "Häufig/stark ausgeprägt: Zeichentrick- oder Fantasy-Gewalt",
        alias = "Scènes fréquentes/intenses de violence (animation ou fantastique)",
        alias = "Violencia en dibujos animados o en imágenes fantásticas frecuente/intensa"
    )]
    FrequentIntenseCartoonOrFantasyViolence,
    #[serde(
        rename = "Frequent/Intense Contests",
        alias = "Häufig/stark ausgeprägt: Wettkämpfe"
    )]
    FrequentIntenseContests,
    #[serde(
        rename = "Frequent/Intense Horror/Fear Themes",
        alias = "Häufig/stark ausgeprägt: Horror-/Gruselszenen"
    )]
    FrequentIntenseHorrorFearThemes,
    #[serde(
        rename = "Frequent/Intense Mature/Suggestive Themes",
        alias = "Häufig/stark ausgeprägt: Szenen mit erotischen Anspielungen",
        alias = "Scènes fréquentes/intenses réservées aux adultes (suggestives)",
        alias = "Temas para adultos/provocativos frecuentes/intensos",
        alias = "Большое/значительное количество тем откровенного содержания, предназначенные только для взрослых"
    )]
    FrequentIntenseMatureSuggestiveThemes,
    #[serde(
        rename = "Frequent/Intense Medical/Treatment Information",
        alias = "Häufig/stark ausgeprägt: medizinische/Behandlungs-Informationen",
        alias = "Scènes fréquentes/intenses de contenu à caractère médical",
        alias = "Información médica/sobre tratamientos frecuente/intensa"
    )]
    FrequentIntenseMedicalTreatmentInformation,
    #[serde(
        rename = "Frequent/Intense Profanity or Crude Humor",
        alias = "Frequent/Intense Profanity or Crude Humour",
        alias = "Häufig/stark ausgeprägt: obszöner oder vulgärer Humor"
    )]
    FrequentIntenseProfanityOrCrudeHumor,
    #[serde(
        rename = "Frequent/Intense Realistic Violence",
        alias = "Häufig/stark ausgeprägt: realistisch dargestellte Gewalt",
        alias = "Scènes fréquentes/intenses de violence réaliste",
        alias = "Violencia realista frecuente/intensa"
    )]
    FrequentIntenseRealisticViolence,
    #[serde(
        rename = "Frequent/Intense Sexual Content or Nudity",
        alias = "Häufig/stark ausgeprägt: sexuelle Inhalte oder Nacktheit",
        alias = "Scènes fréquentes/intenses à caractère sexuel ou de nudité",
        alias = "Contenido sexual o desnudez frecuentes/intensos",
        alias = "Большое/значительное количество контента сексуального или эротического характера"
    )]
    FrequentIntenseSexualContentOrNudity,
    #[serde(
        rename = "Frequent/Intense Simulated Gambling",
        alias = "Häufig/stark ausgeprägt: simuliertes Glücksspiel",
        alias = "Simulations fréquentes/intenses de jeu de hasard",
        alias = "Simulación de apuestas frecuente/intensa"
    )]
    FrequentIntenseSimulatedGambling,
    #[serde(
        rename = "Infrequent/Mild Alcohol, Tobacco, or Drug Use or References",
        // Australia doesn't use the Oxford comma.
        alias = "Infrequent/Mild Alcohol, Tobacco or Drug Use or References",
        alias = "Selten/schwach ausgeprägt: Gebrauch von Alkohol, Tabak oder Drogen bzw. Verweise hierzu",
        alias = "Scènes rares/modérées de consommation ou de référence à l’alcool, au tabac ou à la drogue",
        alias = "Consumo de alcohol, tabaco o drogas, o referencias al mismo poco frecuentes/moderados"
    )]
    InfrequentMildAlcoholTobaccoOrDrugUseOrReferences,
    #[serde(
        rename = "Infrequent/Mild Cartoon or Fantasy Violence",
        alias = "Selten/schwach ausgeprägt: Zeichentrick- oder Fantasy-Gewalt",
        alias = "Scènes rares/modérées de violence (animation ou fantastique)",
        alias = "Violencia en dibujos animados o en imágenes fantásticas poco frecuente/moderada",
        alias = "Малое/умеренное количество мультипликационного или фэнтезийного насилия"
    )]
    InfrequentMildCartoonOrFantasyViolence,
    #[serde(
        rename = "Infrequent/Mild Contests",
        alias = "Selten/schwach ausgeprägt: Wettkämpfe",
        alias = "Concours rares/modérés",
        alias = "Concursos poco frecuentes/moderados"
    )]
    InfrequentMildContests,
    #[serde(
        rename = "Infrequent/Mild Horror/Fear Themes",
        alias = "Selten/schwach ausgeprägt: Horror-/Gruselszenen",
        alias = "Scènes rares/modérées d’horreur ou d’épouvante",
        alias = "Temas de horror/miedo poco frecuentes/moderados",
        alias = "Малое/умеренное количество тем, вызывающих ужас или страх"
    )]
    InfrequentMildHorrorFearThemes,
    #[serde(
        rename = "Infrequent/Mild Mature/Suggestive Themes",
        alias = "Selten/schwach ausgeprägt: Szenen mit erotischen Anspielungen",
        alias = "Scènes rares/modérées réservées aux adultes (suggestives)",
        alias = "Temas para adultos/provocativos poco frecuentes/moderados",
        alias = "Малое/умеренное количество тем, предназначенных только для взрослых"
    )]
    InfrequentMildMatureSuggestiveThemes,
    #[serde(
        rename = "Infrequent/Mild Medical/Treatment Information",
        alias = "Selten/schwach ausgeprägt: medizinische/Behandlungs-Informationen",
        alias = "Scènes rares/modérées de contenu à caractère médical",
        alias = "Información médica/sobre tratamientos poco frecuente/moderada",
        alias = "Малое/умеренное количество медицинской или лечебной тематики"
    )]
    InfrequentMildMedicalTreatmentInformation,
    #[serde(
        rename = "Infrequent/Mild Profanity or Crude Humor",
        alias = "Infrequent/Mild Profanity or Crude Humour",
        alias = "Selten/schwach ausgeprägt: obszöner oder vulgärer Humor",
        alias = "Scènes rares/modérées d’humour vulgaire ou blasphématoire",
        alias = "Blasfemias o humor vulgar poco frecuentes/moderados",
        alias = "Малое/умеренное количество сквернословия или грубого юмора"
    )]
    InfrequentMildProfanityOrCrudeHumor,
    #[serde(
        rename = "Infrequent/Mild Realistic Violence",
        alias = "Selten/schwach ausgeprägt: realistisch dargestellte Gewalt",
        alias = "Scènes rares/modérées de violence réaliste",
        alias = "Violencia realista poco frecuente/moderada",
        alias = "Violencia realista infrecuente/moderada",
        alias = "Малое/умеренное количество реалистичного насилия"
    )]
    InfrequentMildRealisticViolence,
    #[serde(
        rename = "Infrequent/Mild Sexual Content and Nudity",
        alias = "Selten/schwach ausgeprägt: sexuelle Inhalte oder Nacktheit",
        alias = "Scènes rares/modérées à caractère sexuel ou de nudité",
        alias = "Contenido sexual o desnudez poco frecuentes/moderados",
        alias = "Малое/умеренное количество контента сексуального или эротического характера"
    )]
    InfrequentMildSexualContentAndNudity,
    #[serde(
        rename = "Infrequent/Mild Simulated Gambling",
        alias = "Selten/schwach ausgeprägt: simuliertes Glücksspiel",
        alias = "Simulations rares/modérées de jeu de hasard",
        alias = "Simulación de apuestas poco frecuente/moderada"
    )]
    InfrequentMildSimulatedGambling,
    #[serde(alias = "Glücksspiel")]
    Gambling,
    #[serde(
        rename = "Loot Boxes",
        alias = "Lootboxen",
        alias = "Coffres à butin",
        alias = "Cajas de recompensas"
    )]
    LootBoxes,
    #[serde(
        rename = "Unrestricted Web Access",
        alias = "Unbeschränkter Zugang zum Web",
        alias = "Accès au Web non contrôlé.",
        alias = "Acceso web no restringido",
        alias = "Неограниченный доступ к Сети"
    )]
    UnrestrictedWebAccess,
    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Other(String),
}
