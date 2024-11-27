pub enum Value {
    Integer {
        value: i64,
    },
    Float {
        float: f64
    },
    String {
        string: String
    }
}
