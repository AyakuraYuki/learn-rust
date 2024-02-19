// keynote
// 1. HashMap 使用了哈希来保证尽可能少的键值冲突，一个简单的理解是，可以看作为 Key 的数据类型是否实现了 std::cmp::Eq 特征

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use ahash::{AHashMap, RandomState};

    // 一种比较蠢的办法，手动的从元组转换成 HashMap
    #[test]
    fn create_from_tuples() {
        let teams = vec![
            ("Team Spirit".to_string(), 1),
            ("Azure Ray".to_string(), 2),
            ("LGD Gaming".to_string(), 3),
        ];
        let mut team_map = HashMap::with_capacity(6);
        for team in &teams {
            team_map.insert(&team.0, &team.1); // 插入 KV 数据
        }
        team_map.shrink_to_fit();
        println!("{:?}", team_map);
    }

    // 利用迭代器和 collect 构建 HashMap
    #[test]
    fn create_from_tuples_using_iter() {
        let teams = vec![
            ("Team Spirit".to_string(), 1),
            ("Azure Ray".to_string(), 2),
            ("LGD Gaming".to_string(), 3),
        ];
        // 这句代码比较晦涩，其中 HashMap<_, _> 会告诉编译器，收集成为 HashMap 集合类型，两个下划线则可以由类型推导拿到真实类型
        // 如果不写类型声明，编译器会报错，提示 type annotations needed【需要类型标注】
        let team_map: HashMap<_, _> = teams.into_iter().collect();
        println!("{:?}", team_map);
    }

    #[test]
    fn access_by_key() {
        let map: HashMap<_, _> = vec![
            ("hello".to_string(), 5),
            ("world".to_string(), 6),
        ].into_iter().collect();
        let key = String::from("world");
        let val = map.get(&key).copied().unwrap_or(0);
        println!("val: {:?}", val);
    }

    #[test]
    fn access_iterate() {
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);
        for (k, v) in &scores {
            println!("{}: {}", k, v);
        }
    }

    #[test]
    fn access_update() {
        let mut scores = HashMap::new();
        scores.insert("Blue", 10); // 普通插入

        // 覆盖
        let old = scores.insert("Blue", 20);
        assert_eq!(old, Some(10)); // 把被覆盖的原值拿出来

        // 查询更新后的值
        let new = scores.get("Blue");
        assert_eq!(new, Some(&20));

        // 查询，不存在则插入
        let v = scores.entry("Yellow").or_insert(5);
        assert_eq!(*v, 5); // 返回的 v 是 &mut i32，所以需要解引用去对比值

        // 查询，不存在则插入，存在则不更新
        let v = scores.entry("Yellow").or_insert(50);
        assert_eq!(*v, 5); // 返回的 v 是 &mut i32，所以需要解引用去对比值
    }

    #[test]
    fn update_entry_example_counter() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        // 空格切分 text 并遍历单词
        for word in text.split_whitespace() {
            // 这里拿到一个对 value 的可变引用
            // 在这里，or_insert 会对词语做判断，如果之前没有插入过，则插入 0 作为 Value 的一组 KV 数据
            let count = map.entry(word).or_insert(0);
            // 解引用并对 value 自增
            *count += 1;
        }
    }

    #[test]
    fn use_third_party_hash_implement() {
        // 类型声明的第三个类型声明表示这个 HashMap 应该使用哪个哈希函数的实现
        let mut scores: HashMap<_, _, RandomState> = HashMap::default();
        scores.insert(1, 10);
        dbg!(&scores);

        // AHashMap 是第三方哈希函数实现 github.com/tkaitchuck/ahash 的特有 HashMap 类型，本质上是上面那种创建 HashMap 的类型定义的一种类型别名
        let mut scores = AHashMap::new();
        scores.insert(2, 20);
        dbg!(&scores);
    }
}
