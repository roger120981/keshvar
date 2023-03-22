// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// Hungary

#[cfg(all(feature = "hu", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> =
        Some("{{recipient}}\n{{city}}\n{{street}}\n{{postalcode}}\n{{country}}");
    pub const ALPHA2: Alpha2 = Alpha2::HU;
    pub const ALPHA3: Alpha3 = Alpha3::HUN;
    pub const CONTINENT: Continent = Continent::Europe;
    pub const COUNTRY_CODE: usize = 36;
    pub const CURRENCY_CODE: &str = "HUF";
    pub const GEC: Option<GEC> = Some(GEC::HU);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("HUN");
    pub const ISO_SHORT_NAME: &str = "Hungary";
    pub const ISO_LONG_NAME: &str = "Hungary";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["hu"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["hu"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8, 9];
    pub const NATIONAL_PREFIX: &str = "06";
    pub const NATIONALITY: Option<&str> = Some("Hungarian");
    pub const NUMBER: &str = "348";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{4}");
    pub const REGION: Option<Region> = Some(Region::Europe);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::EasternEurope);
    pub const UN_LOCODE: &str = "HU";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Hungary",
        "Ungarn",
        "Hongrie",
        "Hungría",
        "ハンガリー",
        "Hongarije",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Hungary"),
        ("af", "Hongarye"),
        ("ak", "Hungary"),
        ("am", "ሀንጒሱ"),
        ("an", "Hungary"),
        ("ar", "المجر (هنغاريا)"),
        ("as", "হ\u{9be}ংগেৰী"),
        ("ay", "Hungary"),
        ("az", "Macarıstan"),
        ("ba", "Hungary"),
        ("be", "Венгрыя"),
        ("bg", "Унгария"),
        ("bi", "Hungary"),
        ("bn", "হ\u{9be}ঙ\u{9cd}গেরি"),
        ("bn_IN", "হ\u{9be}ঙ\u{9cd}গেরি"),
        ("br", "Hungaria"),
        ("bs", "Mađarska"),
        ("ca", "Hongria"),
        ("ce", "Венгри"),
        ("ch", "Hungary"),
        ("cs", "Maďarsko"),
        ("cv", "Венгри"),
        ("cy", "Hwngari"),
        ("da", "Ungarn"),
        ("de", "Ungarn"),
        ("dv", "ހ\u{7a6}ނ\u{7b0}ގ\u{7ad}ރ\u{7a9}"),
        ("dz", "ཧང་ག་ར\u{f72}།"),
        ("ee", "Hungary"),
        ("el", "Ουγγαρία"),
        ("en", "Hungary"),
        ("eo", "Hungario"),
        ("es", "Hungría"),
        ("et", "Ungari"),
        ("eu", "Hungaria"),
        ("fa", "مجارستان"),
        ("ff", "Hunngariya"),
        ("fi", "Unkari"),
        ("fo", "Ungarn"),
        ("fr", "Hongrie"),
        ("fy", "Hongarije"),
        ("ga", "An Ungáir"),
        ("gl", "Hungría"),
        ("gn", "Hungary"),
        ("gu", "હ\u{a82}ગ\u{ac7}રી"),
        ("gv", "Yn Ungaar"),
        ("ha", "Hungariya"),
        ("he", "הונגריה"),
        ("hi", "ह\u{902}गरी"),
        ("hr", "Mađarska"),
        ("ht", "Ongri"),
        ("hu", "Magyarország"),
        ("hy", "Հունգարիա"),
        ("ia", "Hungaria"),
        ("id", "Hongaria"),
        ("io", "Hungaria"),
        ("is", "Ungverjaland"),
        ("it", "Ungheria"),
        ("iu", "Hungary"),
        ("ja", "ハンガリー"),
        ("ka", "უნგრეთი"),
        ("ki", "Macartsa"),
        ("kk", "Венгрия"),
        ("kl", "Hungary"),
        ("km", "ហ\u{17bb}ងគ\u{17d2}រ\u{17b8}"),
        ("kn", "ಹಂಗರ\u{cbf}"),
        ("ko", "헝가리"),
        ("ku", "Macaristan"),
        ("kv", "Мадьяр Му"),
        ("kw", "Hungari"),
        ("ky", "Мажарстан"),
        ("lo", "ປະເທດຮ\u{ebb}ງກະລ\u{eb5}"),
        ("lt", "Vengrija"),
        ("lv", "Ungārija"),
        ("mi", "Hanekari"),
        ("mk", "Унгарија"),
        ("ml", "ഹംഗറി"),
        ("mn", "Унгар"),
        ("mr", "ह\u{902}ग\u{947}री"),
        ("ms", "Hungari"),
        ("mt", "Ungerija"),
        (
            "my",
            "ဟန\u{103a}ဂေရ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Ungari"),
        ("nb", "Ungarn"),
        ("ne", "हङ\u{94d}ग\u{947}री"),
        ("nl", "Hongarije"),
        ("nn", "Ungarn"),
        ("nv", "Hángewii"),
        ("oc", "Ongria"),
        ("or", "ହଙ\u{b4d}ଗ\u{b3e}ରୀ"),
        ("pa", "ਹ\u{a70}ਗਰੀ"),
        ("pi", "ह\u{902}गरी"),
        ("pl", "Węgry"),
        ("ps", "مجارستان"),
        ("pt", "Hungria"),
        ("pt_BR", "Hungria"),
        ("ro", "Ungaria"),
        ("ru", "Венгрия"),
        ("rw", "Hongiriya"),
        ("sc", "Ungheria"),
        ("sd", "Hungary"),
        ("si", "හංගේර\u{dd2}ය\u{dcf}ව"),
        ("sk", "Maďarsko"),
        ("sl", "Madžarska"),
        ("so", "Hangeri"),
        ("sq", "Hungari"),
        ("sr", "Мађарска"),
        ("sv", "Ungern"),
        ("sw", "Hungary"),
        ("ta", "ஹங\u{bcd}கேரி"),
        ("te", "హంగ\u{c47}ర\u{c40}"),
        ("tg", "Маҷористон"),
        ("th", "ฮ\u{e31}งการ\u{e35}"),
        ("ti", "ሀንጋሪ"),
        ("tk", "Wengriýa"),
        ("tl", "Hungary"),
        ("tr", "Macaristan"),
        ("tt", "Маҗарстан"),
        ("ug", "ۋېنگىرىيە"),
        ("uk", "Угорщина"),
        ("ur", "مجارستان"),
        ("uz", "Mojariston"),
        ("ve", "Hungary"),
        ("vi", "Hun-ga-ri"),
        ("wa", "Hongreye"),
        ("wo", "Oonguri"),
        ("xh", "Hungary"),
        ("yo", "Húngárì"),
        ("zh_CN", "匈牙利"),
        ("zh_HK", "匈牙利"),
        ("zh_TW", "匈牙利"),
        ("zu", "I-Hungariya"),
    ];
    #[cfg(all(feature = "hu", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 47.162494;
        pub const LONGITUDE: f64 = 19.5033041;
        pub const MAX_LATITUDE: f64 = 48.585234;
        pub const MAX_LONGITUDE: f64 = 22.8965438;
        pub const MIN_LATITUDE: f64 = 45.7370889;
        pub const MIN_LONGITUDE: f64 = 16.1133078;
        pub const NORTHEAST_LATITUDE: f64 = 48.585234;
        pub const NORTHEAST_LONGITUDE: f64 = 22.8965438;
        pub const SOUTHWEST_LATITUDE: f64 = 45.7370889;
        pub const SOUTHWEST_LONGITUDE: f64 = 16.1133078;
    }
}
#[cfg(all(feature = "hu", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 47.162494,
            longitude: 19.5033041,
            max_latitude: 48.585234,
            max_longitude: 22.8965438,
            min_latitude: 45.7370889,
            min_longitude: 16.1133078,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 48.585234,
                    longitude: 22.8965438,
                },
                southwest: CountryGeoBound {
                    latitude: 45.7370889,
                    longitude: 16.1133078,
                },
            },
        }
    }
}

