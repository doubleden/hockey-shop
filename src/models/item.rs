
#[derive(PartialEq, Clone)]
pub enum ItemStatus {
    Disponible,
    Vendido,
    Reservado
}
#[derive(Clone)]
pub struct Item {
    pub image_url: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub price: f64,
    pub status: ItemStatus
}

impl Item {
    pub fn get_protections() -> Vec<Item> {
        return vec![
            Item {
                image_url: "https://siamster.com/img/protection/loktiASV.JPG",
                name: "AS-v pro",
                description: "talla señor S",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/loktiAS580.jpeg",
                name: "As580",
                description: "talla señor L",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/koleniAS580.JPG",
                name: "As 580",
                description: "talla 12 y 13",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/koleniWarrior.JPG",
                name: "Warrior Alpha QX",
                description: "talla niños 9",
                price: 40.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/grudAS580.JPG",
                name: "CCM As580",
                description: "talla jr L",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/grudTrue.JPG",
                name: "True Xc9",
                description: "talla jr M",
                price: 45.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/kragi.JPG",
                name: "CCM AS-V Pro",
                description: "Talla señor 14",
                price: 130.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/shlemTacks70.JPG",
                name: "CCM Tacks  70",
                description: "talla YT",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/shlemReact65.JPG",
                name: "Bauer Reakt 65 con mascara",
                description: "talla señor L",
                price: 140.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/shlemIMS5.jpeg",
                name: "Bauer Ims 5.0",
                description: "talla señor L",
                price: 65.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/trusAS580.JPG",
                name: "CCM AS 580",
                description: "talla señor L",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/trusACP.JPG",
                name: "Bauer ACP Elite (transformer)",
                description: "talla intermedium L",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/trusN7000.JPG",
                name: "Bauer Nexus N7000",
                description: "talla junior M",
                price: 65.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/trusCCMU.jpg",
                name: "CCM U+",
                description: "talla  jr L",
                price: 60.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/sumkaBauer.JPG",
                name: "Bolso Bauer con ruedas",
                description: "niños",
                price: 60.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/sumkaCCM.JPG",
                name: "Bolso Ccm pro team bag",
                description: "sin ruedas",
                price: 70.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/konCCM4052.png",
                name: "CCM Tacks 4052",
                description: "talla 3",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/konBauerXLP.png",
                name: "Bauer XLP",
                description: "talla Y12",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/konNexusN2700.png",
                name: "Bauer Nexus N2700",
                description: "talla 3.5 EE; talla 4.5 EE; talla 5 EE",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/konBauer535.png",
                name: "Bauer 535",
                description: "talla 1 D",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/protection/konBauerM1.png",
                name: "Bauer M1",
                description: "talla 2.5 D",
                price: 95.0,
                status: ItemStatus::Disponible,
            },
        ];
    }

    pub fn get_right_sticks() -> Vec<Item> {
        return vec![
            Item {
                image_url: "https://siamster.com/img/rightSticks/2sPro35.JPG",
                name: "Bauer 2s pro",
                description: "35 flex p92",
                price: 60.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/VaporProdigy.JPG",
                name: "Bauer Vapor Prodigy",
                description: "20 flex p01",
                price: 40.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/NexusN2900.JPG",
                name: "Bauer Nexus N2900",
                description: "65 flex p92",
                price: 55.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/Vapor3X.jpeg",
                name: "Bauer 3x",
                description: "77 flex p28",
                price: 100.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/2sProShadow.jpg",
                name: "Bauer 2S pro shadow line",
                description: "87 flex p28",
                price: 110.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/VaporX4.jpg",
                name: "Bauer Vapor x4",
                description: "77 flex p28",
                price: 120.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/Sling87.jpg",
                name: "Bauer Sling",
                description: "87 flex p92",
                price: 125.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/HyperLite55.jpg",
                name: "Bauer Vapor HiperLite",
                description: "55 flex p92",
                price: 150.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/nexus3N.png",
                name: "Bauer nexus 3N",
                description: "77 flex p92",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/hiperLite2.jpg.jpeg",
                name: "Bauer Vapor HiperLite2",
                description: "87 flex p92",
                price: 185.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/Nexusn37.jpg.jpeg",
                name: "Bauer Nexus n37",
                description: "65 flex p92",
                price: 65.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/flyLite70.jpg.jpeg",
                name: "Bauer FlyLite",
                description: "70 flex P92",
                price: 120.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/vaporX2.7.jpeg",
                name: "Bauer Vapor x2.7",
                description: "50 flex P92",
                price: 65.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/sling55.jpg",
                name: "Bauer Sling",
                description: "55 flex p92",
                price: 110.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/ultraSonic.jpg",
                name: "Bauer Supreme Ultra Sonic",
                description: "65 flex p92",
                price: 95.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/2S70.jpeg",
                name: "Bauer Supreme 2s",
                description: "70 flex P28",
                price: 90.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/1XLite.jpg",
                name: "Bauer 1X Lite",
                description: "67 flex P88",
                price: 65.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/nexusGeo.jpg",
                name: "Bauer Nexus Geo",
                description: "70 flex p28",
                price: 120.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/2sProShadow.jpg",
                name: "Bauer Nexus 2n pro shadow line",
                description: "50 flex p92",
                price: 80.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warriorQRE50_40flex.jpg",
                name: "Warrior QRE 50",
                description: "40 Flex W03",
                price: 40.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warrriorDynastyX3.jpg",
                name: "Warrior Dynasty x3",
                description: "70 flex curva Kopitar",
                price: 50.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warriorQRE20_55flex.jpg.jpeg",
                name: "Warrior QRE 20 pro",
                description: "55 Flex W28",
                price: 70.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warriorQRE40_40flex.jpg",
                name: "Warrior QRE 40",
                description: "40 flex W03",
                price: 40.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warriorAlphaDX.png",
                name: "Warrior Alpha Dx",
                description: "65 flex W28",
                price: 120.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warriorAlphaQX3_55flex.jpg.jpeg",
                name: "Warrior Alpha Qx3",
                description: "55flex w03",
                price: 55.0,
                status: ItemStatus::Vendido
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/trueXC5.jpg",
                name: "True Xc5",
                description: "58 flex p92",
                price: 55.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/eastonSynergy.jpg",
                name: "Easton Synergy gx",
                description: "65 flex E3",
                price: 55.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/CCMAS1.jpg",
                name: "CCM Tacks AS1",
                description: "75 flex curva custom 5",
                price: 100.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/trueXC7.jpg",
                name: "True Xc7",
                description: "75 flex TC2T(3ud),TC4T(5ud); 85 flex TC2T(ud1), TC4T(ud1)",
                price: 85.0,
                status: ItemStatus::Disponible
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/ribcor8.jpg",
                name: "CCM Ribcor 8",
                description: "80 flex p90",
                price: 175.0,
                status: ItemStatus::Disponible
            },
        ]
    }

