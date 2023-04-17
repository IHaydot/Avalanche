struct Foo{
    age: Result<u32, String>
}
impl Foo{
    pub fn new(age: u32) -> Self{
        if name < 18{
            Self{age: Err("Too young")} //None means that we basically return Null
        }
        Self{age: Ok(age)}
    }
}