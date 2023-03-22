// DO NOT TOUCH THIS FILE. (Auto-generated via `code_gen/countries.rs`)

// The Kingdom of Cambodia

#[cfg(all(feature = "kh", feature = "constants"))]
pub mod consts {
    #[allow(unused_imports)]
    use crate::{Alpha2, Alpha3, Continent, Region, SubRegion, WeekDay, WorldRegion, GEC};

    pub const ADDRESS_FORMAT: Option<&str> = None;
    pub const ALPHA2: Alpha2 = Alpha2::KH;
    pub const ALPHA3: Alpha3 = Alpha3::KHM;
    pub const CONTINENT: Continent = Continent::Asia;
    pub const COUNTRY_CODE: usize = 855;
    pub const CURRENCY_CODE: &str = "KHR";
    pub const GEC: Option<GEC> = Some(GEC::CB);
    pub const INTERNATIONAL_PREFIX: &str = "00";
    pub const IOC: Option<&str> = Some("CAM");
    pub const ISO_SHORT_NAME: &str = "Cambodia";
    pub const ISO_LONG_NAME: &str = "The Kingdom of Cambodia";
    pub const OFFICIAL_LANGUAGE_LIST: &[&str] = &["km"];
    pub const SPOKEN_LANGUAGE_LIST: &[&str] = &["km"];
    pub const NATIONAL_DESTINATION_CODE_LENGTH_LIST: &[usize] = &[2];
    pub const NATIONAL_NUMBER_LENGTH_LIST: &[usize] = &[8];
    pub const NATIONAL_PREFIX: &str = "0";
    pub const NATIONALITY: Option<&str> = Some("Cambodian");
    pub const NUMBER: &str = "116";
    pub const POSTAL_CODE: bool = true;
    pub const POSTAL_CODE_FORMAT: Option<&str> = Some("\\d{5,6}");
    pub const REGION: Option<Region> = Some(Region::Asia);
    pub const START_DAY_OF_WEEK: WeekDay = WeekDay::Monday;
    pub const SUBREGION: Option<SubRegion> = Some(SubRegion::SouthEasternAsia);
    pub const UN_LOCODE: &str = "KH";
    pub const UNOFFICIAL_NAME_LIST: &[&str] = &[
        "Cambodia",
        "Kambodscha",
        "Cambodge",
        "Camboya",
        "カンボジア",
        "Cambodja",
    ];
    pub const WORLD_REGION: WorldRegion = WorldRegion::APAC;
    #[cfg(feature = "translations")]
    pub const TRANSLATIONS: &[(&str, &str)] = &[
        ("ab", "Cambodia"),
        ("af", "Kambodja"),
        ("ak", "Cambodia"),
        ("am", "ጢሤቦ።።"),
        ("an", "Cambodia"),
        ("ar", "كمبوديا"),
        ("as", "কম\u{9cd}বোডিয়\u{9be}"),
        ("ay", "Cambodia"),
        ("az", "Kambodja"),
        ("ba", "Cambodia"),
        ("be", "Камбоджа"),
        ("bg", "Камбоджа"),
        ("bi", "Cambodia"),
        ("bn", "ক\u{9cd}য\u{9be}ম\u{9cd}বোডিয়\u{9be}"),
        ("bn_IN", "ক\u{9cd}য\u{9be}ম\u{9cd}বোডিয়\u{9be}"),
        ("br", "Kambodja"),
        ("bs", "Kambodža"),
        ("ca", "Cambodja"),
        ("ce", "Камбоджа"),
        ("ch", "Cambodia"),
        ("cs", "Kambodža"),
        ("cv", "Камбоджа"),
        ("cy", "Cambodia"),
        ("da", "Cambodja"),
        ("de", "Kambodscha"),
        ("dv", "ކ\u{7ac}ނ\u{7b0}ބ\u{7af}ޑ\u{7a8}އ\u{7a7}"),
        ("dz", "ཀམ་བ\u{f7c}་ཌ\u{f72}་ཡ།"),
        ("ee", "Cambodia"),
        ("el", "Καμπότζη"),
        ("en", "Cambodia"),
        ("eo", "Kamboĝo"),
        ("es", "Camboya"),
        ("et", "Kambodža"),
        ("eu", "Kanbodia"),
        ("fa", "کامبوج"),
        ("ff", "Kammbooja"),
        ("fi", "Kambodža"),
        ("fo", "Kambodja"),
        ("fr", "Cambodge"),
        ("fy", "Kambodja"),
        ("ga", "An Chambóid"),
        ("gl", "Camboxa"),
        ("gn", "Cambodia"),
        ("gu", "ક\u{a82}બોડિઆ"),
        ("gv", "Yn Chamboyd"),
        ("ha", "Kambodiya"),
        ("he", "קמבודיה"),
        ("hi", "कम\u{94d}बोडिया"),
        ("hr", "Kambodža"),
        ("ht", "Kanbòdj"),
        ("hu", "Kambodzsa"),
        ("hy", "Կամբոջա"),
        ("ia", "Cambodgia"),
        ("id", "Kamboja"),
        ("io", "Kambodja"),
        ("is", "Kambódía"),
        ("it", "Cambogia"),
        ("iu", "Cambodia"),
        ("ja", "カンボジア"),
        ("ka", "კამბოჯა"),
        ("ki", "Cambodia"),
        ("kk", "Камбоджа"),
        ("kl", "Cambodia"),
        ("km", "កម\u{17d2}ព\u{17bb}ជា"),
        ("kn", "ಕಾಂಬೋಡ\u{cbf}ಯಾ"),
        ("ko", "캄보디아"),
        ("ku", "Kamboca"),
        ("kv", "Камбоджа"),
        ("kw", "Kamboji"),
        ("ky", "Камбожа"),
        ("lo", "ປະເທດກຳປ\u{eb9}ເຈຍ"),
        ("lt", "Kambodža"),
        ("lv", "Kambodža"),
        ("mi", "Kamapōtia"),
        ("mk", "Камбоџа"),
        ("ml", "കമ\u{d4d}പോഡിയ"),
        ("mn", "Камбож"),
        ("mr", "क\u{902}बोडिया"),
        ("ms", "Kemboja"),
        ("mt", "Kambodja"),
        (
            "my",
            "ကမ\u{1039}ဘောဒ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
        ),
        ("na", "Kambodja"),
        ("nb", "Kambodsja"),
        ("ne", "क\u{94d}याम\u{94d}बोडिया"),
        ("nl", "Cambodja"),
        ("nn", "Kambodsja"),
        ("nv", "Cambodia"),
        ("oc", "Cambòtja"),
        ("or", "କ\u{b3e}ମ\u{b4d}ବୋଡ\u{b3f}ୟ\u{b3e}"),
        ("pa", "ਕ\u{a4b}ਲ\u{a70}ਬੀਆ"),
        ("pi", "कम\u{94d}बोदिया"),
        ("pl", "Kambodża"),
        ("ps", "کمبودیا"),
        ("pt", "Camboja"),
        ("pt_BR", "Camboja"),
        ("ro", "Cambogia"),
        ("ru", "Камбоджа"),
        ("rw", "Kambodiya"),
        ("sc", "Cambògia"),
        ("sd", "ڪمبوڊيا"),
        ("si", "ක\u{dcf}ම\u{dca}බෝජ\u{dd2}ය\u{dcf}ව"),
        ("sk", "Kambodža"),
        ("sl", "Kambodža"),
        ("so", "Kamboodiya"),
        ("sq", "Kamboxhia"),
        ("sr", "Камбоџа"),
        ("sv", "Kambodja"),
        ("sw", "Cambodia"),
        ("ta", "கம\u{bcd}போடிய\u{bbe}"),
        ("te", "కంబ\u{c4b}డ\u{c3f}య\u{c3e}"),
        ("tg", "Камбоҷа"),
        ("th", "ก\u{e31}มพ\u{e39}ชา"),
        ("ti", "ካምቦዲያ"),
        ("tk", "Kamboçiýa"),
        ("tl", "Kambodya"),
        ("tr", "Kamboçya"),
        ("tt", "Камбодиа"),
        ("ug", "كامبودژا"),
        ("uk", "Камбоджа"),
        ("ur", "کمبوڈیا"),
        ("uz", "Kambodja"),
        ("ve", "Cambodia"),
        ("vi", "Căm Bốt"),
        ("wa", "Cambodje"),
        ("wo", "Kambodia"),
        ("xh", "Cambodia"),
        ("yo", "Kàmbódíà"),
        ("zh_CN", "柬埔寨"),
        ("zh_HK", "柬埔寨"),
        ("zh_TW", "柬埔寨"),
        ("zu", "Cambodia"),
    ];
    #[cfg(all(feature = "kh", feature = "geo", feature = "constants"))]
    pub mod geo {
        pub const LATITUDE: f64 = 12.565679;
        pub const LONGITUDE: f64 = 104.990963;
        pub const MAX_LATITUDE: f64 = 14.6901791;
        pub const MAX_LONGITUDE: f64 = 107.627687;
        pub const MIN_LATITUDE: f64 = 9.6007;
        pub const MIN_LONGITUDE: f64 = 102.333542;
        pub const NORTHEAST_LATITUDE: f64 = 14.6901791;
        pub const NORTHEAST_LONGITUDE: f64 = 107.627687;
        pub const SOUTHWEST_LATITUDE: f64 = 9.6007;
        pub const SOUTHWEST_LONGITUDE: f64 = 102.333542;
    }
}
#[cfg(all(feature = "kh", feature = "geo"))]
pub mod geo {
    use crate::{CountryGeo, CountryGeoBound, CountryGeoBounds};

    pub fn new() -> CountryGeo {
        CountryGeo {
            latitude: 12.565679,
            longitude: 104.990963,
            max_latitude: 14.6901791,
            max_longitude: 107.627687,
            min_latitude: 9.6007,
            min_longitude: 102.333542,
            bounds: CountryGeoBounds {
                northeast: CountryGeoBound {
                    latitude: 14.6901791,
                    longitude: 107.627687,
                },
                southwest: CountryGeoBound {
                    latitude: 9.6007,
                    longitude: 102.333542,
                },
            },
        }
    }
}