#[cfg(all(feature = "hu", feature = "subdivisions"))]
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
                    "BA",
                    Subdivision{
                        name: "BA",
                        country_alpha2: Alpha2::HU,
                        code: "BA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0484585), longitude: Some(18.2719173), max_latitude: Some(46.4172391), min_latitude: Some(45.7370889), max_longitude: Some(18.868302), min_longitude: Some(17.624765)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بارانيا"), ("az", "Baranya əyaləti"), ("be", "Баранья"), ("bg", "Бараня"), ("ca", "Baranya"), ("ccp", "𑄝𑄢𑄚\u{11128}𑄠"), ("ceb", "Baranya county"), ("cs", "Baranya"), ("da", "Baranya"), ("de", "Komitat Baranya"), ("en", "Baranya"), ("es", "Baranya"), ("et", "Baranya komitaat"), ("eu", "Baranya"), ("fa", "بارانیا"), ("fi", "Baranya"), ("fr", "Baranya"), ("gl", "Condado de Baranya"), ("he", "באראניה"), ("hi", "बरानिया काउ\u{902}टी"), ("hr", "Baranjska županija"), ("hu", "Baranya megye"), ("id", "Baranya"), ("it", "Provincia di Baranya"), ("ja", "バラニャ県"), ("ka", "ბარანიის მედიე"), ("ko", "버러녀 주"), ("lt", "Barania"), ("lv", "Baraņas meģe"), ("mk", "Барања"), ("ms", "Baranya"), ("nl", "Baranya"), ("pl", "Komitat Baranya"), ("pt", "Baranya"), ("ro", "Comitatul Baranya"), ("ru", "Баранья"), ("sk", "Baranská župa"), ("sl", "Županija Baranja"), ("sr", "Барања"), ("sr_Latn", "Baranja"), ("sv", "Baranya"), ("sw", "Baranya"), ("tr", "Baranya ili"), ("uk", "Бараня"), ("ur", "بارانیا کاؤنٹی"), ("zh", "巴蘭尼亞州")]),
                        unofficial_name_list: ["Baranya"].to_vec(),
                    }
                ),
                (
                    "BC",
                    Subdivision{
                        name: "BC",
                        country_alpha2: Alpha2::HU,
                        code: "BC",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.6735939), longitude: Some(21.0877309), max_latitude: Some(46.7476239), min_latitude: Some(46.61125999999999), max_longitude: Some(21.2357529), min_longitude: Some(20.9300351)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "بيكيسكسابا"), ("az", "Bekeşçaba"), ("be", "Бекешчаба"), ("bg", "Бекешчаба"), ("bn", "বেকেসস\u{9be}ব\u{9be}"), ("ca", "Békéscsaba"), ("ccp", "𑄝𑄇𑄬𑄌\u{11134}𑄥𑄝"), ("ceb", "Békéscsaba"), ("cs", "Békéscsaba"), ("da", "Békéscsaba"), ("de", "Békéscsaba"), ("el", "Μπεκεστσάμπα"), ("en", "Békéscsaba"), ("es", "Békéscsaba"), ("et", "Békéscsaba"), ("eu", "Békéscsaba"), ("fa", "بیکیشچابا"), ("fi", "Békéscsaba"), ("fr", "Békéscsaba"), ("gl", "Békéscsaba"), ("gu", "બ\u{ac7}ક\u{ac7}સસાબા"), ("he", "בקשצ׳אבה"), ("hi", "ब\u{947}क\u{94d}सस\u{94d}काबा"), ("hr", "Békéscsaba"), ("hu", "Békéscsaba"), ("hy", "Բեկեշչաբա"), ("id", "Békéscsaba"), ("it", "Békéscsaba"), ("ja", "ベーケーシュチャバ"), ("ka", "ბეკეშჩაბა"), ("kn", "ಬ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}ಸ\u{ccd}ಕಾಬ"), ("ko", "베케슈처버"), ("lt", "Bekeščaba"), ("lv", "Bēkēščaba"), ("mk", "Бекешчаба"), ("mr", "ब\u{947}कस\u{94d}साबा"), ("ms", "Békéscsaba"), ("nb", "Békéscsaba"), ("nl", "Békéscsaba"), ("no", "Békéscsaba"), ("pl", "Békéscsaba"), ("pt", "Békéscsaba"), ("ro", "Bichișciaba"), ("ru", "Бекешчаба"), ("si", "බෙකෙසබ\u{dcf}"), ("sk", "Békešská Čaba"), ("sl", "Békéscsaba"), ("sr", "Бекешчаба"), ("sr_Latn", "Bekeščaba"), ("sv", "Békéscsaba"), ("sw", "Békéscsaba"), ("ta", "பேகஸ\u{bcd}ஸப\u{bbe}"), ("te", "బ\u{c46}క\u{c47}స\u{c4d}క\u{c3e}బ\u{c3e}"), ("th", "เบเก\u{e49}ซ\u{e31}คซาบา"), ("tr", "Békéscsaba"), ("uk", "Бекешчаба"), ("ur", "بیکیسکسابا"), ("vi", "Békéscsaba"), ("zh", "貝凱什喬包")]),
                        unofficial_name_list: ["Békéscsaba"].to_vec(),
                    }
                ),
                (
                    "BE",
                    Subdivision{
                        name: "BE",
                        country_alpha2: Alpha2::HU,
                        code: "BE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.7711185), longitude: Some(21.1289753), max_latitude: Some(46.845706), min_latitude: Some(46.715268), max_longitude: Some(21.2195859), min_longitude: Some(21.04361)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بكيش"), ("be", "медзье Бекеш"), ("bg", "Бекеш"), ("bn", "বেকস ক\u{9be}উন\u{9cd}টি"), ("ca", "Békés"), ("ccp", "𑄝𑄇𑄬𑄌\u{11134}"), ("ceb", "Bekes County"), ("cs", "Békés"), ("da", "Békés"), ("de", "Komitat Békés"), ("el", "Μπεκές"), ("en", "Békés"), ("es", "Békés"), ("et", "Békési komitaat"), ("eu", "Békés"), ("fa", "بیکیس"), ("fi", "Békés"), ("fr", "Békés"), ("gl", "Condado de Békés"), ("gu", "બ\u{ac7}ક\u{ac7}ઝ કાઉન\u{acd}ટી"), ("he", "בקש"), ("hi", "ब\u{947}कज\u{93c} काउ\u{902}टी"), ("hr", "Bekeška županija"), ("hu", "Békés megye"), ("id", "Békés"), ("it", "Provincia di Békés"), ("ja", "ベーケーシュ県"), ("ka", "ბეკეშის მედიე"), ("kn", "ಬ\u{cc6}ಕ\u{cc6}ಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "베케시 주"), ("lt", "Bėkėšas"), ("lv", "Bēkēšas meģe"), ("mk", "Бекеш"), ("mr", "ब\u{947}किस काउ\u{902}टी"), ("ms", "Békés"), ("nb", "Bekes Fylke"), ("nl", "Békés"), ("no", "Bekes Fylke"), ("pl", "Komitat Békés"), ("pt", "Békés"), ("ro", "Județul Békés"), ("ru", "Бекеш"), ("si", "බෙකෙස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Békešská župa"), ("sl", "Békés"), ("sr", "Бекеш"), ("sr_Latn", "Bekeš"), ("sv", "Békés"), ("sw", "Békés"), ("ta", "பேக\u{bcd}ஸ\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c46}క\u{c46}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลเบเกช"), ("tr", "Békés ili"), ("uk", "Бекеш"), ("ur", "بیکیش کاؤنٹی"), ("vi", "Hạt Békés"), ("zh", "貝凱什州")]),
                        unofficial_name_list: ["Békés"].to_vec(),
                    }
                ),
                (
                    "BK",
                    Subdivision{
                        name: "BK",
                        country_alpha2: Alpha2::HU,
                        code: "BK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.5661437), longitude: Some(19.4272464), max_latitude: Some(47.134105), min_latitude: Some(45.904761), max_longitude: Some(20.1599619), min_longitude: Some(18.7327931)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة باتش-كيشكون"), ("az", "Baç-Kişkun (medye)"), ("be", "Бач-Кішкун"), ("bg", "Бач-Кишкун"), ("bn", "ব\u{9be}কস-কিস\u{9cd}ক\u{9c1}ন ক\u{9be}উন\u{9cd}টি"), ("ca", "Bács-Kiskun"), ("ccp", "𑄝\u{11133}𑄠𑄇\u{11134}𑄥\u{11134}-𑄇\u{11128}𑄌\u{11134}𑄇\u{1112a}𑄚\u{11134}"), ("ceb", "Bács-Kiskun county"), ("cs", "Bács-Kiskun"), ("da", "Bács-Kiskun"), ("de", "Komitat Bács-Kiskun"), ("el", "Μπακς-Κισκούν"), ("en", "Bács-Kiskun"), ("es", "Bács-Kiskun"), ("et", "Bács-Kiskuni komitaat"), ("eu", "Bács-Kiskun"), ("fa", "باتس-کیسکون"), ("fi", "Bács-Kiskun"), ("fr", "Bács-Kiskun"), ("gl", "Bács-Kiskun"), ("gu", "બ\u{ac5}ક\u{acd}સ-કિસ\u{acd}ક\u{ac1}ન કાઉન\u{acd}ટી"), ("he", "באץ׳-קישקון"), ("hi", "ब\u{947}क\u{94d}स-किस\u{94d}क\u{941}न काउ\u{902}टी"), ("hr", "Bačko-kiškunska županija"), ("hu", "Bács-Kiskun megye"), ("id", "Bács-Kiskun"), ("it", "Provincia di Bács-Kiskun"), ("ja", "バーチ・キシュクン県"), ("ka", "ბაჩ-კიშკუნის მედიე"), ("kn", "ಬ\u{ccd}ಯಾಕ\u{ccd}ಸ\u{ccd}-ಕ\u{cbf}ಸ\u{ccd}ಕುನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "바치키슈쿤 주"), ("lt", "Bač Kiškūnas"), ("lv", "Bāčas-Kiškunas meģe"), ("mk", "Бач-Кишкун"), ("mr", "ब\u{947}क\u{94d}स-किस\u{94d}क\u{941}न काउ\u{902}टी"), ("ms", "Bács-Kiskun"), ("nb", "Bacs-Kiskun Fylke"), ("nl", "Bács-Kiskun"), ("no", "Bacs-Kiskun Fylke"), ("pl", "Komitat Bács-Kiskun"), ("pt", "Bács-Kiskun"), ("ro", "Județul Bács-Kiskun"), ("ru", "Бач-Кишкун"), ("si", "බ\u{dcf}ක\u{dca}ස\u{dca}-ක\u{dd2}ස\u{dca}කන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Báčsko-malokumánska župa"), ("sl", "Bács-Kiskun"), ("sr", "Бач-Кишкун"), ("sr_Latn", "Bač-Kiškun"), ("sv", "Bács-Kiskun"), ("sw", "Bács-Kiskun"), ("ta", "ப\u{bbe}க\u{bcd}ஸ\u{bcd} -கிஸ\u{bcd}க\u{bcd}கும\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c3e}క\u{c4d}స\u{c4d}-క\u{c3f}స\u{c4d}కున\u{c4d} క\u{c4c}\u{c46}ంట\u{c40}"), ("th", "บาช-ค\u{e34}ชค\u{e38}น"), ("tr", "Bács-Kiskun ili"), ("uk", "Бач-Кішкун"), ("ur", "باتش-کیشکون کاؤنٹی"), ("vi", "Hạt Bács-Kiskun"), ("zh", "巴奇-基什孔州")]),
                        unofficial_name_list: ["Bács-Kiskun"].to_vec(),
                    }
                ),
                (
                    "BU",
                    Subdivision{
                        name: "BU",
                        country_alpha2: Alpha2::HU,
                        code: "BU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.497912), longitude: Some(19.040235), max_latitude: Some(47.6130119), min_latitude: Some(47.349415), max_longitude: Some(19.334505), min_longitude: Some(18.9261011)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CapitalCity,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Boedapest"), ("am", "ቡዳፔስት"), ("ar", "بودابست"), ("az", "Budapeşt"), ("be", "Будапешт"), ("bg", "Будапеща"), ("bn", "ব\u{9c1}দ\u{9be}পেস\u{9cd}ট"), ("bs", "Budimpešta"), ("ca", "Budapest"), ("ccp", "𑄝\u{1112a}𑄘𑄛𑄬𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Budapest"), ("cs", "Budapešť"), ("cy", "Budapest"), ("da", "Budapest"), ("de", "Budapest"), ("el", "Βουδαπέστη"), ("en", "Budapest"), ("es", "Budapest"), ("et", "Budapest"), ("eu", "Budapest"), ("fa", "بوداپست"), ("fi", "Budapest"), ("fr", "Budapest"), ("ga", "Búdaipeist"), ("gl", "Budapest"), ("gu", "બ\u{ac1}ડાપ\u{ac7}સ\u{acd}ટ"), ("he", "בודפשט"), ("hi", "ब\u{941}डाप\u{947}स\u{94d}ट"), ("hr", "Budimpešta"), ("hu", "Budapest"), ("hy", "Բուդապեշտ"), ("id", "Budapest"), ("is", "Búdapest"), ("it", "Budapest"), ("ja", "ブダペスト"), ("jv", "Budapest"), ("ka", "ბუდაპეშტი"), ("kk", "Будапешт"), ("kn", "ಬುಡಾಪ\u{cc6}ಸ\u{ccd}ಟ\u{ccd}"), ("ko", "부다페스트"), ("ky", "Будапешт"), ("lt", "Budapeštas"), ("lv", "Budapešta"), ("mk", "Будимпешта"), ("ml", "ബ\u{d41}ഡ\u{d3e}പെസ\u{d4d}റ\u{d4d}റ\u{d4d}"), ("mn", "Будапешт"), ("mr", "ब\u{941}डाप\u{947}स\u{94d}ट"), ("ms", "Budapest"), ("my", "ဗ\u{1030}းဒပက\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Budapest"), ("ne", "ब\u{941}डाप\u{947}स\u{94d}ट"), ("nl", "Boedapest"), ("no", "Budapest"), ("or", "ବ\u{b41}ଦ\u{b3e}ପେଷ\u{b4d}ଟ"), ("pa", "ਬ\u{a41}ਦਾਪ\u{a48}ਸਤ"), ("pl", "Budapeszt"), ("ps", "بوډاپسټ"), ("pt", "Budapeste"), ("ro", "Budapesta"), ("ru", "Будапешт"), ("si", "බ\u{dd4}ඩ\u{dcf}පෙස\u{dca}ට\u{dca}"), ("sk", "Budapešť"), ("sl", "Budimpešta"), ("so", "Budapest"), ("sq", "Budapesti"), ("sr", "Будимпешта"), ("sr_Latn", "Budimpešta"), ("sv", "Budapest"), ("sw", "Budapest"), ("ta", "புட\u{bbe}பெஸ\u{bcd}ட\u{bcd}"), ("te", "బుడ\u{c3e}ప\u{c46}స\u{c4d}ట\u{c4d}"), ("th", "บ\u{e39}ดาเปสต\u{e4c}"), ("tk", "Budapeşt"), ("tr", "Budapeşte"), ("uk", "Будапешт"), ("ur", "بوداپست"), ("uz", "Budapesht"), ("vi", "Budapest"), ("yo", "Budapest"), ("yo_BJ", "Budapest"), ("yue", "布達佩斯"), ("yue_Hans", "布达佩斯"), ("zh", "布达佩斯")]),
                        unofficial_name_list: ["Budapest"].to_vec(),
                    }
                ),
                (
                    "BZ",
                    Subdivision{
                        name: "BZ",
                        country_alpha2: Alpha2::HU,
                        code: "BZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.2939401), longitude: Some(20.6934113), max_latitude: Some(48.585234), min_latitude: Some(47.64265), max_longitude: Some(22.1292827), min_longitude: Some(20.0518941)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بورسود-آبائوي-زمبلن"), ("az", "Borşod-Abauy-Zemplen"), ("be", "Боршад-Абауй-Земплен"), ("bg", "Боршод-Абауй-Земплен"), ("bn", "বোরোসদ-আব\u{9be}উদ-জেমপ\u{9cd}লেন ক\u{9be}উন\u{9cd}টি"), ("ca", "Borsod-Abaúj-Zemplén"), ("ccp", "𑄝\u{1112e}𑄢\u{11134}𑄥\u{1112e}𑄖\u{11134}-𑄃𑄝𑄅\u{1112a}𑄌\u{11134}-𑄎𑄬𑄟\u{11134}𑄛\u{11133}𑄣𑄬𑄚\u{11134}"), ("ceb", "Borsod-Abauj Zemplen county"), ("cs", "Borsod-Abaúj-Zemplén"), ("da", "Borsod-Abaúj-Zemplén"), ("de", "Komitat Borsod-Abaúj-Zemplén"), ("el", "Μπόρσοντ-Αμπαούτζ-Ζεμπλέν"), ("en", "Borsod-Abaúj-Zemplén"), ("es", "Borsod-Abaúj-Zemplén"), ("et", "Borsod-Abaúj-Zempléni komitaat"), ("eu", "Borsod-Abaúj-Zemplén"), ("fa", "بورسود-اباوج-زیمپلن"), ("fi", "Borsod-Abaúj-Zemplén"), ("fr", "Borsod-Abaúj-Zemplén"), ("gl", "Borsod-Abaúj-Zemplén"), ("gu", "બૉર\u{acd}સોડ-અબૌજ-ઝ\u{ac7}મપ\u{acd}લ\u{ac7}ન કાઉન\u{acd}ટી"), ("he", "בורשוד-אבאוי-זמפלן"), ("hi", "बोर\u{94d}सोद-अबौज-ज\u{93c}\u{947}म\u{94d}प\u{94d}ल\u{947}न काउ\u{902}टी"), ("hr", "Boršod-abaújsko-zemplénska županija"), ("hu", "Borsod-Abaúj-Zemplén megye"), ("id", "Borsod-Abaúj-Zemplén"), ("it", "Provincia di Borsod-Abaúj-Zemplén"), ("ja", "ボルショド・アバウーイ・ゼンプレーン県"), ("ka", "ბორშოდ-აბაუი-ზემგლენის მედიე"), ("kn", "ಬೋರ\u{ccd}ಸೋಡ\u{ccd}-ಅಬ\u{ccc}ಜ\u{ccd}-ಝ\u{cc6}ಂಪ\u{ccd}ಲ\u{cc6}ನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "보르쇼드어버우이젬플렌 주"), ("lt", "Boršodas-Abaujus-Zemplėnas"), ("lv", "Boršodas-Abaūjas-Zemplēna meģe"), ("mk", "Боршод-Абауј-Земплен"), ("mr", "बोर\u{94d}सोड-अब\u{941}ज-झ\u{947}\u{902}प\u{94d}ल\u{947}न काउ\u{902}टी"), ("ms", "Borsod-Abaúj-Zemplén"), ("nb", "Borsod Abauj Zemplen Fylke"), ("nl", "Borsod-Abaúj-Zemplén"), ("no", "Borsod Abauj Zemplen Fylke"), ("pl", "Komitat Borsod-Abaúj-Zemplén"), ("pt", "Borsod-Abaúj-Zemplén"), ("ro", "Județul Borsod-Abaúj-Zemplén"), ("ru", "Боршод-Абауй-Земплен"), ("si", "බොර\u{dca}සොඩ\u{dca}-අබෞජ\u{dca}-සෙම\u{dca}ප\u{dca}ලේන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Boršodsko-abovsko-zemplínska župa"), ("sl", "Borsod-Abaúj-Zemplén"), ("sr", "Боршод-Абауј-Земплен"), ("sr_Latn", "Boršod-Abauj-Zemplen"), ("sv", "Borsod-Abaúj-Zemplén"), ("sw", "Borsod-Abaúj-Zemplén"), ("ta", "ப\u{bbe}ற\u{bcd}சோட\u{bcd}-அப\u{bbe}உஜ\u{bcd}-ஸிம\u{bcd}ப\u{bcd}ளென\u{bcd} கவுண\u{bcd}டி"), ("te", "బ\u{c4b}ర\u{c4d}స\u{c4b}ద\u{c4d}-అబ\u{c3e}వుజ\u{c4d}-జ\u{c46}ంప\u{c4d}ల\u{c3f}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลโบร\u{e4c}โชด-ออบออ\u{e39}ย-แซ\u{e47}มเปลน"), ("tr", "Borsod-Abaúj-Zemplén ili"), ("uk", "Боршод-Абауй-Земплен"), ("ur", "بورشود-اباوی-زیمپلین کاؤنٹی"), ("vi", "Hạt Borsod-Abaúj-Zemplén"), ("zh", "包爾紹德-奧包烏伊-曾普倫州")]),
                        unofficial_name_list: ["Borsod-Abaúj-Zemplén"].to_vec(),
                    }
                ),
                (
                    "CS",
                    Subdivision{
                        name: "CS",
                        country_alpha2: Alpha2::HU,
                        code: "CS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.7084264), longitude: Some(20.1436061), max_latitude: Some(46.8063689), min_latitude: Some(46.649019), max_longitude: Some(20.221518), min_longitude: Some(19.969219)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تشونغراد"), ("be", "Чанград"), ("bg", "Чонград"), ("bn", "সনগ\u{9cd}র\u{9be}ড ক\u{9be}উন\u{9cd}টি"), ("ca", "Csongrád"), ("ccp", "𑄥\u{11127}\u{11101}𑄉\u{11133}𑄢𑄖\u{11134}"), ("cs", "Csongrád"), ("da", "Csongrád"), ("de", "Komitat Csongrád"), ("el", "Κσονγκράντ"), ("en", "Csongrád"), ("es", "Csongrád"), ("et", "Csongrádi komitaat"), ("eu", "Csongrád"), ("fa", "سونگراد"), ("fi", "Csongrád"), ("fr", "Csongrád"), ("gl", "Condado de Csongrád"), ("gu", "સિસો\u{a82}ગ\u{acd}ર\u{ac7}ડ કાઉન\u{acd}ટી"), ("he", "מחוז צ׳ונגרד"), ("hi", "सिसो\u{902}ग\u{94d}रा\u{902}ड काउ\u{902}टी"), ("hr", "Čongradska županija"), ("hu", "Csongrád megye"), ("id", "Csongrád"), ("it", "Provincia di Csongrád"), ("ja", "チョングラード県"), ("ka", "ჩონგრადის მედიე"), ("kn", "ಸ\u{cbf}ಂಗ\u{ccd}ರಾಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "촌그라드 주"), ("lt", "Čongradas"), ("lv", "Čongrādas meģe"), ("mk", "Чонград"), ("mr", "को\u{902}गार\u{94d}ड काउ\u{902}टी"), ("ms", "Csongrád"), ("nb", "Csongrád (fylke)"), ("nl", "Csongrád"), ("no", "Csongrád (fylke)"), ("pl", "Komitat Csongrád"), ("pt", "Csongrád"), ("ro", "Csongrád"), ("ru", "Чонград"), ("si", "ක\u{dca}සොන\u{dca}ග\u{dca}රඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Čongrádska župa"), ("sl", "Csongrád"), ("sr", "Чонград"), ("sr_Latn", "Čongrad"), ("sv", "Csongrád"), ("sw", "Csongrád"), ("ta", "கிசோங\u{bcd}கிறட\u{bcd} கவுண\u{bcd}டி"), ("te", "కస\u{c4b}న\u{c4d}\u{200c}గ\u{c4d}ర\u{c3e}డ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เม\u{e37}องคสอนกราด"), ("tr", "Csongrád ili"), ("uk", "Чонград"), ("ur", "چونگراد کاؤنٹی"), ("vi", "Hạt Csongrád"), ("zh", "瓊格拉德州")]),
                        unofficial_name_list: ["Csongrád"].to_vec(),
                    }
                ),
                (
                    "DE",
                    Subdivision{
                        name: "DE",
                        country_alpha2: Alpha2::HU,
                        code: "DE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.5316049), longitude: Some(21.6273123), max_latitude: Some(47.6369639), min_latitude: Some(47.4248649), max_longitude: Some(21.8837061), min_longitude: Some(21.4366579)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Debrecen"), ("ar", "دبرتسن"), ("az", "Debrecen"), ("be", "Горад Дэбрэцэн"), ("bg", "Дебрецен"), ("bn", "দেব\u{9cd}রেচেন"), ("bs", "Debrecen"), ("ca", "Debrecen"), ("ccp", "𑄓𑄝\u{11133}𑄢𑄬𑄥𑄬𑄚\u{11134}"), ("ceb", "Debrecen"), ("cs", "Debrecín"), ("cy", "Debrecen"), ("da", "Debrecen"), ("de", "Debrecen"), ("el", "Ντέμπρετσεν"), ("en", "Debrecen"), ("es", "Debrecen"), ("et", "Debrecen"), ("eu", "Debrecen"), ("fa", "دبرسن"), ("fi", "Debrecen"), ("fr", "Debrecen"), ("gl", "Debrecen"), ("gu", "ડ\u{ac7}બ\u{acd}ર\u{ac7}સ\u{ac7}ન"), ("he", "דברצן"), ("hi", "ड\u{947}ब\u{94d}र\u{947}स\u{947}न"), ("hr", "Debrecin"), ("hu", "Debrecen"), ("hy", "Դեբրեցեն"), ("id", "Debrecen"), ("is", "Debrecen"), ("it", "Debrecen"), ("ja", "デブレツェン"), ("ka", "დებრეცენი"), ("kk", "Дебрецен"), ("kn", "ದೇಬ\u{ccd}ರ\u{cc6}ಸ\u{cc6}ನ\u{ccd}"), ("ko", "데브레첸"), ("ky", "Дебрецен"), ("lt", "Debrecenas"), ("lv", "Debrecena"), ("mk", "Дебрецин"), ("mn", "Дебрецен"), ("mr", "ड\u{947}ब\u{94d}र\u{947}स\u{947}न"), ("ms", "Debrecen"), ("nb", "Debrecen"), ("nl", "Debrecen"), ("no", "Debrecen"), ("pl", "Debreczyn"), ("pt", "Debrecen"), ("ro", "Debrețin"), ("ru", "Дебрецен"), ("si", "ඩෙබ\u{dca}\u{200d}ර\u{dd2}සෙන\u{dca}"), ("sk", "Debrecín"), ("sl", "Debrecen"), ("sr", "Дебрецин"), ("sr_Latn", "Debrecin"), ("sv", "Debrecen"), ("sw", "Debrecen"), ("ta", "டெப\u{bcd}ரெசென\u{bcd}"), ("te", "డ\u{c46}బ\u{c4d}ర\u{c46}స\u{c46}న\u{c4d}"), ("th", "แดแบร\u{e47}ตแซ\u{e47}น"), ("tr", "Debrecen"), ("uk", "Дебрецен"), ("ur", "دیبریکین"), ("uz", "Debretsen"), ("vi", "Debrecen"), ("yue", "德布勒森"), ("yue_Hans", "德布勒森"), ("zh", "德布勒森")]),
                        unofficial_name_list: ["Debrecen"].to_vec(),
                    }
                ),
                (
                    "DU",
                    Subdivision{
                        name: "DU",
                        country_alpha2: Alpha2::HU,
                        code: "DU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.9619059), longitude: Some(18.9355227), max_latitude: Some(47.001966), min_latitude: Some(46.8897881), max_longitude: Some(18.9657249), min_longitude: Some(18.860456)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Dunauyvaroş"), ("bg", "Дунауйварош"), ("ca", "Dunaújváros"), ("ccp", "𑄓\u{1112a}𑄚\u{11127}𑄅\u{1112a}𑄌\u{11134}𑄞𑄢\u{1112e}𑄌\u{11134}"), ("ceb", "Dunaújváros"), ("cs", "Dunaújváros"), ("da", "Dunaújváros"), ("de", "Dunaújváros"), ("en", "Dunaújváros"), ("es", "Dunaújváros"), ("et", "Dunaújváros"), ("fa", "دونائیوروش"), ("fi", "Dunaújváros"), ("fr", "Dunaújváros"), ("gl", "Dunaújváros"), ("he", "דונאויווארוש"), ("hr", "Dunaújváros"), ("hu", "Dunaújváros"), ("hy", "Դունաույվարոշ"), ("id", "Dunaújváros"), ("it", "Dunaújváros"), ("ja", "ドゥナウーイヴァーロシュ"), ("ka", "დუნაუივაროში"), ("ko", "두너우이바로시"), ("ky", "Дунауйварош"), ("lt", "Dunauivarošas"), ("mk", "Дунаујварош"), ("ms", "Dunaújváros"), ("nl", "Dunaújváros"), ("pl", "Dunaújváros"), ("pt", "Dunaújváros"), ("ro", "Dunaújváros"), ("ru", "Дунауйварош"), ("sk", "Dunaújváros"), ("sl", "Dunaújváros"), ("sr", "Дунаујварош"), ("sr_Latn", "Dunaujvaroš"), ("sv", "Dunaújváros"), ("tr", "Dunaújváros"), ("uk", "Дунауйварош"), ("vi", "Dunaújváros"), ("zh", "多瑙新城")]),
                        unofficial_name_list: ["Dunaújváros"].to_vec(),
                    }
                ),
                (
                    "EG",
                    Subdivision{
                        name: "EG",
                        country_alpha2: Alpha2::HU,
                        code: "EG",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.9025348), longitude: Some(20.3772284), max_latitude: Some(48.001331), min_latitude: Some(47.857033), max_longitude: Some(20.471507), min_longitude: Some(20.3113128)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Eqer"), ("be", "Горад Эгер"), ("bg", "Егер"), ("ca", "Eger"), ("ccp", "𑄃𑄉𑄢\u{11134}"), ("ceb", "Eger (kapital sa lalawigan)"), ("cs", "Eger"), ("cy", "Eger"), ("da", "Eger"), ("de", "Eger"), ("el", "Έγκερ"), ("en", "Eger"), ("es", "Eger"), ("et", "Eger"), ("eu", "Eger"), ("fa", "اگر"), ("fi", "Eger"), ("fr", "Eger"), ("gl", "Eger"), ("he", "אגר"), ("hr", "Eger"), ("hu", "Eger"), ("hy", "Էգեր"), ("id", "Eger"), ("it", "Eger"), ("ja", "エゲル"), ("ka", "ეგერი"), ("ko", "에게르"), ("lt", "Egeris"), ("lv", "Egera"), ("mk", "Егер"), ("ms", "Eger"), ("nb", "Eger"), ("nl", "Eger"), ("no", "Eger"), ("pl", "Eger"), ("pt", "Eger"), ("ro", "Eger"), ("ru", "Эгер"), ("sk", "Jáger"), ("sl", "Eger"), ("sr", "Јегра"), ("sr_Latn", "Jegra"), ("sv", "Eger"), ("sw", "Eger"), ("tr", "Eger"), ("uk", "Еґер"), ("vi", "Eger"), ("zh", "埃格爾")]),
                        unofficial_name_list: ["Eger"].to_vec(),
                    }
                ),
                (
                    "ER",
                    Subdivision{
                        name: "ER",
                        country_alpha2: Alpha2::HU,
                        code: "ER",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.3919718), longitude: Some(18.904544), max_latitude: Some(47.430192), min_latitude: Some(47.314573), max_longitude: Some(18.96238), min_longitude: Some(18.833885)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Erd"), ("be", "Горад Эрд"), ("bg", "Ерд"), ("ca", "Érd"), ("ccp", "𑄃\u{11128}𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Érd"), ("cs", "Érd"), ("da", "Érd"), ("de", "Érd"), ("en", "Érd"), ("es", "Érd"), ("et", "Érd"), ("eu", "Érd"), ("fa", "ایرد"), ("fr", "Érd"), ("gl", "Érd"), ("he", "ארד"), ("hr", "Andzabeg"), ("hu", "Érd"), ("it", "Érd"), ("ja", "エールド"), ("ka", "ერდი"), ("ko", "에르드"), ("lt", "Erdris"), ("mk", "Ерд"), ("ms", "Érd"), ("nl", "Érd"), ("pl", "Érd"), ("pt", "Érd"), ("ro", "Érd"), ("ru", "Эрд"), ("sk", "Érd"), ("sl", "Érd"), ("sr", "Ерд"), ("sr_Latn", "Erd"), ("sv", "Érd"), ("tr", "Érd"), ("uk", "Ерд"), ("vi", "Érd"), ("zh", "埃爾德")]),
                        unofficial_name_list: ["Érd"].to_vec(),
                    }
                ),
                (
                    "FE",
                    Subdivision{
                        name: "FE",
                        country_alpha2: Alpha2::HU,
                        code: "FE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.1217932), longitude: Some(18.5294815), max_latitude: Some(47.57704589999999), min_latitude: Some(46.687195), max_longitude: Some(18.967485), min_longitude: Some(18.0337199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فيير"), ("be", "Феер"), ("bg", "Фейер"), ("bn", "ফেযে\u{981}র ক\u{9be}উন\u{9cd}টি"), ("ca", "Fejér"), ("ccp", "𑄜𑄬𑄎𑄢\u{11134}"), ("ceb", "Fejér megye"), ("cs", "Fejér"), ("da", "Fejér"), ("de", "Komitat Fejér"), ("el", "Επαρχία Φετζέρ"), ("en", "Fejér"), ("es", "Fejér"), ("et", "Fejéri komitaat"), ("eu", "Fejér"), ("fa", "فیجر"), ("fi", "Fejér"), ("fr", "Fejér"), ("gl", "Fejér"), ("gu", "ફ\u{ac7}જ\u{ac7}ર કાઉન\u{acd}ટી"), ("he", "מחוז פייר"), ("hi", "फ\u{947}य\u{947}र काउ\u{902}टी"), ("hr", "Bila županija"), ("hu", "Fejér megye"), ("hy", "Ֆեյեր"), ("id", "Fejér"), ("it", "Provincia di Fejér"), ("ja", "フェイェール県"), ("ka", "ფეიერის მედიე"), ("kn", "ಫ\u{cc6}ಜರ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "페예르 주"), ("lt", "Fejėras"), ("lv", "Fejēras meģe"), ("mk", "Фејер"), ("mn", "Фейер"), ("mr", "फ\u{947}जर काउ\u{902}टी"), ("ms", "Fejér"), ("nb", "Fejer fylke"), ("nl", "Fejér"), ("no", "Fejer fylke"), ("pl", "Komitat Fejér"), ("pt", "Fejér"), ("ro", "Județul Fejér"), ("ru", "Фейер"), ("si", "ෆෙජෙර\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Stoličnobelehradská župa"), ("sl", "Fejér"), ("sr", "Фејер"), ("sr_Latn", "Fejer"), ("sv", "Fejér"), ("sw", "Fejér"), ("ta", "பஜ\u{bcd}ர\u{bcd} கவுண\u{bcd}டி"), ("te", "ఫ\u{c46}జ\u{c46}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลเฟเจอร\u{e4c}"), ("tr", "Fejér ili"), ("uk", "Феєр"), ("ur", "فیئیر کاؤنٹی"), ("vi", "Hạt Fejér"), ("zh", "費耶爾州")]),
                        unofficial_name_list: ["Fejér"].to_vec(),
                    }
                ),
                (
                    "GS",
                    Subdivision{
                        name: "GS",
                        country_alpha2: Alpha2::HU,
                        code: "GS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.6509285), longitude: Some(17.2505883), max_latitude: Some(48.022446), min_latitude: Some(47.272214), max_longitude: Some(17.9364819), min_longitude: Some(16.4215489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ديور-موشون-سوبرون"), ("az", "Dyör-Moşon-Şopron"), ("be", "Дзьёр-Мошан-Шопран"), ("bg", "Дьор-Мошон-Шопрон"), ("bn", "গিওর-মোসন-সোপ\u{9cd}রন ক\u{9be}উন\u{9cd}টি"), ("ca", "Gyõr-Moson-Sopron"), ("ccp", "𑄉\u{1112d}𑄠\u{1112e}𑄢\u{11134}-𑄟\u{1112e}𑄥\u{11127}𑄚\u{11134}-𑄥\u{1112e}𑄛\u{11133}𑄢\u{11127}𑄚\u{11134}"), ("ceb", "Győr-Moson-Sopron megye"), ("cs", "Győr-Moson-Sopron"), ("da", "Győr-Moson-Sopron"), ("de", "Komitat Győr-Moson-Sopron"), ("el", "Γκιόρ-Μοσόν-Σοπρόν"), ("en", "Győr-Moson-Sopron"), ("es", "Győr-Moson-Sopron"), ("et", "Győr-Moson-Soproni komitaat"), ("eu", "Győr-Moson-Sopron"), ("fa", "گیور-موسون-سوپرون"), ("fi", "Győr-Moson-Sopron"), ("fr", "Győr-Moson-Sopron"), ("gl", "Győr-Moson-Sopron"), ("gu", "ગ\u{acd}યોર-મોસૉન-સોપ\u{acd}રોન કાઉન\u{acd}ટી"), ("he", "גיור-מושון-שופרון"), ("hi", "ग\u{94d}योर-मोसोन-सोपरोन काउ\u{902}टी"), ("hr", "Đursko-mošonjsko-šopronska županija"), ("hu", "Győr-Moson-Sopron megye"), ("id", "Győr-Moson-Sopron"), ("it", "Provincia di Győr-Moson-Sopron"), ("ja", "ジェール・モション・ショプロン県"), ("ka", "დიორ-მოშონ-შოპრონის მედიე"), ("kn", "ಗ\u{ccd}ಯೋರ\u{ccd}-ಮೋಸನ\u{ccd}-ಸೋಪ\u{ccd}ರಾನ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "죄르모숀쇼프론 주"), ("lt", "Dėras-Mošonas-Šopronas"), ("lv", "Ģēras-Mošonas-Šopronas meģe"), ("mk", "Ѓер-Мошон-Шопрон"), ("mn", "Дьёр-Мошон-Шопрон"), ("mr", "ग\u{94d}योर-मोसोन-सोप\u{94d}रोन काउ\u{902}टी"), ("ms", "Győr-Moson-Sopron"), ("nb", "Gyor Moson Sopron Fylke"), ("nl", "Győr-Moson-Sopron"), ("no", "Gyor Moson Sopron Fylke"), ("pl", "Komitat Győr-Moson-Sopron"), ("pt", "Győr-Moson-Sopron"), ("ro", "Județul Győr-Moson-Sopron"), ("ru", "Дьёр-Мошон-Шопрон"), ("si", "ග\u{dca}යෝර\u{dca}-මොසෝන\u{dca}-සොප\u{dca}රෝන\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Rábsko-mošonsko-šopronská župa"), ("sl", "Győr-Moson-Sopron"), ("sr", "Ђер-Мошон-Шопрон"), ("sr_Latn", "Đer-Mošon-Šopron"), ("sv", "Győr-Moson-Sopron"), ("sw", "Győr-Moson-Sopron"), ("ta", "கியோர\u{bcd} -மோச\u{bbe}ன\u{bcd}-சொப\u{bcd}ரோன\u{bcd} கவுண\u{bcd}டி"), ("te", "గ\u{c4d}య\u{c4b}ర\u{c4d}-మ\u{c4b}సన\u{c4d}-స\u{c4b}ప\u{c4d}ర\u{c4b}న\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลเยอร\u{e4c}-โมโชน-โชโปรน"), ("tr", "Győr-Moson-Sopron ili"), ("uk", "Дьйор-Мошон-Шопрон"), ("ur", "جیور-موشون-شوپرون کاؤنٹی"), ("vi", "Hạt Gyor-Moson-Sopron"), ("zh", "傑爾-莫雄-肖普朗州")]),
                        unofficial_name_list: ["Győr-Moson-Sopron"].to_vec(),
                    }
                ),
                (
                    "GY",
                    Subdivision{
                        name: "GY",
                        country_alpha2: Alpha2::HU,
                        code: "GY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.6874569), longitude: Some(17.6503974), max_latitude: Some(47.747587), min_latitude: Some(47.591275), max_longitude: Some(17.80789), min_longitude: Some(17.50948)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Győr"), ("ar", "جيور"), ("az", "Dyor"), ("be", "Горад Дзьёр"), ("bg", "Дьор"), ("bn", "ইয\u{9bc}োর"), ("ca", "Győr"), ("ccp", "𑄉\u{1112d}𑄠\u{1112e}𑄢\u{11134}"), ("ceb", "Győr"), ("cs", "Győr"), ("cy", "Győr"), ("da", "Győr"), ("de", "Győr"), ("el", "Γκιούρ"), ("en", "Győr"), ("es", "Győr"), ("et", "Győr"), ("eu", "Győr"), ("fa", "گیور"), ("fi", "Győr"), ("fr", "Győr"), ("gl", "Győr"), ("gu", "ગ\u{acd}યોર"), ("he", "גיור"), ("hi", "ग\u{94d}योर"), ("hr", "Jura"), ("hu", "Győr"), ("hy", "Դյոր"), ("id", "Győr"), ("it", "Győr"), ("ja", "ジェール"), ("jv", "Győr"), ("ka", "დიორი"), ("kk", "Дьер"), ("kn", "ಗ\u{ccd}ಯೋರ\u{ccd}"), ("ko", "죄르"), ("lt", "Dėras"), ("lv", "Ģēra"), ("mk", "Ѓер"), ("mr", "गॉर"), ("ms", "Győr"), ("nb", "Győr"), ("nl", "Győr"), ("no", "Győr"), ("pl", "Győr"), ("pt", "Győr"), ("ro", "Győr"), ("ru", "Дьёр"), ("si", "ග\u{dd2}යෝර\u{dca}"), ("sk", "Győr"), ("sl", "Gjur"), ("sq", "Győr"), ("sr", "Ђер"), ("sr_Latn", "Đer"), ("sv", "Győr"), ("sw", "Győr"), ("ta", "க\u{bcd}யூர\u{bcd}"), ("te", "గ\u{c4d}య\u{c4b}ర\u{c4d}"), ("th", "เจอร\u{e4c}"), ("tr", "Győr"), ("uk", "Дьйор"), ("ur", "جیور"), ("uz", "Dyor"), ("vi", "Győr"), ("zh", "杰尔")]),
                        unofficial_name_list: ["Győr"].to_vec(),
                    }
                ),
                (
                    "HB",
                    Subdivision{
                        name: "HB",
                        country_alpha2: Alpha2::HU,
                        code: "HB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.4688355), longitude: Some(21.5453228), max_latitude: Some(47.964842), min_latitude: Some(46.9412041), max_longitude: Some(22.1297241), min_longitude: Some(20.82321)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هايدو-بيهار"), ("be", "Хайду-Біхар"), ("bg", "Хайду-Бихар"), ("bn", "হজদ\u{9c1}-বিহ\u{9be}র ক\u{9be}উন\u{9cd}টি"), ("ca", "Hajdú-Bihar"), ("ccp", "𑄦𑄌\u{11134}𑄓\u{1112a}-𑄝\u{11128}𑄦𑄢\u{11134}"), ("ceb", "Hajdú-Bihar"), ("cs", "Hajdú-Bihar"), ("da", "Hajdú-Bihar"), ("de", "Komitat Hajdú-Bihar"), ("el", "Χάτζντου-Μπιχάρ"), ("en", "Hajdú-Bihar"), ("es", "Hajdú-Bihar"), ("et", "Hajdú-Bihari komitaat"), ("eu", "Hajdú-Bihar"), ("fa", "هاجدو-بیهار"), ("fi", "Hajdú-Bihar"), ("fr", "Hajdú-Bihar"), ("gl", "Hajdú-Bihar"), ("gu", "હાજદ\u{ac1}-બિહાર કાઉન\u{acd}ટી"), ("he", "היידו-ביהר"), ("hi", "हजद\u{941}-बिहार काउ\u{902}टी"), ("hr", "Hajdu-biharska županija"), ("hu", "Hajdú-Bihar megye"), ("id", "Hajdú-Bihar"), ("it", "Provincia di Hajdú-Bihar"), ("ja", "ハイドゥー・ビハール県"), ("ka", "ჰაიდუ-ბიჰარის მედიე"), ("kn", "ಹಜ\u{ccd}ದು-ಬ\u{cbf}ಹಾರ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "허이두비허르 주"), ("lt", "Haidū Biharas"), ("lv", "Hajdū-Biharas meģe"), ("mk", "Хајду-Бихар"), ("mr", "हजद\u{941}-बिहार काउ\u{902}टी"), ("ms", "Hajdú-Bihar"), ("nb", "Hajdu-Bihar Fylke"), ("nl", "Hajdú-Bihar"), ("no", "Hajdu-Bihar Fylke"), ("pl", "Komitat Hajdú-Bihar"), ("pt", "Hajdú-Bihar"), ("ro", "Județul Hajdú-Bihar"), ("ru", "Хайду-Бихар"), ("si", "හජ\u{dca}ද\u{dd4}-බ\u{dd2}හ\u{dcf}ර\u{dca}"), ("sk", "Hajducko-bihárska župa"), ("sl", "Hajdú-Bihar"), ("sr", "Хајду-Бихар"), ("sr_Latn", "Hajdu-Bihar"), ("sv", "Hajdú-Bihar"), ("sw", "Hajdú-Bihar"), ("ta", "ஹஜ\u{bcd}ஜிடு -ப\u{bc0}க\u{bbe}ர\u{bcd} கவுண\u{bcd}டி"), ("te", "హజ\u{c4d}దు-బ\u{c40}హ\u{c3e}ర\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "ฮ\u{e31}จด\u{e38}ส ไบฮา คร\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Hajdú-Bihar ili"), ("uk", "Гайду-Бігар"), ("ur", "ہایدو-بیہار کاؤنٹی"), ("vi", "Hajdu-Bihar"), ("zh", "豪伊杜-比豪爾州")]),
                        unofficial_name_list: ["Hajdú-Bihar"].to_vec(),
                    }
                ),
                (
                    "HE",
                    Subdivision{
                        name: "HE",
                        country_alpha2: Alpha2::HU,
                        code: "HE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.5971694), longitude: Some(20.280156), max_latitude: Some(47.67125799999999), min_latitude: Some(47.528197), max_longitude: Some(20.3831291), min_longitude: Some(20.208671)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة هفش"), ("be", "Хевеш"), ("bg", "Хевеш"), ("bn", "হেভেস ক\u{9be}উন\u{9cd}টি"), ("ca", "Heves"), ("ccp", "𑄦𑄬𑄞𑄬𑄌\u{11134}"), ("ceb", "Heves megye"), ("cs", "Heves"), ("da", "Heves"), ("de", "Komitat Heves"), ("el", "Χέβςς"), ("en", "Heves"), ("es", "Heves"), ("et", "Hevesi komitaat"), ("eu", "Heves"), ("fa", "هیویس"), ("fi", "Heves"), ("fr", "Heves"), ("gl", "Condado de Heves"), ("gu", "હ\u{ac7}વ\u{ac7}સ કાઉન\u{acd}ટી"), ("he", "הווש"), ("hi", "ह\u{947}व\u{947}स काउ\u{902}टी"), ("hr", "Heveška županija"), ("hu", "Heves megye"), ("hy", "Հևեշ"), ("id", "Heves"), ("it", "Provincia di Heves"), ("ja", "ヘヴェシュ県"), ("ka", "ხევეშის მედიე"), ("kn", "ಹ\u{cc6}ವ\u{cc6}ಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "헤베시 주"), ("lt", "Hevešas"), ("lv", "Hevešas meģe"), ("mk", "Хевеш"), ("mr", "ह\u{947}व\u{94d}हस काउ\u{902}टी"), ("ms", "Heves"), ("nb", "Heves Fylke"), ("nl", "Heves"), ("no", "Heves Fylke"), ("pl", "Komitat Heves"), ("pt", "Heves"), ("ro", "Județul Heves"), ("ru", "Хевеш"), ("si", "හෙවෙස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Hevešská župa"), ("sl", "Heves"), ("sr", "Хевеш"), ("sr_Latn", "Heveš"), ("sv", "Heves"), ("sw", "Heves"), ("ta", "ஹெவ\u{bcd}ஸ\u{bcd} கவுண\u{bcd}டி"), ("te", "హ\u{c46}వ\u{c46}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลแฮแว\u{e47}ช"), ("tr", "Heves ili"), ("uk", "Гевеш"), ("ur", "ہیویش کاؤنٹی"), ("vi", "Hạt Heves"), ("zh", "赫維什州")]),
                        unofficial_name_list: ["Heves"].to_vec(),
                    }
                ),
                (
                    "HV",
                    Subdivision{
                        name: "HV",
                        country_alpha2: Alpha2::HU,
                        code: "HV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4181262), longitude: Some(20.3300315), max_latitude: Some(46.528218), min_latitude: Some(46.2947919), max_longitude: Some(20.5828911), min_longitude: Some(20.184785)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("az", "Xodmezövaşarxey"), ("be", "Хадмезёвашархей"), ("bg", "Ходмезьовашархей"), ("ca", "Hódmezővásárhely"), ("ccp", "𑄦\u{1112e}𑄖\u{11134}𑄟𑄬𑄎\u{1112e}𑄞𑄥𑄢\u{11134}𑄦𑄬𑄣\u{11128}"), ("ceb", "Hódmezővásárhely"), ("cs", "Hódmezővásárhely"), ("cy", "Hódmezővásárhely"), ("da", "Hódmezővásárhely"), ("de", "Hódmezővásárhely"), ("el", "Χοντμεζοβάσαρχεϊ"), ("en", "Hódmezővásárhely"), ("es", "Hódmezővásárhely"), ("et", "Hódmezővásárhely"), ("fa", "هدمزواشارهی"), ("fr", "Hódmezővásárhely"), ("gl", "Hódmezővásárhely"), ("he", "הודמזוושרהיי"), ("hr", "Vašrelj"), ("hu", "Hódmezővásárhely"), ("hy", "Հոդմեզյովաշարհեյ"), ("id", "Hódmezővásárhely"), ("it", "Hódmezővásárhely"), ("ja", "ホードメゼーヴァーシャールヘイ"), ("ka", "ჰოდმეზევაშარჰეი"), ("ko", "호드메죄바샤르헤이"), ("lt", "Hodmezėvašarhėjus"), ("mk", "Ходмезевашархељ"), ("ms", "Hódmezővásárhely"), ("nb", "Hódmezővásárhely"), ("nl", "Hódmezővásárhely"), ("no", "Hódmezővásárhely"), ("pl", "Hódmezővásárhely"), ("pt", "Hódmezővásárhely"), ("ro", "Hódmezővásárhely"), ("ru", "Ходмезёвашархей"), ("sk", "Hódmezővásárhely"), ("sl", "Hódmezővásárhely"), ("sr", "Ходмезевашархељ"), ("sr_Latn", "Hodmezevašarhelj"), ("sv", "Hódmezővásárhely"), ("th", "โฮดแมเซอวาชาร\u{e4c}แฮย\u{e4c}"), ("tr", "Hódmezővásárhely"), ("uk", "Годмезевашаргей"), ("vi", "Hódmezővásárhely"), ("zh", "霍德梅澤瓦")]),
                        unofficial_name_list: ["Hódmezővásárhely"].to_vec(),
                    }
                ),
                (
                    "JN",
                    Subdivision{
                        name: "JN",
                        country_alpha2: Alpha2::HU,
                        code: "JN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2555579), longitude: Some(20.5232456), max_latitude: Some(47.682564), min_latitude: Some(46.754459), max_longitude: Some(21.0302648), min_longitude: Some(19.651048)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة ياس-نادكون-سولنك"), ("az", "Yas-Nadkun-Solnok"), ("be", "Яс-Надзькун-Сольнак"), ("bg", "Яс-Надкун-Солнок"), ("bn", "জ\u{9be}য-ন\u{9be}গিক\u{9c1}ন-জোলনক ক\u{9be}উন\u{9cd}টি"), ("ca", "Jász-Nagykun-Szolnok"), ("ccp", "𑄎𑄌\u{11134}-𑄚𑄉\u{1112d}𑄇\u{1112a}𑄚\u{11134}-𑄥\u{1112a}𑄣\u{11134}𑄚\u{11127}𑄇\u{11134}"), ("ceb", "Jász-Nagykun-Szolnok"), ("cs", "Jász-Nagykun-Szolnok"), ("da", "Jász-Nagykun-Szolnok"), ("de", "Komitat Jász-Nagykun-Szolnok"), ("el", "Τζασζ-Ναγκίκουν-Σζόλνοκ"), ("en", "Jász-Nagykun-Szolnok"), ("es", "Jász-Nagykun-Szolnok"), ("et", "Jász-Nagykun-Szolnoki komitaat"), ("eu", "Jász-Nagykun-Szolnok"), ("fa", "جاسز-ناگیکون-زولنوک"), ("fi", "Jász-Nagykun-Szolnok"), ("fr", "Jász-Nagykun-Szolnok"), ("gl", "Jász-Nagykun-Szolnok"), ("gu", "જાસ\u{acd}ઝ-નાગીક\u{ac1}ન-ઝોલનોક કાઉન\u{acd}ટી"), ("he", "יאס-נאג׳קון-סולנוק"), ("hi", "जस\u{94d}ज\u{93c}-नाजिक\u{941}न-सज\u{93c}ोल\u{94d}नोक काउ\u{902}टी"), ("hr", "Jaziško-velikokumansko-szolnočka županija"), ("hu", "Jász-Nagykun-Szolnok megye"), ("id", "Jász-Nagykun-Szolnok"), ("it", "Provincia di Jász-Nagykun-Szolnok"), ("ja", "ヤース・ナジクン・ソルノク県"), ("ka", "იას-ნადკუნ-სოლნოკის მედიე"), ("kn", "ಜಾಸ\u{ccd}ಜ\u{ccd}-ನಾಗ\u{ccd}ಕುನ\u{ccd}-ಸ\u{ccd}ಝೊಲ\u{ccd}ನೋಕ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "야스너지쿤솔노크 주"), ("lt", "Jasas-Nadkūnas-Solnokas"), ("lv", "Jāsas-Naģkunas-Solnokas meģe"), ("mk", "Јас-Наѓкун-Солнок"), ("mn", "Яс-Надькун-Сольнок"), ("mr", "जस\u{94d}स-नाजिक\u{941}क-सोलोन\u{94d}क काउ\u{902}टी"), ("ms", "Jász-Nagykun-Szolnok"), ("nb", "Jasz Nagykun Szolnok fylke"), ("nl", "Jász-Nagykun-Szolnok"), ("no", "Jasz Nagykun Szolnok fylke"), ("pl", "Komitat Jász-Nagykun-Szolnok"), ("pt", "Jász-Nagykun-Szolnok"), ("ro", "Județul Jász-Nagykun-Szolnok"), ("ru", "Яс-Надькун-Сольнок"), ("si", "ජ\u{dcf}ස\u{dca}-නග\u{dd2}ක\u{dd4}න\u{dca} -ස\u{dca}ලොනොක\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Jasovsko-veľkokumánsko-solnocká župa"), ("sl", "Jász-Nagykun-Szolnok"), ("sr", "Јас-Нађкун-Солнок"), ("sr_Latn", "Jas-Nađkun-Solnok"), ("sv", "Jász-Nagykun-Szolnok"), ("sw", "Jász-Nagykun-Szolnok"), ("ta", "ஜ\u{bbe}ஸிஸ\u{bcd}-நகிகுண\u{bcd}-ஸ\u{bcd}ஸ\u{bcd}வ\u{bcd}ள\u{bcd}நோக\u{bcd}க கவுண\u{bcd}டி"), ("te", "జస\u{c4d}జ\u{c4d}-న\u{c3e}గ\u{c4d}య\u{c3e}కున\u{c4d}-జ\u{c4b}ల\u{c4d}న\u{c4b}క\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลยาส-น\u{e47}อจก\u{e38}น-โซลโนก"), ("tr", "Jász-Nagykun-Szolnok ili"), ("uk", "Яс-Надькун-Сольнок"), ("ur", "جاسز-نگیکن-سزولنوک کاؤنٹی"), ("vi", "Hạt Jász-Nagykun-Szolnok"), ("zh", "亞斯-瑙吉孔-索爾諾克州")]),
                        unofficial_name_list: ["Jász-Nagykun-Szolnok"].to_vec(),
                    }
                ),
                (
                    "KE",
                    Subdivision{
                        name: "KE",
                        country_alpha2: Alpha2::HU,
                        code: "KE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.7390852), longitude: Some(18.1267006), max_latitude: Some(47.75665499999999), min_latitude: Some(47.68579099999999), max_longitude: Some(18.2301799), min_longitude: Some(18.016753)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كوماروم-إستركوم"), ("be", "Камаром-Эстэргам"), ("bg", "Комаром-Естергом"), ("bn", "কোম\u{9be}রম-এজত\u{9be}রগম ক\u{9be}উন\u{9cd}টি"), ("ca", "Komárom-Esztergom"), ("ccp", "𑄇\u{1112e}𑄟𑄢\u{1112a}𑄚\u{11134}-𑄃\u{11128}𑄌\u{11134}𑄑𑄢\u{11134}𑄉\u{11127}𑄟\u{11134}"), ("ceb", "Komárom-Esztergom"), ("cs", "Komárom-Esztergom"), ("da", "Komárom-Esztergom"), ("de", "Komitat Komárom-Esztergom"), ("el", "Κομάρομ-Έσζτεργκομ"), ("en", "Komárom-Esztergom"), ("es", "Komárom-Esztergom"), ("et", "Komárom-Esztergomi komitaat"), ("eu", "Komárom-Esztergom"), ("fa", "کوماروم-ایسترگوم"), ("fi", "Komárom-Esztergom"), ("fr", "Komárom-Esztergom"), ("gl", "Komárom-Esztergom"), ("gu", "કોમ\u{ac7}રોમ-એઝટ\u{ac7}ર\u{acd}ગોમ કાઉન\u{acd}ટી"), ("he", "קומארום-אסטרגום"), ("hi", "कोमारोम-एज\u{94d}टरगोम काउ\u{902}टी"), ("hr", "Komoransko-ostrogonska županija"), ("hu", "Komárom-Esztergom megye"), ("id", "Komárom-Esztergom"), ("it", "Provincia di Komárom-Esztergom"), ("ja", "コマーロム・エステルゴム県"), ("ka", "კომარომ-ესტერგომის მედიე"), ("kn", "ಕೊಮೊರೊಮ\u{ccd}-ಎಸ\u{ccd}ಝ\u{cc6}ರ\u{ccd}ಗೊಮ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "코마롬에스테르곰 주"), ("lt", "Komaromas-Estergomas"), ("lv", "Komāromas-Estergomas meģe"), ("mk", "Комаром-Естергом"), ("mn", "Комаром-Эстергом"), ("mr", "कॉमरोम-एसझ\u{947}टर\u{94d}गोम काउ\u{902}टी"), ("ms", "Komárom-Esztergom"), ("nb", "Komarom-Esztergom fylke"), ("nl", "Komárom-Esztergom"), ("no", "Komarom-Esztergom fylke"), ("pl", "Komitat Komárom-Esztergom"), ("pt", "Komárom-Esztergom"), ("ro", "Județul Komárom-Esztergom"), ("ru", "Комаром-Эстергом"), ("si", "කොම\u{dcf}රොම\u{dca} එස\u{dca}ටර\u{dca}ගොම\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Komárňansko-ostrihomská župa"), ("sl", "Komárom-Esztergom"), ("sr", "Комаром-Естергом"), ("sr_Latn", "Komarom-Estergom"), ("sv", "Komárom-Esztergom"), ("sw", "Komárom-Esztergom"), ("ta", "கோம\u{bbe}றோம\u{bcd}-ஸ\u{bcd}ஸ\u{bcd}ட\u{bcd}பேர\u{bcd}கோம\u{bcd} கவுண\u{bcd}டி"), ("te", "క\u{c4b}మ\u{c3e}ర\u{c4b}మ\u{c4d}-ఎస\u{c4d}టర\u{c4d}గ\u{c4b}మ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลโกมาโรม-แอ\u{e47}สแตร\u{e4c}โกม"), ("tr", "Komárom-Esztergom ili"), ("uk", "Комаром-Естерґом"), ("ur", "کوماروم-ایسترگوم کاؤنٹی"), ("vi", "Hạt Komárom-Esztergom"), ("zh", "科馬羅姆-埃斯泰爾戈姆州")]),
                        unofficial_name_list: ["Komárom-Esztergom"].to_vec(),
                    }
                ),
                (
                    "KM",
                    Subdivision{
                        name: "KM",
                        country_alpha2: Alpha2::HU,
                        code: "KM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8963711), longitude: Some(19.6896861), max_latitude: Some(47.0012541), min_latitude: Some(46.769863), max_longitude: Some(19.8515029), min_longitude: Some(19.5066109)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كيكسكيميت"), ("az", "Keçkemet"), ("be", "Кечкемет"), ("bg", "Кечкемет"), ("bn", "কেক\u{9cd}সকেমিট"), ("ca", "Kecskemét"), ("ccp", "𑄇𑄬𑄌\u{11134}𑄇𑄬𑄟𑄬𑄖\u{11134}"), ("ceb", "Kecskemét"), ("cs", "Kecskemét"), ("da", "Kecskemét"), ("de", "Kecskemét"), ("el", "Κετσκέμετ"), ("en", "Kecskemét"), ("es", "Kecskemét"), ("et", "Kecskemét"), ("eu", "Kecskemét"), ("fa", "کچکمیت"), ("fi", "Kecskemét"), ("fr", "Kecskemét"), ("gl", "Kecskemét"), ("gu", "ક\u{ac7}ક\u{acd}સક\u{ac7}મ\u{ac7}\u{a82}ટ"), ("he", "קצ׳קמט"), ("hi", "कएक\u{94d}सक\u{947}म\u{947}ट"), ("hr", "Kečkemet"), ("hu", "Kecskemét"), ("hy", "Կեչկեմետ"), ("id", "Kecskemét"), ("it", "Kecskemét"), ("ja", "ケチケメート"), ("jv", "Kecskemét"), ("ka", "კეჩკემეტი"), ("kk", "Кечкемет"), ("kn", "ಕೇಕ\u{ccd}ಸ\u{ccd}ಕೇಮ\u{cc6}ಟ\u{ccd}"), ("ko", "케치케메트"), ("lt", "Kečkemėtas"), ("lv", "Kečkemēta"), ("mk", "Кечкемет"), ("mr", "क\u{947}क\u{94d}\u{200d}सक\u{947}म\u{947}ट"), ("ms", "Kecskemét"), ("nb", "Kecskemet"), ("nl", "Kecskemét"), ("no", "Kecskemet"), ("pl", "Kecskemét"), ("pt", "Kecskemét"), ("ro", "Kecskemét"), ("ru", "Кечкемет"), ("si", "කෙක\u{dca}ස\u{dca}කෙමට\u{dca}"), ("sk", "Kecskemét"), ("sl", "Kecskemét"), ("sr", "Кечкемет"), ("sr_Latn", "Kečkemet"), ("sv", "Kecskemét"), ("sw", "Kecskemét"), ("ta", "கேக\u{bcd}ஸ\u{bcd}க\u{bcd}மேட\u{bcd}"), ("te", "క\u{c46}క\u{c4d}స\u{c4d}స\u{c47}మ\u{c47}ట\u{c4d}"), ("th", "แก\u{e47}ชแกเมต"), ("tr", "Kecskemét"), ("uk", "Кечкемет"), ("ur", "کیکسکیمیت"), ("vi", "Kecskemét"), ("zh", "凯奇凯梅特")]),
                        unofficial_name_list: ["Kecskemét"].to_vec(),
                    }
                ),
                (
                    "KV",
                    Subdivision{
                        name: "KV",
                        country_alpha2: Alpha2::HU,
                        code: "KV",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3593606), longitude: Some(17.7967639), max_latitude: Some(46.45050699999999), min_latitude: Some(46.290355), max_longitude: Some(17.8911989), min_longitude: Some(17.741116)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "كابوشفار"), ("az", "Kapoşvar"), ("be", "Капашвар"), ("bg", "Капошвар"), ("bn", "ক\u{9be}পোভ\u{9be}র"), ("ca", "Kaposvár"), ("ccp", "𑄇𑄛\u{1112e}𑄌\u{11134}𑄞𑄢\u{11134}"), ("ceb", "Kaposvár"), ("cs", "Kaposvár"), ("cy", "Kaposvár"), ("da", "Kaposvár"), ("de", "Kaposvár"), ("el", "Κάποσβαρ"), ("en", "Kaposvár"), ("es", "Kaposvár"), ("et", "Kaposvár"), ("eu", "Kaposvár"), ("fa", "کاپشوار"), ("fi", "Kaposvár"), ("fr", "Kaposvár"), ("gl", "Kaposvár"), ("gu", "કપોસ\u{acd}વાર"), ("he", "קאפושוואר"), ("hi", "कापोस\u{94d}वार"), ("hr", "Kapošvar"), ("hu", "Kaposvár"), ("hy", "Կապոշվար"), ("id", "Kaposvár"), ("it", "Kaposvár"), ("ja", "カポシュヴァール"), ("ka", "კაპოშვარი"), ("kk", "Капошвар"), ("kn", "ಕಪೋಶ\u{ccd}ವರ\u{ccd}"), ("ko", "커포슈바르"), ("lt", "Kapošvaras"), ("lv", "Kapošvāra"), ("mk", "Капошвар"), ("mr", "क\u{94d}यापोस\u{94d}वार"), ("ms", "Kaposvár"), ("nb", "Kaposvár"), ("nl", "Kaposvár"), ("no", "Kaposvár"), ("pl", "Kaposvár"), ("pt", "Kaposvár"), ("ro", "Kaposvár"), ("ru", "Капошвар"), ("si", "ක\u{dd6}ටහ\u{dca}ය\u{dcf}"), ("sk", "Kaposvár"), ("sl", "Kaposvár"), ("sr", "Капошвар"), ("sr_Latn", "Kapošvar"), ("sv", "Kaposvár"), ("sw", "Kaposvár"), ("ta", "கபோஸ\u{bcd}வர\u{bcd}"), ("te", "క\u{c3e}ప\u{c4b}వ\u{c3e}ర\u{c4d}"), ("th", "คาโปสวาร\u{e4c}"), ("tr", "Kaposvár"), ("uk", "Капошвар"), ("ur", "کیپوسوار"), ("vi", "Kaposvár"), ("zh", "考波什堡")]),
                        unofficial_name_list: ["Kaposvár"].to_vec(),
                    }
                ),
                (
                    "MI",
                    Subdivision{
                        name: "MI",
                        country_alpha2: Alpha2::HU,
                        code: "MI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.0963631), longitude: Some(20.762386), max_latitude: Some(48.154998), min_latitude: Some(48.0233119), max_longitude: Some(20.8697789), min_longitude: Some(20.4817199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Miskolc"), ("ar", "ميشكولتس"), ("az", "Mişkolç"), ("be", "Мішкальц"), ("bg", "Мишколц"), ("bn", "মিসকস"), ("ca", "Miskolc"), ("ccp", "𑄟\u{11128}𑄌\u{11134}𑄇\u{1112e}𑄣\u{11134}𑄇\u{11134}"), ("ceb", "Miskolc"), ("cs", "Miskolc"), ("cy", "Miskolc"), ("da", "Miskolc"), ("de", "Miskolc"), ("el", "Μίσκολτς"), ("en", "Miskolc"), ("es", "Miskolc"), ("et", "Miskolc"), ("eu", "Miskolc"), ("fa", "میشکولتس"), ("fi", "Miskolc"), ("fr", "Miskolc"), ("gl", "Miskolc"), ("gu", "મિસ\u{acd}કોલ\u{acd}ક"), ("he", "מישקולץ"), ("hi", "मिस\u{94d}कोल\u{94d}स"), ("hr", "Miškolc"), ("hu", "Miskolc"), ("hy", "Միշկոլց"), ("id", "Miskolc"), ("it", "Miskolc"), ("ja", "ミシュコルツ"), ("ka", "მიშკოლცი"), ("kk", "Мишкольц"), ("kn", "ಮ\u{cbf}ಸ\u{ccd}ಕೋಲ\u{ccd}ಕ\u{ccd}"), ("ko", "미슈콜츠"), ("ky", "Мишкольц"), ("lt", "Miškolcas"), ("lv", "Miškolca"), ("mk", "Мишколц"), ("mr", "मिस\u{94d}कोल\u{94d}क"), ("ms", "Miskolc"), ("nb", "Miskolc"), ("nl", "Miskolc"), ("no", "Miskolc"), ("pl", "Miszkolc"), ("pt", "Miskolc"), ("ro", "Miskolc"), ("ru", "Мишкольц"), ("si", "ම\u{dd2}ස\u{dca}කොල\u{dca}ක\u{dca}"), ("sk", "Miškovec"), ("sl", "Miskolc"), ("sr", "Мишколц"), ("sr_Latn", "Miškolc"), ("sv", "Miskolc"), ("sw", "Miskolc"), ("ta", "மிஸ\u{bcd}க\u{bcd}கோல\u{bcd}க\u{bcd}"), ("te", "మ\u{c3f}స\u{c4d}క\u{c4b}ల\u{c4d}చ\u{c4d}"), ("th", "ม\u{e34}สโคลค"), ("tr", "Miskolc"), ("uk", "Мішкольц"), ("ur", "مسکولک"), ("vi", "Miskolc"), ("zh", "米什科尔茨")]),
                        unofficial_name_list: ["Miskolc"].to_vec(),
                    }
                ),
                (
                    "NK",
                    Subdivision{
                        name: "NK",
                        country_alpha2: Alpha2::HU,
                        code: "NK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4590218), longitude: Some(16.9896796), max_latitude: Some(46.543614), min_latitude: Some(46.35871299999999), max_longitude: Some(17.0906941), min_longitude: Some(16.919825)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "ناجيكانيزسا"), ("be", "Горад Надзьканіжа"), ("bg", "Надканижа"), ("ca", "Nagykanizsa"), ("ccp", "𑄚𑄉\u{1112d}𑄇𑄚\u{11128}𑄌\u{11134}𑄥"), ("ceb", "Nagykanizsa"), ("cs", "Nagykanizsa"), ("cy", "Nagykanizsa"), ("da", "Nagykanizsa"), ("de", "Nagykanizsa"), ("en", "Nagykanizsa"), ("es", "Nagykanizsa"), ("et", "Nagykanizsa"), ("eu", "Nagykanizsa"), ("fa", "نادکانیژا"), ("fi", "Nagykanizsa"), ("fr", "Nagykanizsa"), ("gl", "Nagykanizsa"), ("he", "נג׳קניז׳ה"), ("hr", "Velika Kaniža"), ("hu", "Nagykanizsa"), ("id", "Nagykanizsa"), ("it", "Nagykanizsa"), ("ja", "ナジカニジャ"), ("jv", "Nagykanizsa"), ("ka", "ნადკანიჟა"), ("lt", "Nadkaniža"), ("ms", "Nagykanizsa"), ("nb", "Nagykanizsa"), ("nl", "Nagykanizsa"), ("no", "Nagykanizsa"), ("pl", "Nagykanizsa"), ("pt", "Nagykanizsa"), ("ro", "Nagykanizsa"), ("ru", "Надьканижа"), ("sk", "Nagykanizsa"), ("sl", "Nagykanizsa"), ("sr", "Велика Канижа"), ("sr_Latn", "Velika Kaniža"), ("sv", "Nagykanizsa"), ("tr", "Nagykanizsa"), ("uk", "Надьканіжа"), ("vi", "Nagykanizsa"), ("zh", "瑙吉考尼饒")]),
                        unofficial_name_list: ["Nagykanizsa"].to_vec(),
                    }
                ),
                (
                    "NO",
                    Subdivision{
                        name: "NO",
                        country_alpha2: Alpha2::HU,
                        code: "NO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.90410310000001), longitude: Some(19.0498504), max_latitude: Some(47.933664), min_latitude: Some(47.867935), max_longitude: Some(19.1055319), min_longitude: Some(18.9853318)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة نوغراد"), ("az", "Noqrad"), ("be", "Ноград"), ("bg", "Ноград"), ("bn", "নোগ\u{9cd}র\u{9be}ড ক\u{9be}উন\u{9cd}টি"), ("ca", "Nógrád"), ("ccp", "𑄚\u{11127}𑄉\u{11133}𑄢𑄖\u{11134}"), ("ceb", "Nógrád megye"), ("cs", "Nógrád"), ("da", "Nógrád"), ("de", "Komitat Nógrád"), ("el", "Νόγκραντ"), ("en", "Nógrád"), ("es", "Nógrád"), ("et", "Nógrádi komitaat"), ("eu", "Nógrád"), ("fa", "نوگراد"), ("fi", "Nógrád"), ("fr", "Nógrád"), ("gl", "Condado de Nógrád"), ("gu", "નોગ\u{acd}ર\u{ac7}ડ કાઉન\u{acd}ટી"), ("he", "נוגראד"), ("hi", "नॉग\u{94d}र\u{948}ड काउ\u{902}टी"), ("hr", "Nogradska županija"), ("hu", "Nógrád megye"), ("id", "Nógrád"), ("it", "Provincia di Nógrád"), ("ja", "ノーグラード県"), ("ka", "ნოგრადის მედიე"), ("kn", "ನೊಗ\u{ccd}ರಾಡ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "노그라드 주"), ("lt", "Nogradas"), ("lv", "Nogrādas meģe"), ("mk", "Ноград"), ("mr", "नॉरग\u{94d}र\u{947}ड काउ\u{902}टी"), ("ms", "Nógrád"), ("nb", "Nógrád"), ("nl", "Nógrád"), ("no", "Nógrád"), ("pl", "Komitat Nógrád"), ("pt", "Nógrád"), ("ro", "Județul Nógrád"), ("ru", "Ноград"), ("si", "නොග\u{dca}ර\u{dcf}ඩ\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Novohradská župa"), ("sl", "Nógrád"), ("sr", "Ноград"), ("sr_Latn", "Nograd"), ("sv", "Nógrád"), ("sw", "Nógrád"), ("ta", "நோக\u{bcd}ர\u{bbe}ட\u{bcd} கவுண\u{bcd}டி"), ("te", "న\u{c4b}గ\u{c4d}ర\u{c3e}డ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เทศมณฑลโนกราด"), ("tr", "Nógrád ili"), ("uk", "Ноґрад"), ("ur", "نوگراد کاؤنٹی"), ("vi", "Hạt Nógrád"), ("zh", "諾格拉德州")]),
                        unofficial_name_list: ["Nógrád"].to_vec(),
                    }
                ),
                (
                    "NY",
                    Subdivision{
                        name: "NY",
                        country_alpha2: Alpha2::HU,
                        code: "NY",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.9495324), longitude: Some(21.7244053), max_latitude: Some(48.03913), min_latitude: Some(47.845661), max_longitude: Some(21.844149), min_longitude: Some(21.5576691)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Nyíregyháza"), ("ar", "نيرغهازا"), ("az", "Niredxaza"), ("be", "Горад Ньірэдзьхаза"), ("bg", "Ниредхаза"), ("bn", "নিরজহ\u{9be}জ\u{9be}"), ("ca", "Nyíregyháza"), ("ccp", "𑄚\u{1112d}𑄢\u{11134}𑄢𑄬𑄉\u{1112d}𑄦𑄎"), ("ceb", "Nyíregyháza"), ("cs", "Nyíregyháza"), ("cy", "Nyíregyháza"), ("da", "Nyíregyháza"), ("de", "Nyíregyháza"), ("el", "Νιρεγιχάσα"), ("en", "Nyíregyháza"), ("es", "Nyíregyháza"), ("et", "Nyíregyháza"), ("eu", "Nyíregyháza"), ("fa", "نیرگهزا"), ("fi", "Nyíregyháza"), ("fr", "Nyíregyháza"), ("gl", "Nyíregyháza"), ("gu", "નાઇરીગ\u{ac7}હાઝા"), ("he", "נירג׳האזה"), ("hi", "नयीर\u{947}गयहाज\u{93c}ा"), ("hr", "Nyíregyháza"), ("hu", "Nyíregyháza"), ("hy", "Նյիրեդխազա"), ("id", "Nyíregyháza"), ("it", "Nyíregyháza"), ("ja", "ニーレジハーザ"), ("ka", "ნირედჰაზა"), ("kn", "ನೈರ\u{cc6}ಜ\u{cbf}ಹಾಝ"), ("ko", "니레지하저"), ("lt", "Nyredhaza"), ("lv", "Ņīreģhāza"), ("mk", "Њиреѓхаза"), ("mr", "नायरीजीहझा"), ("ms", "Nyíregyháza"), ("nb", "Nyíregyháza"), ("nl", "Nyíregyháza"), ("no", "Nyíregyháza"), ("pl", "Nyíregyháza"), ("pt", "Nyíregyháza"), ("ro", "Nyíregyháza"), ("ru", "Ньиредьхаза"), ("si", "නය\u{dd2}රේග\u{dd3}හස\u{dcf}"), ("sk", "Níreďháza"), ("sl", "Nyíregyháza"), ("sr", "Њиређхаза"), ("sr_Latn", "Njiređhaza"), ("sv", "Nyíregyháza"), ("sw", "Nyíregyháza"), ("ta", "இரேஜிய\u{bbe}ஜ\u{bbe}"), ("te", "నయ\u{c3f}ర\u{c47}గయహ\u{c3e}జ\u{c3e}"), ("th", "น\u{e35}เอ\u{e35}ยร\u{e4c}ก\u{e35}ฮาซ\u{e48}า"), ("tr", "Nyíregyháza"), ("uk", "Ньїредьгаза"), ("ur", "نییریجیحازا"), ("vi", "Nyíregyháza"), ("zh", "尼賴吉哈佐")]),
                        unofficial_name_list: ["Nyíregyháza"].to_vec(),
                    }
                ),
                (
                    "PE",
                    Subdivision{
                        name: "PE",
                        country_alpha2: Alpha2::HU,
                        code: "PE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.44800009999999), longitude: Some(19.4618128), max_latitude: Some(48.05865989999999), min_latitude: Some(46.944111), max_longitude: Some(20.1127141), min_longitude: Some(18.6884409)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بشت"), ("be", "Пешт"), ("bg", "Пещ"), ("ca", "Pest"), ("ccp", "𑄛𑄬𑄌\u{11134}𑄑\u{11134}"), ("ceb", "Pest megye"), ("cs", "Pest"), ("da", "Pest"), ("de", "Komitat Pest"), ("en", "Pest"), ("es", "Pest"), ("et", "Pesti komitaat"), ("eu", "Pest konderria"), ("fa", "پست کانتی"), ("fi", "Pest"), ("fr", "Pest"), ("gl", "Condado de Pest"), ("he", "מחוז פשט"), ("hr", "Peštanska županija"), ("hu", "Pest megye"), ("hy", "Պեշտ"), ("id", "Pest"), ("it", "Provincia di Pest"), ("ja", "ペシュト県"), ("ka", "პეშტის მედიე"), ("ko", "페슈트 주"), ("lt", "Peštas"), ("lv", "Peštas meģe"), ("mk", "Пешта"), ("ms", "Pest"), ("nb", "Pest"), ("nl", "Pest"), ("no", "Pest"), ("pl", "Komitat Pest"), ("pt", "Pest"), ("ro", "Județul Pest"), ("ru", "Пешт"), ("sk", "Peštianska župa"), ("sl", "Pešta"), ("sr", "Пешта"), ("sr_Latn", "Pešta"), ("sv", "Pest"), ("sw", "Pest"), ("tr", "Pest ili"), ("uk", "Пешт"), ("ur", "پاشت کاؤنٹی"), ("zh", "佩斯州")]),
                        unofficial_name_list: ["Pest"].to_vec(),
                    }
                ),
                (
                    "PS",
                    Subdivision{
                        name: "PS",
                        country_alpha2: Alpha2::HU,
                        code: "PS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.0727345), longitude: Some(18.232266), max_latitude: Some(46.161421), min_latitude: Some(45.994045), max_longitude: Some(18.3654829), min_longitude: Some(18.13568)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Pécs"), ("ar", "بيتش"), ("az", "Peç"), ("be", "Печ"), ("bg", "Печ"), ("bn", "পিকস"), ("bs", "Pečuh"), ("ca", "Pécs"), ("ccp", "𑄛𑄬𑄌\u{11134}𑄥\u{11134}"), ("ceb", "Pécs"), ("cs", "Pécs"), ("cy", "Pécs"), ("da", "Pécs"), ("de", "Pécs"), ("el", "Πετς"), ("en", "Pécs"), ("es", "Pécs"), ("et", "Pécs"), ("eu", "Pécs"), ("fa", "پچ"), ("fi", "Pécs"), ("fr", "Pécs"), ("gl", "Pécs"), ("gu", "પ\u{ac7}ક\u{acd}સ"), ("he", "פץ׳"), ("hi", "प\u{947}क\u{94d}स"), ("hr", "Pečuh"), ("hu", "Pécs"), ("hy", "Պեչ"), ("id", "Pécs"), ("it", "Pécs"), ("ja", "ペーチ"), ("ka", "პეჩი"), ("kn", "ಪ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}"), ("ko", "페치"), ("lt", "Pėčas"), ("lv", "Pēča"), ("mk", "Печ"), ("mr", "प\u{947}क\u{94d}स"), ("ms", "Pécs"), ("nb", "Pécs"), ("nl", "Pécs"), ("no", "Pécs"), ("pl", "Pecz"), ("pt", "Pécs"), ("ro", "Pécs"), ("ru", "Печ"), ("si", "පෙක\u{dca}ස\u{dca}"), ("sk", "Pécs"), ("sl", "Pécs"), ("sq", "Pécs"), ("sr", "Печуј"), ("sr_Latn", "Pečuj"), ("sv", "Pécs"), ("sw", "Pécs"), ("ta", "பேக\u{bcd}ஸ\u{bcd}"), ("te", "ప\u{c40}స\u{c4d}"), ("th", "เพซ"), ("tr", "Pécs"), ("uk", "Печ"), ("ur", "پیکس"), ("vi", "Pécs"), ("zh", "佩奇")]),
                        unofficial_name_list: ["Pécs"].to_vec(),
                    }
                ),
                (
                    "SD",
                    Subdivision{
                        name: "SD",
                        country_alpha2: Alpha2::HU,
                        code: "SD",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.2530102), longitude: Some(20.1414253), max_latitude: Some(46.349812), min_latitude: Some(46.115766), max_longitude: Some(20.3085971), min_longitude: Some(19.97234)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Szeged"), ("ar", "سيجد"), ("az", "Seqed"), ("be", "Сегед"), ("bg", "Сегед"), ("bn", "সেজড"), ("ca", "Szeged"), ("ccp", "𑄎𑄎𑄬𑄖\u{11134}"), ("cs", "Szeged"), ("cy", "Szeged"), ("da", "Szeged"), ("de", "Szeged"), ("el", "Σέγκεντ"), ("en", "Szeged"), ("es", "Szeged"), ("et", "Szeged"), ("eu", "Szeged"), ("fa", "سگد"), ("fi", "Szeged"), ("fr", "Szeged"), ("gl", "Szeged"), ("gu", "સ\u{ac7}ગ\u{ac7}ડ"), ("he", "סגד"), ("hi", "जग\u{947}ड"), ("hr", "Segedin"), ("hu", "Szeged"), ("hy", "Սեգեդ"), ("id", "Szeged"), ("is", "Szeged"), ("it", "Seghedino"), ("ja", "セゲド"), ("jv", "Szeged"), ("ka", "სეგედი"), ("kn", "ಜಗ\u{cc6}ಡ\u{ccd}"), ("ko", "세게드"), ("lt", "Segedas"), ("lv", "Segeda"), ("mk", "Сегедин"), ("mr", "स\u{947}ग\u{947}ड"), ("ms", "Szeged"), ("nb", "Szeged"), ("nl", "Szeged"), ("no", "Szeged"), ("pl", "Segedyn"), ("pt", "Szeged"), ("ro", "Seghedin"), ("ru", "Сегед"), ("si", "සෙගෙඩ\u{dca}"), ("sk", "Segedín"), ("sl", "Szeged"), ("sr", "Сегедин"), ("sr_Latn", "Segedin"), ("sv", "Szeged"), ("sw", "Szeged"), ("ta", "செஜெட\u{bcd}"), ("te", "జగ\u{c47}డ\u{c4d}"), ("th", "เซเกด"), ("tr", "Szeged"), ("uk", "Сегед"), ("ur", "سزیجید"), ("vi", "Szeged"), ("zh", "塞格德")]),
                        unofficial_name_list: ["Szeged"].to_vec(),
                    }
                ),
                (
                    "SF",
                    Subdivision{
                        name: "SF",
                        country_alpha2: Alpha2::HU,
                        code: "SF",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.18602620000001), longitude: Some(18.4221358), max_latitude: Some(47.28114), min_latitude: Some(47.107929), max_longitude: Some(18.553754), min_longitude: Some(18.3287498)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سيكشفهيرفار"), ("az", "Sekeşfexervar"), ("be", "Горад Секешфехервар"), ("bg", "Секешфехервар"), ("bn", "জেকেসফেরহেভ\u{9be}র"), ("ca", "Székesfehérvár"), ("ccp", "𑄎𑄬𑄇𑄬𑄌\u{11134}𑄜𑄬𑄦𑄬𑄢\u{11134}𑄞𑄢\u{11134}"), ("ceb", "Székesfehérvár"), ("cs", "Székesfehérvár"), ("cy", "Székesfehérvár"), ("da", "Székesfehérvár"), ("de", "Székesfehérvár"), ("el", "Σεκεσφεχερβάρ"), ("en", "Székesfehérvár"), ("es", "Székesfehérvár"), ("et", "Székesfehérvár"), ("eu", "Székesfehérvár"), ("fa", "سیکشفهروار"), ("fi", "Székesfehérvár"), ("fr", "Székesfehérvár"), ("gl", "Székesfehérvár"), ("gu", "ઝ\u{ac7}ક\u{ac7}સફ\u{ac7}હરવાર"), ("he", "סקשפהרוואר"), ("hi", "ज\u{93c}\u{947}क\u{947}सफहरवार"), ("hr", "Stolni Biograd"), ("hu", "Székesfehérvár"), ("hy", "Սեկաշֆեհերվար"), ("id", "Székesfehérvár"), ("it", "Székesfehérvár"), ("ja", "セーケシュフェヘールヴァール"), ("ka", "სეკეშფეჰერვარი"), ("kk", "Секешфехервар"), ("kn", "ಝ\u{cc6}ಕ\u{ccd}ಎಸ\u{ccd}ಫ\u{cc6}ಹೇರ\u{ccd}ವಾರ\u{ccd}"), ("ko", "세케슈페헤르바르"), ("lt", "Sėkešfehėrvaras"), ("lv", "Sēkešfehērvāra"), ("mk", "Секешфехервар"), ("mr", "झ\u{947}क\u{947}सफ\u{947}हरवर"), ("ms", "Székesfehérvár"), ("nb", "Székesfehérvár"), ("nl", "Székesfehérvár"), ("no", "Székesfehérvár"), ("pl", "Székesfehérvár"), ("pt", "Székesfehérvár"), ("ro", "Székesfehérvár"), ("ru", "Секешфехервар"), ("si", "සේකෙස\u{dca}ෆෙර\u{dca}ව\u{dcf}ර\u{dca}"), ("sk", "Stoličný Belehrad"), ("sl", "Székesfehérvár"), ("sr", "Столни Београд"), ("sr_Latn", "Stolni Beograd"), ("sv", "Székesfehérvár"), ("sw", "Székesfehérvár"), ("ta", "ஸிகேஸ\u{bcd}பிஹெர\u{bcd}வ\u{bbe}ர\u{bcd}"), ("te", "జ\u{c46}క\u{c46}స\u{c4d}ఫ\u{c46}హ\u{c47}ర\u{c4d}వ\u{c3e}ర\u{c4d}"), ("th", "เซแก\u{e47}ชแฟเฮร\u{e4c}วาร\u{e4c}"), ("tr", "Székesfehérvár"), ("uk", "Секешфегервар"), ("ur", "سزیکیسفیحیروار"), ("vi", "Székesfehérvár"), ("zh", "塞克什白堡")]),
                        unofficial_name_list: ["Székesfehérvár"].to_vec(),
                    }
                ),
                (
                    "SH",
                    Subdivision{
                        name: "SH",
                        country_alpha2: Alpha2::HU,
                        code: "SH",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.2306851), longitude: Some(16.6218441), max_latitude: Some(47.274314), min_latitude: Some(47.154338), max_longitude: Some(16.6982231), min_longitude: Some(16.5270979)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زومباثلي"), ("az", "Sombatxey"), ("be", "Сомбатхей"), ("bg", "Сомбатхей"), ("bn", "সোম\u{9cd}বেতলি"), ("ca", "Szombathely"), ("ccp", "𑄎\u{1112e}𑄟\u{11134}𑄝𑄗𑄬𑄣\u{11128}"), ("ceb", "Szombathely"), ("cs", "Szombathely"), ("da", "Szombathely"), ("de", "Szombathely"), ("el", "Σζομπαθέλι"), ("en", "Szombathely"), ("es", "Szombathely"), ("et", "Szombathely"), ("eu", "Szombathely"), ("fa", "سمباتهی"), ("fi", "Szombathely"), ("fr", "Szombathely"), ("gl", "Szombathely"), ("gu", "ઝોમ\u{acd}બાથ\u{ac7}લી"), ("he", "סומבטהיי"), ("hi", "ज\u{93c}ोम\u{94d}बएथ\u{947}ली"), ("hr", "Sambotel"), ("hu", "Szombathely"), ("hy", "Սոմբատհեյ"), ("id", "Szombathely"), ("it", "Szombathely"), ("ja", "ソンバトヘイ"), ("ka", "სომბატჰეი"), ("kn", "ಸ\u{ccd}ಜಂಬಾಥ\u{cc6}ಲ\u{cbf}"), ("ko", "솜버트헤이"), ("lt", "Sombathėjus"), ("lv", "Sombatheja"), ("mk", "Сомбатхељ"), ("mr", "झो\u{902}बाथली"), ("ms", "Szombathely"), ("nb", "Szombathely"), ("nl", "Szombathely"), ("no", "Szombathely"), ("pl", "Szombathely"), ("pt", "Szombathely"), ("ro", "Szombathely"), ("ru", "Сомбатхей"), ("si", "සොම\u{dca}බතෙල\u{dd2}"), ("sk", "Szombathely"), ("sl", "Sombotel"), ("sq", "Szombathely"), ("sr", "Сомбатхељ"), ("sr_Latn", "Sombathelj"), ("sv", "Szombathely"), ("sw", "Szombathely"), ("ta", "கிட\u{bbe}"), ("te", "జ\u{c3e}మ\u{c4d}బత\u{c46}ల\u{c40}"), ("th", "โซมบ\u{e47}อตแฮย\u{e4c}"), ("tr", "Szombathely"), ("uk", "Сомбатгей"), ("ur", "سزومباتحیلی"), ("vi", "Szombathely"), ("zh", "松博特海伊")]),
                        unofficial_name_list: ["Szombathely"].to_vec(),
                    }
                ),
                (
                    "SK",
                    Subdivision{
                        name: "SK",
                        country_alpha2: Alpha2::HU,
                        code: "SK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.16213550000001), longitude: Some(20.1824712), max_latitude: Some(47.264515), min_latitude: Some(47.096289), max_longitude: Some(20.296315), min_longitude: Some(20.0710291)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سزولنوك"), ("az", "Solnok"), ("be", "Горад Сольнак"), ("bg", "Солнок"), ("bn", "সোনক"), ("ca", "Szolnok"), ("ccp", "𑄎\u{1112e}𑄣\u{11134}𑄚\u{11127}𑄇\u{11134}"), ("ceb", "Szolnok (kapital sa lalawigan)"), ("cs", "Szolnok"), ("cy", "Szolnok"), ("da", "Szolnok"), ("de", "Szolnok"), ("el", "Ζόλνοκ"), ("en", "Szolnok"), ("es", "Szolnok"), ("et", "Szolnok"), ("eu", "Szolnok"), ("fa", "سلنوک"), ("fi", "Szolnok"), ("fr", "Szolnok"), ("gl", "Szolnok"), ("gu", "સ\u{acd}ઝોલનોક"), ("he", "סולנוק"), ("hi", "ज\u{93c}ोलनोक"), ("hr", "Szolnok"), ("hu", "Szolnok"), ("hy", "Սոլնոկ"), ("id", "Szolnok"), ("it", "Szolnok"), ("ja", "ソルノク"), ("ka", "სოლნოკი"), ("kn", "ಸ\u{ccd}ಝೊಲ\u{ccd}ನೋಕ\u{ccd}"), ("ko", "솔노크"), ("lt", "Solnokas"), ("lv", "Solnoka"), ("mk", "Солнок"), ("mr", "स\u{94d}लोनोक"), ("ms", "Szolnok"), ("nb", "Szolnok"), ("nl", "Szolnok"), ("no", "Szolnok"), ("pl", "Szolnok"), ("pt", "Szolnok"), ("ro", "Solnoca"), ("ru", "Сольнок"), ("si", "සොල\u{dca}නොක\u{dca}"), ("sk", "Szolnok"), ("sl", "Szolnok"), ("sr", "Солнок"), ("sr_Latn", "Solnok"), ("sv", "Szolnok"), ("sw", "Szolnok"), ("ta", "ஸ\u{bcd}ஸ\u{bcd}வ\u{bcd}ல\u{bcd}நோக\u{bcd}"), ("te", "జ\u{c4b}ల\u{c4d}న\u{c4b}క\u{c4d}"), ("th", "โซลนอค"), ("tr", "Szolnok"), ("uk", "Сольнок"), ("ur", "سزولنوک"), ("vi", "Szolnok"), ("zh", "索尔诺克")]),
                        unofficial_name_list: ["Szolnok"].to_vec(),
                    }
                ),
                (
                    "SN",
                    Subdivision{
                        name: "SN",
                        country_alpha2: Alpha2::HU,
                        code: "SN",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.68166189999999), longitude: Some(16.5844795), max_latitude: Some(47.7590711), min_latitude: Some(47.6267501), max_longitude: Some(16.7462818), min_longitude: Some(16.4215489)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Sopron"), ("ar", "شوبرون"), ("az", "Şopron"), ("be", "Горад Шопран"), ("bg", "Шопрон"), ("ca", "Sopron"), ("ccp", "𑄥\u{11127}𑄛\u{11133}𑄢\u{11127}\u{1112e}𑄚\u{11134}"), ("ceb", "Sopron"), ("cs", "Sopron"), ("da", "Sopron"), ("de", "Sopron"), ("el", "Σόπρον"), ("en", "Sopron"), ("es", "Sopron"), ("et", "Sopron"), ("eu", "Sopron"), ("fa", "شپرن"), ("fi", "Sopron"), ("fr", "Sopron"), ("gl", "Sopron"), ("he", "שופרון"), ("hi", "सोपरोन"), ("hr", "Šopron"), ("hu", "Sopron"), ("hy", "Շոպրոն"), ("id", "Sopron"), ("it", "Sopron"), ("ja", "ショプロン"), ("ka", "შოპრონი"), ("kk", "Шопрон"), ("ko", "쇼프론"), ("lt", "Šopronas"), ("lv", "Šoprona"), ("mk", "Шопрон"), ("ms", "Sopron"), ("nb", "Sopron"), ("nl", "Sopron"), ("no", "Sopron"), ("pl", "Sopron"), ("pt", "Sopron"), ("ro", "Sopron"), ("ru", "Шопрон"), ("sk", "Šopron"), ("sl", "Šopron"), ("sr", "Шопрон"), ("sr_Latn", "Šopron"), ("sv", "Sopron"), ("tr", "Sopron"), ("uk", "Шопрон"), ("vi", "Sopron"), ("zh", "肖普朗")]),
                        unofficial_name_list: ["Sopron"].to_vec(),
                    }
                ),
                (
                    "SO",
                    Subdivision{
                        name: "SO",
                        country_alpha2: Alpha2::HU,
                        code: "SO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.554859), longitude: Some(17.5866732), max_latitude: Some(46.997146), min_latitude: Some(45.848009), max_longitude: Some(18.2128418), min_longitude: Some(16.8751449)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة شومود"), ("be", "медзье Шомадзь"), ("bg", "Шомод"), ("bn", "সোমোগি ক\u{9be}উন\u{9cd}টি"), ("ca", "Somogy"), ("ccp", "𑄥\u{1112e}𑄟\u{1112e}𑄉\u{1112d}"), ("cs", "Somogy"), ("da", "Somogy"), ("de", "Komitat Somogy"), ("el", "Σομόγκι"), ("en", "Somogy"), ("es", "Somogy"), ("et", "Somogyi komitaat"), ("eu", "Somogy"), ("fa", "سوموگی"), ("fi", "Somogy"), ("fr", "Somogy"), ("gl", "Somogy"), ("gu", "સોમોગી કાઉન\u{acd}ટી"), ("he", "שומוג׳"), ("hi", "सोमोगी काउ\u{902}टी"), ("hr", "Šomođska županija"), ("hu", "Somogy megye"), ("hy", "Սոմոգի"), ("id", "Somogy"), ("it", "Provincia di Somogy"), ("ja", "ショモジ県"), ("ka", "შომოდის მედიე"), ("kn", "ಸೊಮೊಗ\u{cbf} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "쇼모지 주"), ("lt", "Šomodis"), ("lv", "Šomoģas meģe"), ("mk", "Шомоѓ"), ("mn", "Шомодь"), ("mr", "सोमोगी काउ\u{902}टी"), ("ms", "Somogy"), ("nb", "Saomogy fylke"), ("nl", "Somogy"), ("no", "Saomogy fylke"), ("pl", "Komitat Somogy"), ("pt", "Somogy"), ("ro", "Județul Somogy"), ("ru", "Шомодь"), ("si", "සොමෝග\u{dd3} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Šomoďská župa"), ("sl", "Šomodska županija"), ("sr", "Шомођ"), ("sr_Latn", "Šomođ"), ("sv", "Somogy"), ("sw", "Somogy"), ("ta", "சோமோகி கவுண\u{bcd}டி"), ("te", "స\u{c4b}మ\u{c4b}గ\u{c3f} క\u{c4c}ంట\u{c40}"), ("th", "เขตปกครองโซโมก\u{e35}"), ("tr", "Somogy ili"), ("uk", "Шомодь"), ("ur", "شوموج کاؤنٹی"), ("vi", "Hạt Somogy"), ("zh", "紹莫吉州")]),
                        unofficial_name_list: ["Somogy"].to_vec(),
                    }
                ),
                (
                    "SS",
                    Subdivision{
                        name: "SS",
                        country_alpha2: Alpha2::HU,
                        code: "SS",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.3474326), longitude: Some(18.7062293), max_latitude: Some(46.404041), min_latitude: Some(46.2943501), max_longitude: Some(18.851653), min_longitude: Some(18.6108151)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سكسارد"), ("az", "Seksard"), ("be", "Горад Сексард"), ("bg", "Сексард"), ("bn", "শেকজ\u{9be}দ"), ("ca", "Szekszárd"), ("ccp", "𑄎𑄬𑄇\u{11134}𑄎𑄢\u{11134}𑄓\u{11134}"), ("ceb", "Szekszárd"), ("cs", "Szekszárd"), ("cy", "Szekszárd"), ("da", "Szekszárd"), ("de", "Szekszárd"), ("el", "Σέξαρντ (Σέκσαρντ)"), ("en", "Szekszárd"), ("es", "Szekszárd"), ("et", "Szekszárd"), ("eu", "Szekszárd"), ("fa", "سکسارد"), ("fi", "Szekszárd"), ("fr", "Szekszárd"), ("gl", "Szekszárd"), ("gu", "જ\u{abc}\u{ac7}ક\u{acd}સર\u{acd}ડ"), ("he", "סקסארד"), ("hi", "सज\u{93c}एक\u{94d}सजार\u{94d}ड"), ("hr", "Szekszárd"), ("hu", "Szekszárd"), ("hy", "Սեկսարդ"), ("id", "Szekszárd"), ("it", "Szekszárd"), ("ja", "セクサールド"), ("kn", "ಝ\u{cc6}ಕ\u{ccd}ಸ\u{ccd}ಕ\u{ccd}ಸ\u{ccd}ಝರ\u{ccd}ಡ\u{ccd}"), ("ko", "섹사르드"), ("lt", "Seksardas"), ("lv", "Seksārda"), ("mk", "Сексард"), ("mr", "झ\u{947}कसर\u{94d}ड"), ("ms", "Szekszárd"), ("nb", "Szekszárd"), ("nl", "Szekszárd"), ("no", "Szekszárd"), ("pl", "Szekszárd"), ("pt", "Szekszárd"), ("ro", "Szekszárd"), ("ru", "Сексард"), ("si", "සෙක\u{dca}ස\u{dcf}ර\u{dca}ඩ\u{dca}"), ("sk", "Szekszárd"), ("sl", "Szekszárd"), ("sr", "Сексард"), ("sr_Latn", "Seksard"), ("sv", "Szekszárd"), ("sw", "Szekszárd"), ("ta", "ஸிக\u{bcd}ஸ\u{bcd}ர\u{bcd}டு"), ("te", "స\u{c46}క\u{c4d}జ\u{c3e}ర\u{c4d}డ\u{c4d}"), ("th", "เซคซาร\u{e4c}ด"), ("tr", "Szekszárd"), ("uk", "Сексард"), ("ur", "زیکسارڈ"), ("vi", "Szekszárd"), ("zh", "塞克薩德")]),
                        unofficial_name_list: ["Szekszárd"].to_vec(),
                    }
                ),
                (
                    "ST",
                    Subdivision{
                        name: "ST",
                        country_alpha2: Alpha2::HU,
                        code: "ST",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.0935237), longitude: Some(19.7999813), max_latitude: Some(48.178421), min_latitude: Some(48.039501), max_longitude: Some(19.9221181), min_longitude: Some(19.7267319)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سالجوتارجان"), ("az", "Şalqotariyan"), ("bg", "Шалготарян"), ("bn", "স\u{9be}ল\u{9cd}গোত\u{9be}রিয\u{9bc}\u{9be}ন"), ("ca", "Salgótarján"), ("ccp", "𑄥𑄣\u{11134}𑄉\u{1112e}𑄑𑄢\u{11134}𑄎𑄚\u{11134}"), ("ceb", "Salgótarján"), ("cs", "Salgótarján"), ("cy", "Salgótarján"), ("da", "Salgótarján"), ("de", "Salgótarján"), ("el", "Σαλγκόταριαν"), ("en", "Salgótarján"), ("es", "Salgótarján"), ("et", "Salgótarján"), ("eu", "Salgótarján"), ("fa", "شالگوتاریان"), ("fi", "Salgótarján"), ("fr", "Salgótarján"), ("gl", "Salgótarján"), ("gu", "સાલ\u{acd}ગોટાર\u{acd}જાન"), ("he", "שאלגוטאריאן"), ("hi", "साल\u{94d}गोटार\u{94d}जन"), ("hr", "Salgótarján"), ("hu", "Salgótarján"), ("id", "Salgótarján"), ("it", "Salgótarján"), ("ja", "シャルゴータルヤーン"), ("ka", "შალგოტარიანი"), ("kn", "ಸಾಲ\u{ccd}ಗೋಟಾರ\u{ccd}ಜಾನ\u{ccd}"), ("ko", "셜고터랸"), ("lt", "Šalgotarjanas"), ("lv", "Šalgotarjana"), ("mk", "Шалготарјан"), ("mr", "सलगोर\u{94d}त\u{94d}जन"), ("ms", "Salgótarján"), ("nb", "Salgótarján"), ("nl", "Salgótarján"), ("no", "Salgótarján"), ("pl", "Salgótarján"), ("pt", "Salgótarján"), ("ro", "Salgótarján"), ("ru", "Шальготарьян"), ("si", "සල\u{dca}ගෝටර\u{dca}ජ\u{dcf}න\u{dca}"), ("sk", "Salgótarján"), ("sl", "Salgótarján"), ("sr", "Шалготарјан"), ("sr_Latn", "Šalgotarjan"), ("sv", "Salgótarján"), ("sw", "Salgótarján"), ("ta", "ச\u{bbe}ல\u{bcd}கோடர\u{bcd}ஜ\u{bbe}ன\u{bcd}"), ("te", "స\u{c3e}ల\u{c4d}గ\u{c4a}ట\u{c3e}ర\u{c4d}జన\u{c4d}"), ("th", "ซาลโกทาจาน"), ("tr", "Salgótarján"), ("uk", "Шалґотарʼян"), ("ur", "سالجوتارجان"), ("vi", "Salgótarján"), ("zh", "紹爾戈陶爾揚")]),
                        unofficial_name_list: ["Salgótarján"].to_vec(),
                    }
                ),
                (
                    "SZ",
                    Subdivision{
                        name: "SZ",
                        country_alpha2: Alpha2::HU,
                        code: "SZ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(48.0394954), longitude: Some(22.00333), max_latitude: Some(48.4256611), min_latitude: Some(47.5916749), max_longitude: Some(22.896544), min_longitude: Some(21.0789789)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زابولكس-زاتمار-بيريج"), ("az", "Sabolç-Satmar-Bereq"), ("be", "Сабальч-Сатмар-Берэг"), ("bg", "Саболч-Сатмар-Берег"), ("bn", "য\u{9be}বলচেস-য\u{9be}ত\u{9cd}ম\u{9be}র-বেরেগ ক\u{9be}উন\u{9cd}টি"), ("ca", "Szabolcs-Szatmár-Bereg"), ("ccp", "𑄎𑄝\u{1112e}𑄣\u{11134}𑄇\u{11134}-𑄎𑄖\u{11134}𑄟𑄢\u{11134}-𑄝𑄬𑄢𑄬𑄇\u{11134}"), ("ceb", "Szabolcs-Szatmár-Bereg"), ("cs", "Szabolcs-Szatmár-Bereg"), ("da", "Szabolcs-Szatmár-Bereg"), ("de", "Komitat Szabolcs-Szatmár-Bereg"), ("el", "Σζαμπόλκς-Σζατμάρ-Μπερέκ"), ("en", "Szabolcs-Szatmár-Bereg"), ("es", "Szabolcs-Szatmár-Bereg"), ("et", "Szabolcs-Szatmár-Beregi komitaat"), ("eu", "Szabolcs-Szatmár-Bereg"), ("fa", "زابولکس-زاتمار-بریج"), ("fi", "Szabolcs-Szatmár-Bereg"), ("fr", "Szabolcs-Szatmár-Bereg"), ("gl", "Szabolcs-Szatmár-Bereg"), ("gu", "ઝાબોલ\u{acd}ક\u{acd}સ-ઝાટમાર-બ\u{ac7}ર\u{ac7}ગ કાઉન\u{acd}ટી"), ("he", "סאבולץ׳-סאטמר-ברג"), ("hi", "सबोल\u{94d}ट-सतमार-ब\u{947}र\u{947}ग काउ\u{902}टी"), ("hr", "Szabolčko-szatmársko-bereška županija"), ("hu", "Szabolcs-Szatmár-Bereg megye"), ("hy", "Սաբոլճ-Սատմար-Բերեգ շրջան"), ("id", "Szabolcs-Szatmár-Bereg"), ("it", "Provincia di Szabolcs-Szatmár-Bereg"), ("ja", "サボルチ・サトマール・ベレグ県"), ("ka", "საბოლჩ-სატმარ-ბერეგის მედიე"), ("kn", "ಸ\u{ccd}ಝಾಬಾಲ\u{ccd}ಕ\u{ccd}ಸ\u{ccd}-ಸ\u{ccd}ಜತ\u{ccd}ಮಾರ\u{ccd}-ಬ\u{cc6}ರ\u{cc6}ಗ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}ಯು"), ("ko", "서볼치서트마르베레그 주"), ("lt", "Sabolčas-Satmaras-Beregas"), ("lv", "Sabolčas-Satmāras-Beregas meģe"), ("mk", "Саболч-Сатмар-Берег"), ("mr", "स\u{94d}झबॉल\u{94d}क\u{94d}स-झाटमार-ब\u{947}यग काउ\u{902}टी"), ("ms", "Szabolcs-Szatmár-Bereg"), ("nb", "Szaboles Szatmar Bereg fylke"), ("nl", "Szabolcs-Szatmár-Bereg"), ("no", "Szaboles Szatmar Bereg fylke"), ("pl", "Komitat Szabolcs-Szatmár-Bereg"), ("pt", "Szabolcs-Szatmár-Bereg"), ("ro", "Județul Szabolcs-Szatmár-Bereg"), ("ru", "Сабольч-Сатмар-Берег"), ("si", "ස\u{dca}කබෝල\u{dca}ක\u{dca}ස\u{dca}-සට\u{dca}ම\u{dcf}ර\u{dca}-බෙරෙග\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Sabolčsko-satmársko-berežská župa"), ("sl", "Županija Szabolcs-Szatmár-Bereg"), ("sr", "Саболч-Сатмар-Берег"), ("sr_Latn", "Sabolč-Satmar-Bereg"), ("sv", "Szabolcs-Szatmár-Bereg"), ("sw", "Szabolcs-Szatmár-Bereg"), ("ta", "சசபல\u{bcd}க\u{bcd}ஸ\u{bcd} -சச\u{bbe}த\u{bcd}மர\u{bcd}-பேரெக\u{bcd} கவுண\u{bcd}டி"), ("te", "జ\u{c3e}బ\u{c4b}ల\u{c4d}క\u{c4d}-స\u{c3e}ట\u{c4d}మర\u{c4d}-బ\u{c46}ర\u{c46}గ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "สซาบอค ซแซทมาร\u{e4c} เบเรค"), ("tr", "Szabolcs-Szatmár-Bereg ili"), ("uk", "Саболч-Сатмар-Береґ"), ("ur", "سابولچ-ساتمار-بیریگ کاؤنٹی"), ("vi", "Hạt Szabolcs-Szatmár-Bereg"), ("zh", "索博爾奇-索特馬爾-貝拉格州")]),
                        unofficial_name_list: ["Szabolcs-Szatmár-Bereg"].to_vec(),
                    }
                ),
                (
                    "TB",
                    Subdivision{
                        name: "TB",
                        country_alpha2: Alpha2::HU,
                        code: "TB",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.569246), longitude: Some(18.404818), max_latitude: Some(47.6096499), min_latitude: Some(47.48642599999999), max_longitude: Some(18.4973549), min_longitude: Some(18.3379901)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "تاتابانيا"), ("az", "Tatabanya"), ("be", "Горад Татабанья"), ("bg", "Татабаня"), ("bn", "ত\u{9be}ত\u{9be}ব\u{9be}ন\u{9be}য\u{9bc}\u{9be}ন"), ("ca", "Tatabánya"), ("ccp", "𑄑𑄑𑄝𑄚\u{11128}𑄠"), ("ceb", "Tatabánya"), ("cs", "Tatabánya"), ("cy", "Tatabánya"), ("da", "Tatabánya"), ("de", "Tatabánya"), ("el", "Ταταμπάνια"), ("en", "Tatabánya"), ("es", "Tatabánya"), ("et", "Tatabánya"), ("eu", "Tatabánya"), ("fa", "تاتابانیا"), ("fi", "Tatabánya"), ("fr", "Tatabánya"), ("gl", "Tatabánya"), ("gu", "તતાબાન\u{acd}યા"), ("he", "טטבאניה"), ("hi", "ताताबनया"), ("hr", "Tatabánya"), ("hu", "Tatabánya"), ("hy", "Թաթաբանյա"), ("id", "Tatabánya"), ("it", "Tatabánya"), ("ja", "タタバーニャ"), ("ka", "ტატაბანია"), ("kn", "ಟಾಟಾಬನ\u{ccd}ಯ"), ("ko", "터터바녀"), ("lt", "Tatabania"), ("lv", "Tatabāņa"), ("mk", "Татабања"), ("mr", "तत\u{94d}ताब\u{94d}यआ"), ("ms", "Tatabánya"), ("nb", "Tatabanya"), ("nl", "Tatabánya"), ("no", "Tatabanya"), ("pl", "Tatabánya"), ("pt", "Tatabánya"), ("ro", "Tatabánya"), ("ru", "Татабанья"), ("si", "ටටබන\u{dca}ය\u{dcf}"), ("sk", "Tatabánya"), ("sl", "Tatabánya"), ("sr", "Татабања"), ("sr_Latn", "Tatabanja"), ("sv", "Tatabánya"), ("sw", "Tatabánya"), ("ta", "ட\u{bbe}ட\u{bbe}ப\u{bbe}ன\u{bcd}ய\u{bbe}"), ("te", "త\u{c3e}తబన\u{c4d}య\u{c3e}"), ("th", "ทาทาบ\u{e31}นย\u{e48}า"), ("tr", "Tatabánya"), ("uk", "Татабанья"), ("ur", "تاتابانیا"), ("vi", "Tatabánya"), ("zh", "陶陶巴尼奧")]),
                        unofficial_name_list: ["Tatabánya"].to_vec(),
                    }
                ),
                (
                    "TO",
                    Subdivision{
                        name: "TO",
                        country_alpha2: Alpha2::HU,
                        code: "TO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.4258265), longitude: Some(18.7752069), max_latitude: Some(46.5007989), min_latitude: Some(46.3655339), max_longitude: Some(18.8259278), min_longitude: Some(18.7066851)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة تولنا"), ("be", "медзье Тольна"), ("bg", "Толна"), ("bn", "টোলন\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Tolna"), ("ccp", "𑄑\u{1112e}𑄣\u{11134}𑄚"), ("ceb", "Tolna megye"), ("cs", "Tolna"), ("da", "Tolna"), ("de", "Komitat Tolna"), ("el", "Κομητεία Τόλνα"), ("en", "Tolna"), ("es", "Tolna"), ("et", "Tolna komitaat"), ("eu", "Tolna"), ("fa", "تولنا"), ("fi", "Tolna"), ("fr", "Tolna"), ("gl", "Condado de Tolna"), ("gu", "ટોલના કાઉન\u{acd}ટી"), ("he", "מחוז טולנה"), ("hi", "टोलना काउ\u{902}टी"), ("hr", "Tolnanska županija"), ("hu", "Tolna megye"), ("id", "Tolna"), ("it", "Provincia di Tolna"), ("ja", "トルナ県"), ("ka", "ტოლნის მედიე"), ("kn", "ಟೋಲ\u{ccd}ನಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "톨너 주"), ("lt", "Tolna"), ("lv", "Tolnas meģe"), ("mk", "Толна"), ("mr", "टोलना काउ\u{902}टी"), ("ms", "Tolna"), ("nb", "Tolna Fylke"), ("nl", "Tolna"), ("no", "Tolna Fylke"), ("pl", "Komitat Tolna"), ("pt", "Tolna"), ("ro", "Județul Tolna"), ("ru", "Тольна"), ("si", "ටොල\u{dca}න\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Tolnianska župa"), ("sl", "Tolna"), ("sr", "Толна"), ("sr_Latn", "Tolna"), ("sv", "Tolna"), ("sw", "Tolna"), ("ta", "தொல\u{bcd}ந கவுண\u{bcd}டி"), ("te", "ట\u{c4b}ల\u{c4d}న\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "มณฑลโตลน\u{e48}า"), ("tr", "Tolna ili"), ("uk", "Толна"), ("ur", "تولنا کاؤنٹی"), ("vi", "Hạt Tolna"), ("zh", "托爾瑙州")]),
                        unofficial_name_list: ["Tolna"].to_vec(),
                    }
                ),
                (
                    "VA",
                    Subdivision{
                        name: "VA",
                        country_alpha2: Alpha2::HU,
                        code: "VA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.09291109999999), longitude: Some(16.6812183), max_latitude: Some(47.460547), min_latitude: Some(46.714203), max_longitude: Some(17.295531), min_longitude: Some(16.1138579)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فاس"), ("be", "Ваш"), ("bg", "Ваш"), ("bn", "ভ\u{9be}স ক\u{9be}উন\u{9cd}টি"), ("ca", "Vas"), ("ccp", "𑄞𑄌\u{11134}"), ("ceb", "Vas megye"), ("cs", "Vas"), ("da", "Vas"), ("de", "Komitat Vas"), ("el", "Βας"), ("en", "Vas"), ("es", "Vas"), ("et", "Vasi komitaat"), ("eu", "Vas konderria"), ("fa", "شهرستان واس"), ("fi", "Vas"), ("fr", "Vas"), ("gl", "Vas"), ("gu", "વાસ કાઉન\u{acd}ટી"), ("he", "מחוז ואש"), ("hi", "वास काउ\u{902}टी"), ("hr", "Željezna županija"), ("hu", "Vas megye"), ("hy", "Վաս"), ("id", "Vas"), ("it", "Provincia di Vas"), ("ja", "ヴァシュ県"), ("ka", "ვაშის მედიე"), ("kn", "ವಾಸ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "버시 주"), ("lt", "Vašas"), ("lv", "Vašas meģe"), ("mk", "Ваш"), ("mn", "Ваш"), ("mr", "वास काउ\u{902}टी"), ("ms", "Vas"), ("nb", "Vas Fylke"), ("nl", "Vas"), ("no", "Vas Fylke"), ("pl", "Komitat Vas"), ("pt", "Vas"), ("ro", "Județul Vas"), ("ru", "Ваш"), ("si", "ව\u{dcf}ස\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vašská župa"), ("sl", "Železna županija"), ("sr", "Ваш"), ("sr_Latn", "Vaš"), ("sv", "Vas"), ("sw", "Vas"), ("ta", "வ\u{bbe}ஸ\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c3e}స\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เขตปกครองแวส"), ("tr", "Vas ili"), ("uk", "Ваш"), ("ur", "واش کاؤنٹی"), ("vi", "Hạt Vas"), ("zh", "沃什州")]),
                        unofficial_name_list: ["Vas"].to_vec(),
                    }
                ),
                (
                    "VE",
                    Subdivision{
                        name: "VE",
                        country_alpha2: Alpha2::HU,
                        code: "VE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.1028087), longitude: Some(17.9093019), max_latitude: Some(47.205633), min_latitude: Some(47.0459651), max_longitude: Some(17.9937111), min_longitude: Some(17.8207858)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة فسبرم"), ("be", "Веспрэм"), ("bg", "Веспрем"), ("bn", "ভেস\u{9cd}প\u{9cd}রেম ক\u{9be}উন\u{9cd}টি"), ("ca", "Veszprém"), ("ccp", "𑄞𑄬𑄌\u{11134}𑄛\u{11133}𑄢𑄬𑄟\u{11134} 𑄇𑄅\u{1112a}𑄚\u{11134}𑄑\u{11128}"), ("ceb", "Veszprém megye"), ("cs", "Veszprém"), ("da", "Veszprém"), ("de", "Komitat Veszprém"), ("el", "Κομητεία Βετσπρέμ"), ("en", "Veszprém County"), ("es", "Veszprém"), ("et", "Veszprémi komitaat"), ("eu", "Veszprém"), ("fa", "وزپریم"), ("fi", "Veszprém"), ("fr", "Veszprém"), ("gl", "Condado de Veszprém"), ("gu", "વ\u{ac7}સ\u{acd}ઝપ\u{acd}ર\u{ac7}મ કાઉન\u{acd}ટી"), ("he", "מחוז וספרם"), ("hi", "व\u{947}स\u{94d}ज\u{93c}प\u{94d}रिम काउ\u{902}टी"), ("hr", "Vesprimska županija"), ("hu", "Veszprém megye"), ("id", "Veszprém"), ("it", "Provincia di Veszprém"), ("ja", "ヴェスプレーム県"), ("ka", "ვესპრემის მედიე"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಜ\u{ccd}ಪ\u{ccd}ರ\u{cc6}ಮ\u{ccd} ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "베스프렘 주"), ("lt", "Vesprėmas"), ("lv", "Vesprēmas meģe"), ("mk", "Веспрем"), ("mr", "व\u{947}स\u{94d}झप\u{94d}रिम काउ\u{902}टी"), ("ms", "Veszprém"), ("nb", "Veszprém"), ("nl", "Veszprém"), ("no", "Veszprém"), ("pl", "Komitat Veszprém"), ("pt", "Veszprém"), ("ro", "Județul Veszprém"), ("ru", "Веспрем"), ("si", "වෙස\u{dca}ප\u{dca}\u{200d}රේම\u{dca} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Vesprémska župa"), ("sl", "Veszprém"), ("sr", "Веспрем"), ("sr_Latn", "Vesprem"), ("sv", "Veszprém"), ("sw", "Veszprém"), ("ta", "வெஸ\u{bcd}ஸ\u{bcd}ப\u{bcd}ரேம\u{bcd} கவுண\u{bcd}டி"), ("te", "వ\u{c46}స\u{c4d}\u{200c}ప\u{c4d}ర\u{c46}మ\u{c4d} క\u{c4c}ంట\u{c40}"), ("th", "เวซสเพรม ค\u{e31}นทร\u{e35}\u{e48}"), ("tr", "Veszprém ili"), ("uk", "Веспрем"), ("ur", "ویسپریم کاؤنٹی"), ("vi", "Hạt Veszprém"), ("zh", "維斯普雷姆州")]),
                        unofficial_name_list: ["Veszprém"].to_vec(),
                    }
                ),
                (
                    "VM",
                    Subdivision{
                        name: "VM",
                        country_alpha2: Alpha2::HU,
                        code: "VM",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(47.1028087), longitude: Some(17.9093019), max_latitude: Some(47.205633), min_latitude: Some(47.0459651), max_longitude: Some(17.9937111), min_longitude: Some(17.8207858)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "فيسبرم"), ("az", "Vesprem"), ("be", "Горад Веспрэм"), ("bg", "Веспрем²"), ("bn", "ভেসপ\u{9cd}রেম"), ("ca", "Veszprém²"), ("ccp", "𑄞𑄬𑄌\u{11134}𑄛\u{11133}𑄢𑄬𑄟\u{11134}"), ("ceb", "Veszprém"), ("cs", "Veszprém²"), ("cy", "Veszprém"), ("da", "Veszprém²"), ("de", "Veszprém"), ("el", "Βεσζπρέμ"), ("en", "Veszprém"), ("es", "Veszprém²"), ("et", "Veszprém"), ("eu", "Veszprém²"), ("fa", "وسپریم"), ("fi", "Veszprém²"), ("fr", "Veszprém²"), ("gl", "Veszprém"), ("gu", "વ\u{ac7}સ\u{acd}ઝપ\u{acd}ર\u{ac7}મ"), ("he", "וספרם"), ("hi", "व\u{947}स\u{94d}ज\u{93c}प\u{94d}र\u{947}म"), ("hr", "Vesprim"), ("hu", "Veszprém"), ("hy", "Վեսպրեմ"), ("id", "Veszprém²"), ("it", "Veszprém"), ("ja", "ヴェスプレーム"), ("ka", "ვესპრემი"), ("kn", "ವ\u{cc6}ಸ\u{ccd}ಜ\u{ccd}ಪ\u{ccd}ರ\u{cc6}ಮ\u{ccd}"), ("ko", "베스프렘"), ("lt", "Vesprėmas²"), ("lv", "Vesprēma"), ("mk", "Веспрем²"), ("mr", "व\u{947}सीझप\u{94d}र\u{947}म"), ("ms", "Veszprém²"), ("nb", "Veszprém"), ("nl", "Veszprém²"), ("no", "Veszprém²"), ("pl", "Veszprém"), ("pt", "Veszprém²"), ("ro", "Veszprém"), ("ru", "Веспрем²"), ("si", "වෙස\u{dca}ප\u{dca}\u{200d}රේම\u{dca}"), ("sk", "Vesprém"), ("sl", "Veszprém²"), ("sr", "Веспрем²"), ("sr_Latn", "Vesprem²"), ("sv", "Veszprém²"), ("sw", "Veszprém²"), ("ta", "வெஸ\u{bcd}பிரேம\u{bcd}"), ("te", "వ\u{c47}స\u{c4d}జ\u{c4d} ప\u{c4d}ర\u{c47}మ\u{c4d}"), ("th", "เวสเปรม"), ("tr", "Veszprém"), ("uk", "Веспрем²"), ("ur", "ویسزپریم"), ("vi", "Veszprém"), ("zh", "維斯普雷姆")]),
                        unofficial_name_list: ["Veszprém"].to_vec(),
                    }
                ),
                (
                    "ZA",
                    Subdivision{
                        name: "ZA",
                        country_alpha2: Alpha2::HU,
                        code: "ZA",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.73844039999999), longitude: Some(16.9152252), max_latitude: Some(47.048693), min_latitude: Some(46.290997), max_longitude: Some(17.3809683), min_longitude: Some(16.361846)}),
                        comments: None,
                        subdivision_type: SubdivisionType::County,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة زالا"), ("bg", "Зала"), ("bn", "জ\u{9be}ল\u{9be} ক\u{9be}উন\u{9cd}টি"), ("ca", "Zala"), ("ccp", "𑄎𑄣"), ("ceb", "Zala megye"), ("cs", "Zala"), ("da", "Zala"), ("de", "Komitat Zala"), ("el", "Ζάλα"), ("en", "Zala"), ("es", "Zala"), ("et", "Zala komitaat"), ("eu", "Zala konderria"), ("fa", "زالا"), ("fi", "Zala"), ("fr", "Zala"), ("gl", "Zala"), ("gu", "ઝાલા કાઉન\u{acd}ટી"), ("he", "זאלה"), ("hi", "ज\u{93c}ला काउ\u{902}टी"), ("hr", "Zalska županija"), ("hu", "Zala megye"), ("id", "Zala"), ("it", "Provincia di Zala"), ("ja", "ザラ県"), ("ka", "ზალის მედიე"), ("kn", "ಝಲಾ ಕ\u{ccc}ಂಟ\u{cbf}"), ("ko", "절러 주"), ("lt", "Zala"), ("lv", "Zalas meģe"), ("mk", "Зала"), ("mn", "Зала"), ("mr", "झला काउ\u{902}टी"), ("ms", "Zala"), ("nb", "Zala Fylke"), ("nl", "Zala"), ("no", "Zala Fylke"), ("pl", "Komitat Zala"), ("pt", "Zala"), ("ro", "Județul Zala"), ("ru", "Зала"), ("si", "ස\u{dcf}ල\u{dcf} ප\u{dca}\u{200d}ර\u{dcf}න\u{dca}තය"), ("sk", "Zalianska župa"), ("sl", "Zala"), ("sr", "Зала"), ("sr_Latn", "Zala"), ("sv", "Zala"), ("sw", "Zala"), ("ta", "ச\u{bbe}ல\u{bbe} கவுண\u{bcd}டி"), ("te", "జ\u{c3e}ల\u{c3e} క\u{c4c}ంట\u{c40}"), ("th", "เขตปกครองซาล\u{e48}า"), ("tr", "Zala ili"), ("uk", "Зала"), ("ur", "زالا کاؤنٹی"), ("vi", "Hạt Zala"), ("zh", "佐洛州")]),
                        unofficial_name_list: ["Zala"].to_vec(),
                    }
                ),
                (
                    "ZE",
                    Subdivision{
                        name: "ZE",
                        country_alpha2: Alpha2::HU,
                        code: "ZE",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(46.8416936), longitude: Some(16.8416322), max_latitude: Some(46.90018389999999), min_latitude: Some(46.780261), max_longitude: Some(16.90627), min_longitude: Some(16.768593)}),
                        comments: None,
                        subdivision_type: SubdivisionType::CityWithCountyRights,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "زالاجيرسيج"), ("az", "Zalaeqerseq"), ("be", "Горад Залаэгерсег"), ("bg", "Залаегерсег"), ("bn", "জ\u{9be}ল\u{9be}য\u{9bc}েসেগ"), ("ca", "Zalaegerszeg"), ("ccp", "𑄎𑄣𑄎𑄬𑄢\u{11134}𑄎𑄬𑄇\u{11134}"), ("ceb", "Zalaegerszeg"), ("cs", "Zalaegerszeg"), ("cy", "Zalaegerszeg"), ("da", "Zalaegerszeg"), ("de", "Zalaegerszeg"), ("el", "Ζάλαεγκερσεγκ"), ("en", "Zalaegerszeg"), ("es", "Zalaegerszeg"), ("et", "Zalaegerszeg"), ("eu", "Zalaegerszeg"), ("fa", "زالائگرسگ"), ("fi", "Zalaegerszeg"), ("fr", "Zalaegerszeg"), ("gl", "Zalaegerszeg"), ("gu", "ઝલાગ\u{ac7}ર\u{acd}ઝ\u{ac7}ગ"), ("he", "זאלאגרסג"), ("hi", "ज\u{93c}लाइजर\u{94d}सजग"), ("hr", "Jegersek"), ("hu", "Zalaegerszeg"), ("hy", "Զալաեգերսեգ"), ("id", "Zalaegerszeg"), ("it", "Zalaegerszeg"), ("ja", "ザラエゲルセグ"), ("ka", "ზალაეგერსეგი"), ("kn", "ಝಲೇಜ\u{cc6}ರ\u{ccd}ಜ\u{cc6}ಗ\u{ccd}"), ("ko", "절러에게르세그"), ("lt", "Zalaegersegas"), ("lv", "Zalaegersega"), ("mk", "Залаегерсег"), ("mr", "झलाग\u{947}र\u{94d}स\u{947}झ\u{947}ग"), ("ms", "Zalaegerszeg"), ("nb", "Zalaegerszeg"), ("nl", "Zalaegerszeg"), ("no", "Zalaegerszeg"), ("pl", "Zalaegerszeg"), ("pt", "Zalaegerszeg"), ("ro", "Zalaegerszeg"), ("ru", "Залаэгерсег"), ("si", "සලගර\u{dca}සෙග\u{dca}"), ("sk", "Zalaegerszeg"), ("sl", "Zalaegerszeg"), ("sr", "Залаегерсег"), ("sr_Latn", "Zalaegerseg"), ("sv", "Zalaegerszeg"), ("sw", "Zalaegerszeg"), ("ta", "ச\u{bbe}க\u{bcd}லேகெர\u{bcd}செக\u{bcd}"), ("te", "జలయ\u{c46}జ\u{c46}ర\u{c4d}సజగ\u{c4d}"), ("th", "ซาลาเอแกร\u{e4c}เซก."), ("tr", "Zalaegerszeg"), ("uk", "Залаеґерсеґ"), ("ur", "زالایجیرسزیج"), ("vi", "Zalaegerszeg"), ("zh", "佐洛埃格塞格")]),
                        unofficial_name_list: ["Zalaegerszeg"].to_vec(),
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
#[cfg(feature = "hu")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::HU,
        alpha3: Alpha3::HUN,
        address_format: Some("{{recipient}}\n{{city}}\n{{street}}\n{{postalcode}}\n{{country}}"),
        continent: Continent::Europe,
        country_code: 36,
        currency_code: "HUF",
        gec: Some(GEC::HU),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("HUN"),
        iso_long_name: "Hungary",
        iso_short_name: "Hungary",
        official_language_list: ["hu"].to_vec(),
        spoken_language_list: ["hu"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8, 9].to_vec(),
        national_prefix: "06",
        nationality: Some("Hungarian"),
        number: "348",
        postal_code: true,
        postal_code_format: Some("\\d{4}"),
        region: Some(Region::Europe),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::EasternEurope),
        un_locode: "HU",
        unofficial_name_list: [
            "Hungary",
            "Ungarn",
            "Hongrie",
            "Hungría",
            "ハンガリー",
            "Hongarije",
        ]
        .to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Hungary"),
            ("af", "Hongarye"),
            ("ak", "Hungary"),
            ("am", "ሀንጒሱ"),
            ("an", "Hungary"),
            ("ar", "المجر (هنغاريا)"),
            ("as", "হ\u{9be}ংগেৰী"),
            ("ay", "Hungary"),
            ("az", "Macarıstan"),
            ("ba", "Hungary"),
            ("be", "Венгрыя"),
            ("bg", "Унгария"),
            ("bi", "Hungary"),
            ("bn", "হ\u{9be}ঙ\u{9cd}গেরি"),
            ("bn_IN", "হ\u{9be}ঙ\u{9cd}গেরি"),
            ("br", "Hungaria"),
            ("bs", "Mađarska"),
            ("ca", "Hongria"),
            ("ce", "Венгри"),
            ("ch", "Hungary"),
            ("cs", "Maďarsko"),
            ("cv", "Венгри"),
            ("cy", "Hwngari"),
            ("da", "Ungarn"),
            ("de", "Ungarn"),
            ("dv", "ހ\u{7a6}ނ\u{7b0}ގ\u{7ad}ރ\u{7a9}"),
            ("dz", "ཧང་ག་ར\u{f72}།"),
            ("ee", "Hungary"),
            ("el", "Ουγγαρία"),
            ("en", "Hungary"),
            ("eo", "Hungario"),
            ("es", "Hungría"),
            ("et", "Ungari"),
            ("eu", "Hungaria"),
            ("fa", "مجارستان"),
            ("ff", "Hunngariya"),
            ("fi", "Unkari"),
            ("fo", "Ungarn"),
            ("fr", "Hongrie"),
            ("fy", "Hongarije"),
            ("ga", "An Ungáir"),
            ("gl", "Hungría"),
            ("gn", "Hungary"),
            ("gu", "હ\u{a82}ગ\u{ac7}રી"),
            ("gv", "Yn Ungaar"),
            ("ha", "Hungariya"),
            ("he", "הונגריה"),
            ("hi", "ह\u{902}गरी"),
            ("hr", "Mađarska"),
            ("ht", "Ongri"),
            ("hu", "Magyarország"),
            ("hy", "Հունգարիա"),
            ("ia", "Hungaria"),
            ("id", "Hongaria"),
            ("io", "Hungaria"),
            ("is", "Ungverjaland"),
            ("it", "Ungheria"),
            ("iu", "Hungary"),
            ("ja", "ハンガリー"),
            ("ka", "უნგრეთი"),
            ("ki", "Macartsa"),
            ("kk", "Венгрия"),
            ("kl", "Hungary"),
            ("km", "ហ\u{17bb}ងគ\u{17d2}រ\u{17b8}"),
            ("kn", "ಹಂಗರ\u{cbf}"),
            ("ko", "헝가리"),
            ("ku", "Macaristan"),
            ("kv", "Мадьяр Му"),
            ("kw", "Hungari"),
            ("ky", "Мажарстан"),
            ("lo", "ປະເທດຮ\u{ebb}ງກະລ\u{eb5}"),
            ("lt", "Vengrija"),
            ("lv", "Ungārija"),
            ("mi", "Hanekari"),
            ("mk", "Унгарија"),
            ("ml", "ഹംഗറി"),
            ("mn", "Унгар"),
            ("mr", "ह\u{902}ग\u{947}री"),
            ("ms", "Hungari"),
            ("mt", "Ungerija"),
            (
                "my",
                "ဟန\u{103a}ဂေရ\u{102e}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Ungari"),
            ("nb", "Ungarn"),
            ("ne", "हङ\u{94d}ग\u{947}री"),
            ("nl", "Hongarije"),
            ("nn", "Ungarn"),
            ("nv", "Hángewii"),
            ("oc", "Ongria"),
            ("or", "ହଙ\u{b4d}ଗ\u{b3e}ରୀ"),
            ("pa", "ਹ\u{a70}ਗਰੀ"),
            ("pi", "ह\u{902}गरी"),
            ("pl", "Węgry"),
            ("ps", "مجارستان"),
            ("pt", "Hungria"),
            ("pt_BR", "Hungria"),
            ("ro", "Ungaria"),
            ("ru", "Венгрия"),
            ("rw", "Hongiriya"),
            ("sc", "Ungheria"),
            ("sd", "Hungary"),
            ("si", "හංගේර\u{dd2}ය\u{dcf}ව"),
            ("sk", "Maďarsko"),
            ("sl", "Madžarska"),
            ("so", "Hangeri"),
            ("sq", "Hungari"),
            ("sr", "Мађарска"),
            ("sv", "Ungern"),
            ("sw", "Hungary"),
            ("ta", "ஹங\u{bcd}கேரி"),
            ("te", "హంగ\u{c47}ర\u{c40}"),
            ("tg", "Маҷористон"),
            ("th", "ฮ\u{e31}งการ\u{e35}"),
            ("ti", "ሀንጋሪ"),
            ("tk", "Wengriýa"),
            ("tl", "Hungary"),
            ("tr", "Macaristan"),
            ("tt", "Маҗарстан"),
            ("ug", "ۋېنگىرىيە"),
            ("uk", "Угорщина"),
            ("ur", "مجارستان"),
            ("uz", "Mojariston"),
            ("ve", "Hungary"),
            ("vi", "Hun-ga-ri"),
            ("wa", "Hongreye"),
            ("wo", "Oonguri"),
            ("xh", "Hungary"),
            ("yo", "Húngárì"),
            ("zh_CN", "匈牙利"),
            ("zh_HK", "匈牙利"),
            ("zh_TW", "匈牙利"),
            ("zu", "I-Hungariya"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}
