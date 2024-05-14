use std::collections::HashMap;
use std::io;

#[derive(PartialEq)]
enum TempUnit {
    CELSIUS, FAHRENHEIT, KELVIN
}

const TEMP_UNITS: HashMap<&str, TempUnit> = HashMap::from([
    ("c", TempUnit::CELSIUS),
    ("f", TempUnit::FAHRENHEIT),
    ("k", TempUnit::KELVIN)
]);

fn main() {
    let from_unit = read_temp_unit(true, None);
    let to_unit = read_temp_unit(false, Some(from_unit));


}

fn read_temp_unit(from: bool, ignore_unit: Option<TempUnit>) -> TempUnit {
    let from: &str = if from { "from" } else { "to" };
    let units: &str = {
        let mut output = "";
        for unit in TEMP_UNITS.keys() {
            if ignore_unit.is_none() || TEMP_UNITS.get(unit).unwrap().eq(&ignore_unit.unwrap()) {
                output = &*(String::from(output) + unit + "/");
            }
        }

        &output[0..output.len() - 1]
    };

    let mut unit: TempUnit = TempUnit::CELSIUS;

    loop {
        print!("What would you like to convert {from}? ({units}) ");
        let mut from_unit_input = String::new();

        io::stdin()
            .read_line(&mut from_unit_input)
            .expect("Failed to read line");

        from_unit_input = from_unit_input.trim().parse().unwrap();

        break unit = TEMP_UNITS.get(&from_unit_input).unwrap();
    }

    return unit;
}