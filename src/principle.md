# 原理

## 场景预设
假设工厂有一批产品需要在生产完成时马上决定这批产品是否符合交付要求，但是由于时间紧迫，无法等所有产品
都生产完成后再进行抽样质检。此时，工厂领导提出了以下要求：

- 被刚生产的产品 **马上进行检测**
- 取出 \\( K \\) （\\( K \le N \\)）个样品的质量结果报送
- 所有产品被报送的概率是**均等**

也就是, 在抽样总体数量 \\( N \\) 不确定的情况下，使得每个样本被抽到的的概率为 \\( K \over N \\)。
要达到以上要求，可以采用[蓄水池抽样算法]（[Reservoir Sampling Algorithm]）

[蓄水池抽样算法]: https://www.jianshu.com/p/7a9ea6ece2af
[Reservoir Sampling Algorithm]: https://www.geeksforgeeks.org/reservoir-sampling/

## 算法描述
蓄水池抽样算法对于第 \\( k \\) 个待抽取样本做如下操作：

1. 当前已经决定是否要抽取的样本数量 \\( n \\) 增加1；
2. 分情况讨论：
    - （初始化）如果 \\( n \le K \\)，则将 \\( k \\) 放到已抽取的样品池 \\( S \\)中，并记为第 \\( n \\)个元素 \\( S[n] \\)；
    - （概率逐渐变小地替换）如果 \\( n \gt K \\)，则随机生成数字 \\( r \\) ( \\( 1 \le r \le n \\) )：
        - 当且仅当 \\( r \le K \\)时，将 \\( k \\) 替换 \\( S[r] \\)。

## 公平性（等概率 \\( \frac{K}{N} \\) 抽取) 证明
因为蓄水池抽样是流式算法（[Streaming Algorithm](https://en.wikipedia.org/wiki/Streaming_algorithm)），所以使用数学归纳法证明比较清晰：

设:
- 参与抽样的总体数量为 \\( N \\)
- 要抽取样本数为 \\( K \\)（\\( K \le N \\)）
- \\( n \\) 表示第 \\( n \\)个要处理的元素
- \\( k \\) 表示前 \\( n \\)个元素中的任意一个

求证：蓄水池抽样抽取样本的概率为 \\( \frac{K}{N} \\)。

### 奠基
在初始化阶段，\\( n = N \\)，并且 \\( N \le K \\)，故样本 \\( n \\) 一定被抽样，故被抽样概率
\\( P\\{抽中n\\} = 1 = \frac{K}{K} \\)。在初始化该段最后，有:

\\[ \label{1} \tag{1}
    P\\{抽中n\\} = 1 =  \frac{K}{K} = \frac{K}{N} ( 1 \le n \le K = N )
\\]

### 归纳
假设在处理第 \\( n(n \ge K) \\) 个样本的时候，公式 \\( (\ref{1}) \\) 成立， 即有：

\\[ \label{2.0} \tag{2.0}
    P\\{上一轮已抽中k\\} = \frac{K}{n}
\\]

则当处理 第\\( n+1(= N \gt K) \\) 个样本时，有：

\\[ \label{2.1} \tag{2.1}
    P\\{抽中n+1\\} = \frac{K}{n+1} = \frac{K}{N}
\\]

前面已经处理过的 \\( n \\) 个样本被抽中的概率同样变化了，因为这些样本可能被第 \\( n + 1 \\) 样本替代：
\\[ \begin{equation}
        \begin{aligned}
            P\\{抽中k\\} & = P\\{上一轮已抽中k\\} \cdot (P\\{没有抽中n+1\\}
                + P\\{抽中n+1\\} \cdot P\\{n+1替换其他K-1个元素\\}) \\\\
            & = \frac{K}{n} \cdot [(1-\frac{K}{n+1}) + (\frac{K}{n+1} \cdot \frac{K-1}{K})] \\\\
            & = \frac{K}{n} \cdot [\frac{n+1-K}{n+1} + \frac{K-1}{n+1}] \\\\
            & = \frac{K}{n} \cdot [\frac{n+(1-K) + (K-1)}{n+1}] \\\\
            & = \frac{K}{n} \cdot \frac{n}{n+1} \\\\
            & = \frac{K}{n+1} \\\\
            & = \frac{K}{N}
        \end{aligned}
        \label{2.2} \tag{2.2}
    \end{equation}
\\]

由公式 \\( (\ref{2.1}) \\) 和公式 \\( (\ref{2.2}) \\) 可知：
在处理第 \\( n + 1 \\) 个样本时，任一样本被抽中的概率为 \\( \frac{K}{N} \\)。

### 结论
由前面证明可知，无论在[抽样池初始化阶段](#奠基)还是在之后的[随机替换阶段](#归纳)，均可以保持
每个样本被抽取的概率为 \\( \frac{K}{N} \\)。 证明完毕。
