# Filter-macros
Easy to implement [filter](https://github.com/Spxg/filter) traits.

## Macros
* add_fields
* BitAnd
* BitOr
* OpUnitTrait

## Example
```rust
#[add_fields]
#[derive(BitAnd, BitOr, OpUnitTrait)]
enum NumType {
    Odd,
    Even,
    DivByThree,
    DivByFour,
    DivByFive,
    IsMagicNum(i32),
}
```
expand:
```rust
enum NumType {
    Odd,
    Even,
    DivByThree,
    DivByFour,
    DivByFive,
    IsMagicNum(i32),
    Unit(Box<OpUnit>),
    Unknown,
}

impl std::ops::BitAnd for NumType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let node = OpUnit::new(Some(self), Some(rhs), Operation::And);
        NumType::Unit(Box::new(node))
    }
}

impl std::ops::BitOr for NumType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let node = OpUnit::new(Some(self), Some(rhs), Operation::Or);
        NumType::Unit(Box::new(node))
    }
}

impl OpUnitTrait for NumType {
    fn op_unit(&self) -> OpUnit<Self> {
        match self {
            NumType::Unit(unit) => *unit.to_owned(),
            ty => OpUnit::new(Some(ty.clone()), None, Operation::Single),
        }
    }
}

impl std::default::Default for NumType {
    fn default() -> Self {
        Self::Unknown
    }
}
```
