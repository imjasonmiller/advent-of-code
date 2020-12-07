use anyhow::Context;

struct PasswordPolicy {
    password: String,
    repeat_min: usize,
    repeat_max: usize,
    repeat_char: u8,
}

impl PasswordPolicy {
    fn is_valid_1(&self) -> bool {
        let char_count = self
            .password
            .bytes()
            .filter(|&b| b == self.repeat_char)
            .count();

        self.repeat_min <= char_count && char_count <= self.repeat_max
    }

    fn is_valid_2(&self) -> bool {
        let has_char_at_min = self
            .password
            .bytes()
            .nth(self.repeat_min - 1)
            .map_or(false, |b| b == self.repeat_char);
        let has_char_at_max = self
            .password
            .bytes()
            .nth(self.repeat_max - 1)
            .map_or(false, |b| b == self.repeat_char);

        (has_char_at_min || has_char_at_max) && !(has_char_at_min && has_char_at_max)
    }
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let valid_passwords = input
        .lines()
        .filter_map(parse_password_policy)
        .filter(|p| p.is_valid_1())
        .count();

    Ok(valid_passwords)
}

fn parse_password_policy(s: &str) -> Option<PasswordPolicy> {
    let (rest, (repeat_min, repeat_max)) = parse_repeat_minmax(&s).ok()?;
    let (rest, repeat_char) = parse_repeat_char(&rest).ok()?;
    let password = parse_password(&rest).ok()?;

    Some(PasswordPolicy {
        password,
        repeat_min,
        repeat_max,
        repeat_char,
    })
}

fn parse_password(s: &str) -> anyhow::Result<String> {
    let (_, password) = take_while(&s, |&b| matches!(b, b'a'..=b'z'));

    Ok(password)
}

fn parse_repeat_char(s: &str) -> anyhow::Result<(&str, u8)> {
    let (rest, repeat_char) = take_while(&s, |&b| matches!(b, b'a'..=b'z'));
    let repeat_char = repeat_char
        .bytes()
        .next()
        .context("No character was found")?;

    let rest = consume_while(&rest, |&b| matches!(b, b':' | b' '));

    Ok((rest, repeat_char))
}

fn parse_repeat_minmax(s: &str) -> anyhow::Result<(&str, (usize, usize))> {
    let (rest, repeat_min) = take_while(&s, |&b| matches!(b, b'0'..=b'9'));
    let repeat_min = repeat_min.parse::<usize>()?;

    let rest = consume_while(&rest, |&b| matches!(b, b'-'));

    let (rest, repeat_max) = take_while(&rest, |&b| matches!(b, b'0'..=b'9'));
    let repeat_max = repeat_max.parse::<usize>()?;

    let rest = consume_while(&rest, |&b| matches!(b, b' '));

    Ok((rest, (repeat_min, repeat_max)))
}

fn take_while<F: Fn(&u8) -> bool>(s: &str, cond: F) -> (&str, String) {
    let take = s
        .bytes()
        .take_while(cond)
        .map(char::from)
        .collect::<String>();
    (&s[take.len()..], take)
}

fn consume_while<F: Fn(&u8) -> bool>(s: &str, cond: F) -> &str {
    let consumed = s.bytes().take_while(cond).count();
    &s[consumed..]
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let valid_passwords = input
        .lines()
        .filter_map(parse_password_policy)
        .filter(|p| p.is_valid_2())
        .count();

    Ok(valid_passwords)
}
