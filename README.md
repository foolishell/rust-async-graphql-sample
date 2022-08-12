# README
Rust でクレートの分離を利用して、オニオンアーキテクチャを実現できるかの実験。

## 題材
簡易な注文システムを題材とする
```mermaid
erDiagram
    PRODUCT {
        string productId
        string name
        int price
        int inventoryQuantity
    }
    CUSTOMER ||--o{ ORDER : places
    CUSTOMER {
        string customerId
        string name
    }
    ORDER ||--|{ LINE-ITEM : contains
    ORDER {
        string orderId
        string Status
        int totalPrice
    }
    LINE-ITEM {
        string productId
        int quantity
        int unitPrice
        int totalPrice
    }
```

## Architecture
### オニオンアーキテクチャ
```mermaid
flowchart TD
    PRE --> USECASE;
    USECASE --> DOMAIN;
    INFRA --> USECASE;
    INFRA --> DOMAIN;

    PRE[プレゼンテーション層]
    USECASE[ユースケース層]
    DOMAIN[ドメイン層]
    INFRA[インフラ層]
```
[ドメイン駆動設計 モデリング/実装ガイド](https://booth.pm/ja/items/1835632)を参照

### 依存関係
```mermaid
graph TD;
    subgraph pj_app
        main.rs --> config;
        main.rs --> infra/repository;
    end
    subgraph pj_graphql
        schema;
    end
    subgraph pj_core
        usecase;
        service;
        event;
        model;
        repository_trait;
        service-->repository_trait;
    end

    main.rs --> schema;
    main.rs --> usecase;
    usecase --> service;
    usecase --> repository_trait;
    usecase --> event;
    usecase --> model;
    infra/repository --> repository_trait;
```



### ルール
- アプリ外通信が発生する部分は全て、インフラ層に配置する。
- `pj_common`は全ての`crate`から参照可能だが、極力利用しない
