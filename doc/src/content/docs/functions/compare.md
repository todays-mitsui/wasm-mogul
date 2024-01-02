---
title: 比較演算
---

Tuber には [自然数](/functions/number) に対して下記の比較演算が組み込みで用意されています。

- ゼロ等価: `IS_ZERO`
- 等価 (EQual) : `EQ`
- 小なりイコール (Less Than Equal) : `LTE`
- 大なりイコール (Grater Than Equal) : `GTE`

### ゼロ等価 `IS_ZERO`

`IS_ZERO` は引数 n が `0` と同等かを判定し、`TRUE` または `FALSE` を返します。

```
IS_ZERO(0)  # => TRUE
IS_ZERO(1)  # => FALSE
IS_ZERO(6)  # => FALSE
```

### 等価 (EQual) `EQ`

`EQ` は引数 m, n が等しければ `TRUE` を等しくなければ `FALSE` を返します。

```
EQ(3, 3)  # => TRUE
EQ(3, 4)  # => FALSE
```

### 小なりイコール (Less Than Equal) `LTE`

`LTE` は引数 m, n に対して m が n より小さいか等しければ `TRUE` を、さもなくば `FALSE` を返します。

```
LTE(3, 4)  # => TRUE
LTE(3, 3)  # => TRUE
LTE(3, 2)  # => FALSE
```

### 大なりイコール (Grater Than Equal) `GTE`

`GTE` は引数 m, n に対して m が n より大きいか等しければ `TRUE` を、さもなくば `FALSE` を返します。

```
GTE(3, 4)  # => FALSE
GTE(3, 3)  # => TRUE
GTE(3, 2)  # => TRUE
```
