---
title: 真理値
---

ラムダ計算には true や false といった値はありません。
が、Tuber においてはそれらと同等に扱える関数 `TRUE`, `FALSE` が定義されています。

`TRUE` は２つの引数を取る関数で下記のように振る舞います。

```
TRUE(x, y)  # x が返る
```

`FALSE` は２つの引数を取る関数で下記のように振る舞います。

```
FALSE(x, y)  # y が返る
```

### `TRUE`, `FALSE` は三項演算子のように振る舞う

`TRUE`, `FALSE` はそれ自体が三項演算子のように振る舞うよう定義されています。

```
b = ...  # 変数 b に TRUE, FALSE のいずれかが紐づいているとして

x = b(a, b)  # もし b が TRUE なら x = a になる
             # もし b が FALSE なら x = b になる
```

## 論理演算

Tuber には下記の論理演算が組み込みで用意されています。

- 否定: `NOT`
- 論理和: `OR`
- 論理積: `AND`
- 排他的論理和: `XOR`

### 否定 `NOT`

`NOT` は 1 つの引数を受け取り、`TRUE` に対しては `FALSE` を返し、`FALSE` に対しては `TRUE` を返します。

```
NOT(TRUE)   # => FALSE
NOT(FALSE)  # => TRUE
```

### 論理和 `OR`

`OR` は 2 つの引数に対してそれらの論理和を返します。

```
OR(FALSE, FALSE)  # => FALSE
OR(TRUE , FALSE)  # => TRUE
OR(TRUE , TRUE )  # => TRUE
```

### 論理積 `AND`

`AND` は 2 つの引数に対してそれらの論理積を返します。

```
AND(FALSE, FALSE)  # => FALSE
AND(TRUE , FALSE)  # => FALSE
AND(TRUE , TRUE )  # => TRUE
```

### 排他的論理和 `XOR`

`XOR` は 2 つの引数に対してそれらの排他的論理和を返します。

```
AND(FALSE, FALSE)  # => FALSE
AND(TRUE , FALSE)  # => TRUE
AND(TRUE , TRUE )  # => FALSE
```
