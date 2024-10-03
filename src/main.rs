        use std::io::{self, Write};
        use std::fs::File;
        //use std::string;w
        use rand::distributions::{Alphanumeric, DistString};


        fn main() {
            let boo = true;
            match boo {
                true => robtaining_data_for_the_password(),
                false => println!("Hello world"),
            }
        }

        fn robtaining_data_for_the_password() {
            let h = "Привіт! Я генератор паролів, я тобі можу згенерувати пароль";
            let b = "Введіть кількість символів для пароля ";
            println!("{} \n{} ", h, b);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Щось пішло не так");

            let input = input.trim(); // Обрізаємо зайві пробіли
            let number: usize = match input.parse() {
                Ok(num) => num, // Успішне парсування
                Err(_) => {
                    println!("Будь ласка, введіть правильне число.");
                    return;
                }
            };

            println!("Яку кількість паролів згенерувати ?");

            let mut quantity = String::new();
            io::stdin()
            .read_line(&mut quantity)
            .expect("ойо щось пішло не так");

            let quantity = quantity.trim(); // Обрізаємо зайві пробіли
            let number2: usize = match quantity.parse() {
                Ok(num) => num, // Успішне парсування
                Err(_) => {
                    println!("Будь ласка, введіть правильне число.");
                    return;
                }
            };      
            let mut passwords: Vec<String> = Vec::new();
            
            for i in 1..=number2 {   
                let string = Alphanumeric.sample_string(&mut rand::thread_rng(), number);
                println!("{}. Пароль згенерований: {}", i, &string);
                passwords.push(string); // Додаємо пароль у вектор

                let dffd = passwords.join("\n");

                let passwd = "passwd.txt";
                let mut f = File::create(passwd)
                .expect("ойо щось пішло не так");

                f.write_all(dffd.as_bytes())
                .expect("йома йо мама дзвонить");  
            }

            println!("Ваші паролі збереглися в файлі під назвою password.txt");

            print!("Попередження!!!  \nякщо ви запустете скрипт повторно все що знаходиться в файлі file.txt буде стерто і перезаписано");


            
        }


