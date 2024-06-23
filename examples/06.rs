enum One {
    Two(Two),
    Three(Three),
}

enum Two {
    A(i32),
    B { x: u32, y: u32 },
}

enum Three {
    C(String, f64, u128),
    D,
}
