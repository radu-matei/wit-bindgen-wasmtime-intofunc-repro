wit_bindgen_wasmtime::export!("test.wit");

struct Test {}

impl test::Test for Test {
    fn testfunc(
        &mut self,
        p1: test::Param1<'_>,
        p2: Option<test::Param2<'_>>,
    ) -> Result<test::Ret, test::Error> {
        todo!()
    }
}
