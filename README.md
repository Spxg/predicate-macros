# predicate-macros
Easy to implement [predicate](https://github.com/Spxg/predicate) traits.

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
    Unit(OpUnitInnerType<OpUnit<NumType>>),
    Unknown,
}

impl std::default::Default for NumType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::ops::BitAnd for NumType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let node = OpUnit::new(
            Some(OpUnitInnerType::new(self)),
            Some(OpUnitInnerType::new(rhs)),
            Operation::And,
        );
        NumType::Unit(OpUnitInnerType::new(node))
    }
}

impl std::ops::BitOr for NumType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let node = OpUnit::new(
            Some(OpUnitInnerType::new(self)),
            Some(OpUnitInnerType::new(rhs)),
            Operation::Or,
        );
        NumType::Unit(OpUnitInnerType::new(node))
    }
}

impl OpUnitTrait for NumType {
    fn get_op_unit(self: &OpUnitInnerType<Self>) -> OpUnitInnerType<OpUnit<Self>> {
        match self.as_ref() {
            NumType::Unit(unit) => unit.clone(),
            _ => OpUnitInnerType::new(OpUnit::new(Some(self.clone()), None, Operation::Single)),
        }
    }
}
```
