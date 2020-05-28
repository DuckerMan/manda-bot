extern crate vkapi;
use vkapi::*;
use vkapi::longpoll::EventType;
use rand::Rng;

pub const MESSAGES: &[&str] = &["Манда", "Хуй на", "Пизда", "Провода", "еда", "балда", "банда", "барда", "брада", "будда", "бурда", "вайда", "ванда", "вежда", "венда", "гайда", "ганда", "гарда", "гнида", "груда", "гряда", "дылда", "ехида", "жажда", "жеода", "заеда", "карда", "когда", "корда", "лайда", "магда", "морда", "мунда", "мурда", "наяда", "нужда", "обида", "осада", "панда", "рында", "сайда", "свида", "слада", "слюда", "спида", "среда", "ссуда", "тогда", "тында", "угода", "упада", "файда", "фалда", "фиада", "халда", "хорда", "хурда", "чреда", "шкода", "эгида", "ябеда", "ягода"];
pub const MANDA_DETECTOR: &[&str] = &["да", "da", "дa", "dа", "д@", "d@"];
pub const CASINO_DETECTOR: &[&str] = &["Казик", "казино", "карт", "казинo", "кaзинo", "ебанный"];
pub const DONT_BE_AFRAID: &[&str] = &["Never mind the darkness", "Never mind the storm", "Never mind the blood red moon", "The Night will be over soon", "Brush away your sorrow", "Brush away your tears", "Sign away with your heavy heart", "The Night will be over soon", "The sun will be rising soon"];
pub const YASNO: &[&str] = &["Хуясно", "Хуй с маслом!"];
pub const YOBANNIY_WROTE_ETOGO_KAZINO: &str = "– Ёбаный рот этого казино, блядь! Ты кто такой, сука, чтоб это сделать?

– Я всегда это делал, когда.
.
– ВЫ ЧЁ, ДЕБИЛЫ? Вы чё, ебанутые, что ли? Действи.. вы в натуре ебанутые? Эта сидит там, чешет колоду, блядь. Этот стоит, грит: `Я те щас тут тоже раздам`..

– Ну посмотрите..

– ЁБ ТВОЮ МАТЬ! У вас дилер есть, чтобы это делать на моих глазах, мудак ёбаный!

– Хорошо, будет делать дилер. Раньше это делал всегда я..

– ДЕГЕНЕРАТ ЕБУЧИЙ! Вот пока ты это делал, дебил, ебаная сука, БЛЯДЬ, так все и происходило!

– В ВИПе?

–  В ХУИПЕ! Блядь, вы чё, действительно идиоты, что ли, а? Бля, дифиченты какие-то, ёбаный ваш рот, а.. А ты-то чё делаешь?

– Да, смотрите. Туз не на месте..";


#[tokio::main]
async fn main() {
    let token = std::env::var("ACCESS_TOKEN").unwrap();
    let mut vkapi = vkapi::VK::new("5.103", "ru", token);
    let events = vkapi.start_longpoll(101144933, 25);
    let mut rng = rand::thread_rng();
    for (event, data) in events {
        match event {
            EventType::NewMessage => {
                let mut attachment = "".to_owned();
                let random_id: u32 = rng.gen();
                let random_id = random_id.to_string();
                let msg = &data["message"];
                println!("{}", msg);
                let peer_id = msg["peer_id"].as_u32().unwrap();
                let message_id = msg["conversation_message_id"].as_u32().unwrap();
                let user_message = msg["text"].as_str().unwrap().to_lowercase();
                let user_id = msg["from_id"].as_u32().unwrap().to_string();
                // Разные варианты с РАЗНЫМИ буквами
                let mut message_for_send = String::new();

                if detect(MANDA_DETECTOR, &user_message){
                    let generated_message = rng.gen_range(0, MESSAGES.len()-1);
                    message_for_send = format!("@id{}, {}!", user_id, MESSAGES[generated_message].to_uppercase());
                }

                if user_message.contains("ясно") || user_message.contains("яснo") || user_message.contains("ясно") || user_message.contains("яcнo") {
                    let generated_message = rng.gen_range(0, YASNO.len()-1);
                    message_for_send = format!("@id{}, {}", user_id, YASNO[generated_message]);
                }

                if user_message.contains("казино") || user_message.contains("казик"){
                    message_for_send = YOBANNIY_WROTE_ETOGO_KAZINO.to_owned();
                }

                if user_message.contains("я"){
                    message_for_send = String::from("Головка от хуя!");
                }
                if user_message.contains("ты"){
                    message_for_send = "Тебя затыкали коТЫ!".to_owned();
                }
                if user_message.contains("Clem"){
                    message_for_send = "Опять этот долбаеб со своим говном пришел".to_owned();
                }
                if user_message.contains("mind"){
                    message_for_send = DONT_BE_AFRAID[rng.gen_range(0, DONT_BE_AFRAID.len()-1)].to_owned();
                    
                }
                if user_message.contains("still"){
                    message_for_send = "STILL. NOT. BITTEN".to_owned();
                    attachment = "photo164124208_457256690".to_owned();
                }
                if !message_for_send.is_empty(){
                    vkapi.request("messages.send", &mut param! {"attachment" => &attachment ,"random_id" => &random_id, "forward_messages" => &(peer_id+message_id).to_string(), "peer_id" => &peer_id.to_string(), "message" => &message_for_send}).await.unwrap();
                }
            },
            _ => {}
        }
    }
}

pub fn detect(detector: &[&str], message: &str) -> bool {
    let mut result = false;
    detector.iter().for_each(|det|{
        if message.contains(det){
            result = true;
        }
    });
    result
}