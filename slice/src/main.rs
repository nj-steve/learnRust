

fn main() {
    ///////array type
    let arr: [i32; 3] = [1, 2, 3];       // 定义一个[i32; 3]类型的数组，默认不可变
    let mut mut_arr = [1, 2, 3];        // 定义一个可变数组
    assert_eq!(1, mut_arr[0]);         // 数组索引从0开始，验证第一位元素等于1
    mut_arr[0] = 3;                            // 修改mut_arr第一个元素为3，因为它是可变数组
    assert_eq!(3, mut_arr[0]);        // 验证修改之后的mut_arr数组第一个元素为3
    let init_arr = [0; 10];                  // 创建一个初始值为0，长度为10的数组
    assert_eq!(0, init_arr[5]);         // 通过数组下标访问数组元素，验证init_arr数组中任意一个元素的值是否为0
    assert_eq!(10, init_arr.len());  // 验证数组的长度是否为10
    // println!("{:?}", arr[5]); // error: 索引超出范围

    //////range type
    // (1..5)是结构体std::ops::Range的一个实例
    assert_eq!((1..5), std::ops::Range{ start: 1, end: 5 });
    // (1..=5)是结构体std::ops::Range的一个实例
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3+4+5, (3..6).sum());
    assert_eq!(3+4+5+6, (3..=6).sum());
    // 每个范围都是一个迭代器，可用for 循环打印范围内的元素
    for i in (1..5) {
        println!("(1..5) {}", i);
    }
    for i in (1..=5) {
        println!("(1..=5) {}", i);
    }

    //////slice type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2,3,4,5]);
    assert_eq!(&arr[1..], [2,3,4,5]);
    assert_eq!(&arr.len(), &5);
    assert_eq!(&arr.is_empty(), &false);
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);
    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1,2,3]);
    let str_slice: &[&str] = &["one", "two", "three"];
    assert_eq!(str_slice, ["one", "two", "three"]);
    //切片类型是对一个数组的的引用片段，有利于安全有效的访问数组的一部分，不需要拷贝。理论上，切片引用的是已经存在的变量。
    //系统底层，切片代表一个指向数组起始位置的指针和数组长度。用[T]来表示连续序列，那么切片类型就是&[T]和&mut[T]。
    //通过引用操作符&对数组进行引用，就产生一个切片&arr。也可以结合范围对数组进行切割，比如&arr[1..],表示获取arr数组中索引位置1之后的所有元素。
    //可以通过&mut定义可变切片，可以直接通过索引来修改相应位置的值。
    //可以通过vec！宏定义的动态数组，可以通过操作符得到一个切片。

    let x = &[1, 2, 3, 4, 5, 6];
    unsafe {
        assert_eq!(x.get_unchecked(1), &2);
        //assert_eq!(x.get_unchecked(1), 2); no implementation for `&{integer} == {integer}`
        assert_eq!(x[1], 2);
        //assert_eq!(x[1], &2);no implementation for `{integer} == &{integer}`
        println!("Hello, world == {}!",x.get_unchecked(1) + 1 );
        println!("Hello, world x[1] == {}!",x[1] + 1);

        let ret = x.get_unchecked(..);
        for i in ret {
            println!("iiiii = {}",i);
        }
    }
    //get_unchecked
    //Returns a reference to an element or subslice, without doing bounds checking. //返回对元素或子片的引用，而不进行边界检查。
    //This is generally not recommended, use with caution!
    //Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used. For a safe alternative see get.
    //这通常是不推荐的，小心使用!使用越界索引调用此方法是未定义的行为，即使结果引用未被使用。要找到一个安全的替代方案，请参阅get。

    //get_unchecked_mut
    let xx = &mut [1, 2, 4];
    unsafe {
        let elem = xx.get_unchecked_mut(1);
        assert_eq!(elem, &2);
        //assert_eq!(elem +1, &2 +1);
        //&mut {integer}  {integer}
        //`+` can be used on '{integer}', you can dereference `elem`: `*elem`
        *elem = 13;
    }
    assert_eq!(xx, &[1, 13, 4]);

    //copy_from_slice
    //Rust enforces that there can only be one mutable reference with no immutable references to a particular piece of data in a particular scope.
    //Because of this, attempting to use `copy_from_slice` on a single slice will result in a compile failure:
    //Rust强制要求在特定范围内只能有一个可变引用，而不能有对特定数据段的不可变引用。
    //因此，尝试使用' copy_from_slice '在单个片将导致编译失败:
    /// compile_fail
    /// let mut slice = [1, 2, 3, 4, 5];
    /// slice[..2].copy_from_slice(&slice[3..]); // compile fail!
    /// To work around this, we can use [`split_at_mut`] to create two distinct sub-slices from a slice:
    /// 要解决这个问题，我们可以使用[' split_at_mut ']从一个切片创建两个不同的子切片:
    let mut slice = [1, 2, 3, 4, 5];
    {
        let (left, right) = slice.split_at_mut(2);
        left.copy_from_slice(&right[1..]);
    }
    assert_eq!(slice, [4, 5, 3, 4, 5]);

    ///split_at_mut
    let mut v = [1, 0, 3, 0, 5, 6];
    // scoped to restrict the lifetime of the borrows
    {
    let (left, right) = v.split_at_mut(2);
    assert!(left == [1, 0]);
    assert!(right == [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
    }
    assert!(v == [1, 2, 3, 4, 5, 6]);

    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));

    println!("Hello, world!");
}
