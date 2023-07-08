const SYMBOL_TABLE: [char; 10] = ['零', '壹', '贰', '叁', '肆', '伍', '陆', '柒', '捌', '玖'];
const UNIT_TABLE: [char; 4] = ['\0', '拾', '佰', '仟'];
const STEP_UNIT_TABLE: [char; 5] = ['\0', '万', '亿', '兆', '京'];

/// Convert u64 number to Chinese uppercase string
pub fn to_uppercase(number: u64) -> String {
    if number < 10 {
        return SYMBOL_TABLE[number as usize].to_string();
    }

    let mut chars: Vec<char> = Vec::new();
    let mut copy_number = number;
    let mut step: usize = 0;
    while copy_number > 0 {
        let r = copy_number % 10000;

        if step > 0 {
            if r > 0 {
                chars.push(STEP_UNIT_TABLE[step]);
            } else {
                if let Some(&'零') = chars.last() {
                    chars.pop();
                }
                chars.push('零');
            }
        }

        if r > 0 {
            chars.append(&mut number_slice_to_uppercase(pad_number(r).as_str()));
        }

        copy_number /= 10000;
        step += 1;
    }

    if let Some(&'零') = chars.first() {
        chars.remove(0);
    }

    if let Some(&'零') = chars.last() {
        chars.pop();
    }

    return String::from_iter(chars.iter().rev());
}

fn pad_number(number: u64) -> String {
    format!("{:04}", number)
}

fn number_slice_to_uppercase(number: &str) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();
    for (i, c) in number
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .rev()
        .enumerate()
    {
        if let (Some(&'零'), true) = (chars.last(), c == 0) {
            continue;
        }
        if c != 0 && i > 0 {
            chars.push(UNIT_TABLE[i]);
        }
        chars.push(SYMBOL_TABLE[c as usize]);
    }

    if let Some(&'零') = chars.first() {
        chars.remove(0);
    }

    chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase(0), "零");
        assert_eq!(to_uppercase(1), "壹");
        assert_eq!(to_uppercase(9), "玖");
        assert_eq!(to_uppercase(10), "壹拾");
        assert_eq!(to_uppercase(20), "贰拾");
        assert_eq!(to_uppercase(100), "壹佰");
        assert_eq!(to_uppercase(101), "壹佰零壹");
        assert_eq!(to_uppercase(1010), "壹仟零壹拾");
        assert_eq!(to_uppercase(10000), "壹万");
        assert_eq!(to_uppercase(10101), "壹万零壹佰零壹");
        assert_eq!(to_uppercase(1000000), "壹佰万");
        assert_eq!(to_uppercase(1000100), "壹佰万零壹佰");
        assert_eq!(to_uppercase(10000100), "壹仟万零壹佰");
        assert_eq!(to_uppercase(100001000), "壹亿零壹仟");
        assert_eq!(to_uppercase(1010001001), "壹拾亿壹仟万壹仟零壹");
        assert_eq!(to_uppercase(1011000101), "壹拾亿壹仟壹佰万零壹佰零壹");
        assert_eq!(
            to_uppercase(999999999999),
            "玖仟玖佰玖拾玖亿玖仟玖佰玖拾玖万玖仟玖佰玖拾玖"
        );
        assert_eq!(to_uppercase(1000000000000), "壹兆");
        assert_eq!(to_uppercase(1000000000001), "壹兆零壹");
        assert_eq!(
            to_uppercase(18446744073709551615),
            "壹仟捌佰肆拾肆京陆仟柒佰肆拾肆兆零柒佰叁拾柒亿零玖佰伍拾伍万壹仟陆佰壹拾伍"
        );
    }

    #[test]
    fn test_number_slice_to_uppercase() {
        assert_eq!(
            number_slice_to_uppercase("0001").iter().collect::<String>(),
            "壹零"
        );
        assert_eq!(
            number_slice_to_uppercase("0123").iter().collect::<String>(),
            "叁拾贰佰壹零"
        );
        assert_eq!(
            number_slice_to_uppercase("0120").iter().collect::<String>(),
            "拾贰佰壹零"
        );
        assert_eq!(
            number_slice_to_uppercase("1230").iter().collect::<String>(),
            "拾叁佰贰仟壹"
        );
        assert_eq!(
            number_slice_to_uppercase("1000").iter().collect::<String>(),
            "仟壹"
        );
    }
}
