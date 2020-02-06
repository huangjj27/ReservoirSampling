# 实现
## Rust
作者偏好 [Rust 编程语言](https://kaisery.gitbooks.io/trpl-zh-cn/)，故使用 Rust 实现。

### 特质（trait）
Rust 中的[特质（trait）](https://kaisery.gitbooks.io/trpl-zh-cn/ch10-02-traits.html)
是其用于复用行为抽象的特性，尽管比起 Java 或 C# 的接口 （Interface）更加强大，但在此文中，
熟悉 Java/C# 的读者把特质视作接口就可以了。

## 建模与实现
本文使用面向对象（Object-Oriented）编程范式[^1]来进行抽象，如下所示：

```rust
# extern crate rand;
use rand::random;

trait StreamSampler<Item> {
    // 流式采样器无法知道总体数据有多少个样本，因此只逐个处理
    fn process(&mut self, it: Item);

    // 任意时候应当知道当前抽取的样本有哪些
    fn samples(&self) -> &[Item];
}

struct ReserviorSampler<Item> {
    n: usize,
    K: usize,
    samples: Vec<Item>,
}

impl<Item> ReserviorSampler<Item> {
    // 采样之前就应该知道要抽K个样本。
    fn with_cap(K: usize) -> Self {
        Self {
            n: 0,
            K,
            samples: Vec::with_capacity(K),
        }
    }
}

impl<Item> StreamSampler<Item> for ReserviorSampler<Item> {
    fn process(&mut self, it: Item) {
        self.n += 1;

        // 初始化
        if self.n <= self.K {
            self.samples.push(it);
            return;
        }

        // 概率渐小的随机抽样
        let r = random::<usize>() % self.n;
        if r < self.K {
            self.samples[r] = it;
        }
    }

    fn samples(&self) -> &[Item] {
        &self.samples
    }
}

fn main() {
    let v = vec![8, 1, 1, 9, 2];
    let mut sampler = ReserviorSampler::<usize>::with_cap(3);

    for it in v {
        sampler.process(it);
    }

    println!("{:?}", sampler.samples());
}
```


[^1]: 作者理解的面向对象 = 对象是交互的最基本单元 + 对象通过相互发送消息进行交互。而特质/接口以及
对象其他公开的方法定义了对象可以向外发送/从外接收的消息。
