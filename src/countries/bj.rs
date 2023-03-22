// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Republic of Benin

#[cfg(all(feature = "bj", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::BJ;
    pub const ALPHA3: Alpha3 = Alpha3::BEN;
    pub const CONTINENT: Continent = Continent::Africa;
    pub const COUNTRY_CODE: usize = 229;
    pub const CURRENCY_CODE: &str = "XOF";
    pub const GEC: Option<GEC> = Some(GEC::BN);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("BEN");
    pub const ISO_SHORT_NAME: &str = "Benin";
    pub const ISO_LONG_NAME: &str = "The Republic of Benin";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["fr"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "None";
    pub const NATIONALITY: Option<&str> = Some("Beninese");
    pub const NUMBER: &str = "204";
    pub const POSTAL_CODE: bool = false;
    pub const POSTAL_CODE_FORMAT: Option<&str> = None;
    pub const REGION: Option<Region> = Some(Region::Africa);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::WesternAfrica);
    pub const UN_LOCODE: &str = "BJ";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &["Benin", "Bénin", "ベナン"];
    pub const WORLD_REGION: WorldRegion = WorldRegion::EMEA;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Benin"),
        ("af", "Benin"),
        ("ak", "Benin"),
        ("am", "Benin"),
        ("an", "Benín"),
        ("ar", "بنين"),
        ("as", "বেনিন"),
        ("ay", "Benin"),
        ("az", "Benin"),
        ("ba", "Benin"),
        ("be", "Бенін"),
        ("bg", "Бенин"),
        ("bi", "Benin"),
        ("bn", "বেনিন"),
        ("bn_IN", "বেনিন"),
        ("br", "Benin"),
        ("bs", "Benin"),
        ("ca", "Benín"),
        ("ce", "Бенин"),
        ("ch", "Benin"),
        ("cs", "Benin"),
        ("cv", "Бенин"),
        ("cy", "Benin"),
        ("da", "Benin"),
        ("de", "Benin"),
        ("dv", "ބ\u{7ac}ނ\u{7a9}ނ\u{7b0}"),
        ("dz", "བ\u{f7a}་ན\u{f72}ན།"),
        ("ee", "Benin"),
        ("el", "Μπενίν"),
        ("en", "Benin"),
        ("eo", "Benino"),
        ("es", "Benín"),
        ("et", "Benin"),
        ("eu", "Benin"),
        ("fa", "بنین"),
        ("ff", "Benen"),
        ("fi", "Benin"),
        ("fo", "Benin"),
        ("fr", "Bénin"),
        ("fy", "Benyn"),
        ("ga", "Beinin"),
        ("gl", "Benin"),
        ("gn", "Benin"),
        ("gu", "બ\u{ac7}નિન"),
        ("gv", "Benin"),
        ("ha", "Benin"),
        ("he", "בנין"),
        ("hi", "ब\u{947}निन"),
        ("hr", "Benin"),
        ("ht", "Benen"),
        ("hu", "Benin"),
        ("hy", "Բենին"),
        ("ia", "Benin"),
        ("id", "Benin"),
        ("io", "Benin"),
        ("is", "Benín"),
        ("it", "Benin"),
        ("iu", "Benin"),
        ("ja", "ベナン"),
        ("ka", "ბენინი"),
        ("ki", "Benin"),
        ("kk", "Бенин"),
        ("kl", "Benin"),
        ("km", "បេណា\u{17c6}ង"),
        ("kn", "ಬ\u{cc6}ನ\u{cbf}ನ\u{ccd}"),
        ("ko", "베냉"),
        ("ku", "Benîn"),
        ("kv", "Benin"),
        ("kw", "Benin"),
        ("ky", "Бенин"),
        ("lo", "Benin"),
        ("lt", "Beninas"),
        ("lv", "Benina"),
        ("mi", "Pēnina"),
        ("mk", "Бенин"),
        ("ml", "ബെനിന\u{d4d}\u{200d}"),
        ("mn", "Бенин"),
        ("mr", "ब\u{947}निन"),
        ("ms", "Benin"),
        ("mt", "Benin"),
        (
            "my",
            "ဘ\u{102e}နင\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Benin"),
        ("nb", "Benin"),
        ("ne", "ब\u{947}निन"),
        ("nl", "Benin"),
        ("nn", "Benin"),
        ("nv", "Benin"),
        ("oc", "Benin"),
        ("or", "ବେନ\u{b3f}ନ\u{b4d}"),
        ("pa", "ਬੀਨਾਨ"),
        ("pi", "ब\u{947}निन"),
        ("pl", "Benin"),
        ("ps", "بېنين"),
        ("pt", "Benim"),
        ("pt_BR", "Benin"),
        ("ro", "Benin"),
        ("ru", "Бенин"),
        ("rw", "Bene"),
        ("sc", "Benin"),
        ("sd", "بينن"),
        ("si", "බෙන\u{dd2}න\u{dca}"),
        ("sk", "Benin"),
        ("sl", "Benin"),
        ("so", "Beniin"),
        ("sq", "Benin"),
        ("sr", "Бенин"),
        ("sv", "Benin"),
        ("sw", "Benin"),
        ("ta", "பெனின\u{bcd}"),
        ("te", "బ\u{c47}న\u{c3f}న\u{c4d}"),
        ("tg", "Бенин"),
        ("th", "เบน\u{e34}น"),
        ("ti", "ቤኒን"),
        ("tk", "Benin"),
        ("tl", "Benin"),
        ("tr", "Benin"),
        ("tt", "Бенин"),
        ("ug", "بېنىن"),
        ("uk", "Бенін"),
        ("ur", "بینن"),
        ("uz", "Benin"),
        ("ve", "Benin"),
        ("vi", "Bê-ninh"),
        ("wa", "Benin"),
        ("wo", "Benin"),
        ("xh", "Benin"),
        ("yo", "Benin"),
        ("zh_CN", "贝宁"),
        ("zh_HK", "貝寧"),
        ("zh_TW", "貝南"),
        ("zu", "IBenini"),
    ];
    #[cfg(all(feature = "bj", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 9.30769;
        pub const LONGITUDE: f64 = 2.315834;
        pub const MAX_LATITUDE: f64 = 12.4086111;
        pub const MAX_LONGITUDE: f64 = 3.8433429;
        pub const MIN_LATITUDE: f64 = 6.2061001;
        pub const MIN_LONGITUDE: f64 = 0.7754124000000001;
        pub const NORTHEAST_LATITUDE: f64 = 12.4086111;
        pub const NORTHEAST_LONGITUDE: f64 = 3.8433429;
        pub const SOUTHWEST_LATITUDE: f64 = 6.2061001;
        pub const SOUTHWEST_LONGITUDE: f64 = 0.7754124000000001;
    }
}
#[cfg(all(feature = "bj", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 9.30769,
            longitude: 2.315834,
            max_latitude: 12.4086111,
            max_longitude: 3.8433429,
            min_latitude: 6.2061001,
            min_longitude: 0.7754124000000001,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 12.4086111,
                    longitude: 3.8433429,
                },
                southwest: CountryGeoBound {
                    latitude: 6.2061001,
                    longitude: 0.7754124000000001,
                },
            },
        }
    }
}

