use std::io;

fn main() {
    loop {
        println!("1 Fahrenheit to Celsius");
        println!("2 Foot to Metre");
        println!("3 Inch to Centimetre");
        println!("4 Cup to Millilitre");
        println!("5 Fluid Ounce to Millilitre");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if choice < 1 || choice > 5 {
            break;
        }

        println!("Input:");

        let mut userinput = String::new();

        io::stdin()
            .read_line(&mut userinput)
            .expect("Failed to read line");

        let string_to_number: f64 = match userinput.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if choice == 1 {
            let _result: f64 = (string_to_number - 32.0) / 1.8;
            println!("{} °F equals {:.1} °C", string_to_number, _result);
        } else if choice == 2 {
            let _result: f64 = string_to_number * 0.3048;
            println!("{} ft equals {:.2} m", string_to_number, _result);
        } else if choice == 3 {
            let _result: f64 = string_to_number * 2.54;
            println!("{} in equals {:.2} cm", string_to_number, _result);
        } else if choice == 4 {
            let _result: f64 = string_to_number * 236.5882365;
            if string_to_number == 1.0 {
                println!("{} cup equals {:.0} mL", string_to_number, _result);
            } else {
                println!("{} cups equals {:.0} mL", string_to_number, _result);
            }
        } else {
            let _result: f64 = string_to_number * 28.4130625;
            println!("{} fl oz equals {:.0} mL", string_to_number, _result);
        }

        println!("Press 1 for another conversion");

        let mut userinput = String::new();

        io::stdin()
            .read_line(&mut userinput)
            .expect("Failed to read line");

        let exit: u32 = match userinput.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if exit == 1 {
            continue;
        } else {
            break;
        }
    }
}
