# predicate-macros
[![crates.io version](https://img.shields.io/crates/v/predicate-macros.svg)](https://crates.io/crates/predicate-macros)

Easy to implement [predicate](https://github.com/Spxg/predicate) traits.

## Macros
* add_field
* BitAnd
* BitOr
* OpUnitTrait

## Example
```rust
#[add_field]
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
    Unit(OpUnitRcType<OpUnit<NumType>>),
}

impl std::ops::BitAnd for NumType {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let node = OpUnit::new(
            Some(OpUnitRcType::new(self)),
            Some(OpUnitRcType::new(rhs)),
            Operation::And,
        );
        NumType::Unit(OpUnitRcType::new(node))
    }
}

impl std::ops::BitOr for NumType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let node = OpUnit::new(
            Some(OpUnitRcType::new(self)),
            Some(OpUnitRcType::new(rhs)),
            Operation::Or,
        );
        NumType::Unit(OpUnitRcType::new(node))
    }
}

impl OpUnitTrait for NumType {
    fn get_op_unit(self: &OpUnitRcType<Self>) -> OpUnitRcType<OpUnit<Self>> {
        match self.as_ref() {
            NumType::Unit(unit) => unit.clone(),
            _ => OpUnitRcType::new(OpUnit::new(Some(self.clone()), None, Operation::Single)),
        }
    }
}
```
