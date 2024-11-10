use async_graphql::Object;

pub struct HelloWorldQuery;

#[Object]
impl HelloWorldQuery {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