#[cfg(all(feature = "kh", feature = "subdivisions"))]
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
                    "1",
                    Subdivision{
                        name: "1",
                        country_alpha2: Alpha2::KH,
                        code: "1",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.6672596), longitude: Some(102.8975098), max_latitude: Some(14.24883), min_latitude: Some(13.343196), max_longitude: Some(103.44429), min_longitude: Some(102.3400039)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بانتياي مينتشي"), ("bg", "Бантеай Меантей"), ("bn", "ব\u{9be}ন\u{9cd}টি মিঞ\u{9cd}চি প\u{9cd}রদেশ"), ("ccp", "𑄝𑄚\u{11134}𑄑\u{11128}𑄠 𑄟\u{11128}𑄚\u{11134}𑄌𑄬"), ("ceb", "Banteay Meanchey"), ("da", "Banteay Meanchey Province"), ("de", "Banteay Meanchey"), ("el", "Μπαντό Μίντσεϊ"), ("en", "Banteay Meanchey"), ("es", "Banteay Mean Chey"), ("fi", "Bântéay Méancheăy"), ("fr", "province de Banteay Mean Chey"), ("gu", "બાન\u{acd}ત\u{ac7}ય મિન\u{acd}ચ\u{ac7}ય પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{948}\u{902}तीए मीनचिए प\u{94d}रा\u{902}त"), ("id", "Provinsi Banteay Meanchey"), ("it", "provincia di Banteay Meanchey"), ("ja", "バンテイメンチェイ州"), ("km", "ខេត\u{17d2}តបន\u{17d2}ទាយមានជ\u{17d0}យ"), ("kn", "ಬಂಟೇಯ\u{ccd} ಮೀನ\u{ccd}ಚ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "반테아이메안체이 주"), ("lt", "Bantėj Mančėjaus provincija"), ("lv", "Bantīejmīenķeajas province"), ("mr", "ब\u{947}\u{902}ट\u{947}य म\u{947}क\u{947}सी प\u{94d}रा\u{902}त"), ("ms", "Banteay Meanchey Province"), ("nb", "Banteay Meancheay"), ("nl", "Banteay Mean Cheay"), ("no", "Banteay Meancheay"), ("pl", "Prowincja Banteay Mean Cheay"), ("pt", "Banteay Meanchey"), ("ru", "Бантеаймеантьей"), ("si", "බැන\u{dca}ටේ පළ\u{dcf}ත"), ("sv", "Banteay Meanchey"), ("ta", "ப\u{bbe}ண\u{bcd}டேய\u{bbe}ய\u{bcd} மேயஞ\u{bcd}செய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బ\u{c3e}ంట\u{c47} మ\u{c40}ంచ\u{c47} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดบ\u{e31}นทายม\u{e35}ช\u{e31}ย"), ("tr", "Banteay Meanchey"), ("uk", "Провінція Бантеаймеантьей"), ("ur", "بانتیئی مینچیئی صوبہ"), ("vi", "Banteay Meanchey"), ("zh", "班迭棉吉省")]),
                        unofficial_name_list: ["Banteay Mean Chey [Bântéay Méanchey]", "b.chey", "b.m", "banteay meanchey", "bm"].to_vec(),
                    }
                ),
                (
                    "10",
                    Subdivision{
                        name: "10",
                        country_alpha2: Alpha2::KH,
                        code: "10",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.5759862), longitude: Some(106.2522143), max_latitude: Some(12.7869431), min_latitude: Some(12.332707), max_longitude: Some(106.409197), min_longitude: Some(105.9990121)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كراتي"), ("bg", "Кратех"), ("bn", "ক\u{9cd}র\u{9be}টি প\u{9cd}রদেশ"), ("ccp", "𑄇\u{11133}𑄢𑄑\u{1112d}"), ("ceb", "Kratie (lalawigan sa Kamboya)"), ("da", "Kratié Province"), ("de", "Kratie"), ("el", "Κρατιέ"), ("en", "Kratié"), ("es", "Kratié"), ("et", "Krâchéhi provints"), ("fi", "Krâchéh"), ("fr", "province de Kratie"), ("gu", "ક\u{acd}ર\u{ac7}ટી પ\u{acd}રા\u{a82}ત"), ("hi", "कराती प\u{94d}रा\u{902}त"), ("id", "Kratié"), ("it", "provincia di Kratié"), ("ja", "クラチエ州"), ("km", "ខេត\u{17d2}ត ក\u{17d2}រចេះ"), ("kn", "ಕ\u{ccd}ರಾಟ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "크라티에 주"), ("lt", "Kratės provincija"), ("lv", "Kraķēhas province"), ("mr", "क\u{94d}र\u{947}टी प\u{94d}रा\u{902}त"), ("ms", "Kratie Province"), ("nb", "Kratie"), ("nl", "Khett Kracheh"), ("no", "Kratie"), ("pl", "Prowincja Krachen"), ("pt", "Kratié"), ("ru", "Кратьэх"), ("si", "ක\u{dca}රටය\u{dd2} පල\u{dcf}ත"), ("sv", "Kratie (provins i Kambodja)"), ("ta", "க\u{bcd}ர\u{bbe}டில\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4d}ర\u{c47}ట\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกระแจะ"), ("tr", "Kratie District"), ("uk", "Провінція Кратьех"), ("ur", "کراتیہ صوبہ"), ("vi", "Kratié"), ("zh", "桔井省")]),
                        unofficial_name_list: ["Kratie", "Kratié", "k.r", "kr", "kr.ch"].to_vec(),
                    }
                ),
                (
                    "11",
                    Subdivision{
                        name: "11",
                        country_alpha2: Alpha2::KH,
                        code: "11",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.7879427), longitude: Some(107.1011931), max_latitude: Some(13.423394), min_latitude: Some(12.059729), max_longitude: Some(107.606102), min_longitude: Some(106.3519529)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة موندولكيري"), ("bg", "Мондул Кири"), ("bn", "মন\u{9cd}ড\u{9c1}লকিরী প\u{9cd}রদেশ"), ("ca", "Província de Mondulkiri"), ("ccp", "𑄟\u{11127}𑄚\u{11134}𑄓\u{1112a}𑄣\u{11134}𑄇\u{11128}𑄢\u{11128}"), ("ceb", "Mondolkiri"), ("da", "Mondulkiri Province"), ("de", "Mondulkiri"), ("el", "Μοντουλκίρι"), ("en", "Mondulkiri"), ("es", "Provincia de Mondol Kirí"), ("et", "Môndôl Kĭri provints"), ("fi", "Môndôl Kiri"), ("fr", "province de Mondol Kiri"), ("gu", "મો\u{a82}ડ\u{ac1}લ\u{acd}કીરી પ\u{acd}રા\u{a82}ત"), ("hi", "मौण\u{94d}ड\u{941}लकिरी प\u{94d}रा\u{902}त"), ("id", "Provinsi Mondulkiri"), ("it", "provincia di Mondulkiri"), ("ja", "モンドルキリ州"), ("km", "ខេត\u{17d2}តមណ\u{17d2}ឌលគ\u{17b7}រ\u{17b8}"), ("kn", "ಮೊಂಡುಲ\u{ccd}ಕ\u{cbf}ರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "몬둘키리 주"), ("lt", "Modulkirio provincija"), ("lv", "Mondolkīrī province"), ("mr", "मो\u{902}ड\u{941}लक\u{947}री प\u{94d}रा\u{902}त"), ("ms", "Mondulkiri Province"), ("nb", "Mondulkiri"), ("nl", "Mondol Kiri"), ("no", "Mondulkiri"), ("pl", "Prowincja Mondol Kiri"), ("pt", "Mondul Kiri"), ("ru", "Мондолькири"), ("si", "මොන\u{dca}ඩ\u{dd4}ල\u{dca}ක\u{dd2}ර\u{dd2} පළ\u{dcf}ත"), ("sv", "Mondolkiri"), ("ta", "மொண\u{bcd}டுலகிரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "మ\u{c3e}ండుల\u{c4d}కుర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดมณฑลค\u{e35}ร\u{e35}"), ("tr", "Mondulkiri Province"), ("uk", "Провінція Мондулкірі"), ("ur", "موندولکیری صوبہ"), ("vi", "Mondulkiri"), ("zh", "蒙多基里省")]),
                        unofficial_name_list: ["Mondolkiri", "Mondulkiri", "m.k", "m.ri", "mk"].to_vec(),
                    }
                ),
                (
                    "12",
                    Subdivision{
                        name: "12",
                        country_alpha2: Alpha2::KH,
                        code: "12",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.5448729), longitude: Some(104.8921668), max_latitude: Some(11.682486), min_latitude: Some(11.428726), max_longitude: Some(104.968803), min_longitude: Some(104.779653)}),
                        comments: None,
                        subdivision_type: SubdivisionType::AutonomousMunicipality,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("af", "Phnom Penh"), ("am", "ፕኖም ፔን"), ("ar", "بنوم بنه"), ("az", "Pnompen"), ("be", "Пнампень"), ("bg", "Пном Пен"), ("bn", "প\u{9cd}\u{200c}নম পেন"), ("bs", "Phnom Penh"), ("ca", "Phnom Penh"), ("ccp", "𑄚\u{11127}𑄟\u{11134} 𑄛𑄬𑄚\u{11134}"), ("ceb", "Phnom Penh (lalawigan)"), ("cs", "Phnompenh"), ("cy", "Phnom Penh"), ("da", "Phnom Penh"), ("de", "Phnom Penh"), ("el", "Πνομ Πεν"), ("en", "Phnom Penh"), ("es", "Nom Pen"), ("et", "Phnom Penh"), ("eu", "Phnom Penh"), ("fa", "پنوم\u{200c}پن"), ("fi", "Phnom Penh"), ("fr", "Phnom Penh"), ("ga", "Phnom Penh"), ("gl", "Phnom Penh"), ("gu", "પનામ પ\u{ac7}ન\u{acd}હ"), ("he", "פנום פן"), ("hi", "नामप\u{947}न\u{94d}ह"), ("hr", "Phnom Penh"), ("hu", "Phnompen"), ("hy", "Պնոմպեն"), ("id", "Phnom Penh"), ("is", "Phnom Penh"), ("it", "Phnom Penh"), ("ja", "プノンペン"), ("ka", "პნომპენი"), ("kk", "Пномпень"), ("km", "ភ\u{17d2}ន\u{17c6}ពេញ"), ("kn", "ನೋಮ\u{ccd} ಫ\u{cc6}ನ\u{ccd}"), ("ko", "프놈펜"), ("ky", "Пномпень"), ("lo", "ພະນ\u{ebb}ມເປ\u{eb1}ນ"), ("lt", "Pnompenis"), ("lv", "Pnompeņa"), ("mk", "Пном Пен"), ("ml", "നോം പെൻ"), ("mn", "Пномпень"), ("mr", "पनॉम प\u{947}न"), ("ms", "Phnom Penh"), ("my", "ဖန\u{103d}မ\u{103a}းပင\u{103a}မြ\u{102d}\u{102f}\u{1037}"), ("nb", "Phnom Penh"), ("ne", "प\u{947}नोम प\u{947}न\u{94d}ह"), ("nl", "Phnom-Penh"), ("no", "Phnom Penh"), ("pa", "ਪਨਾਮ ਪ\u{a48}ਨ"), ("pl", "Phnom Penh"), ("pt", "Phnom Penh"), ("ro", "Phnom Penh"), ("ru", "Пномпень"), ("si", "නොම\u{dca} පෙන\u{dca}"), ("sk", "Phnom Pénh"), ("sl", "Phnom Penh"), ("so", "Phnom Penh"), ("sq", "Phnom Penh"), ("sr", "Пном Пен"), ("sr_Latn", "Pnom Pen"), ("sv", "Phnom Penh"), ("sw", "Phnom Penh"), ("ta", "புனோம\u{bcd} பென\u{bcd}"), ("te", "ఫ\u{c4d}న\u{c4b}ం ప\u{c46}న\u{c4d}"), ("th", "พนมเปญ"), ("tk", "Fnom-Penh"), ("tr", "Phnom Penh"), ("uk", "Пномпень"), ("ur", "پنوم پن"), ("uz", "Pnompen"), ("vi", "Phnôm Pênh"), ("yo", "Phnom Penh"), ("yo_BJ", "Phnom Penh"), ("yue", "金邊"), ("yue_Hans", "金边"), ("zh", "金边")]),
                        unofficial_name_list: ["Phnom Penh", "p.p", "pp"].to_vec(),
                    }
                ),
                (
                    "13",
                    Subdivision{
                        name: "13",
                        country_alpha2: Alpha2::KH,
                        code: "13",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.0085797), longitude: Some(104.8454619), max_latitude: Some(14.434327), min_latitude: Some(13.099064), max_longitude: Some(105.8868709), min_longitude: Some(104.3539239)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة برياه فيهير"), ("bg", "Преах Вихеа"), ("bn", "প\u{9cd}রিয\u{9bc}ে ভিহে প\u{9cd}রদেশ"), ("ccp", "𑄛\u{11133}𑄢𑄬𑄦\u{11134} 𑄞\u{11128}𑄦𑄢\u{11134}"), ("ceb", "Preah Vihear"), ("da", "Preah Vihear"), ("de", "Preah Vihear"), ("el", "Πρέα Βιχεάρ"), ("en", "Preah Vihear"), ("es", "Provincia de Preah Wijía"), ("et", "Preăh Vĭhéari provints"), ("fi", "Preăh Vihéar"), ("fr", "province de Preah Vihear"), ("gu", "પ\u{acd}ર\u{ac7}હ વિહિઅર પ\u{acd}રા\u{a82}ત"), ("hi", "प\u{94d}रीह वीहियर प\u{94d}रा\u{902}त"), ("id", "Provinsi Preah Vihear"), ("it", "provincia di Preah Vihear"), ("ja", "プレアヴィヒア州"), ("km", "ខេត\u{17d2}តព\u{17d2}រះវ\u{17b7}ហារ"), ("kn", "ಪ\u{ccd}ರೀಹ\u{ccd} ವ\u{cbf}ಹಾರ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "프레아비헤아르 주"), ("lt", "Prėja Vicharo provincija"), ("lv", "Prīehvihīeras province"), ("mr", "प\u{94d}र\u{947}ह हिहर प\u{94d}रा\u{902}त"), ("ms", "Wilayah Preah Vihear"), ("nb", "Preah Vihear"), ("nl", "Preah Vihear"), ("no", "Preah Vihear"), ("pl", "Prowincja Preah Vihear"), ("pt", "Preah Vihear"), ("ru", "Прэахвихеа"), ("si", "පප\u{dca}රෙආහ\u{dca} ව\u{dd2}හ\u{dd3}ර\u{dca} පළ\u{dcf}ත"), ("sv", "Preah Vihear"), ("ta", "பிர\u{bc0}ஹ\u{bcd} விஹெயர\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c3f}య వ\u{c40}హ\u{c47}ర\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพระว\u{e34}หาร"), ("tr", "Preah Vihear Province"), ("uk", "Провінція Преахвіхеа"), ("ur", "پریاہ ویہیار صوبہ"), ("vi", "Preah Vihear"), ("zh", "柏威夏省")]),
                        unofficial_name_list: ["Preah Vihear [Preah Vihéar]", "p.h", "ph", "pr.h"].to_vec(),
                    }
                ),
                (
                    "14",
                    Subdivision{
                        name: "14",
                        country_alpha2: Alpha2::KH,
                        code: "14",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.485114), longitude: Some(105.328098), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بري فنغ"), ("be", "Прэйвэнг"), ("bg", "Прей Венг"), ("bn", "প\u{9cd}রে ভেং প\u{9cd}রদেশ"), ("ccp", "𑄛\u{11133}𑄢𑄬 𑄞𑄬𑄋\u{11134}"), ("ceb", "Prey Veng (lalawigan)"), ("da", "Prey Veng Province"), ("de", "Prey Veng"), ("el", "Πρέι Βενγκ"), ("en", "Prey Veng"), ("es", "Provincia de Prey Veng"), ("fi", "Prey Vêng"), ("fr", "province de Prey Veng"), ("gu", "પ\u{acd}ર\u{ac7} વ\u{ac7}\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "प\u{94d}रीई व\u{947}\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Prejveng tartomány"), ("id", "Provinsi Prey Veng"), ("it", "provincia di Prey Veng"), ("ja", "プレイベン州"), ("km", "ខេត\u{17d2}តព\u{17d2}រៃវែង"), ("kn", "ಪ\u{ccd}ರೇ ವ\u{cc6}ಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "프레이벵 주"), ("lt", "Prei Vengo provincija"), ("lv", "Prijvēnas province"), ("mr", "प\u{94d}रीई व\u{947}\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Prey Veng Province"), ("nb", "Prey Veng"), ("nl", "Prey Veng"), ("no", "Prey Veng"), ("pl", "Prowincja Prey Veng"), ("pt", "Prey Veng"), ("ru", "Прейвэнг"), ("si", "ප\u{dca}\u{200d}රේ වෙන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Prey Veng"), ("ta", "பிர\u{bbe}ய வெங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c4d}ర\u{c47} వ\u{c46}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไพรแวง"), ("tr", "Prey Veng Profince"), ("uk", "Провінція Прей-Венг"), ("ur", "پریی وینگ صوبہ"), ("vi", "Prey Veng"), ("zh", "波萝勉省")]),
                        unofficial_name_list: ["Prey Veng", "Prey Vêng", "p.v", "pr.v", "pv"].to_vec(),
                    }
                ),
                (
                    "15",
                    Subdivision{
                        name: "15",
                        country_alpha2: Alpha2::KH,
                        code: "15",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.533333), longitude: Some(103.916667), max_latitude: Some(12.5827952), min_latitude: Some(12.3524528), max_longitude: Some(104.0180611), min_longitude: Some(103.7871553)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة بورسات"), ("bg", "Поусат"), ("bn", "প\u{9c1}রস\u{9be}ত প\u{9cd}রদেশ"), ("ca", "Província de Pursat"), ("ccp", "𑄛𑄢\u{11134}𑄥𑄖\u{11134}"), ("ceb", "Pursat (lalawigan)"), ("da", "Pursat Province"), ("de", "Pursat"), ("el", "Πουρσάτ"), ("en", "Pursat"), ("es", "Provincia de Pursat"), ("et", "Poŭthĭsăti provints"), ("fi", "Poŭthĭsăt"), ("fr", "province de Pouthisat"), ("gu", "પ\u{ac1}રસત પ\u{acd}રા\u{a82}ત"), ("hi", "पर\u{94d}सट प\u{94d}रा\u{902}त"), ("hu", "Puszat tartomány"), ("id", "Provinsi Pursat"), ("it", "provincia di Pursat"), ("ja", "ポーサット州"), ("km", "ខេត\u{17d2}តពោធ\u{17b7}\u{17cd}សាត\u{17cb}"), ("kn", "ಪರ\u{ccd}ಸತ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "푸르사트 주"), ("lt", "Pursuato provincija"), ("lv", "Pothisatas province"), ("mr", "प\u{942}रसट प\u{94d}रा\u{902}त"), ("ms", "Pursat Province"), ("nb", "Pursat"), ("nl", "Pouthisat"), ("no", "Pursat"), ("pl", "Prowincja Pouthisat"), ("pt", "Pursat"), ("ru", "Поусат"), ("si", "ප\u{dd4}ර\u{dca}සැට\u{dca} පළ\u{dcf}ත"), ("sv", "Pursat (provins)"), ("ta", "புர\u{bcd}சட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "పుర\u{c4d}స\u{c3e}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดโพธ\u{e34}ส\u{e31}ตว\u{e4c}"), ("tr", "Pursat Province"), ("uk", "Провінція Поусат"), ("ur", "پورسات صوبہ"), ("vi", "Pursat"), ("zh", "菩萨省")]),
                        unofficial_name_list: ["Poŭthĭsăt", "Pursat", "p.s", "ps"].to_vec(),
                    }
                ),
                (
                    "16",
                    Subdivision{
                        name: "16",
                        country_alpha2: Alpha2::KH,
                        code: "16",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.8576607), longitude: Some(107.1011931), max_latitude: Some(14.6864041), min_latitude: Some(13.1666069), max_longitude: Some(107.6277161), min_longitude: Some(106.5465848)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة راتاناكيري"), ("bg", "Ратана Кири"), ("bn", "রতনকিরি প\u{9cd}রদেশ"), ("ccp", "𑄢\u{11127}𑄑\u{11127}𑄚𑄇\u{11128}𑄢\u{11128}"), ("ceb", "Ratanakiri"), ("da", "Ratanakiri Province"), ("de", "Ratanakiri"), ("el", "Ρατανακίρι"), ("en", "Ratanakiri"), ("es", "Provincia de Ratanak Kirí"), ("et", "Rôtânăh Kĭri provints"), ("fi", "Rôtanak Kiri"), ("fr", "province de Rotanah Kiri"), ("gu", "રત\u{acd}નાકીરી પ\u{acd}રા\u{a82}ત"), ("hi", "रत\u{94d}नाकिरी प\u{94d}रा\u{902}त"), ("id", "Provinsi Ratanakiri"), ("it", "provincia di Ratanakiri"), ("ja", "ラタナキリ州"), ("km", "ខេត\u{17d2}ត រតនគ\u{17b7}រ\u{17b8}"), ("kn", "ರತಾನಕ\u{ccd}ರ\u{cbf} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "라타나키리 주"), ("lt", "Ratanakirio provincija"), ("lv", "Ratanakiri province"), ("mr", "रत\u{94d}नाकिरी प\u{94d}रा\u{902}त"), ("ms", "Ratanakiri Province"), ("nb", "Ratanakiri"), ("nl", "Ratanakiri"), ("no", "Ratanakiri"), ("pl", "Prowincja Rotanah Kiri"), ("pt", "Ratanakiri"), ("ru", "Ратанакири"), ("si", "රටනක\u{dd2}ර\u{dd2} පළ\u{dcf}ත"), ("sv", "Ratanakiri"), ("ta", "ரட\u{bcd}ணகிரி ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "రట\u{c3e}నక\u{c3f}ర\u{c3f} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดร\u{e31}ตนค\u{e35}ร\u{e35}"), ("tr", "Ratanakiri"), ("uk", "Провінція Ратанакірі"), ("ur", "راتاناکیری صوبہ"), ("vi", "Ratanakiri"), ("zh", "腊塔纳基里省")]),
                        unofficial_name_list: ["Ratanakiri", "Rotanokiri", "Rôtanah Kiri", "r.k", "r.r", "rk", "rr"].to_vec(),
                    }
                ),
                (
                    "17",
                    Subdivision{
                        name: "17",
                        country_alpha2: Alpha2::KH,
                        code: "17",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.3529103), longitude: Some(103.8594171), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة سيم رياب"), ("bg", "Сием Реап"), ("bn", "সিয\u{9bc}েম রিপ প\u{9cd}রদেশ"), ("ca", "Província de Siem Reap"), ("ccp", "𑄥\u{11128}𑄠𑄬𑄟\u{11134} 𑄢\u{11128}𑄛\u{11134}"), ("ceb", "Siem Reap"), ("da", "Siem Reap Province"), ("de", "Siem Reap"), ("el", "Σιέμ Ριπ"), ("en", "Siem Reap"), ("es", "Provincia de Siem Riep"), ("et", "Siĕm Réabi provints"), ("fi", "Siĕm Réab"), ("fr", "province de Siem Reap"), ("gl", "Provincia de Siem Reap"), ("gu", "સિએમ રીપ પ\u{acd}રા\u{a82}ત"), ("hi", "सिएम रीप प\u{94d}रा\u{902}त"), ("hu", "Sziemreap tartomány"), ("id", "Provinsi Siem Reap"), ("it", "provincia di Siem Reap"), ("ja", "シェムリアップ州"), ("km", "ខេត\u{17d2}តសៀមរាប"), ("kn", "ಸೀಯ\u{cc6}ಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "시엠레아프 주"), ("lt", "Sijem Ripo provincija"), ("lv", "Sīemrīebas province"), ("mr", "सिएम रीप प\u{94d}रा\u{902}त"), ("ms", "Siem Reap Province"), ("nb", "Siem Reap"), ("nl", "Siem Reap"), ("no", "Siem Reap"), ("pl", "Prowincja Siem Reap"), ("pt", "Siem Reap"), ("ru", "Сиемреап"), ("si", "ස\u{dd2}යෙම\u{dca} ර\u{dd3}ප\u{dca} පළ\u{dcf}ත"), ("sv", "Siem Reap"), ("ta", "சிஎம\u{bcd} ர\u{bc0}ப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c3f}య\u{c46}మ\u{c4d} ర\u{c40}ప\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเส\u{e35}ยมราฐ"), ("tr", "Siem Reap Province"), ("uk", "Провінція Сиємреап"), ("ur", "صوبہ سیئم ریئپ"), ("vi", "Xiêm Riệp"), ("zh", "暹粒省")]),
                        unofficial_name_list: ["Siem Reap", "Siemréab", "s.r", "sr"].to_vec(),
                    }
                ),
                (
                    "18",
                    Subdivision{
                        name: "18",
                        country_alpha2: Alpha2::KH,
                        code: "18",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.6253016), longitude: Some(103.5233963), max_latitude: Some(10.7803603), min_latitude: Some(10.5427244), max_longitude: Some(103.5925877), min_longitude: Some(103.1816841)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "سيهانوكفيل"), ("be", "Горад Сіануквіль"), ("bn", "শিহ\u{9be}ন\u{9c1}কভিলি"), ("ccp", "𑄥\u{11128}𑄦𑄚\u{1112f}𑄇\u{11134}𑄞\u{11128}𑄣𑄬"), ("ceb", "Sihanoukville (lalawigan)"), ("cs", "Sihanoukville"), ("da", "Sihanoukville"), ("de", "Sihanoukville"), ("el", "Σιχανουκβίλε"), ("en", "Sihanoukville"), ("es", "Ciudad de Sihanoukville"), ("et", "Preăh Seihânŭ"), ("eu", "Sihanoukville"), ("fa", "سیهانوکویل"), ("fi", "Sihanoukville"), ("fr", "Sihanoukville"), ("gu", "સિહાનોકવિલ"), ("hi", "सिहानोकविल\u{947}"), ("hu", "Kampongszom"), ("id", "Sihanoukville"), ("it", "Sihanoukville"), ("ja", "シアヌークビル"), ("km", "ខេត\u{17d2}ត ព\u{17d2}រះស\u{17b8}ហន\u{17bb}"), ("kn", "ಸ\u{cbf}ಹಾನ\u{ccc}ಕ\u{ccd}ವ\u{cbf}ಲ\u{ccd}ಲ\u{cc6}"), ("ko", "시아누크빌"), ("lt", "Sianukvilis"), ("lv", "Sianukvila"), ("mr", "सिहानोकविल\u{947}"), ("ms", "Sihanoukville"), ("nb", "Sihanoukville"), ("nl", "Sihanoukville"), ("no", "Sihanoukville"), ("pl", "Preah Seihanu"), ("pt", "Sihanoukville"), ("ro", "Sihanoukville"), ("ru", "Сиануквиль"), ("si", "ස\u{dd2}හනොක\u{dd4}ව\u{dd2}ල\u{dd2}"), ("sr", "Сихануквил"), ("sr_Latn", "Sihanukvil"), ("sv", "Sihanoukville"), ("ta", "சிஹ\u{bbe}னோக\u{bcd}வில\u{bcd}லே"), ("te", "స\u{c3f}హ\u{c3e}న\u{c4c}క\u{c4d}\u{200c}వ\u{c3f}ల\u{c4d}ల\u{c46}"), ("th", "กร\u{e38}งพระส\u{e35}หน\u{e38}"), ("tr", "Sihanoukville"), ("uk", "Сіануквіль"), ("ur", "سیہانوکویل صوبہ"), ("vi", "Sihanoukville"), ("zh", "西哈努克省")]),
                        unofficial_name_list: ["Kampong Saom", "Kampong Som", "Kompong Saom", "Kompong Som", "Preah Seihânu", "Sihanoukville", "k.saom", "k.som", "s.v"].to_vec(),
                    }
                ),
                (
                    "19",
                    Subdivision{
                        name: "19",
                        country_alpha2: Alpha2::KH,
                        code: "19",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(13.576473), longitude: Some(105.9699878), max_latitude: Some(14.587619), min_latitude: Some(13.108477), max_longitude: Some(106.6629029), min_longitude: Some(105.521177)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة ستونغ ترينغ"), ("bg", "Стънг Тренг"), ("bn", "সে\u{9cd}ত\u{9be} স\u{9cd}ট\u{9be}ং প\u{9cd}রদেশ"), ("ca", "Província de Stung Treng"), ("ccp", "𑄌\u{11133}𑄑\u{11101} 𑄑\u{11133}𑄢𑄬\u{11101}"), ("ceb", "Stung Treng"), ("da", "Stung Treng Province"), ("de", "Stung Treng (Provinz)"), ("el", "Στουνγκ Τρενγκ"), ("en", "Stung Treng"), ("es", "Provincia de Stung Treng"), ("fi", "Stŏeng Trêng"), ("fr", "Stoeng Treng"), ("gu", "સ\u{acd}ટ\u{a82}ગ ટ\u{acd}ર\u{ac7}\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{94d}ट\u{902}ग ट\u{94d}र\u{947}\u{902}ग प\u{94d}रा\u{902}त"), ("id", "Provinsi Stung Treng"), ("it", "provincia di Stung Treng"), ("ja", "ストゥントレン州"), ("km", "ខេត\u{17d2}តស\u{17d2}ទ\u{17b9}ងត\u{17d2}រែង"), ("kn", "ಸ\u{ccd}ಟುಂಗ\u{ccd} ಟ\u{ccd}ರ\u{cc6}ಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "스퉁트렝 주"), ("lt", "Sting Trengo provincija"), ("lv", "Stentrēnas province"), ("mr", "स\u{94d}ट\u{942}न\u{94d}ग ट\u{94d}र\u{945}न\u{94d}ग प\u{94d}रा\u{902}त"), ("ms", "Stung Treng Province"), ("nb", "Stung Treng (provins)"), ("nl", "Stoeng Treng (provincie)"), ("no", "Stung Treng (provins)"), ("pl", "Prowincja Stoeng Treng"), ("pt", "Stung Treng"), ("ro", "Stung Treng"), ("ru", "Стынгтраенг"), ("si", "ස\u{dca}ටන\u{dca}ග\u{dca} ට\u{dca}\u{200d}රෙන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Stung Treng (provins)"), ("ta", "ஸ\u{bcd}டுங\u{bcd} ட\u{bcd}ரெங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4d}టంగ\u{c4d} ట\u{c4d}ర\u{c46}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดสต\u{e36}งแตรง"), ("tr", "Stung Treng Province"), ("uk", "Провінція Стинг-Тренг"), ("ur", "ستونگ ترینگ صوبہ"), ("vi", "Stung Treng (tỉnh)"), ("zh", "上丁省")]),
                        unofficial_name_list: ["Stoeng Trêng", "Stung Treng", "s.t", "st"].to_vec(),
                    }
                ),
                (
                    "2",
                    Subdivision{
                        name: "2",
                        country_alpha2: Alpha2::KH,
                        code: "2",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة باتامبانغ"), ("bg", "Батамбанг (провинция)"), ("bn", "ব\u{9cd}য\u{9be}ট\u{9be}মবং প\u{9cd}রদেশ"), ("ca", "Província de Battambang"), ("ccp", "𑄝𑄑𑄟\u{11134}𑄝𑄋\u{11134}"), ("ceb", "Battambang"), ("da", "Battambang Province"), ("de", "Battambang (Provinz)"), ("el", "Μπαταμπάνγκ"), ("en", "Battambang"), ("es", "Provincia de Battambang"), ("et", "Bătdâmbângi provints"), ("fi", "Băt Dâmbâng"), ("fr", "province de Battambang"), ("gu", "બટ\u{acd}ટમ\u{acd}બ\u{ac7}\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "ब\u{948}टमब\u{948}\u{902}ग प\u{94d}रा\u{902}त"), ("id", "Provinsi Battambang"), ("it", "provincia di Battambang"), ("ja", "バタンバン州"), ("km", "ខេត\u{17d2}តបាត\u{17cb}ដ\u{17c6}បង"), ("kn", "ಬಟಾಂಬಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "바탐방 주"), ("lt", "Batambango provincija"), ("lv", "Batdambanas province"), ("mr", "बाट\u{902}ब\u{945}\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Battambang Province"), ("nb", "Battambang (provins)"), ("nl", "Battambang"), ("no", "Battambang (provins)"), ("pl", "Prowincja Battambang"), ("pt", "Battambang (província)"), ("ru", "Баттамбанг"), ("si", "බට\u{dca}ටම\u{dca}බන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Battambang (provins)"), ("ta", "ப\u{bbe}ட\u{bcd}டம\u{bcd}ப\u{bbe}ங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "బటంబ\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดพระตะบอง"), ("tr", "Battambang Province"), ("uk", "Провінція Батамбанг"), ("ur", "باتامبانگ صوبہ"), ("vi", "Battambang (tỉnh)"), ("zh", "馬德望省")]),
                        unofficial_name_list: ["Batdâmbâng", "Battambang", "b.b", "bb"].to_vec(),
                    }
                ),
                (
                    "20",
                    Subdivision{
                        name: "20",
                        country_alpha2: Alpha2::KH,
                        code: "20",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.0877866), longitude: Some(105.800951), max_latitude: Some(11.1564031), min_latitude: Some(11.024966), max_longitude: Some(105.8909941), min_longitude: Some(105.7366598)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة سفاي رينغ"), ("bg", "Свай Риенг"), ("bn", "সেভ\u{9be}য\u{9bc} রিং প\u{9cd}রদেশ"), ("ccp", "𑄞𑄬 𑄢\u{11128}𑄠𑄬𑄇\u{11134}"), ("ceb", "Svay Rieng"), ("da", "Svay Rieng Province"), ("de", "Svay Rieng"), ("el", "Επαρχία Σβέι Ριένγκ"), ("en", "Svay Rieng"), ("es", "Provincia de Svay Rieng"), ("et", "Svay Riĕngi provints"), ("fi", "Svay Riĕng"), ("fr", "province de Svay Rieng"), ("gu", "સ\u{acd}વય રીએ\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "स\u{94d}वाय रिए\u{902}ग प\u{94d}रा\u{902}त"), ("hu", "Szvajrieng tartomány"), ("id", "Provinsi Svay Rieng"), ("it", "provincia di Svay Rieng"), ("ja", "スヴァイリエン州"), ("km", "ខេត\u{17d2}តស\u{17d2}វាយរៀង"), ("kn", "ಎಸ\u{ccd}ವೈ ರೈಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "스바이리엥 주"), ("lt", "Svei Rengo provincija"), ("lv", "Svājrīenas province"), ("mr", "स\u{94d}वान\u{947} र\u{947}\u{902}ग प\u{94d}रा\u{902}त"), ("ms", "Wilayah Svay Rieng"), ("nb", "Svay Rieng (provins)"), ("nl", "Svay Rieng (provincie)"), ("no", "Svay Rieng (provins)"), ("pl", "Prowincja Svay Rieng"), ("pt", "Svay Rieng"), ("ro", "Svay Rieng"), ("ru", "Свайриенг"), ("si", "ස\u{dca}වේ ර\u{dd2}න\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Svay Rieng"), ("ta", "சுவ\u{bbe}ய\u{bcd} ரியங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "స\u{c4d}వ\u{c47} ర\u{c3f}య\u{c46}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดสวายเร\u{e35}ยง"), ("tr", "Svay Rieng Province"), ("uk", "Провінція Свайріенг"), ("ur", "سوای ریئنگ صوبہ"), ("vi", "Svay Rieng (tỉnh)"), ("zh", "柴楨省")]),
                        unofficial_name_list: ["Svaay Rieng [Svay Rieng]", "sv.r", "svay rieng", "svr"].to_vec(),
                    }
                ),
                (
                    "21",
                    Subdivision{
                        name: "21",
                        country_alpha2: Alpha2::KH,
                        code: "21",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.9321519), longitude: Some(104.798771), max_latitude: Some(11.3528621), min_latitude: Some(10.5201891), max_longitude: Some(105.0994111), min_longitude: Some(104.4379809)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة تاكو"), ("bg", "Такео"), ("bn", "ট\u{9be}কেও প\u{9cd}রদেশ"), ("ccp", "𑄑\u{11127}𑄇\u{11128}𑄃\u{1112e}"), ("ceb", "Takeo"), ("da", "Takéo Province"), ("de", "Takeo"), ("el", "Τακέο"), ("en", "Takéo"), ("es", "Takéo"), ("fi", "Takêv"), ("fr", "province de Takeo"), ("gu", "ટ\u{ac7}ક\u{ac7}ઓ પ\u{acd}રા\u{a82}ત"), ("hi", "ताकौ प\u{94d}रा\u{902}त"), ("hu", "Takeo tartomány"), ("id", "Provinsi Takéo"), ("it", "provincia di Takéo"), ("ja", "タケオ州"), ("km", "ខេត\u{17d2}តតាកែវ"), ("kn", "ಟಕ\u{cbf}ಯೋ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "타케오 주"), ("lt", "Tako provincija"), ("lv", "Tākēvas province"), ("mr", "ताक\u{94d}यो प\u{94d}रा\u{902}त"), ("ms", "Takeo Province"), ("nb", "Takeo"), ("nl", "Takev"), ("no", "Takeo"), ("pl", "Prowincja Takev"), ("pt", "Takéo"), ("ru", "Такео"), ("si", "ටකෙයෝ පළ\u{dcf}ත"), ("sv", "Takeo (provins)"), ("ta", "த\u{bbe}க\u{bcd}கிய ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ట\u{c3e}క\u{c3f}య\u{c4b} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดตาแก\u{e49}ว"), ("tr", "Takéo Belediyesi"), ("uk", "Провінція Такео"), ("ur", "تاکیو صوبہ"), ("vi", "Takéo"), ("zh", "茶胶省")]),
                        unofficial_name_list: ["Takeo", "Takêv", "t.k", "tk"].to_vec(),
                    }
                ),
                (
                    "22",
                    Subdivision{
                        name: "22",
                        country_alpha2: Alpha2::KH,
                        code: "22",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(14.1717195), longitude: Some(103.6362715), max_latitude: Some(14.4395071), min_latitude: Some(13.909678), max_longitude: Some(104.4748511), min_longitude: Some(103.0405579)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة أودار ميانتشي"), ("be", "Адармеанцьей"), ("bg", "Одар Меантей"), ("bn", "অড\u{9cd}ড\u{9be}র মিনকে প\u{9cd}রদেশ"), ("ca", "Província d’Oddar Meancheay"), ("ccp", "𑄃\u{11127}𑄓𑄢\u{11134} 𑄟\u{11128}𑄚\u{11134}𑄌𑄬"), ("ceb", "Ŏtâr Méanchey"), ("da", "Oddar Meanchey Province"), ("de", "Oddar Meanchey"), ("el", "Οντάρ Μίντσεϊ"), ("en", "Oddar Meanchey"), ("es", "Oddar Mean Chey"), ("fi", "Ŏtâr Méancheăy"), ("fr", "province d’Otdar Mean Cheay"), ("gu", "ઓડર મીનચ\u{ac7} પ\u{acd}રા\u{a82}ત"), ("hi", "ओडार मीनच\u{947} प\u{94d}रा\u{902}त"), ("id", "Provinsi Oddar Meanchey"), ("it", "provincia di Oddar Meancheay"), ("ja", "ウドンメンチェイ州"), ("km", "ខេត\u{17d2}តឧត\u{17d2}ដរមានជ\u{17d0}យ"), ("kn", "ಒಡ\u{ccd}ಡಾರ\u{ccd} ಮೀನ\u{ccd}ಚ\u{cc6} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "오다르메안체이 주"), ("lt", "Odaro Mičėjaus provincija"), ("lv", "Otarmīenķijas province"), ("mr", "ओड\u{947}र म\u{947}च\u{947}\u{902}सी प\u{94d}रा\u{902}त"), ("ms", "Oddar Meanchey Province"), ("nb", "Oddar Meancheay"), ("nl", "Oddar Meancheay"), ("no", "Oddar Meancheay"), ("pl", "Prowincja Otdar Mean Cheay"), ("pt", "Oddar Mean Cheay"), ("ru", "Оддармеантьей"), ("si", "ඔද\u{dca}ද\u{dcf}ර\u{dca} ම\u{dd3}න\u{dca}චේ පළ\u{dcf}ත"), ("sv", "Ŏtâr Méanchey"), ("ta", "ஓடர\u{bcd} மேயஞ\u{bcd}செய\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ఒద\u{c4d}ద\u{c3e}ర\u{c4d} మ\u{c3f}య\u{c3e}ంచ\u{c40} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดอ\u{e38}ดรม\u{e35}ช\u{e31}ย"), ("tr", "Odday Mecanhey Province"), ("uk", "Провінція Оддармеантьей"), ("ur", "اودار مینچیئی صوبہ"), ("vi", "Oddar Meancheay"), ("zh", "奥多棉吉省")]),
                        unofficial_name_list: ["Oddar Meanchey", "Otdar Mean Chey [Otdâr Méanchey] ", "o.chey", "o.m", "om"].to_vec(),
                    }
                ),
                (
                    "23",
                    Subdivision{
                        name: "23",
                        country_alpha2: Alpha2::KH,
                        code: "23",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.543246), longitude: Some(104.319142), max_latitude: Some(10.5781139), min_latitude: Some(10.3438702), max_longitude: Some(104.3620275), min_longitude: Some(104.2153591)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كيب"), ("bn", "কেপ প\u{9cd}রদেশ"), ("ccp", "𑄇𑄬𑄛\u{11134}"), ("da", "Kep Province"), ("de", "Kep"), ("el", "Κεπ"), ("en", "Kep"), ("es", "Ciudad de Kep"), ("et", "Kêbi provints"), ("fi", "Kêb"), ("fr", "Kep"), ("gu", "ક\u{ac7}પ પ\u{acd}રા\u{a82}ત"), ("he", "קפ"), ("hi", "क\u{947}प प\u{94d}रा\u{902}त"), ("id", "Kep"), ("it", "Kep"), ("ja", "ケップ"), ("km", "ខេត\u{17d2}ត កែប"), ("kn", "ಕ\u{cc6}ಪ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "케프"), ("lt", "Kepo provincija"), ("lv", "Kēbas province"), ("mr", "क\u{947}प प\u{94d}रा\u{902}त"), ("ms", "Kep Province"), ("nb", "Kep"), ("nl", "Keb"), ("no", "Kep"), ("pl", "Keb"), ("pt", "Keb"), ("ru", "Каеп"), ("si", "කෙප\u{dca} පළ\u{dcf}ත"), ("sv", "Kep (provins)"), ("ta", "கேப\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c46}ప\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "แกบ"), ("tr", "Kep, Kamboçya"), ("uk", "Провінція Каеп"), ("ur", "ک\u{650}یپ صوبہ"), ("vi", "Kep"), ("zh", "白馬市")]),
                        unofficial_name_list: ["Kep", "Krong Kep [Krong Kêb]"].to_vec(),
                    }
                ),
                (
                    "24",
                    Subdivision{
                        name: "24",
                        country_alpha2: Alpha2::KH,
                        code: "24",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.8539496), longitude: Some(102.6083506), max_latitude: Some(12.8922183), min_latitude: Some(12.6638293), max_longitude: Some(102.7534103), min_longitude: Some(102.4972057)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة بايلين"), ("bn", "প\u{9be}ইলিন প\u{9cd}রদেশ"), ("ccp", "𑄛\u{1112d}𑄣\u{11128}𑄚\u{11134}"), ("ceb", "Pailin"), ("da", "Pailin Province"), ("de", "Pailin"), ("el", "Παϊλίν"), ("en", "Pailin"), ("es", "Provincia de Pailín"), ("et", "Pailĭn"), ("fa", "پیلین"), ("fi", "Pailĭn"), ("fr", "Pailin"), ("gu", "પાઈલિન પ\u{acd}રા\u{a82}ત"), ("hi", "प\u{948}लिन प\u{94d}रा\u{902}त"), ("id", "Pailin"), ("it", "Pailin"), ("ja", "パイリン"), ("km", "ខេត\u{17d2}ត ប\u{17c9}ៃល\u{17b7}ន"), ("kn", "ಪೈಲ\u{cbf}ನ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "파일린"), ("lt", "Pailino provincija"), ("lv", "Pajlinas province"), ("mr", "पाइलिन प\u{94d}रा\u{902}त"), ("ms", "Pailin"), ("nb", "Pailin"), ("nl", "Pailin"), ("no", "Pailin"), ("pl", "Pailin"), ("pt", "Pailin"), ("ro", "Pailin"), ("ru", "Пайлин"), ("si", "පල\u{dd2}න\u{dca} පළ\u{dcf}ත"), ("sv", "Pailin"), ("ta", "பைலின\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "ప\u{c48}ల\u{c3f}న\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดไพล\u{e34}น"), ("tr", "Pailin Province"), ("uk", "Провінція Пайлін"), ("ur", "پائلن صوبہ"), ("vi", "Pailin"), ("zh", "拜林市")]),
                        unofficial_name_list: ["Krong Pailin [Krong Pailin]", "Pailin", "p.l", "pl"].to_vec(),
                    }
                ),
                (
                    "25",
                    Subdivision{
                        name: "25",
                        country_alpha2: Alpha2::KH,
                        code: "25",
                        #[cfg(feature = "geo")]
                        geo: None,
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ccp", "𑄝\u{11127}\u{11101} 𑄟𑄟\u{11134}"), ("en", "Tbong Khmum")]),
                        unofficial_name_list: [].to_vec(),
                    }
                ),
                (
                    "3",
                    Subdivision{
                        name: "3",
                        country_alpha2: Alpha2::KH,
                        code: "3",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.9924294), longitude: Some(105.4645408), max_latitude: Some(12.0178724), min_latitude: Some(11.9600879), max_longitude: Some(105.4737067), min_longitude: Some(105.4002787)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كامبونغ تشام"), ("be", "Кампангцям"), ("bg", "Кампонг Тям"), ("bn", "ক\u{9be}ম\u{9cd}পোং প\u{9cd}রদেশ"), ("ca", "Província de Kampong Cham"), ("ccp", "𑄟\u{11133}𑄠𑄟\u{11134}𑄛\u{11127}\u{11101} 𑄌𑄟\u{11134}"), ("ceb", "Kampong Cham (lalawigan)"), ("da", "Kampong Cham Province"), ("de", "Kampong Cham"), ("el", "Καμπόνγκ Τσαμ"), ("en", "Kampong Cham"), ("es", "Kompung Cham"), ("et", "Kâmpóng Chami provints"), ("fa", "استان کامپونگ چام"), ("fi", "Kâmpóng Cham"), ("fr", "province de Kampong Cham"), ("gu", "કામ\u{acd}પો\u{a82}ગ ચ\u{ac7}મ પ\u{acd}રા\u{a82}ત"), ("hi", "कम\u{94d}पो\u{902}ग चाम प\u{94d}रा\u{902}त"), ("id", "Provinsi Kampong Cham"), ("it", "provincia di Kampong Cham"), ("ja", "コンポンチャム州"), ("km", "ខេត\u{17d2}ត\u{200b}ក\u{17c6}ពង\u{17cb}ចាម"), ("kn", "ಕಂಪೊಂಗ\u{ccd} ಚಾಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "캄퐁참 주"), ("lt", "Kampong Čiamo provincija"), ("lv", "Kamponķhnanas province"), ("mr", "काम\u{94d}पो\u{902}\u{902}ग चाम प\u{94d}रा\u{902}त"), ("ms", "Kampong Cham Province"), ("nb", "Kampong Cham"), ("nl", "Kampong Cham"), ("no", "Kampong Cham"), ("pl", "Prowincja Kampong Cham"), ("pt", "Kampong Cham"), ("ru", "Кампонгтям"), ("si", "කැම\u{dca}පොන\u{dca}ග\u{dca} චැම\u{dca} පළ\u{dcf}ත"), ("sv", "Kampong Cham"), ("ta", "கம\u{bcd}போங\u{bcd} ச\u{bbe}ம\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ంప\u{c4b}ంగ\u{c4d} చ\u{c3e}మ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกำปงจาม"), ("tr", "Kampong Cham"), ("uk", "Кампонгтям"), ("ur", "کامپونگ چام صوبہ"), ("vi", "Kampong Cham"), ("zh", "磅湛省")]),
                        unofficial_name_list: ["Kampong Cham", "Kompong Chaam", "Kompong Cham", "k.c", "k.cham", "kc"].to_vec(),
                    }
                ),
                (
                    "4",
                    Subdivision{
                        name: "4",
                        country_alpha2: Alpha2::KH,
                        code: "4",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.25), longitude: Some(104.666667), max_latitude: Some(12.3243638), min_latitude: Some(12.2167587), max_longitude: Some(104.7148133), min_longitude: Some(104.6459342)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "مقاطعة كامبونغ تشنانغ"), ("bg", "Кампонг Чнанг (провинция)"), ("bn", "ক\u{9be}ম\u{9cd}প\u{9be}ং চ\u{9be}ং প\u{9cd}রদেশ"), ("ccp", "𑄟\u{11133}𑄠𑄟\u{11134}𑄛\u{11127}\u{11101} 𑄍𑄚\u{11127}\u{11101}"), ("ceb", "Kampong Chhnang (lalawigan)"), ("da", "Kampong Chhnang Province"), ("de", "Kampong Chhnang"), ("el", "Κάμπονγκ Τσνάνγκ"), ("en", "Kampong Chhnang"), ("es", "Kompung Chinang"), ("fi", "Kâmpóng Chhnăng"), ("fr", "province de Kampong Chhnang"), ("gu", "ક\u{a82}મ\u{acd}પોન છા\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{948}म\u{94d}पो\u{902}ग चन\u{948}\u{902}ग प\u{94d}रा\u{902}त"), ("id", "Kampong Chhnang"), ("it", "provincia di Kampong Chhnang"), ("ja", "コンポンチュナン州"), ("km", "ខេត\u{17d2}តក\u{17c6}ពង\u{17cb}ឆ\u{17d2}នា\u{17c6}ង"), ("kn", "ಕಂಪೊಂಗ\u{ccd} ಛ\u{ccd}ನಾಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "캄퐁치낭 주"), ("lt", "Kampong Čiango provincija"), ("lv", "Kampongčnangas province"), ("mr", "काम\u{94d}पो\u{902}ग छ\u{901}नग प\u{94d}रा\u{902}त"), ("ms", "Kampong Chhnang Province"), ("nb", "Kampong Chhnang"), ("nl", "Kampong Chhnang"), ("no", "Kampong Chhnang"), ("pl", "Prowincja Kampong Chhnang"), ("pt", "Kampong Chhnang"), ("ru", "Кампонгчнанг"), ("si", "කම\u{dca}පොන\u{dca}ග\u{dca} ච\u{dca}න\u{dcf}න\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Kampong Chhnang"), ("ta", "கம\u{bcd}போங\u{bcd} சஹன\u{bcd}ங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ంప\u{c4b}ంగ\u{c4d} చన\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกำปงชน\u{e31}ง"), ("tr", "Kampong Chhang Province"), ("uk", "Провінція Кампонгчнанг"), ("ur", "کامپونگ چھنانگ صوبہ"), ("vi", "Kampong Chhnang"), ("zh", "磅清扬省")]),
                        unofficial_name_list: ["Kompong Chhnang", "k.chhnang", "k.n", "kn"].to_vec(),
                    }
                ),
                (
                    "5",
                    Subdivision{
                        name: "5",
                        country_alpha2: Alpha2::KH,
                        code: "5",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.45311), longitude: Some(104.5202599), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كامبونغ سبيو"), ("bg", "Кампонг Спъ"), ("bn", "ক\u{9cd}য\u{9be}ম\u{9cd}পং স\u{9cd}পিউ প\u{9cd}রদেশ"), ("ccp", "𑄟\u{11133}𑄠𑄟\u{11134}𑄛\u{11127}\u{11101} 𑄌\u{11133}𑄛𑄬𑄅\u{1112a}"), ("ceb", "Kampong Speu"), ("da", "Kampong Speu Province"), ("de", "Kampong Speu"), ("el", "Κάμπονγκ Σπο"), ("en", "Kampong Speu"), ("es", "Provincia de Kompung Speu"), ("et", "Kâmpóng Speu provints"), ("fi", "Kâmpóng Spœ"), ("fr", "province de Kampong Spoe"), ("gu", "ક\u{ac7}મ\u{acd}પો\u{a82}ગ સ\u{acd}પ\u{ac1} પ\u{acd}રા\u{a82}ત"), ("hi", "काम\u{94d}पो\u{902}ग स\u{94d}प\u{942} प\u{94d}रा\u{902}त"), ("id", "Provinsi Kampong Speu"), ("it", "provincia di Kampong Speu"), ("ja", "コンポンスプー州"), ("km", "ខេត\u{17d2}តក\u{17c6}ពង\u{17cb}ស\u{17d2}ព\u{17ba}"), ("kn", "ಕಾಂಪೊಂಗ\u{ccd} ಸ\u{ccd}ಪೀ ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "캄퐁스페우 주"), ("lt", "Kampong Spu provincija"), ("lv", "Kampongspi province"), ("mr", "काम\u{94d}पोल स\u{94d}प\u{942} प\u{94d}रा\u{902}त"), ("ms", "Kampong Speu Province"), ("nb", "Kampong Spoe"), ("nl", "Kampong Spoe"), ("no", "Kampong Spoe"), ("pl", "Prowincja Kampong Spoe"), ("pt", "Kampong Speu"), ("ru", "Кампонгспы"), ("si", "කැම\u{dca}පොන\u{dca}ග\u{dca} ස\u{dca}පෙය\u{dd4} පළ\u{dcf}ත"), ("sv", "Kampong Spoe"), ("ta", "கம\u{bcd}போங\u{bcd} ஸ\u{bcd}பூ ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ంప\u{c4b}ంగ\u{c4d} స\u{c4d}ప\u{c4d}యూ ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกำปงสป\u{e37}อ"), ("tr", "Kampong Speu Province"), ("uk", "Провінція Кампонгспи"), ("ur", "کامپونگ سپیو صوبہ"), ("vi", "Kampong Speu"), ("zh", "磅士卑省")]),
                        unofficial_name_list: ["Kampong Speu", "Kampong Spoe", "Kompong Speu", "Kompong Spoe", "k.s", "k.speu", "ks"].to_vec(),
                    }
                ),
                (
                    "6",
                    Subdivision{
                        name: "6",
                        country_alpha2: Alpha2::KH,
                        code: "6",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(12.71197), longitude: Some(104.888603), max_latitude: None, min_latitude: None, max_longitude: None, min_longitude: None}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كامبونغ ثوم"), ("be", "Кампонгтхам"), ("bg", "Кампонг Тхом"), ("bn", "ক\u{9be}ম\u{9cd}পং থম প\u{9cd}রদেশ"), ("ccp", "𑄟\u{11133}𑄠𑄟\u{11134}𑄛\u{11127}\u{11101} 𑄗\u{11127}𑄟\u{11134}"), ("ceb", "Kampong Thom (lalawigan)"), ("cs", "Kampong Thom"), ("da", "Kampong Thom Province"), ("de", "Kampong Thom"), ("el", "Κάμπονγκ Θομ"), ("en", "Kampong Thom"), ("es", "Provincia de Kompung Thom"), ("et", "Kâmpóng Thumi provints"), ("fi", "Kâmpóng Thum"), ("fr", "province de Kampong Thom"), ("gu", "ક\u{a82}મ\u{acd}પો\u{a82}ગ થોમ પ\u{acd}રા\u{a82}ત"), ("hi", "कम\u{94d}पो\u{902}ग थोम प\u{94d}रा\u{902}त"), ("id", "Provinsi Kampong Thom"), ("it", "provincia di Kampong Thom"), ("ja", "コンポントム州"), ("km", "ខេត\u{17d2}តក\u{17c6}ពង\u{17cb}ធ\u{17c6}"), ("kn", "ಕಂಪಾಂಗ\u{ccd} ಥಾಮ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "캄퐁톰 주"), ("lt", "Kampong Tomo provincija"), ("lv", "Kamponthumas province"), ("mr", "काम\u{94d}पॉ\u{901}ग थॉम प\u{94d}रा\u{902}त"), ("ms", "Kampong Thom Province"), ("nb", "Kampong Thom"), ("nl", "Kampong Thum"), ("no", "Kampong Thom"), ("pl", "Prowincja Kampong Thum"), ("pt", "Kampong Thom"), ("ru", "Кампонгтхом"), ("si", "කැම\u{dca}පොන\u{dca}ග\u{dca} තොම\u{dca} පළ\u{dcf}ත"), ("sv", "Kompong Thom"), ("ta", "கம\u{bcd}போங\u{bcd} தோம\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ంప\u{c4b}ంగ\u{c4d} థ\u{c3e}మ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกำปงธม"), ("tr", "Kampong Thom"), ("uk", "Провінція Кампонг-Том"), ("ur", "کامپونگ تھوم صوبہ"), ("vi", "Kampong Thom"), ("zh", "磅同省")]),
                        unofficial_name_list: ["Kampong Thom", "Kampong Thum", "Kompong Thom", "Kompong Thum", "k.t", "k.thom", "kt"].to_vec(),
                    }
                ),
                (
                    "7",
                    Subdivision{
                        name: "7",
                        country_alpha2: Alpha2::KH,
                        code: "7",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(10.7412089), longitude: Some(104.1930918), max_latitude: Some(11.1730128), min_latitude: Some(9.9039915), max_longitude: Some(104.7244262), min_longitude: Some(103.8328105)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كامبوت"), ("bg", "Кампот"), ("bn", "ক\u{9be}ম\u{9cd}পট প\u{9cd}রদেশ"), ("ccp", "𑄇𑄟\u{11134}𑄛\u{11127}𑄖\u{11134}"), ("ceb", "Kampot (lalawigan)"), ("da", "Kampot Province"), ("de", "Kampot"), ("el", "Καμπότ"), ("en", "Kampot"), ("es", "Kompot"), ("et", "Kâmpôti provints"), ("fa", "استان کامپوت"), ("fi", "Kâmpôt"), ("fr", "province de Kampot"), ("gu", "ક\u{ac7}મ\u{acd}પોટ પ\u{acd}રા\u{a82}ત"), ("hi", "कम\u{94d}पोट प\u{94d}रा\u{902}त"), ("id", "Provinsi Kampot"), ("it", "provincia di Kampot"), ("ja", "カンポット州"), ("km", "ខេត\u{17d2}តក\u{17c6}ពត"), ("kn", "ಕಂಪಾಟ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "캄포트 주"), ("lt", "Kampoto provincija"), ("lv", "Kampotas province"), ("mr", "काम\u{94d}पोट प\u{94d}रा\u{902}त"), ("ms", "Kampot Province"), ("nb", "Kampot"), ("nl", "Kampot"), ("no", "Kampot"), ("pl", "Prowincja Kampot"), ("pt", "Kampot"), ("ru", "Кампот"), ("si", "කැම\u{dca}පට\u{dca} පළ\u{dcf}ත"), ("sv", "Kampot"), ("ta", "கம\u{bcd}போட\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ంప\u{c4b}ట\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดกำปอต"), ("tr", "Kampot Province"), ("uk", "Провінція Кампот"), ("ur", "کامپوت صوبہ"), ("vi", "Kam pốt"), ("zh", "贡布省")]),
                        unofficial_name_list: ["Kampot [Kâmpôt]", "k.p", "k.pot", "kp"].to_vec(),
                    }
                ),
                (
                    "8",
                    Subdivision{
                        name: "8",
                        country_alpha2: Alpha2::KH,
                        code: "8",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.4573319), longitude: Some(104.693403), max_latitude: Some(11.4589902), min_latitude: Some(11.4568189), max_longitude: Some(104.6949489), min_longitude: Some(104.6909094)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كندال"), ("bg", "Кандал"), ("bn", "ক\u{9be}ন\u{9cd}দ\u{9be}ল প\u{9cd}রদেশ"), ("ca", "Kândal"), ("ccp", "𑄇𑄚\u{11134}𑄓𑄣\u{11134}"), ("ceb", "Kandal"), ("da", "Kandal Province"), ("de", "Kandal"), ("el", "Καντάλ"), ("en", "Kandal"), ("es", "Provincia de Kandal"), ("fi", "Kândal"), ("fr", "province de Kandal"), ("gu", "ક\u{a82}દાલ પ\u{acd}રા\u{a82}ત"), ("hi", "क\u{902}डल प\u{94d}रा\u{902}त"), ("id", "Provinsi Kandal"), ("it", "provincia di Kandal"), ("ja", "カンダル州"), ("km", "ខេត\u{17d2}តកណ\u{17d2}ដាល"), ("kn", "ಕಂಡಲ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "칸달 주"), ("lt", "Kandalo provincija"), ("lv", "Kandālas province"), ("mr", "का\u{902}डल प\u{94d}रा\u{902}त"), ("ms", "Wilayah Kandal"), ("nb", "Kandal"), ("nl", "Kandal"), ("no", "Kandal"), ("pl", "Prowincja Kandal"), ("pt", "Kandal"), ("ru", "Кандаль"), ("si", "කන\u{dca}ද\u{dcf}ල\u{dca} පළ\u{dcf}ත"), ("sv", "Kandal"), ("ta", "கண\u{bcd}ட\u{bbe}ல\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c3e}ండ\u{c3e}ల\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดก\u{e31}นดาล"), ("tr", "Kandal Province"), ("uk", "Провінція Кандал"), ("ur", "کاندال صوبہ"), ("vi", "Kandal")]),
                        unofficial_name_list: ["Kandaal [Kândal]", "Kandal", "Kondal", "k.d", "kd"].to_vec(),
                    }
                ),
                (
                    "9",
                    Subdivision{
                        name: "9",
                        country_alpha2: Alpha2::KH,
                        code: "9",
                        #[cfg(feature = "geo")]
                        geo: Some(SubdivisionGeo{latitude: Some(11.5762804), longitude: Some(103.3587288), max_latitude: Some(12.1494559), min_latitude: Some(9.287840300000001), max_longitude: Some(104.146877), min_longitude: Some(102.7800221)}),
                        comments: None,
                        subdivision_type: SubdivisionType::Province,
                        #[cfg(feature = "translations")]
                        translations: HashMap::from([("ar", "محافظة كوه كونغ"), ("bg", "Кох Конг"), ("bn", "কোহ কোং প\u{9cd}রদেশ"), ("ca", "Província de Koh Kong"), ("ccp", "𑄇\u{1112e}𑄦\u{11134} 𑄇\u{11127}\u{11101}"), ("ceb", "Koh Kong"), ("da", "Koh Kong Province"), ("de", "Koh Kong"), ("el", "Κοχ Κονγκ"), ("en", "Koh Kong"), ("es", "Provincia de Koh Kong"), ("et", "Kaôh Kŏngi provints"), ("fi", "Kaôh Kông"), ("fr", "province de Kaoh Kong"), ("gu", "કોહ કો\u{a82}ગ પ\u{acd}રા\u{a82}ત"), ("hi", "कोह का\u{901}ग प\u{94d}रा\u{902}त"), ("id", "Provinsi Koh Kong"), ("it", "provincia di Koh Kong"), ("ja", "ココン州"), ("km", "ខេត\u{17d2}តកោះក\u{17bb}ង"), ("kn", "ಕೊಹ\u{ccd} ಕಾಂಗ\u{ccd} ಪ\u{ccd}ರಾಂತ\u{ccd}ಯ"), ("ko", "코콩 주"), ("lt", "Kog Kongo provincija"), ("lv", "Kohkongas province"), ("mr", "कोह का\u{901}ग प\u{94d}रा\u{902}त"), ("ms", "Koh Kong Province"), ("nb", "Koh Kong"), ("nl", "Koh Kong"), ("no", "Koh Kong"), ("pl", "Prowincja Kaoh Kong"), ("pt", "Koh Kong"), ("ru", "Кохконг"), ("si", "කෝ කොන\u{dca}ග\u{dca} පළ\u{dcf}ත"), ("sv", "Koh Kong"), ("ta", "கோஹ\u{bcd} க\u{bbe}ங\u{bcd} ம\u{bbe}க\u{bbe}ணம\u{bcd}"), ("te", "క\u{c4b}హ\u{c4d} క\u{c3e}ంగ\u{c4d} ప\u{c4d}ర\u{c3e}వ\u{c3f}న\u{c4d}స\u{c4d}"), ("th", "จ\u{e31}งหว\u{e31}ดเกาะกง"), ("tr", "Koh kong"), ("uk", "Провінція Кахконг"), ("ur", "کوہ کونگ صوبہ"), ("vi", "Koh Kong"), ("zh", "戈公省")]),
                        unofficial_name_list: ["Koh Kong", "k.k", "kk"].to_vec(),
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
#[cfg(feature = "kh")]
pub fn new() -> Country {
    Country {
        alpha2: Alpha2::KH,
        alpha3: Alpha3::KHM,
        address_format: None,
        continent: Continent::Asia,
        country_code: 855,
        currency_code: "KHR",
        gec: Some(GEC::CB),
        #[cfg(feature = "geo")]
        geo: geo::new(),
        international_prefix: "00",
        ioc: Some("CAM"),
        iso_long_name: "The Kingdom of Cambodia",
        iso_short_name: "Cambodia",
        official_language_list: ["km"].to_vec(),
        spoken_language_list: ["km"].to_vec(),
        national_destination_code_length_list: [2].to_vec(),
        national_number_length_list: [8].to_vec(),
        national_prefix: "0",
        nationality: Some("Cambodian"),
        number: "116",
        postal_code: true,
        postal_code_format: Some("\\d{5,6}"),
        region: Some(Region::Asia),
        start_of_week: WeekDay::Monday,
        subregion: Some(SubRegion::SouthEasternAsia),
        un_locode: "KH",
        unofficial_name_list: [
            "Cambodia",
            "Kambodscha",
            "Cambodge",
            "Camboya",
            "カンボジア",
            "Cambodja",
        ]
        .to_vec(),
        world_region: WorldRegion::APAC,
        #[cfg(feature = "translations")]
        translations: HashMap::from([
            ("ab", "Cambodia"),
            ("af", "Kambodja"),
            ("ak", "Cambodia"),
            ("am", "ጢሤቦ።።"),
            ("an", "Cambodia"),
            ("ar", "كمبوديا"),
            ("as", "কম\u{9cd}বোডিয়\u{9be}"),
            ("ay", "Cambodia"),
            ("az", "Kambodja"),
            ("ba", "Cambodia"),
            ("be", "Камбоджа"),
            ("bg", "Камбоджа"),
            ("bi", "Cambodia"),
            ("bn", "ক\u{9cd}য\u{9be}ম\u{9cd}বোডিয়\u{9be}"),
            ("bn_IN", "ক\u{9cd}য\u{9be}ম\u{9cd}বোডিয়\u{9be}"),
            ("br", "Kambodja"),
            ("bs", "Kambodža"),
            ("ca", "Cambodja"),
            ("ce", "Камбоджа"),
            ("ch", "Cambodia"),
            ("cs", "Kambodža"),
            ("cv", "Камбоджа"),
            ("cy", "Cambodia"),
            ("da", "Cambodja"),
            ("de", "Kambodscha"),
            ("dv", "ކ\u{7ac}ނ\u{7b0}ބ\u{7af}ޑ\u{7a8}އ\u{7a7}"),
            ("dz", "ཀམ་བ\u{f7c}་ཌ\u{f72}་ཡ།"),
            ("ee", "Cambodia"),
            ("el", "Καμπότζη"),
            ("en", "Cambodia"),
            ("eo", "Kamboĝo"),
            ("es", "Camboya"),
            ("et", "Kambodža"),
            ("eu", "Kanbodia"),
            ("fa", "کامبوج"),
            ("ff", "Kammbooja"),
            ("fi", "Kambodža"),
            ("fo", "Kambodja"),
            ("fr", "Cambodge"),
            ("fy", "Kambodja"),
            ("ga", "An Chambóid"),
            ("gl", "Camboxa"),
            ("gn", "Cambodia"),
            ("gu", "ક\u{a82}બોડિઆ"),
            ("gv", "Yn Chamboyd"),
            ("ha", "Kambodiya"),
            ("he", "קמבודיה"),
            ("hi", "कम\u{94d}बोडिया"),
            ("hr", "Kambodža"),
            ("ht", "Kanbòdj"),
            ("hu", "Kambodzsa"),
            ("hy", "Կամբոջա"),
            ("ia", "Cambodgia"),
            ("id", "Kamboja"),
            ("io", "Kambodja"),
            ("is", "Kambódía"),
            ("it", "Cambogia"),
            ("iu", "Cambodia"),
            ("ja", "カンボジア"),
            ("ka", "კამბოჯა"),
            ("ki", "Cambodia"),
            ("kk", "Камбоджа"),
            ("kl", "Cambodia"),
            ("km", "កម\u{17d2}ព\u{17bb}ជា"),
            ("kn", "ಕಾಂಬೋಡ\u{cbf}ಯಾ"),
            ("ko", "캄보디아"),
            ("ku", "Kamboca"),
            ("kv", "Камбоджа"),
            ("kw", "Kamboji"),
            ("ky", "Камбожа"),
            ("lo", "ປະເທດກຳປ\u{eb9}ເຈຍ"),
            ("lt", "Kambodža"),
            ("lv", "Kambodža"),
            ("mi", "Kamapōtia"),
            ("mk", "Камбоџа"),
            ("ml", "കമ\u{d4d}പോഡിയ"),
            ("mn", "Камбож"),
            ("mr", "क\u{902}बोडिया"),
            ("ms", "Kemboja"),
            ("mt", "Kambodja"),
            (
                "my",
                "ကမ\u{1039}ဘောဒ\u{102e}းယားန\u{102d}\u{102f}င\u{103a}င\u{1036}",
            ),
            ("na", "Kambodja"),
            ("nb", "Kambodsja"),
            ("ne", "क\u{94d}याम\u{94d}बोडिया"),
            ("nl", "Cambodja"),
            ("nn", "Kambodsja"),
            ("nv", "Cambodia"),
            ("oc", "Cambòtja"),
            ("or", "କ\u{b3e}ମ\u{b4d}ବୋଡ\u{b3f}ୟ\u{b3e}"),
            ("pa", "ਕ\u{a4b}ਲ\u{a70}ਬੀਆ"),
            ("pi", "कम\u{94d}बोदिया"),
            ("pl", "Kambodża"),
            ("ps", "کمبودیا"),
            ("pt", "Camboja"),
            ("pt_BR", "Camboja"),
            ("ro", "Cambogia"),
            ("ru", "Камбоджа"),
            ("rw", "Kambodiya"),
            ("sc", "Cambògia"),
            ("sd", "ڪمبوڊيا"),
            ("si", "ක\u{dcf}ම\u{dca}බෝජ\u{dd2}ය\u{dcf}ව"),
            ("sk", "Kambodža"),
            ("sl", "Kambodža"),
            ("so", "Kamboodiya"),
            ("sq", "Kamboxhia"),
            ("sr", "Камбоџа"),
            ("sv", "Kambodja"),
            ("sw", "Cambodia"),
            ("ta", "கம\u{bcd}போடிய\u{bbe}"),
            ("te", "కంబ\u{c4b}డ\u{c3f}య\u{c3e}"),
            ("tg", "Камбоҷа"),
            ("th", "ก\u{e31}มพ\u{e39}ชา"),
            ("ti", "ካምቦዲያ"),
            ("tk", "Kamboçiýa"),
            ("tl", "Kambodya"),
            ("tr", "Kamboçya"),
            ("tt", "Камбодиа"),
            ("ug", "كامبودژا"),
            ("uk", "Камбоджа"),
            ("ur", "کمبوڈیا"),
            ("uz", "Kambodja"),
            ("ve", "Cambodia"),
            ("vi", "Căm Bốt"),
            ("wa", "Cambodje"),
            ("wo", "Kambodia"),
            ("xh", "Cambodia"),
            ("yo", "Kàmbódíà"),
            ("zh_CN", "柬埔寨"),
            ("zh_HK", "柬埔寨"),
            ("zh_TW", "柬埔寨"),
            ("zu", "Cambodia"),
        ]),
        #[cfg(feature = "subdivisions")]
        subdivisions: subdivisions::new(),
    }
}