    pub fn get_left_sticks() -> Vec<Item> {
        return vec![
            Item {
                image_url: "https://siamster.com/img/rightSticks/vaporX2.7.jpeg",
                name: "Bauer Vapor X2.7",
                description: "50 flex p92",
                price: 65.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/nexus3N.png",
                name: "Bauer Nexus N3",
                description: "55 flex p92",
                price: 80.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/nexusNSX.png",
                name: "Bauer NSX",
                description: "60 Flex p92",
                price: 60.0,
                status: ItemStatus::Vendido,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/bauerProto.png",
                name: "Bauer Proto",
                description: "65 flex 90tm (ud2)",
                price: 195.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/bauerCustom.jpeg",
                name: "Bauer Nexus Custom",
                description: "77 flex p92",
                price: 130.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/bauerNexusTracer.png",
                name: "Bauer Nexus Tracer",
                description: "77 flex p92",
                price: 195.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/HyperLite55.jpg",
                name: "Bauer HyperLite",
                description: "87 flex p28",
                price: 140.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/flyLite70.jpg.jpeg",
                name: "Bauer Fly Lite",
                description: "102 flex p28",
                price: 120.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/warriorQRE50_40flex.jpg",
                name: "Warrior QRE 50",
                description: "55 Flex p92",
                price: 60.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/warriorAlpha_65flex.png",
                name: "Warrior Alpha",
                description: "65 flex w88",
                price: 60.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/warriorCovert.png",
                name: "Warrior Covert",
                description: "70 flex p92",
                price: 80.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/warriorCovertQ3.png",
                name: "Warrior covert qr3 pro t1",
                description: "75 Flex w02",
                price: 90.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/warriorGR6Pro.png",
                name: "Warrior GR6 Pro",
                description: "75 flex w03; 85 flex w03",
                price: 220.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/warriorAlphaLX2.png",
                name: "Warrior Alpha LX2 Pro",
                description: "85 flex p92",
                price: 220.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/eastonSynergy.jpg",
                name: "Easton Synergy GX",
                description: "50 flex p92",
                price: 55.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/leftSticks/eastonStealth.png",
                name: "Easton Stealth",
                description: "65 flex E3",
                price: 70.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/eastonSynergy.jpg",
                name: "Easton Synergy GX",
                description: "100 flex E3",
                price: 85.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/rightSticks/eastonSynergy.jpg",
                name: "True Xc7",
                description: "75 flex TC2T(1ud); 85 flex TC2T(3), TC4T(1); 95 flex TC4T(1)",
                price: 85.0,
                status: ItemStatus::Disponible,
            },
        ]
    }

    pub fn get_portero() -> Vec<Item> {
        return vec![
            Item {
                image_url: "https://siamster.com/img/goali/mach.jpeg",
                name: "Bauer Mach Shadow",
                description: "26inc P31",
                price: 230.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/goali/vpor3X.jpg.webp",
                name: "Bauer Vapor 3X",
                description: "25inc P31; 27inc P31",
                price: 115.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/goali/blinAndLovushka.jpg",
                name: "Bauer Blocker y Catcher",
                description: "Sr",
                price: 350.0,
                status: ItemStatus::Disponible,
            },
        ]
    }

    pub fn get_linea() -> Vec<Item> {
        return vec![
            Item {
                image_url: "https://siamster.com/img/linea/vapor3xPro.jpg.webp",
                name: "Bauer Vapor 3xPro",
                description: "talla 8 fit 2",
                price: 580.0,
                status: ItemStatus::Disponible,
            },
            Item {
                image_url: "https://siamster.com/img/linea/vapor3x.jpg",
                name: "Bauer Vapor 3x",
                description: "tallas 7 fit 2; 7.5 fit 2; 8 fit 2",
                price: 390.0,
                status: ItemStatus::Disponible,
            },
        ]
    }
}