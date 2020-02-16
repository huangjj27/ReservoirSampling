# 应用-多种奖励抽奖（Lottery）

另外一个经常用到蓄水池抽样算法的场景，可能是像这样的抽奖场景：

- 总抽奖人数未知
- 参加抽奖的玩家随机顺序与时间加入
- 要求每个抽奖者中奖的机会均等

以上场景如果是只有一种奖品的抽奖，那么一般[前述的蓄水池算法](/principle.html#算法描述)即可满足需要。但如果增加了一下要求：

- 奖项一共分为 \\( k_1, k_2, \ldots, k_m \\) 一共 \\( m \\) 种，人数分别为 \\( k_1, k_2, \ldots, k_m \\)，合计中奖人数 \\( K = \sum_{i=1}^{m} k_i \\) 人
- 每个玩家最多只能抽取一个奖项

因为原本的蓄水池算法没有对抽样池进行不同类型的区分，为了满足新要求，我们需要增加不同类型样本的区分，同时还要保持抽样等概率性。

## 改进
一种保证概率均等的方式是在抽完中奖者之后，打乱一次中奖者的顺序，再依次分发奖品。这样，因为每个用户停留在一个特定位置的概率为
\\( \frac{1}{K} \\)，并且获得奖品由在奖池钟的位置决定，故抽中奖品 \\( k_i \\) 的概率为 \\( \frac{k_i}{K} \\)。
而每个参加者中奖的概率为 \\( \frac{K}{N} \\)，故每个中奖者抽中奖品 \\( k_i \\) 的概率为 \\( \frac{k_i}{N} \\)。

此外，考虑到可能在抽奖截止时参与抽奖的人数仍然没有超过 \\( K \\) 个人，在此时我们需要补足 \\( K \\) 个位置再打乱中奖者顺序，否则参与抽奖者一定能获得最好的奖励。

## 实现

```rust
# extern crate rand;
use rand::random;
use rand::seq::SliceRandom;
use rand::thread_rng;

# trait StreamSampler {
#     // 每种抽样器只会在一种总体中抽样，而总体中所有个体都属于相同类型
#     type Item;
#
#     // 流式采样器无法知道总体数据有多少个样本，因此只逐个处理
#     fn process(&mut self, it: Self::Item);
#
#     // 任意时候应当知道当前抽取的样本有哪些
#     fn samples(&self) -> &[Self::Item];
# }
#
struct Lottery<Item> {
    // 记录一共有多少人参与抽奖
    total: usize,

    // 奖品的名称与人数
    prices: Vec<Price>,

    lucky: Vec<Item>,
}

#[derive(Clone)]
struct Price {
    name: String,
    cap: usize,
}

impl<Item> StreamSampler for Lottery<Item> {
    type Item = Item;

    fn process(&mut self, it: Self::Item) {
        let lucky_cap = self.prices.iter()
            .map(|p| p.cap)
            .sum::<usize>();

        self.total += 1;

        // 初始化
        if self.total <= lucky_cap {
            self.lucky.push(it);
            return;
        }

        // 概率渐小的随机替换
        let r = random::<usize>() % self.total;
        if r < lucky_cap {
            self.lucky[r] = it;
        }
    }

    fn samples(&self) -> &[Self::Item] {
        &self.lucky[..]
    }
}

impl<Item> Lottery<Item> {
    fn result(self) -> Result<Vec<(String, Vec<Item>)>, &'static str> {
        let lucky_cap = self.prices.iter()
            .map(|p| p.cap)
            .sum::<usize>();

        if self.lucky.len() == 0 {
            return Err("No one attended to the lottery!");
        }

        let mut final_lucky = self.lucky.into_iter()
            .map(|it| Some(it))
            .collect::<Vec<Option<Item>>>();

        if final_lucky.len() < lucky_cap {
            final_lucky.resize_with(lucky_cap, || Option::<Item>::None);
        }

        let mut rng = thread_rng();
        final_lucky.shuffle(&mut rng);

        let mut result = Vec::with_capacity(self.prices.len());

        let mut counted = 0;
        for p in self.prices {
            let mut luck = Vec::with_capacity(p.cap);

            for i in 0 .. p.cap {
                if let Some(it) = final_lucky[counted + i].take() {
                    luck.push(it);
                }
            }

            result.push((p.name, luck));
            counted += p.cap;
        }

        Ok(result)
    }
}

// 构建者模式（Builder Pattern），将所有可能的初始化行为提取到单独的构建者结构中，以保证初始化
// 后的对象(Target)的数据可靠性。此处用以保证所有奖品都确定后才能开始抽奖
struct LotteryBuilder {
    prices: Vec<Price>,
}

impl LotteryBuilder {
    fn new() -> Self {
        LotteryBuilder {
            prices: Vec::new(),
        }
    }

    fn add_price(&mut self, name: &str, cap: usize) -> &mut Self {
        self.prices.push(Price { name: name.into(), cap });
        self
    }

    fn build<Item>(&self) -> Lottery<Item> {
        Lottery {
            total: 0,
            prices: self.prices.clone(),
            lucky: Vec::new(),
        }
    }
}

fn main() {
    let v = vec![8, 1, 1, 9, 2];
    let mut lottery = LotteryBuilder::new()
        .add_price("一等奖", 1)
        .add_price("二等奖", 1)
        .add_price("三等奖", 5)
        .build::<usize>();


    for it in v {
        lottery.process(it);
    }

    println!("{:?}", lottery.samples());

    println!("{:?}", lottery.result().unwrap());
}
```
