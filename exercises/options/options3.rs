// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // some ref
        // 避免不必要的复制或克隆
        // 允许在不获取所有权的情况下访问数据
        // 可以用于不可复制（non-Copy）类型
        // 在处理嵌套的Option或复杂结构时特别有用
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
