// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Equatorial Guinea

#[cfg(all(feature = "gq", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::GQ;
    pub const ALPHA3: Alpha3 = Alpha3::GNQ;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 240;
    pub const CURRENCY_CODE: &str = "XAF";
    pub const GEC: Option<GEC> = Some(GEC::EK);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("GEQ");
    pub const ISO_SHORT_NAME: &str = "Equatorial Guinea";
    pub const ISO_LONG_NAME: &str = "The Republic of Equatorial Guinea";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["es", "fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["es", "fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[6];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Equatorial Guinean");
    pub const NUMBER: &str = "226";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::MiddleAfrica);
    pub const UN_LOCODE: &str = "GQ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Equatorial Guinea",
        "Äquatorial-Guinea",
        "Guinée Équatoriale",
        "Guinea Ecuatorial",
        "赤道ギニア",
        "Equatoriaal-Guinea",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Equatorial Guinea"),
        ("af", "Ekwatoriaal-Guinee"),
        ("ak", "Equatorial Guinea"),
        ("am", "Equatorial Guinea"),
        ("an", "Equatorial Guinea"),
        ("ar", "غينيا الاستوائي\u{651}ة"),
        ("as", "বিষ\u{9c1}বীয় গিনি"),
        ("ay", "Equatorial Guinea"),
        ("az", "Ekvatorial Qvineya"),
        ("ba", "Equatorial Guinea"),
        ("be", "Экватарыяльная Гвінея"),
        ("bg", "Екваториална Гвинея"),
        ("bi", "Equatorial Guinea"),
        ("bn", "ইক\u{9c1}য়েটোরিয়\u{9be}ল গিনি"),
        ("bn_IN", "ইক\u{9c1}য়েটোরিয়\u{9be}ল গিনি"),
        ("br", "Ginea ar C'heheder"),
        ("bs", "Ekvatorijalna Gvineja"),
        ("ca", "Guinea Equatorial"),
        ("ce", "Экваторан Гвиней"),
        ("ch", "Equatorial Guinea"),
        ("cs", "Rovníková Guinea"),
        ("cv", "Экваторан Гвиней"),
        ("cy", "Guinea Cyhydeddol"),
        ("da", "Ækvatorialguinea"),
        ("de", "Äquatorialguinea"),
        (
            "dv",
            "އ\u{7a8}ކ\u{7aa}އ\u{7ac}ޓ\u{7af}ރ\u{7a8}އ\u{7a6}ލ\u{7b0} ގ\u{7a8}ނ\u{7a9}",
        ),
        (
            "dz",
            "ཨ\u{f72}་ཀ\u{f74}ཡ\u{f7a}་ཊ\u{f7c}་ར\u{f72}་ཡ\u{f71}ལ་ ག\u{f72}་ན\u{f72}།",
        ),
        ("ee", "Equatorial Guinea"),
        ("el", "Ισημερινή Γουινέα"),
        ("en", "Equatorial Guinea"),
        ("eo", "Ekvatora Gvineo"),
        ("es", "Guinea Ecuatorial"),
        ("et", "Ekvatoriaal-Guinea"),
        ("eu", "Ekuatore Ginea"),
        ("fa", "گینه\u{654} استوایی"),
        ("ff", "Gine Ekwatoriyal"),
        ("fi", "Päiväntasaajan Guinea"),
        ("fo", "Ekvator Guinea"),
        ("fr", "Guinée Équatoriale"),
        ("fy", "Ekwatoriaal-Guinee"),
        ("ga", "An Ghuine Mheánchiorclach"),
        ("gl", "Guinea Ecuatorial"),
        ("gn", "Equatorial Guinea"),
        ("gu", "ઇક\u{acd}વોરીઅલ ગ\u{ac1}એના"),
        ("gv", "Guinea Chryss ny Cruinney"),
        ("ha", "Equatorial Guinea"),
        ("he", "גינאה המשוונית"),
        ("hi", "भ\u{942}मध\u{94d}यर\u{947}खीय गिनी"),
        ("hr", "Ekvatorijalna Gvineja"),
        ("ht", "Gine ekwateryal"),
        ("hu", "Egyenlítői-Guinea"),
        ("hy", "Հասարակածային Գվինեա"),
        ("ia", "Guinea Equatorial"),
        ("id", "Guinea Khatulistiwa"),
        ("io", "Equatorala Guinea"),
        ("is", "Miðbaugs-Gínea"),
        ("it", "Guinea equatoriale"),
        ("iu", "Equatorial Guinea"),
        ("ja", "赤道ギニア"),
        ("ka", "ეკვატორული გვინეა"),
        ("ki", "Equatorial Guinea"),
        ("kk", "Экваториалдық Гвинея"),
        ("kl", "Equatorial Guinea"),
        ("km", "ហ\u{17d2}គ\u{17b8}ណេ\u{200b}អេក\u{17d2}វាទ\u{17d0}រ"),
        ("kn", "ಈಕ\u{ccd}ವಟೋರ\u{cbf}ಯಲ\u{ccd} ಗ\u{cbf}ನೀ"),
        ("ko", "적도 기니"),
        ("ku", "Gîneya Ekvatorî"),
        ("kv", "Equatorial Guinea"),
        ("kw", "Gyni Ekwadoriel"),
        ("ky", "Экваториалдык Гвинея"),
        ("lo", "Equatorial Guinea"),
        ("lt", "Pusiaujo Gvinėja"),
        ("lv", "Ekvatoriālā Gvineja"),
        ("mi", "Kini Ekuatoria"),
        ("mk", "Екваторијална Гвинеја"),
        ("ml", "ഇക\u{d4d}വടോറിയല\u{d4d}\u{200d} ഗിനിയ"),
        ("mn", "Equatorial Guinea"),
        ("mr", "इक\u{94d}व\u{947}टोरियल गिनी"),
        ("ms", "Guinea Khatulistiwa"),
        ("mt", "Ginea Ekwatorjali"),
        (
            "my",
            "အ\u{102e}က\u{103d}ေတာဂ\u{102e}န\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Gini t Ekwador"),
        ("nb", "Ekvatorial-Guinea"),
        ("ne", "इक\u{94d}व\u{947}टरियल जिनीया"),
        ("nl", "Equatoriaal-Guinea"),
        ("nn", "Ekvatorial-Guinea"),
        ("nv", "Gíní Nahasdzáán Ałníiʼgi Siʼánígíí"),
        ("oc", "Guinèa eqüatoriala"),
        ("or", "ବ\u{b3f}ଷ\u{b41}ବରେଖୀୟ ଗ\u{b3f}ନୀ"),
        ("pa", "ਭ\u{a42}-ਖ\u{a70}ਡੀ ਗ\u{a41}ਆਨਾ"),
        ("pi", "Equatorial Guinea"),
        ("pl", "Gwinea Równikowa"),
        ("ps", "Equatorial Guinea"),
        ("pt", "Guiné Equatorial"),
        ("pt_BR", "Guiné Equatorial"),
        ("ro", "Guinea Ecuatorială"),
        ("ru", "Экваториальная Гвинея"),
        ("rw", "Gineya Ekwatoriyale"),
        ("sc", "Guinea Ecuadoriale"),
        ("sd", "Equatorial Guinea"),
        ("si", "ග\u{dd2}න\u{dd2}ය\u{dcf} සම\u{dcf}න\u{dcf}ත\u{dca}මය"),
        ("sk", "Rovníková Guinea"),
        ("sl", "Ekvatorialna Gvineja"),
        ("so", "Ikweetiga Guinea"),
        ("sq", "Guinea Ekuatoriale"),
        ("sr", "Екваторијална Гвинеја"),
        ("sv", "Ekvatorialguinea"),
        ("sw", "Equatorial Guinea"),
        ("ta", "ஈக\u{bcd}குவிடோரியல\u{bcd} கினி"),
        (
            "te",
            "ఇక\u{c4d}వ\u{c3f}ట\u{c4b}ర\u{c3f}యల\u{c4d} గ\u{c3f}న\u{c40}",
        ),
        ("tg", "Гвинеяи Экваторӣ"),
        ("th", "อ\u{e34}เควทอเร\u{e35}ยลก\u{e34}น\u{e35}"),
        ("ti", "ኢኳቶሪያል ጊኒ"),
        ("tk", "Ekwatorial Gwineýa"),
        ("tl", "Equatorial Guinea"),
        ("tr", "Ekvator Ginesi"),
        ("tt", "Екуаторлы Gуинеа"),
        ("ug", "ئېكۋاتور گىۋىنېيەسى"),
        ("uk", "Екваторіальна Гвінея"),
        ("ur", "استوائی گنی"),
        ("uz", "Ekvatorli Gvineya"),
        ("ve", "Equatorial Guinea"),
        ("vi", "Ghi-nê Xích Đạo"),
        ("wa", "Guinêye Ecwåtoriåle"),
        ("wo", "Ginne Ekwatoriaal"),
        ("xh", "Equatorial Guinea"),
        ("yo", "Guinea Alágedeméjì"),
        ("zh_CN", "赤道几内亚"),
        ("zh_HK", "赤道畿內亞"),
        ("zh_TW", "赤道幾內亞"),
        ("zu", "IGini Enkabazwe"),
    ];
    #[cfg(all(feature = "gq", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 1.650801;
        pub const LONGITUDE: f64 = 10.267895;
        pub const MAX_LATITUDE: f64 = 3.8355;
        pub const MAX_LONGITUDE: f64 = 11.3333;
        pub const MIN_LATITUDE: f64 = -1.5475;
        pub const MIN_LONGITUDE: f64 = 5.541900099999999;
        pub const NORTHEAST_LATITUDE: f64 = 3.8355;
        pub const NORTHEAST_LONGITUDE: f64 = 11.3333;
        pub const SOUTHWEST_LATITUDE: f64 = -1.5475;
        pub const SOUTHWEST_LONGITUDE: f64 = 5.541900099999999;
    }
}
#[cfg(all(feature = "gq", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 1.650801,
            longitude: 10.267895,
            max_latitude: 3.8355,
            max_longitude: 11.3333,
            min_latitude: -1.5475,
            min_longitude: 5.541900099999999,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 3.8355,
                    longitude: 11.3333,
                },
                southwest: CountryGeoBound {
                    latitude: -1.5475,
                    longitude: 5.541900099999999,
                },
            },
        }
    }
}

