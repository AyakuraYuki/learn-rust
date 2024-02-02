mod hashmap;

fn main() {}

trait IPAddress {
    fn display(&self);
}

#[derive(Debug, PartialEq, Eq)]
struct V4(String);

impl IPAddress for V4 {
    fn display(&self) {
        println!("(IPv4) {}", self.0);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct V6(String);

impl IPAddress for V6 {
    fn display(&self) {
        println!("(IPv6) {}", self.0);
    }
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use crate::{IPAddress, V4, V6};

    #[test]
    fn test_borrow_from_vec() {
        // 下面这四句的写法会引起编译错误，因为 first 尝试借用 v 的第一个元素，并且在使用 first 前给 v 添加了一个新的元素
        // 添加元素的操作会改变 v 的大小，重新分配更大的空间会拷贝旧数组的数据，这种情况下，之前的引用会指向一块无效的内存空间
        // let mut v = vec![1, 2, 3, 4, 5];
        // let first = &v[0];
        // v.push(6);
        // println!("first element is {first}");

        // 下面是正确的写法
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        println!("first element is {first}");
        v.push(6);
    }

    // 存储不同的枚举对象
    #[test]
    fn store_different_type_of_elements() {
        let v = vec![
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        ];
        for ip in v {
            println!("ip: {}", ip.to_string());
        }
    }

    // 存储同一个特性的不同实现的对象
    #[test]
    fn store_different_objects_with_same_trait() {
        let v: Vec<Box<dyn IPAddress>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];
        for ip in v {
            ip.display();
        }
    }

    #[test]
    fn some_methods() {
        let mut v = vec![1, 2];
        assert!(!v.is_empty()); // 检查空

        println!("size: {}", v.len()); // 数组的长度

        v.insert(2, 3); // 在指定位置插入数据，第一个参数不可以大于 v 的长度
        println!("after insert: {:?}", v);

        assert_eq!(v.remove(1), 2); // 移除指定索引的元素，返回被移除的元素的值

        assert_eq!(v.pop(), Some(3)); // 推出尾部元素
        assert_eq!(v.pop(), Some(1)); // 这个时候 v 已空
        assert_eq!(v.pop(), None); // pop() 操作返回的是 Option，所以空的数组再 pop() 是一个 None

        v.clear(); // 清空数组

        let mut another_v = [11, 22].to_vec();
        v.append(&mut another_v); // 追加元素，这个操作会清空原数组，所以需要增加可变声明
        v.truncate(1); // 截断数组到指定长度，这句话在这里会把 v 截断到只剩下 [11]
        v.retain(|x| *x > 10); // 保留满足条件的元素，不满足条件的会被删除

        let mut v = vec![11, 22, 33, 44, 55];
        let mut m: Vec<_> = v.drain(1..=3).collect(); // 从原数组摘出指定范围的元素，这个操作后，v: [11, 55]，m: [22, 33, 44]
        println!("v: {:?}, m: {:?}", v, m);

        v.shrink_to_fit(); // （一般不会主动调用的）释放剩余的容量，这个方法会把例如容量 100 的数组释放掉没有使用的部分，剩下的容量足够存放已有元素

        let another_v = m.split_off(1); // 从指定索引切分数组，拿到切分的后面部分成为新的数组
        println!("m: {:?}, another_v: {:?}", m, another_v);

        let v = vec![1, 2, 3, 4, 5];
        let slice = &v[1..=3]; // 这种截取，v 不会改变
        println!("v: {:?}, slice: {:?}", v, slice);

        let v = vec![
            V4("192.168.1.101".to_string()),
            V4("192.168.1.102".to_string()),
            V4("192.168.1.103".to_string()),
            V4("192.168.1.104".to_string()),
        ];
        let slice = &v[1..=2];
        println!("v: {:?}, slice: {:?}", v, slice);
    }
}
