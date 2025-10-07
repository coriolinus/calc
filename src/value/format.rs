use num_runtime_fmt::Numeric;

use crate::Value;

impl Numeric for Value {
    type BinIter = Box<dyn Iterator<Item = char>>;
    type OctIter = Box<dyn Iterator<Item = char>>;
    type DecLeftIter = Box<dyn Iterator<Item = char>>;
    type DecRightIter = Box<dyn Iterator<Item = char>>;
    type HexIter = Box<dyn Iterator<Item = char>>;

    fn binary(&self) -> Option<Self::BinIter> {
        match self {
            Value::UnsignedInt(n) => n.binary().map(|iter| Box::new(iter) as _),
            Value::UnsignedBigInt(n) => n.binary().map(|iter| Box::new(iter) as _),
            Value::SignedInt(n) => n.binary().map(|iter| Box::new(iter) as _),
            Value::SignedBigInt(n) => n.binary().map(|iter| Box::new(iter) as _),
            Value::Float(n) => n.raw().binary().map(|iter| Box::new(iter) as _),
        }
    }

    fn octal(&self) -> Option<Self::OctIter> {
        match self {
            Value::UnsignedInt(n) => n.octal().map(|iter| Box::new(iter) as _),
            Value::UnsignedBigInt(n) => n.octal().map(|iter| Box::new(iter) as _),
            Value::SignedInt(n) => n.octal().map(|iter| Box::new(iter) as _),
            Value::SignedBigInt(n) => n.octal().map(|iter| Box::new(iter) as _),
            Value::Float(n) => n.raw().octal().map(|iter| Box::new(iter) as _),
        }
    }

    fn decimal(&self) -> (Self::DecLeftIter, Option<Self::DecRightIter>) {
        match self {
            Value::UnsignedInt(n) => {
                let (l, r) = n.decimal();
                let l = Box::new(l) as _;
                let r = r.map(|iter| Box::new(iter) as _);
                (l, r)
            }
            Value::UnsignedBigInt(n) => {
                let (l, r) = n.decimal();
                let l = Box::new(l) as _;
                let r = r.map(|iter| Box::new(iter) as _);
                (l, r)
            }
            Value::SignedInt(n) => {
                let (l, r) = n.decimal();
                let l = Box::new(l) as _;
                let r = r.map(|iter| Box::new(iter) as _);
                (l, r)
            }
            Value::SignedBigInt(n) => {
                let (l, r) = n.decimal();
                let l = Box::new(l) as _;
                let r = r.map(|iter| Box::new(iter) as _);
                (l, r)
            }
            Value::Float(n) => {
                let (l, r) = n.raw().decimal();
                let l = Box::new(l) as _;
                let r = r.map(|iter| Box::new(iter) as _);
                (l, r)
            }
        }
    }

    fn hex(&self) -> Option<Self::HexIter> {
        match self {
            Value::UnsignedInt(n) => n.hex().map(|iter| Box::new(iter) as _),
            Value::UnsignedBigInt(n) => n.hex().map(|iter| Box::new(iter) as _),
            Value::SignedInt(n) => n.hex().map(|iter| Box::new(iter) as _),
            Value::SignedBigInt(n) => n.hex().map(|iter| Box::new(iter) as _),
            Value::Float(n) => n.raw().hex().map(|iter| Box::new(iter) as _),
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Value::UnsignedInt(_) | Value::UnsignedBigInt(_) => false,
            Value::SignedInt(n) => n.is_negative(),
            Value::SignedBigInt(n) => n.is_negative(),
            Value::Float(n) => n.raw().is_sign_negative(),
        }
    }
}
