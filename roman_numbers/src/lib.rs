#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(number: u32) -> Self {
        match number {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla, // fallback, though normally not used
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut number: u32) -> Self {
        use RomanDigit::*;

        let mut digits = Vec::new();

        if number == 0 {
            return RomanNumber(vec![Nulla]);
        }

        while number >= 1000 {
            digits.push(M);
            number -= 1000;
        }
        if number >= 900 {
            digits.push(C);
            digits.push(M);
            number -= 900;
        }
        if number >= 500 {
            digits.push(D);
            number -= 500;
        }
        if number >= 400 {
            digits.push(C);
            digits.push(D);
            number -= 400;
        }
        while number >= 100 {
            digits.push(C);
            number -= 100;
        }
        if number >= 90 {
            digits.push(X);
            digits.push(C);
            number -= 90;
        }
        if number >= 50 {
            digits.push(L);
            number -= 50;
        }
        if number >= 40 {
            digits.push(X);
            digits.push(L);
            number -= 40;
        }
        while number >= 10 {
            digits.push(X);
            number -= 10;
        }
        if number >= 9 {
            digits.push(I);
            digits.push(X);
            number -= 9;
        }
        if number >= 5 {
            digits.push(V);
            number -= 5;
        }
        if number >= 4 {
            digits.push(I);
            digits.push(V);
            number -= 4;
        }
        while number >= 1 {
            digits.push(I);
            number -= 1;
        }

        RomanNumber(digits)
    }
}
