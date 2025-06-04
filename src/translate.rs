use crate::rng::XorShift64;

const MIN_CHAR: usize = 'А' as usize; // 1040
const MAX_CHAR: usize = 'я' as usize; // 1103

static MAP: [Option<&[&str]>; MAX_CHAR - MIN_CHAR + 1] = {
    let mut arr: [Option<&[&str]>; MAX_CHAR - MIN_CHAR + 1] = [None; MAX_CHAR - MIN_CHAR + 1];

    macro_rules! fill {
        ($( $c:expr => [$($v:expr),+ $(,)?] ),+ $(,)?) => {
            $(
                let idx = $c as usize;
                if idx >= MIN_CHAR && idx <= MAX_CHAR {
                    arr[idx - MIN_CHAR] = Some(&[$($v),+]);
                }
            )+
        }
    }

    fill! {
        'А' => ["О", "А"], 'а' => ["о", "а"],
        'Б' => ["П"], 'б' => ["п"],
        'В' => ["В"], 'в' => ["в"],
        'Г' => ["Г"], 'г' => ["г"],
        'Д' => ["Т"], 'д' => ["т"],
        'Е' => ["E", "И", "Э"], 'е' => ["e", "и", "э"],
        'Ё' => ["E", "И", "Э"], 'ё' => ["e", "и", "э"],
        'Ж' => ["Ж"], 'ж' => ["ж"],
        'З' => ["С", "Z"], 'з' => ["с", "Z"],
        'И' => ["И", "u"], 'и' => ["и", "u"],
        'Й' => [""], 'й' => [""],
        'К' => ["К"], 'к' => ["к"],
        'Л' => ["Л"], 'л' => ["л"],
        'М' => ["М"], 'м' => ["м"],
        'Н' => ["Н"], 'н' => ["н"],
        'О' => ["О"], 'о' => ["о"],
        'П' => ["П"], 'п' => ["п"],
        'Р' => ["Р"], 'р' => ["р"],
        'С' => ["С"], 'с' => ["с"],
        'Т' => ["T"], 'т' => ["т"],
        'У' => ["У"], 'у' => ["y"],
        'Ф' => ["Ф"], 'ф' => ["ф"],
        'Х' => ["X"], 'х' => ["x"],
        'Ц' => ["ТС"], 'ц' => ["тс"],
        'Ч' => ["Ч"], 'ч' => ["ч"],
        'Ш' => ["Ш"], 'ш' => ["ш"],
        'Щ' => ["Ш", "Щ"], 'щ' => ["ш", "щ"],
        'Ъ' => [""], 'ъ' => [""],
        'Ы' => ["И"], 'ы' => ["и"],
        'Ь' => [""], 'ь' => [""],
        'Э' => ["Е"], 'э' => ["е"],
        'Ю' => ["У", "УЮ"], 'ю' => ["У", "ую"],
        'Я' => ["Я", "ЙА"], 'я' => ["я", "йа"],
    }

    arr
};

pub fn translate(
    text: &str,
    randomize: bool,
    seed: u64,
) -> String {
    let mut rng = XorShift64::new(seed);
    let mut result = String::with_capacity(text.len() * 3);

    for c in text.chars() {
        if let Some(variants) = get_variants(c) {
            let idx = if randomize && variants.len() > 1 {
                rng.next_usize(variants.len())
            } else {
                0
            };
            result.push_str(variants[idx]);
        } else {
            result.push(c);
        }
    }

    result
}

fn get_variants(c: char) -> Option<&'static [&'static str]> {
    let idx = c as usize;
    if (MIN_CHAR..=MAX_CHAR).contains(&idx) {
        MAP[idx - MIN_CHAR]
    } else {
        None
    }
}