#[cfg(all(feature = "bj", feature = "subdivisions"))]
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
                    "AK",
                    Subdivision{
                        name: "AK",
                        country_alpha2: Alpha2::BJ,
                        code: "AK",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.7954931), longitude: Some(1.6760691), max_latitude: Some(11.4738341), min_latitude: Some(9.994575), max_longitude: Some(2.357681), min_longitude: Some(0.7745750000000001)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أتاكورا"), ("az", "Ataqora departamenti"), ("bg", "Атакора"), ("bn", "আত\u{9be}কোর\u{9be} বিভ\u{9be}গ"), ("ca", "Atakora"), ("ccp", "𑄃𑄑𑄇\u{1112e}𑄢"), ("ceb", "Atakora Department"), ("da", "Atakora"), ("de", "Atakora"), ("el", "Ατακόρα"), ("en", "Atakora"), ("es", "Atakora"), ("eu", "Atakora departamendua"), ("fi", "Atakora"), ("fr", "Atacora"), ("gu", "અટકોરા વિભાગ"), ("hi", "अटकोरा विभाग"), ("hu", "Atakora"), ("id", "Departemen Atakora"), ("it", "dipartimento di Atakora"), ("ja", "アタコラ県"), ("kn", "ಅಟಕೊರಾ ಇಲಾಖ\u{cc6}"), ("ko", "아타코라 주"), ("lt", "Atakoros departamentas"), ("lv", "Atakoras departaments"), ("mr", "अटकोर विभाग"), ("ms", "Atakora Department"), ("nb", "Atakora"), ("nl", "Atacora"), ("no", "Atakora"), ("pl", "Departament Atakora"), ("pt", "Atakora"), ("ro", "Departamentul Atakora"), ("ru", "Атакора"), ("si", "අටකොර\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Atacora"), ("ta", "அட\u{bcd}கோர\u{bbe} துறை"), ("te", "అట\u{c3e}క\u{c4b}ర\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "อะตาโกรา"), ("tr", "Atakora Departmanı"), ("uk", "Атакора"), ("ur", "اتاکورا محکمہ"), ("vi", "Atakora"), ("yo", "Apá Atakora"), ("yo_BJ", "Apá Atakora"), ("zh", "阿塔科拉省")]),
                        unofficial_name_list: ["Atakora"].to_vec(),
                    }
                ),
                (
                    "AL",
                    Subdivision{
                        name: "AL",
                        country_alpha2: Alpha2::BJ,
                        code: "AL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.9681093), longitude: Some(2.7779813), max_latitude: Some(12.4183461), min_latitude: Some(10.5044699), max_longitude: Some(3.8480219), min_longitude: Some(2.019249)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أليبوري"), ("az", "Alibori departamenti"), ("bg", "Алибори"), ("bn", "আলিব\u{9c1}রি বিভ\u{9be}গ"), ("ca", "Alibori"), ("ccp", "𑄃𑄣\u{11128}𑄝\u{11127}𑄢\u{11128}"), ("ceb", "Alibori (departamento)"), ("da", "Alibori"), ("de", "Alibori"), ("el", "Αλιμπόρι"), ("en", "Alibori"), ("es", "Alibori"), ("eu", "Alibori departamendua"), ("fa", "استان آلیبوری"), ("fi", "Alibori"), ("fr", "Alibori"), ("gu", "અલિબો વિભાગ"), ("hi", "अलीबोरी विभाग"), ("hu", "Alibori"), ("id", "Departemen Alibori"), ("it", "dipartimento di Alibori"), ("ja", "アリボリ県"), ("kn", "ಅಲ\u{cbf}ಬೋರ\u{cbf} ಇಲಾಖ\u{cc6}"), ("ko", "알리보리 주"), ("lt", "Aliborio departamentas"), ("lv", "Alibori departaments"), ("mr", "अलीबोरी विभाग"), ("ms", "Alibori Department"), ("nb", "Alibori"), ("nl", "Alibori"), ("no", "Alibori"), ("pl", "Departament Alibori"), ("pt", "Alibori"), ("ro", "Departamentul Alibori"), ("ru", "Алибори"), ("si", "ඇල\u{dd2}බොර\u{dd3} ද\u{dd2}ස\u{dca}ත\u{dca}\u{200d}ර\u{dd2}ක\u{dca}කය"), ("sv", "Alibori"), ("ta", "அலிபோரி துறை"), ("te", "అల\u{c3f}బ\u{c4b}ర\u{c3f} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตอาล\u{e35}โบร\u{e35}"), ("tr", "Alibori Departmanı"), ("uk", "Регіон Аліборі"), ("ur", "الیبوری محکمہ"), ("vi", "Alibori"), ("yo", "Apá Alibori"), ("yo_BJ", "Apá Alibori"), ("zh", "阿黎博里省")]),
                        unofficial_name_list: ["Alibori"].to_vec(),
                    }
                ),
                (
                    "AQ",
                    Subdivision{
                        name: "AQ",
                        country_alpha2: Alpha2::BJ,
                        code: "AQ",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.6588391), longitude: Some(2.2236667), max_latitude: Some(7.0136289), min_latitude: Some(6.3027403), max_longitude: Some(2.483183), min_longitude: Some(1.953851)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة أتلانتيك"), ("az", "Atlantiq departamenti"), ("bg", "Атлантик"), ("bn", "আটল\u{9be}ন\u{9cd}টিক বিভ\u{9be}গ"), ("ca", "Atlantique"), ("ccp", "𑄃𑄖\u{11134}𑄣𑄚\u{11134}𑄑\u{11128}𑄇\u{11134}"), ("ceb", "Atlantique Department"), ("da", "Atlantique"), ("de", "Atlantique"), ("el", "Αντλάτικ"), ("en", "Atlantique"), ("es", "Atlantique"), ("eu", "Atlantique departamendua"), ("fa", "استان آتلانتیک"), ("fi", "Atlantique"), ("fr", "Atlantique"), ("gu", "એટલાન\u{acd}ટીક વિભાગ"), ("hi", "अटला\u{902}टिक विभाग"), ("hu", "Atlantique"), ("id", "Departemen Atlantique"), ("it", "dipartimento dell’Atlantico"), ("ja", "アトランティック県"), ("kn", "ಅಟ\u{ccd}ಲಾಂಟ\u{cbf}ಕ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "아틀랑티크 주"), ("lt", "Atlanto departamentas"), ("lv", "Atlantikas departaments"), ("mr", "अटला\u{902}टिक विभाग"), ("ms", "Atlantique Department"), ("nb", "Atlantique"), ("nl", "Atlantique"), ("no", "Atlantique"), ("pl", "Departament Atlantique"), ("pt", "Atlantique"), ("ro", "Departamentul Atlantique"), ("ru", "Атлантическая провинция"), ("si", "ඇට\u{dca}ලන\u{dca}ට\u{dd2}ක\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Atlantique"), ("ta", "அட\u{bcd}ல\u{bbe}ண\u{bcd}டிக\u{bcd} துறை"), ("te", "అట\u{c4d}ల\u{c3e}ంట\u{c3f}క\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "อาต\u{e4c}ลองต\u{e35}ก"), ("tr", "Atlantique Departmanı"), ("uk", "Атлантичний Регіон"), ("ur", "اٹلانٹک محکمہ"), ("vi", "Atlantique"), ("yo", "Apá Atlantique"), ("yo_BJ", "Apá Atlantique"), ("zh", "大西洋省")]),
                        unofficial_name_list: ["Atlantique"].to_vec(),
                    }
                ),
                (
                    "BO",
                    Subdivision{
                        name: "BO",
                        country_alpha2: Alpha2::BJ,
                        code: "BO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.5340864), longitude: Some(2.7779813), max_latitude: Some(10.6676449), min_latitude: Some(8.7726829), max_longitude: Some(3.851700999999999), min_longitude: Some(1.972203)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بورغو"), ("az", "Borqu departamenti"), ("bg", "Боргу"), ("bn", "ব\u{9c1}রগোয\u{9bc}\u{9be} বিভ\u{9be}গ"), ("ca", "Borgou"), ("ccp", "𑄝\u{11127}𑄢\u{11134}𑄉\u{1112f}"), ("ceb", "Borgou Department"), ("da", "Borgou"), ("de", "Borgou"), ("el", "Μπόργκου"), ("en", "Borgou"), ("es", "Borgou"), ("eu", "Borgou departamendua"), ("fa", "استان بورگو"), ("fi", "Borgou"), ("fr", "Borgou"), ("gu", "બોર\u{acd}ગ\u{ac2} વિભાગ"), ("hi", "बोर\u{94d}ग\u{942} विभाग"), ("hu", "Borgou"), ("id", "Departemen Borgou"), ("it", "dipartimento di Borgou"), ("ja", "ボルグー県"), ("kn", "ಬೊರ\u{ccd}ಗ\u{ccc} ಇಲಾಖ\u{cc6}"), ("ko", "보르구 주"), ("lt", "Borgu departamentas"), ("lv", "Borgu departaments"), ("mr", "बॉर\u{94d}गो विभाग"), ("ms", "Borgou Department"), ("nb", "Borgou"), ("nl", "Borgou"), ("no", "Borgou"), ("pl", "Departament Borgou"), ("pt", "Borgou"), ("ro", "Departamentul Borgou"), ("ru", "Боргу"), ("si", "බොර\u{dca}ගෝඋ දෙප\u{dcf}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Borgou"), ("ta", "போர\u{bcd}கோவு துறை"), ("te", "బ\u{c4b}ర\u{c4d}గ\u{c4b}వ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "บอร\u{e4c}ก\u{e39}"), ("tr", "Borgou Departmanı"), ("uk", "Боргу"), ("ur", "بورگوؤ محکمہ"), ("vi", "Borgou"), ("yo", "Apá Borgou"), ("yo_BJ", "Apá Borgou"), ("zh", "博爾古省")]),
                        unofficial_name_list: ["Borgou"].to_vec(),
                    }
                ),
                (
                    "CO",
                    Subdivision{
                        name: "CO",
                        country_alpha2: Alpha2::BJ,
                        code: "CO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(8.3022297), longitude: Some(2.302446), max_latitude: Some(8.777360999999999), min_latitude: Some(7.458557000000001), max_longitude: Some(2.761885), min_longitude: Some(1.619739)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كولاينيس"), ("az", "Kollins departamenti"), ("bg", "Колине"), ("bn", "কলিন\u{9cd}স বিভ\u{9be}গ"), ("ca", "Collines"), ("ccp", "𑄇\u{11127}𑄣\u{11128}𑄚\u{11133}𑄎\u{11134}"), ("ceb", "Collines Department"), ("da", "Collines"), ("de", "Collines"), ("el", "Κολλίνες"), ("en", "Collines"), ("es", "Collines"), ("eu", "Collines departamendua"), ("fa", "استان کالینس"), ("fi", "Collines"), ("fr", "Collines"), ("gu", "કોલિન\u{acd}સ વિભાગ"), ("hi", "कॉलि\u{902}स डिपार\u{94d}टम\u{947}\u{902}ट"), ("hu", "Collines"), ("id", "Departemen Collines"), ("it", "dipartimento delle Colline"), ("ja", "コリネス県"), ("kn", "ಕಾಲ\u{cbf}ನ\u{ccd}ಸ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "콜린 주"), ("lt", "Kalvų departamentas"), ("lv", "Kolinzas departaments"), ("mr", "कॉलिन\u{94d}स विभाग"), ("ms", "Collines Department"), ("nb", "Collines"), ("nl", "Collines"), ("no", "Collines"), ("pl", "Departament Collines"), ("pt", "Collines"), ("ro", "Departamentul Collines"), ("ru", "Коллинз"), ("si", "කොලය\u{dd2}න\u{dca}ස\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Collines"), ("ta", "க\u{bbe}லின\u{bcd}ஸ\u{bcd} துறை"), ("te", "క\u{c3e}ల\u{c3f}న\u{c4d}స\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "โกลล\u{e35}น"), ("tr", "Collines Departmanı"), ("uk", "Калін"), ("ur", "کولینز محکمہ"), ("vi", "Collines"), ("yo", "Apá Collines"), ("yo_BJ", "Apá Collines"), ("zh", "丘陵省")]),
                        unofficial_name_list: ["Collines"].to_vec(),
                    }
                ),
                (
                    "DO",
                    Subdivision{
                        name: "DO",
                        country_alpha2: Alpha2::BJ,
                        code: "DO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(9.7191867), longitude: Some(1.6760691), max_latitude: Some(10.1165749), min_latitude: Some(8.477538899999999), max_longitude: Some(2.2243771), min_longitude: Some(1.3415889)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة دونغا"), ("az", "Donqa departamenti"), ("bg", "Донга"), ("bn", "ডংগ\u{9be} বিভ\u{9be}গ"), ("ca", "Donga"), ("ccp", "𑄓\u{11127}\u{11101}𑄉"), ("ceb", "Donga (departamento)"), ("da", "Donga"), ("de", "Donga"), ("el", "Ντόνγκα"), ("en", "Donga"), ("es", "Donga"), ("eu", "Donga departamendua"), ("fa", "استان دونگا"), ("fi", "Donga"), ("fr", "Donga"), ("gu", "ડો\u{a82}ગા વિભાગ"), ("hi", "डो\u{902}गा विभाग"), ("hu", "Donga"), ("id", "Departemen Donga"), ("it", "dipartimento di Donga"), ("ja", "ドンガ県"), ("kn", "ಡೊಂಗಾ ಇಲಾಖ\u{cc6}"), ("ko", "동가 주"), ("lt", "Dongos departamentas"), ("lv", "Dongas departaments"), ("mr", "डो\u{902}गा विभाग"), ("ms", "Donga Department"), ("nb", "Donga"), ("nl", "Donga"), ("no", "Donga"), ("pl", "Departament Donga"), ("pt", "Donga"), ("ro", "Departamentul Donga"), ("ru", "Донга"), ("si", "ඩොන\u{dca}ග\u{dcf} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Donga"), ("ta", "டோங\u{bcd}க\u{bbe} துறை"), ("te", "డ\u{c4b}ంగ\u{c3e} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ดองกา"), ("tr", "Donga Departmanı"), ("uk", "Регіон Донга"), ("ur", "دونگا محکمہ"), ("vi", "Donga"), ("yo", "Apá Donga"), ("yo_BJ", "Apá Donga"), ("zh", "峽谷省")]),
                        unofficial_name_list: ["Donga"].to_vec(),
                    }
                ),
                (
                    "KO",
                    Subdivision{
                        name: "KO",
                        country_alpha2: Alpha2::BJ,
                        code: "KO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.003589400000001), longitude: Some(1.7538817), max_latitude: Some(7.497909899999999), min_latitude: Some(6.664442999999999), max_longitude: Some(2.086982), min_longitude: Some(1.557832)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة كوفو"), ("az", "Kuffo departamenti"), ("bg", "Куфо"), ("bn", "ক\u{9c1}ফ\u{9c1} বিভ\u{9be}গ"), ("ca", "Kouffo"), ("ccp", "𑄇\u{1112f}𑄜\u{1112e}"), ("ceb", "Kouffo Department"), ("da", "Kouffo"), ("de", "Couffo"), ("el", "Κούφο"), ("en", "Kouffo"), ("es", "Kouffo"), ("fa", "استان کوفو"), ("fi", "Kouffo"), ("fr", "Couffo"), ("gu", "કૌફો વિભાગ"), ("hi", "कोफ\u{93c}ो विभाग"), ("hu", "Couffo"), ("id", "Departemen Kouffo"), ("it", "dipartimento di Kouffo"), ("ja", "クッフォ県"), ("kn", "ಕೋಫೊ ಇಲಾಖ\u{cc6}"), ("ko", "쿠포 주"), ("lt", "Kufo departamentas"), ("lv", "Kufo departaments"), ("mr", "कौफो विभाग"), ("ms", "Kouffo Department"), ("nb", "Kouffo"), ("nl", "Couffo"), ("no", "Kouffo"), ("pl", "Departament Kouffo"), ("pt", "Kouffo"), ("ro", "Departamentul Kouffo"), ("ru", "Куффо"), ("si", "කොඋෆ\u{dca}ෆෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Couffo"), ("ta", "கோபிபிஒ துறை"), ("te", "క\u{c4c}ఫ\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ก\u{e39}ฟโฟ"), ("tr", "Kouffo Department"), ("uk", "Муніципалітет Куфо"), ("ur", "کوؤفو محکمہ"), ("vi", "Kouffo"), ("yo", "Apá Kouffo"), ("yo_BJ", "Apá Kouffo"), ("zh", "庫福省")]),
                        unofficial_name_list: ["Kouffo"].to_vec(),
                    }
                ),
                (
                    "LI",
                    Subdivision{
                        name: "LI",
                        country_alpha2: Alpha2::BJ,
                        code: "LI",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.3806973), longitude: Some(2.4406387), max_latitude: Some(6.4078179), min_latitude: Some(6.3403411), max_longitude: Some(2.5401967), min_longitude: Some(2.3329545)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "الإدارة الساحلية"), ("az", "Litoral departamenti"), ("bg", "Литорал"), ("bn", "লিট\u{9cd}টোর\u{9be}ল বিভ\u{9be}গ"), ("ca", "Littoral"), ("ccp", "𑄣\u{11128}𑄑\u{11133}𑄦\u{11127}𑄢𑄣\u{11134}"), ("ceb", "Littoral"), ("da", "Littoral (Benin)"), ("de", "Littoral"), ("el", "Λίττοραλ"), ("en", "Littoral"), ("es", "Littoral"), ("fa", "استان لیتورال"), ("fi", "Littoralin departmentti"), ("fr", "Littoral"), ("gu", "લિટોરલ વિભાગ"), ("hi", "लिटोरल विभाग"), ("hu", "Littoral"), ("id", "Departemen Littoral"), ("it", "dipartimento del Litorale"), ("ja", "リトラル県"), ("kn", "ಲ\u{cbf}ಟೊರಲ\u{ccd} ಇಲಾಖ\u{cc6}"), ("ko", "리토랄 주"), ("lt", "Pakrantės departamentas"), ("lv", "Litorālais departaments"), ("mr", "लिट\u{94d}टोर\u{945}ल विभाग"), ("ms", "Littoral Department"), ("nb", "Littoral"), ("nl", "Littoral"), ("no", "Littoral"), ("pl", "Departament Littoral"), ("pt", "Littoral (departamento)"), ("ro", "Departamentul Littoral"), ("ru", "Литораль"), ("si", "ල\u{dd2}ට\u{dca}ටොරල\u{dca} දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Littoral"), ("ta", "ளிட\u{bcd}டோரல\u{bcd} துறை"), ("te", "ల\u{c3f}ట\u{c4d}ట\u{c4b}రల\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ล\u{e34}ทโทลอล ด\u{e35}พาทเม\u{e49}น"), ("tr", "Littoral Departmanı"), ("uk", "Регіон Літораль (Береговий Регіон)"), ("ur", "لیتورال محکمہ"), ("vi", "Littoral"), ("yo", "Apá Littoral"), ("yo_BJ", "Apá Littoral"), ("zh", "濱海省")]),
                        unofficial_name_list: ["Littoral"].to_vec(),
                    }
                ),
                (
                    "MO",
                    Subdivision{
                        name: "MO",
                        country_alpha2: Alpha2::BJ,
                        code: "MO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.6607182), longitude: Some(1.7538817), max_latitude: Some(6.7976141), min_latitude: Some(6.2340596), max_longitude: Some(2.019586), min_longitude: Some(1.5739199)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة مونو"), ("az", "Mono departamenti"), ("bg", "Моно"), ("bn", "মনো বিভ\u{9be}গ"), ("ca", "Mono"), ("ccp", "𑄟\u{11127}𑄚\u{1112e}"), ("ceb", "Mono"), ("da", "Mono (Benin)"), ("de", "Mono"), ("el", "Μόνο"), ("en", "Mono"), ("es", "Mono"), ("fa", "استان مونو"), ("fi", "Monon depatermentti"), ("fr", "Mono"), ("gu", "મોનો વિભાગ"), ("hi", "मोनो विभाग"), ("hu", "Mono"), ("id", "Departemen Mono"), ("it", "dipartimento di Mono"), ("ja", "モノ県"), ("kn", "ಮೊನೊ ಇಲಾಖ\u{cc6}"), ("ko", "모노 주"), ("lt", "Mono departamentas"), ("lv", "Mono departaments"), ("mr", "मोनो विभाग"), ("ms", "Mono Department"), ("nb", "Mono"), ("nl", "Mono"), ("no", "Mono"), ("pl", "Departament Mono"), ("pt", "Mono"), ("ro", "Departamentul Mono"), ("ru", "Моно"), ("si", "මොනෝ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Mono"), ("ta", "மோனோ துறை"), ("te", "మ\u{c4b}న\u{c4b} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "กรมโมโน"), ("tr", "Mono Departmanı"), ("uk", "Регіон Моно"), ("ur", "مونو محکمہ"), ("vi", "Mono"), ("yo", "Apá Mono"), ("yo_BJ", "Apá Mono"), ("zh", "莫諾省")]),
                        unofficial_name_list: ["Mono"].to_vec(),
                    }
                ),
                (
                    "OU",
                    Subdivision{
                        name: "OU",
                        country_alpha2: Alpha2::BJ,
                        code: "OU",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(6.6148152), longitude: Some(2.4999918), max_latitude: Some(6.99444), min_latitude: Some(6.3598054), max_longitude: Some(2.742933), min_longitude: Some(2.3608521)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أويميه"), ("az", "Veme departamenti"), ("bg", "Уеме"), ("bn", "উয\u{9bc}েমে বিভ\u{9be}গ"), ("ca", "Ouémé"), ("ccp", "𑄃\u{1112e}𑄠𑄬𑄟𑄬"), ("ceb", "Département de l’Ouémé"), ("da", "Ouémé"), ("de", "Ouémé"), ("el", "Ουεμέ"), ("en", "Ouémé"), ("es", "Ouémé"), ("fa", "استان اومه"), ("fi", "Ouémén departmentti"), ("fr", "Ouémé"), ("gu", "ઓએમ\u{ac7} વિભાગ"), ("hi", "औउम\u{947} विभाग"), ("hu", "Ouémé"), ("id", "Departemen Ouémé"), ("it", "dipartimento di Ouémé"), ("ja", "ウェメ県"), ("kn", "ಒಯ\u{cc6}ಮ\u{cc6} ಡ\u{cbf}ಪಾರ\u{ccd}ಟ\u{ccd}ಮ\u{cc6}ಂಟ\u{ccd}"), ("ko", "우에메 주"), ("lt", "Uemės departamentas"), ("lv", "Uemes departaments"), ("mr", "औइम\u{947} विभाग"), ("ms", "Oueme Department"), ("nb", "Ouémé"), ("nl", "Ouémé"), ("no", "Ouémé"), ("pl", "Departament Ouémé"), ("pt", "Oueme"), ("ro", "Departamentul Ouémé"), ("ru", "Уэме"), ("si", "ඔඑමේ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Ouémé"), ("ta", "வுஏமே துறை"), ("te", "ఓ\u{c4b}య\u{c46}మ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เขตอ\u{e39}เอเม"), ("tr", "Ouémé Departmanı"), ("uk", "Регіон Веме"), ("ur", "اؤیمے محکمہ"), ("vi", "Khu vực hành chính Ouémé"), ("yo", "Apá Ouémé"), ("yo_BJ", "Apá Ouémé"), ("zh", "韋梅省")]),
                        unofficial_name_list: ["Ouémé"].to_vec(),
                    }
                ),
                (
                    "PL",
                    Subdivision{
                        name: "PL",
                        country_alpha2: Alpha2::BJ,
                        code: "PL",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.3445141), longitude: Some(2.539603), max_latitude: Some(7.6579461), min_latitude: Some(6.5454279), max_longitude: Some(2.799681), min_longitude: Some(2.4065919)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة بلاتو"), ("az", "Plato departamenti"), ("bg", "Плато"), ("bn", "প\u{9cd}ল\u{9be}তেও বিভ\u{9be}গ"), ("ca", "Plateau"), ("ccp", "𑄛\u{11133}𑄣\u{11127}𑄑\u{1112e}"), ("ceb", "Plateau Department"), ("da", "Plateau (Benin)"), ("de", "Plateau"), ("el", "Γεωγραφικό Διαμέρισμα Πλατό"), ("en", "Plateau"), ("es", "Plateau"), ("fa", "استان پلاتئو"), ("fi", "Plateaun departmentti"), ("fr", "Plateau"), ("gu", "પ\u{acd}લ\u{ac7}ટ\u{ac1} વિભાગ"), ("hi", "पठार विभाग"), ("hu", "Plateau"), ("id", "Departemen Plateau"), ("it", "dipartimento dell’Altopiano"), ("ja", "プラトー県"), ("kn", "ಪ\u{ccd}ರಸ\u{ccd}ಥಭ\u{cc2}ಮ\u{cbf} ಇಲಾಖ\u{cc6}"), ("ko", "플라토 주"), ("lt", "Plynaukštės departamentas"), ("lv", "Plato departaments"), ("mr", "प\u{94d}ल\u{947}टय\u{942} विभाग"), ("ms", "Jabatan Plateau"), ("nb", "Plateau"), ("nl", "Plateau"), ("no", "Plateau"), ("pl", "Departament Plateau"), ("pt", "Plateau"), ("ro", "Departamentul Plateau"), ("ru", "Плато"), ("si", "ප\u{dca}ලටේඌ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Plateau"), ("ta", "பிள\u{bbe}டோவ\u{bcd} துறை"), ("te", "ప\u{c4d}ల\u{c3e}టూ డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "เม\u{e37}องปลาโต"), ("tr", "Plateau Departmanı"), ("uk", "Регіон Плато"), ("ur", "پلیٹیو محکمہ"), ("vi", "Khu hành chính Plateau"), ("yo", "Apá Plateau"), ("yo_BJ", "Apá Plateau"), ("zh", "高原省")]),
                        unofficial_name_list: ["Plateau"].to_vec(),
                    }
                ),
                (
                    "ZO",
                    Subdivision{
                        name: "ZO",
                        country_alpha2: Alpha2::BJ,
                        code: "ZO",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(7.346926799999999), longitude: Some(2.0665197), max_latitude: Some(7.6493191), min_latitude: Some(6.9083269), max_longitude: Some(2.555015), min_longitude: Some(1.634848)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Departments,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "إدارة زو"), ("az", "Zu departamenti"), ("bg", "Зу"), ("bn", "জ\u{9c1} বিভ\u{9be}গ"), ("ca", "Zou"), ("ccp", "𑄎\u{1112f}"), ("ceb", "Zou Department"), ("da", "Zou (Benin)"), ("de", "Zou"), ("el", "Ζου"), ("en", "Zou"), ("es", "Zou"), ("fa", "استان زو"), ("fi", "Zoun depatermentti"), ("fr", "Zou"), ("gu", "પ\u{ac2}ર\u{acd}વોનિયા"), ("he", "מחוז זאו"), ("hi", "झाऊ विभाग"), ("hu", "Zou"), ("id", "Departemen Zou"), ("it", "dipartimento di Zou"), ("ja", "ズー県"), ("kn", "ಝ\u{ccc} ವ\u{cbf}ಭಾಗ"), ("ko", "주 주"), ("lt", "Zu departamentas"), ("lv", "Zu departaments"), ("mr", "झोऊ विभाग"), ("ms", "Zou Department"), ("nb", "Zou"), ("nl", "Zou"), ("no", "Zou"), ("pl", "Departament Zou"), ("pt", "Zou"), ("ro", "Departamentul Zou"), ("ru", "Зу"), ("si", "සෝඋ දෙප\u{dcf}ර\u{dca}තමේන\u{dca}ත\u{dd4}ව"), ("sv", "Zou"), ("ta", "ச\u{bbe}வ\u{bcd} துறை"), ("te", "జ\u{c4b}వ\u{c4d} డ\u{c3f}ప\u{c3e}ర\u{c4d}ట\u{c4d}మ\u{c46}ంట\u{c4d}"), ("th", "ซ\u{e39}ว ด\u{e35}พาทเม\u{e49}น"), ("tr", "Zou Departmanı"), ("uk", "Регіон Зу"), ("ur", "زوؤ محکمہ"), ("uz", "Zu"), ("vi", "Khu vực hành chính Zou"), ("yo", "Apá Zou"), ("yo_BJ", "Apá Zou"), ("zh", "祖省")]),
                        unofficial_name_list: ["Zou"].to_vec(),
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
#[cfg(feature = "bj")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::BJ,
        alpha3: Alpha3::BEN,
        address_format: None,
        continent: Continent::Africa,
        country_code: 229,
        currency_code: "XOF",
        gec: Some(GEC::BN),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("BEN"),
        iso_long_name: "The Republic of Benin",
        iso_short_name: "Benin",
        official_language_list: ["fr"].to_vec(),
        spoken_language_list: ["fr"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "None",
        nationality: Some("Beninese"),
        number: "204",
        postal_code: false,
        postal_code_format: None,
        region: Some(Region::Africa),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::WesternAfrica),
        un_locode: "BJ",
        unofficial_name_list: ["Benin", "Bénin", "ベナン"].to_vec(),
        world_region: WorldRegion::EMEA,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Benin"),
            ("af", "Benin"),
            ("ak", "Benin"),
            ("am", "Benin"),
            ("an", "Benín"),
            ("ar", "بنين"),
            ("as", "বেনিন"),
            ("ay", "Benin"),
            ("az", "Benin"),
            ("ba", "Benin"),
            ("be", "Бенін"),
            ("bg", "Бенин"),
            ("bi", "Benin"),
            ("bn", "বেনিন"),
            ("bn_IN", "বেনিন"),
            ("br", "Benin"),
            ("bs", "Benin"),
            ("ca", "Benín"),
            ("ce", "Бенин"),
            ("ch", "Benin"),
            ("cs", "Benin"),
            ("cv", "Бенин"),
            ("cy", "Benin"),
            ("da", "Benin"),
            ("de", "Benin"),
            ("dv", "ބ\u{7ac}ނ\u{7a9}ނ\u{7b0}"),
            ("dz", "བ\u{f7a}་ན\u{f72}ན།"),
            ("ee", "Benin"),
            ("el", "Μπενίν"),
            ("en", "Benin"),
            ("eo", "Benino"),
            ("es", "Benín"),
            ("et", "Benin"),
            ("eu", "Benin"),
            ("fa", "بنین"),
            ("ff", "Benen"),
            ("fi", "Benin"),
            ("fo", "Benin"),
            ("fr", "Bénin"),
            ("fy", "Benyn"),
            ("ga", "Beinin"),
            ("gl", "Benin"),
            ("gn", "Benin"),
            ("gu", "બ\u{ac7}નિન"),
            ("gv", "Benin"),
            ("ha", "Benin"),
            ("he", "בנין"),
            ("hi", "ब\u{947}निन"),
            ("hr", "Benin"),
            ("ht", "Benen"),
            ("hu", "Benin"),
            ("hy", "Բենին"),
            ("ia", "Benin"),
            ("id", "Benin"),
            ("io", "Benin"),
            ("is", "Benín"),
            ("it", "Benin"),
            ("iu", "Benin"),
            ("ja", "ベナン"),
            ("ka", "ბენინი"),
            ("ki", "Benin"),
            ("kk", "Бенин"),
            ("kl", "Benin"),
            ("km", "បេណា\u{17c6}ង"),
            ("kn", "ಬ\u{cc6}ನ\u{cbf}ನ\u{ccd}"),
            ("ko", "베냉"),
            ("ku", "Benîn"),
            ("kv", "Benin"),
            ("kw", "Benin"),
            ("ky", "Бенин"),
            ("lo", "Benin"),
            ("lt", "Beninas"),
            ("lv", "Benina"),
            ("mi", "Pēnina"),
            ("mk", "Бенин"),
            ("ml", "ബെനിന\u{d4d}\u{200d}"),
            ("mn", "Бенин"),
            ("mr", "ब\u{947}निन"),
            ("ms", "Benin"),
            ("mt", "Benin"),
            (
                "my",
                "ဘ\u{102e}နင\u{103a}န\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Benin"),
            ("nb", "Benin"),
            ("ne", "ब\u{947}निन"),
            ("nl", "Benin"),
            ("nn", "Benin"),
            ("nv", "Benin"),
            ("oc", "Benin"),
            ("or", "ବେନ\u{b3f}ନ\u{b4d}"),
            ("pa", "ਬੀਨਾਨ"),
            ("pi", "ब\u{947}निन"),
            ("pl", "Benin"),
            ("ps", "بېنين"),
            ("pt", "Benim"),
            ("pt_BR", "Benin"),
            ("ro", "Benin"),
            ("ru", "Бенин"),
            ("rw", "Bene"),
            ("sc", "Benin"),
            ("sd", "بينن"),
            ("si", "බෙන\u{dd2}න\u{dca}"),
            ("sk", "Benin"),
            ("sl", "Benin"),
            ("so", "Beniin"),
            ("sq", "Benin"),
            ("sr", "Бенин"),
            ("sv", "Benin"),
            ("sw", "Benin"),
            ("ta", "பெனின\u{bcd}"),
            ("te", "బ\u{c47}న\u{c3f}న\u{c4d}"),
            ("tg", "Бенин"),
            ("th", "เบน\u{e34}น"),
            ("ti", "ቤኒን"),
            ("tk", "Benin"),
            ("tl", "Benin"),
            ("tr", "Benin"),
            ("tt", "Бенин"),
            ("ug", "بېنىن"),
            ("uk", "Бенін"),
            ("ur", "بینن"),
            ("uz", "Benin"),
            ("ve", "Benin"),
            ("vi", "Bê-ninh"),
            ("wa", "Benin"),
            ("wo", "Benin"),
            ("xh", "Benin"),
            ("yo", "Benin"),
            ("zh_CN", "贝宁"),
            ("zh_HK", "貝寧"),
            ("zh_TW", "貝南"),
            ("zu", "IBenini"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}