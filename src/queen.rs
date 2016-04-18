use std::collections::HashSet;

struct Location {
    row: i8,
    col: i8
}

fn get_disable_col(queen_location: Location, row: i8) -> Vec<i8>{
    // 获取所有不能用的列
    let left = queen_location.col + queen_location.row - row;
    let right = queen_location.col - queen_location.row + row;
    let result = vec![queen_location.col];
    if left > 0 { result.push(left) }
    if right < 8 { result.push(right) }
    return result;
}

fn find_queen_position(result: Vec<Location>, row: i8) -> HashSet<i8>{
    let mut disabled_col:HashSet<i8> = HashSet::new();
    for location in result{
        for col in get_disable_col(location, row){
            disabled_col.insert(col);
        }
    }
    let available_col: HashSet<i8> = [0, 1, 2, 3, 4, 5, 6, 7].iter().cloned().collect();
    return available_col.difference(&disabled_col)
}