#[cfg(all(feature = "gq", feature = "subdivisions"))]
pub mod subdivisions {
    use crate::Subdivision;
    use std::collections::HashMap;
    // In this state, We do not know if subdivisions have geo or not!
    #[cfg(feature = "geo")]
    #[allow(unused_imports)]
    use crate::{Alpha2, SubdivisionGeo, SubdivisionType};

    pub fn new() -> HashMap<&'static str, Subdivision> {
        HashMap::from(
            [

                (
                    "AN",
                    Subdivision{
                        name: "AN",
                        country_alpha2: Alpha2::GQ,
                        code: "AN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(-1.4221087), longitude: Some(5.6195112), max_latitude: Some(-1.4041375), min_latitude: Some(-1.457793), max_longitude: Some(5.635208), min_longitude: Some(5.6119729)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة أنوبون"), ("bg", "Анобон"), ("bn", "অ\u{9cd}য\u{9be}নোবোন প\u{9cd}রদেশ"), ("ca", "Província d’Annobón"), ("ccp", "𑄃𑄚\u{11133}𑄦\u{1112e}𑄝\u{11127}𑄚\u{11134}"), ("ceb", "Provincia de Annobón"), ("da", "Annobón Province"), ("de", "Provinz Annobón"), ("el", "Ανομπόν"), ("en", "Annobón"), ("es", "Provincia de Annobón"), ("et", "Annobóni provints"), ("eu", "Annobongo probintzia"), ("fa", "استان آنوبون"), ("fi", "Annobónin provinssi"), ("fr", "province d’Annobón"), ("gu", "એનોબોન પ\u{acd}રા\u{a82}ત"), ("he", "אנובון"), ("hi", "एनोबोन प\u{94d}रा\u{902}त"), ("id", "Provinsi Annobón"), ("it", "Provincia di Annobón"), ("ja", "アンノボン県"), ("ka", "ანობონის პროვინცია"), ("kn", "ಅನ\u{ccd}ನೊಬಾನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "안노본 주"), ("lt", "Anobono provincija"), ("lv", "Annobonas province"), ("ml", "അന\u{d4d}നോബോൺ"), ("mr", "ऍनोबोन प\u{94d}रा\u{902}त"), ("ms", "Annobon Province"), ("nb", "Annobón"), ("nl", "Annobón Province"), ("no", "Annobón"), ("pl", "Prowincja Annobón"), ("pt", "Província de Annobón"), ("ro", "Provincia Annobón"), ("ru", "Провинция Аннобон"), ("si", "අන\u{dca}නොබෝන\u{dca} පළ\u{dcf}ත"), ("sv", "Annobón (provins)"), ("ta", "அந\u{bcd}நோபோன\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "అన\u{c4b}బ\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดแอนโนบอน"), ("tr", "Annobón Province"), ("uk", "Провінція Анобон"), ("ur", "اننوبون صوبہ"), ("vi", "Tỉnh Annobón")]),
                        unofficial_name_list: ["Annobón"].to_vec(),
                    }
                ),
                (
                    "BN",
                    Subdivision{
                        name: "BN",
                        country_alpha2: Alpha2::GQ,
                        code: "BN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(3.6595072), longitude: Some(8.7921836), max_latitude: Some(3.7858361), min_latitude: Some(3.511882), max_longitude: Some(8.937981599999999), min_longitude: Some(8.6119104)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بيوكو نورت"), ("bg", "Северна Биоко"), ("bn", "বিওকো\u{981} নর\u{9cd}টে প\u{9cd}রদেশ"), ("ca", "Bioko Nord"), ("ccp", "𑄝\u{1112d}𑄇\u{1112e} 𑄚\u{11127}𑄢\u{11134}𑄑𑄬"), ("ceb", "Provincia de Bioko Norte"), ("da", "Bioko Norte Province"), ("de", "Bioko Norte"), ("el", "Μπιόκο Νόρτε"), ("en", "Bioko Norte"), ("es", "Provincia de Bioko Norte"), ("eu", "Iparraldeko Bioko"), ("fi", "Bioko Norten provinssi"), ("fr", "Bioko-Norte"), ("gu", "બાયોકો નોર\u{acd}ટ પ\u{acd}રા\u{a82}ત"), ("hi", "बियोको नॉर\u{94d}ट प\u{94d}रा\u{902}त"), ("hr", "Sjeverni Bioko"), ("hu", "Bioko Norte tartomány"), ("id", "Provinsi Bioko Norte"), ("it", "provincia di Bioko Nord"), ("ja", "北ビオコ県"), ("ka", "ჩრდილოეთ ბიოკოს პროვინცია"), ("kn", "ಬಯೋಕೊ ನಾರ\u{ccd}ಟ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "북비오코 주"), ("lt", "Šiaurės Bioko provincija"), ("lv", "Ziemeļbioko province"), ("mr", "ज\u{948}को नॉर\u{94d}ट प\u{94d}रा\u{902}त"), ("ms", "Bioko Norte Province"), ("nb", "Bioko Norte"), ("nl", "Bioko Norte"), ("no", "Bioko Norte"), ("pl", "Bioko Północne"), ("pt", "Bioko Norte"), ("ro", "Provincia Bioko Norte"), ("ru", "Северный Биоко"), ("si", "බයොකො නොර\u{dca}ටේ පළ\u{dcf}ත"), ("sv", "Provincia de Bioko Norte"), ("ta", "பியோகோ நோர\u{bcd}ட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3f}య\u{c4b}క\u{c4b} న\u{c3e}ర\u{c4d}ట\u{c46} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e35}โอโกเหน\u{e37}อ"), ("tr", "Bioko Norte Province"), ("uk", "Бйоко Норте"), ("ur", "بیوکو نورتی صوبہ"), ("vi", "Tỉnh Bioko Norte"), ("zh", "北比奥科省")]),
                        unofficial_name_list: ["Bioko Norte"].to_vec(),
                    }
                ),
                (
                    "BS",
                    Subdivision{
                        name: "BS",
                        country_alpha2: Alpha2::GQ,
                        code: "BS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(3.4209785), longitude: Some(8.6160674), max_latitude: Some(3.6476928), min_latitude: Some(3.2099564), max_longitude: Some(8.850613899999999), min_longitude: Some(8.423113899999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بيوكو سور"), ("bg", "Южна Биоко"), ("bn", "বিয\u{9bc}োকো স\u{9c1}র প\u{9cd}রদেশ"), ("ca", "Bioko Sud"), ("ccp", "𑄝\u{1112d}𑄇\u{1112e} 𑄥𑄢\u{11134}"), ("ceb", "Provincia de Bioko Sur"), ("da", "Bioko Sur Province"), ("de", "Bioko Sur"), ("el", "Μπιόκο Σούρ"), ("en", "Bioko Sur"), ("es", "Provincia de Bioko Sur"), ("eu", "Hegoaldeko Bioko"), ("fa", "استان بیوکو سور"), ("fi", "Bioko Surin provinssi"), ("fr", "Bioko-Sur"), ("gu", "બાઈકો સ\u{ac1}ર પ\u{acd}રા\u{a82}ત"), ("hi", "बिओको-स\u{941}र प\u{94d}रा\u{902}त"), ("hr", "Južni Bioko"), ("id", "Provinsi Bioko Sur"), ("it", "provincia di Bioko Sud"), ("ja", "南ビオコ県"), ("ka", "სამხრეთ ბიოკოს პროვინცია"), ("kn", "ಬಯೋಕೊ ಸುರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "남비오코 주"), ("lt", "Pietų Bioko provincija"), ("lv", "Dienvidbioko province"), ("mr", "बीओवो स\u{941}र प\u{94d}रा\u{902}त"), ("ms", "Bioko Sur Province"), ("nb", "Bioko Sur"), ("nl", "Bioko Sur"), ("no", "Bioko Sur"), ("pl", "Bioko Południowe"), ("pt", "Bioko Sur"), ("ro", "Provincia Bioko Sur"), ("ru", "Южный Биоко"), ("si", "බ\u{dd2}යොකෝ සර\u{dca} පළ\u{dcf}ත"), ("sv", "Provincia de Bioko Sur"), ("ta", "பியோகோ சூர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బయ\u{c4b}క\u{c4b} సుర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไบโอโค เซอ"), ("tr", "Bioko Sur Province"), ("uk", "Бйоко Сур"), ("ur", "بیوکو سر صوبہ"), ("vi", "Tỉnh Bioko Sur"), ("zh", "南比奥科省")]),
                        unofficial_name_list: ["Bioko Sur"].to_vec(),
                    }
                ),
                (
                    "C",
                    Subdivision{
                        name: "C",
                        country_alpha2: Alpha2::GQ,
                        code: "C",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(51.2798759), longitude: Some(8.8744572), max_latitude: Some(51.28056), min_latitude: Some(51.2798269), max_longitude: Some(8.8745578), min_longitude: Some(8.873045399999999)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Rio Muni"), ("bg", "Мбини"), ("ca", "Mbini"), ("ccp", "𑄢\u{11128}𑄃\u{1112e} 𑄟\u{1112a}𑄚\u{11128}"), ("ceb", "Woleu"), ("cs", "Río Muni"), ("de", "Mbini"), ("en", "Río Muni"), ("es", "Río Muni"), ("et", "Río Muni"), ("eu", "Río Muni"), ("fi", "Río Muni"), ("fr", "Région continentale"), ("he", "ריו מוני"), ("hr", "Rijeka Muni"), ("it", "Rio Muni"), ("ja", "リオ・ムニ"), ("ka", "რიო-მუნი"), ("ko", "리오무니"), ("lt", "Rio Munis"), ("nb", "Río Muni"), ("nl", "Mbini"), ("no", "Río Muni"), ("pl", "Mbini"), ("pt", "Rio Muni"), ("ru", "Рио-Муни"), ("sv", "Río Muni"), ("uk", "Ріо-Муні"), ("vi", "Río Muni"), ("zh", "木尼河区")]),
                        unofficial_name_list: ["Región Continental"].to_vec(),
                    }
                ),
                (
                    "CS",
                    Subdivision{
                        name: "CS",
                        country_alpha2: Alpha2::GQ,
                        code: "CS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.3436084), longitude: Some(10.439656), max_latitude: Some(2.1742861), min_latitude: Some(0.9966689999999999), max_longitude: Some(10.98445), min_longitude: Some(9.9033)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سينترو سور"), ("bg", "Южна централна провинция"), ("bn", "সেন\u{9cd}ট\u{9cd}রো স\u{9c1}র প\u{9cd}রদেশ"), ("ca", "Centre Sud"), ("ccp", "𑄥𑄬𑄚\u{11134}𑄑\u{11133}𑄢\u{1112e} 𑄥𑄢\u{11134}"), ("ceb", "Provincia de Centro Sur"), ("da", "Centro Sur Province"), ("de", "Centro Sur"), ("el", "Κέντρο Σούρ"), ("en", "Centro Sur"), ("es", "Provincia Centro Sur"), ("et", "Kesk-Lõunaprovints"), ("eu", "Erdi-Hegoaldea"), ("fa", "استان سنترو سور"), ("fi", "Centro Surin provinssi"), ("fr", "Centro-Sur"), ("gu", "સ\u{ac7}ન\u{acd}ટ\u{acd}રો સ\u{ac1}ર પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{947}\u{902}ट\u{94d}रो स\u{941}र प\u{94d}रा\u{902}त"), ("id", "Provinsi Centro Sur"), ("it", "provincia Centro Sud"), ("ja", "中南部県"), ("ka", "ცენტრო-სურის პროვინცია"), ("kn", "ಸ\u{cc6}ಂಟ\u{ccd}ರೊ ಸುರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "중남부 주"), ("lt", "Centro Pietų provincija"), ("lv", "Centrālā Dienvidu province"), ("mr", "स\u{947}\u{902}ट\u{94d}रो स\u{941}र प\u{94d}रा\u{902}त"), ("ms", "Centro Sur Province"), ("nb", "Centro Sur"), ("nl", "Centro Sur"), ("no", "Centro Sur"), ("pl", "Prowincja Środkowo-Południowa"), ("pt", "Centro Sur"), ("ro", "Provincia Centro Sur"), ("ru", "Центро-Сур"), ("si", "සෙන\u{dca}ට\u{dca}රෝ සර\u{dca} පළ\u{dcf}ත"), ("sv", "Provincia de Centro Sur"), ("ta", "சென\u{bcd}ட\u{bcd}ரோ சூர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c46}ంట\u{c4d}ర\u{c4b} సుర\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "เซนโตรซ\u{e39}ร\u{e4c}"), ("tr", "Centro Sur Province"), ("uk", "Сентро-Сур"), ("ur", "سینترو سر صوبہ"), ("vi", "Tỉnh Centro Sur"), ("zh", "中南省")]),
                        unofficial_name_list: ["Centro Sur"].to_vec(),
                    }
                ),
                (
                    "DJ",
                    Subdivision{
                        name: "DJ",
                        country_alpha2: Alpha2::GQ,
                        code: "DJ",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("en", "Djibloho")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "I",
                    Subdivision{
                        name: "I",
                        country_alpha2: Alpha2::GQ,
                        code: "I",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(49.662167), longitude: Some(-125.746833), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Region,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ca", "Illes de Guinea Equatorial"), ("ccp", "𑄃\u{11128}𑄚\u{11134}𑄥\u{1112a}𑄣𑄢\u{11134}"), ("en", "Insular"), ("es", "Islas de Guinea Ecuatorial"), ("fr", "Région insulaire"), ("it", "Regione Insulare"), ("ja", "島嶼地方 (赤道ギニア)"), ("ka", "კუნძულოვანი რეგიონი"), ("ko", "적도 기니 도서 지방"), ("nl", "Insular Region"), ("pt", "Região Insular"), ("ru", "Инсуляр"), ("zh", "海岛大区")]),
                        unofficial_name_list: ["Región Insular"].to_vec(),
                    }
                ),
                (
                    "KN",
                    Subdivision{
                        name: "KN",
                        country_alpha2: Alpha2::GQ,
                        code: "KN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(2.028093), longitude: Some(11.0711758), max_latitude: Some(2.187128), min_latitude: Some(1.7260761), max_longitude: Some(11.3356839), min_longitude: Some(10.401541)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كيي - نتم"), ("bg", "Кие-Нтем"), ("bn", "কিন\u{9cd}টেম প\u{9cd}রদেশ"), ("ca", "Kié-Ntem"), ("ccp", "𑄇\u{1112d}-𑄑𑄬𑄟\u{11134}"), ("ceb", "Provincia de Kié-Ntem"), ("da", "Kié-Ntem Province"), ("de", "Kié-Ntem"), ("el", "Κιε-Ντεμ"), ("en", "Kié-Ntem"), ("es", "Provincia Kié-Ntem"), ("eu", "Kié-Ntem"), ("fi", "Kié-Ntemin provinssi"), ("fr", "Kié-Ntem"), ("gu", "કિએ-એન\u{acd}ટ\u{ac7}મ પ\u{acd}રા\u{a82}ત"), ("hi", "काई-ट\u{947}म प\u{94d}रा\u{902}त"), ("hu", "Kié-Ntem tartomány"), ("id", "Provinsi Kié-Ntem"), ("it", "provincia Kié-Ntem"), ("ja", "キエンテム県"), ("ka", "კე-ნტემის პროვინცია"), ("kn", "ಕೀ-ಎನ\u{ccd}ಟ\u{cc6}ಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "키에은템 주"), ("lt", "Ke Ntemo provincija"), ("lv", "Kē-Ntemas province"), ("mr", "क\u{947}ए-न\u{94d}ट\u{947}म प\u{94d}रा\u{902}त"), ("ms", "Kie-Ntem Province"), ("nb", "Kié-Ntem"), ("nl", "Kié-Ntem"), ("no", "Kié-Ntem"), ("pl", "Kié-Ntem"), ("pt", "Kié-Ntem"), ("ro", "Provincia Kié-Ntem"), ("ru", "Ке-Нтем"), ("si", "කය\u{dd2}-එන\u{dca}ටෙම\u{dca} පළ\u{dcf}ත"), ("sv", "Provincia de Kié-Ntem"), ("ta", "கிய -நட\u{bcd}டம\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c48}-ఎంట\u{c46}మ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดค\u{e35}-นเท\u{e4b}ม"), ("tr", "Kié-Ntem Province"), ("uk", "Ке-Нтем"), ("ur", "کیی-نتیم صوبہ"), ("vi", "Tỉnh Kié-Ntem"), ("zh", "基埃恩特姆省")]),
                        unofficial_name_list: ["Kie-Ntem"].to_vec(),
                    }
                ),
                (
                    "LI",
                    Subdivision{
                        name: "LI",
                        country_alpha2: Alpha2::GQ,
                        code: "LI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.5750244), longitude: Some(9.8124935), max_latitude: Some(2.346989), min_latitude: Some(0.8891347), max_longitude: Some(10.263524), min_longitude: Some(9.302264)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ليتورال"), ("bg", "Литорал"), ("bn", "লিটর\u{9be}ল প\u{9cd}রদেশ"), ("ca", "Província Litoral"), ("ccp", "𑄣\u{11128}𑄑\u{1112e}𑄢𑄣\u{11134}"), ("ceb", "Provincia de Litoral"), ("da", "Litoral Province"), ("de", "Litoral"), ("el", "Λίτοραλ"), ("en", "Litoral"), ("es", "Provincia Litoral"), ("eu", "Kostaldea"), ("fi", "Litoralin provinssi"), ("fr", "Litoral"), ("gu", "લિટોરલ પ\u{acd}રા\u{a82}ત"), ("hi", "लिटोरल प\u{94d}रा\u{902}त (इक\u{94d}व\u{947}टोरियल गिनी)"), ("hr", "Primorje"), ("id", "Provinsi Litoral"), ("it", "provincia Litorale"), ("ja", "リトラル県"), ("ka", "ლიტორალის პროვინცია"), ("kn", "ಲ\u{cbf}ಟೋರಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "리토랄 주"), ("lt", "Litoralės provincija"), ("lv", "Litorālā province"), ("mr", "लिटोरल प\u{94d}रा\u{902}त"), ("ms", "Litoral Province"), ("nb", "Litoral"), ("nl", "Litoral"), ("no", "Litoral"), ("pl", "Prowincja Nadmorska"), ("pt", "Litoral"), ("ro", "Provincia Litoral, Guineea Ecuatorială"), ("ru", "Литорал"), ("si", "ල\u{dd2}ටොරල\u{dca} පළ\u{dcf}ත"), ("sv", "Litoral"), ("ta", "லிட\u{bcd}டோர\u{bbe}ல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ల\u{c3f}ట\u{c4b}రల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดล\u{e34}โทรอล"), ("tr", "Litoral Province"), ("uk", "Літорал"), ("ur", "لیتورال صوبہ"), ("vi", "Tỉnh Litoral"), ("zh", "滨海省")]),
                        unofficial_name_list: ["Litoral"].to_vec(),
                    }
                ),
                (
                    "WN",
                    Subdivision{
                        name: "WN",
                        country_alpha2: Alpha2::GQ,
                        code: "WN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(1.4166162), longitude: Some(11.0711758), max_latitude: Some(1.994745), min_latitude: Some(0.996211), max_longitude: Some(11.3357239), min_longitude: Some(10.5228439)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ويلي-نزاس"), ("bg", "Уеле-Нзас"), ("bn", "ওয\u{9bc}েলে-জ\u{9be}স প\u{9cd}রদেশ"), ("ca", "Wele-Nzas"), ("ccp", "𑄤𑄬𑄠𑄬 𑄚𑄌\u{11134}"), ("ceb", "Provincia de Wele-Nzas"), ("da", "Wele-Nzas Province"), ("de", "Wele-Nzas"), ("el", "Βέλε-Νζας"), ("en", "Wele-Nzas"), ("es", "Provincia Wele-Nzas"), ("eu", "Wele-Nzas"), ("fi", "Wele-Nzasin provinssi"), ("fr", "Wele-Nzas"), ("gu", "વ\u{ac7}લ\u{ac7}-નઝાસ પ\u{acd}રા\u{a82}ત"), ("hi", "व\u{947}ल\u{947}-नज\u{93c}ास प\u{94d}रा\u{902}त"), ("id", "Provinsi Wele-Nzas"), ("it", "provincia Wele-Nzas"), ("ja", "ウェレンザス県"), ("ka", "ველე-ნზასის პროვინცია"), ("kn", "ವೈಲ\u{ccd}-ಎನ\u{ccd}ಝಾ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "웰레은사스 주"), ("lt", "Vele Nzaso provincija"), ("lv", "Veles-Nzasas province"), ("mr", "व\u{947}ल\u{947}-नझस प\u{94d}रा\u{902}त"), ("ms", "Wele-Nzas Province"), ("nb", "Wele-Nzás"), ("nl", "Wele-Nzas"), ("no", "Wele-Nzás"), ("pl", "Wele-Nzas"), ("pt", "Wele-Nzas"), ("ro", "Provincia Wele-Nzas"), ("ru", "Веле-Нзас"), ("si", "වෙලේ -එන\u{dca}ස\u{dcf}ස\u{dca} පළ\u{dcf}ත"), ("sv", "Provincia de Wele-Nzas"), ("ta", "வெலே -ன\u{bcd}சஸ\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "వ\u{c46}ల\u{c46}-ఎన\u{c4d}జ\u{c3e}స\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเวเลแนสซ\u{e4c}"), ("tr", "Wele-Nzas Province"), ("uk", "Веле-Нзас"), ("ur", "ویلی-نزاس صوبہ"), ("vi", "Tỉnh Wele-Nzas"), ("zh", "韦莱恩萨斯省")]),
                        unofficial_name_list: ["Wele-Nzás"].to_vec(),
                    }
                ),
            ]

        )
    }
}
#[allow(unused_imports)]
use crate::{Alpha2, Alpha3, Continent, Country, Region, SubRegion, WeekDay, WorldRegion, GEC};
#[allow(unused_imports)]
use std::collections::HashMap;
#[cfg(feature = "gq")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::GQ,
        alpha3: Alpha3::GNQ,
        address_format: None,
        continent: Continent::Africa,
        country_code: 240,
        currency_code: "XAF",
        gec: Some(GEC::EK),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("GEQ"),
        iso_long_name: "The Republic of Equatorial Guinea",
        iso_short_name: "Equatorial Guinea",
        official_language_list: ["es", "fr"].to_vec(),
        spoken_language_list: ["es", "fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [6].to_vec(),
        national_prefix: "None",
        nationality: Some("Equatorial Guinean"),
        number: "226",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::MiddleAfrica),
        un_locode: "GQ",
        unofficial_name_list: [
            "Equatorial Guinea",
            "Äquatorial-Guinea",
            "Guinée Équatoriale",
            "Guinea Ecuatorial",
            "赤道ギニア",
            "Equatoriaal-Guinea",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Equatorial Guinea"),
            ("af", "Ekwatoriaal-Guinee"),
            ("ak", "Equatorial Guinea"),
            ("am", "Equatorial Guinea"),
            ("an", "Equatorial Guinea"),
            ("ar", "غينيا الاستوائي\u{651}ة"),
            ("as", "বিষ\u{9c1}বীয় গিনি"),
            ("ay", "Equatorial Guinea"),
            ("az", "Ekvatorial Qvineya"),
            ("ba", "Equatorial Guinea"),
            ("be", "Экватарыяльная Гвінея"),
            ("bg", "Екваториална Гвинея"),
            ("bi", "Equatorial Guinea"),
            ("bn", "ইক\u{9c1}য়েটোরিয়\u{9be}ল গিনি"),
            ("bn_IN", "ইক\u{9c1}য়েটোরিয়\u{9be}ল গিনি"),
            ("br", "Ginea ar C'heheder"),
            ("bs", "Ekvatorijalna Gvineja"),
            ("ca", "Guinea Equatorial"),
            ("ce", "Экваторан Гвиней"),
            ("ch", "Equatorial Guinea"),
            ("cs", "Rovníková Guinea"),
            ("cv", "Экваторан Гвиней"),
            ("cy", "Guinea Cyhydeddol"),
            ("da", "Ækvatorialguinea"),
            ("de", "Äquatorialguinea"),
            (
                "dv",
                "އ\u{7a8}ކ\u{7aa}އ\u{7ac}ޓ\u{7af}ރ\u{7a8}އ\u{7a6}ލ\u{7b0} ގ\u{7a8}ނ\u{7a9}",
            ),
            (
                "dz",
                "ཨ\u{f72}་ཀ\u{f74}ཡ\u{f7a}་ཊ\u{f7c}་ར\u{f72}་ཡ\u{f71}ལ་ ག\u{f72}་ན\u{f72}།",
            ),
            ("ee", "Equatorial Guinea"),
            ("el", "Ισημερινή Γουινέα"),
            ("en", "Equatorial Guinea"),
            ("eo", "Ekvatora Gvineo"),
            ("es", "Guinea Ecuatorial"),
            ("et", "Ekvatoriaal-Guinea"),
            ("eu", "Ekuatore Ginea"),
            ("fa", "گینه\u{654} استوایی"),
            ("ff", "Gine Ekwatoriyal"),
            ("fi", "Päiväntasaajan Guinea"),
            ("fo", "Ekvator Guinea"),
            ("fr", "Guinée Équatoriale"),
            ("fy", "Ekwatoriaal-Guinee"),
            ("ga", "An Ghuine Mheánchiorclach"),
            ("gl", "Guinea Ecuatorial"),
            ("gn", "Equatorial Guinea"),
            ("gu", "ઇક\u{acd}વોરીઅલ ગ\u{ac1}એના"),
            ("gv", "Guinea Chryss ny Cruinney"),
            ("ha", "Equatorial Guinea"),
            ("he", "גינאה המשוונית"),
            ("hi", "भ\u{942}मध\u{94d}यर\u{947}खीय गिनी"),
            ("hr", "Ekvatorijalna Gvineja"),
            ("ht", "Gine ekwateryal"),
            ("hu", "Egyenlítői-Guinea"),
            ("hy", "Հասարակածային Գվինեա"),
            ("ia", "Guinea Equatorial"),
            ("id", "Guinea Khatulistiwa"),
            ("io", "Equatorala Guinea"),
            ("is", "Miðbaugs-Gínea"),
            ("it", "Guinea equatoriale"),
            ("iu", "Equatorial Guinea"),
            ("ja", "赤道ギニア"),
            ("ka", "ეკვატორული გვინეა"),
            ("ki", "Equatorial Guinea"),
            ("kk", "Экваториалдық Гвинея"),
            ("kl", "Equatorial Guinea"),
            ("km", "ហ\u{17d2}គ\u{17b8}ណេ\u{200b}អេក\u{17d2}វាទ\u{17d0}រ"),
            ("kn", "ಈಕ\u{ccd}ವಟೋರ\u{cbf}ಯಲ\u{ccd} ಗ\u{cbf}ನೀ"),
            ("ko", "적도 기니"),
            ("ku", "Gîneya Ekvatorî"),
            ("kv", "Equatorial Guinea"),
            ("kw", "Gyni Ekwadoriel"),
            ("ky", "Экваториалдык Гвинея"),
            ("lo", "Equatorial Guinea"),
            ("lt", "Pusiaujo Gvinėja"),
            ("lv", "Ekvatoriālā Gvineja"),
            ("mi", "Kini Ekuatoria"),
            ("mk", "Екваторијална Гвинеја"),
            ("ml", "ഇക\u{d4d}വടോറിയല\u{d4d}\u{200d} ഗിനിയ"),
            ("mn", "Equatorial Guinea"),
            ("mr", "इक\u{94d}व\u{947}टोरियल गिनी"),
            ("ms", "Guinea Khatulistiwa"),
            ("mt", "Ginea Ekwatorjali"),
            (
                "my",
                "အ\u{102e}က\u{103d}ေတာဂ\u{102e}န\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Gini t Ekwador"),
            ("nb", "Ekvatorial-Guinea"),
            ("ne", "इक\u{94d}व\u{947}टरियल जिनीया"),
            ("nl", "Equatoriaal-Guinea"),
            ("nn", "Ekvatorial-Guinea"),
            ("nv", "Gíní Nahasdzáán Ałníiʼgi Siʼánígíí"),
            ("oc", "Guinèa eqüatoriala"),
            ("or", "ବ\u{b3f}ଷ\u{b41}ବରେଖୀୟ ଗ\u{b3f}ନୀ"),
            ("pa", "ਭ\u{a42}-ਖ\u{a70}ਡੀ ਗ\u{a41}ਆਨਾ"),
            ("pi", "Equatorial Guinea"),
            ("pl", "Gwinea Równikowa"),
            ("ps", "Equatorial Guinea"),
            ("pt", "Guiné Equatorial"),
            ("pt_BR", "Guiné Equatorial"),
            ("ro", "Guinea Ecuatorială"),
            ("ru", "Экваториальная Гвинея"),
            ("rw", "Gineya Ekwatoriyale"),
            ("sc", "Guinea Ecuadoriale"),
            ("sd", "Equatorial Guinea"),
            ("si", "ග\u{dd2}න\u{dd2}ය\u{dcf} සම\u{dcf}න\u{dcf}ත\u{dca}මය"),
            ("sk", "Rovníková Guinea"),
            ("sl", "Ekvatorialna Gvineja"),
            ("so", "Ikweetiga Guinea"),
            ("sq", "Guinea Ekuatoriale"),
            ("sr", "Екваторијална Гвинеја"),
            ("sv", "Ekvatorialguinea"),
            ("sw", "Equatorial Guinea"),
            ("ta", "ஈக\u{bcd}குவிடோரியல\u{bcd} கினி"),
            (
                "te",
                "ఇక\u{c4d}వ\u{c3f}ట\u{c4b}ర\u{c3f}యల\u{c4d} గ\u{c3f}న\u{c40}",
            ),
            ("tg", "Гвинеяи Экваторӣ"),
            ("th", "อ\u{e34}เควทอเร\u{e35}ยลก\u{e34}น\u{e35}"),
            ("ti", "ኢኳቶሪያል ጊኒ"),
            ("tk", "Ekwatorial Gwineýa"),
            ("tl", "Equatorial Guinea"),
            ("tr", "Ekvator Ginesi"),
            ("tt", "Екуаторлы Gуинеа"),
            ("ug", "ئېكۋاتور گىۋىنېيەسى"),
            ("uk", "Екваторіальна Гвінея"),
            ("ur", "استوائی گنی"),
            ("uz", "Ekvatorli Gvineya"),
            ("ve", "Equatorial Guinea"),
            ("vi", "Ghi-nê Xích Đạo"),
            ("wa", "Guinêye Ecwåtoriåle"),
            ("wo", "Ginne Ekwatoriaal"),
            ("xh", "Equatorial Guinea"),
            ("yo", "Guinea Alágedeméjì"),
            ("zh_CN", "赤道几内亚"),
            ("zh_HK", "赤道畿內亞"),
            ("zh_TW", "赤道幾內亞"),
            ("zu", "IGini Enkabazwe"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}