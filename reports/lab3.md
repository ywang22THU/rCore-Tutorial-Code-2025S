# 实验报告 3

##### 王溢 2022010760

## 实现的功能

1. 实现了 `spawn` 与 `set_priority` 两个系统调用
2. 在 `TaskManager` 的 `fetch` 函数中实现了 Stride 调度算法
3. 修改了 `TaskControlBlockInner` 结构体的数据，添加了 Stride 调度算法相关的成员

## 问答题

1. 不是 `p1` 执行，因为使用 8bit 无符号整数存储时，更新后 `p2.stride = 250 + 10 = 4 < 255`，因此下一次还是 `p2` 执行
2. 每次递增的步长为 `pass = BigStride / 2`，假设第 `n` 次调度之前满足不同进程的 stride 差的绝对值范围为 `[0, BigStride / 2]`，则第 `n + 1` ，最大差值为 `|0 - pass| = Bigstride / 2`，同样满足定义，而在初始状态下 `stride >= BigStride / 2`，满足假设，因此可以归纳
3. 补全代码如下：
```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let delta = self.0.wrapping_sub(other.0);
        let half = u64::MAX / 2;
        if delta > half {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
```

## honor code

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

