use yew::prelude::*;

pub fn keyboard_code_to_character(code: KeyboardEvent) -> char {
    if code.shift_key() {
        let conversion = match code.code().as_str() {
            "Equal" => '+',
            "Digit5" => '%',
            &_ => '\0',
        };
        return conversion;
    }

    let conversion = match code.code().as_str() {
        "Numpad0" => '0',
        "Numpad1" => '1',
        "Numpad2" => '2',
        "Numpad3" => '3',
        "Numpad4" => '4',
        "Numpad5" => '5',
        "Numpad6" => '6',
        "Numpad7" => '7',
        "Numpad8" => '8',
        "Numpad9" => '9',
        "Digit0" => '0',
        "Digit1" => '1',
        "Digit2" => '2',
        "Digit3" => '3',
        "Digit4" => '4',
        "Digit5" => '5',
        "Digit6" => '6',
        "Digit7" => '7',
        "Digit8" => '8',
        "Digit9" => '9',

        "Enter" => '=',
        "Minus" => '-',
        "Period" => '.',

        "NumpadEnter" => '=',
        "NumpadAdd" => '+',
        "NumpadSubtract" => '-',
        "NumpadMultiply" => '*',
        "NumpadDivide" => '/',
        "NumpadDecimal" => '.',

        "Delete" => 'd',
        "Backspace" => 'd',
        &_ => '\0',
    };
    conversion
}
