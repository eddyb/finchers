# Understanding Endpoint

Finchers では、HTTP アプリケーションにおけるルーティングとリクエストの解析を `Endpoint` というトレイトにより抽象化しています。
大まかに言うと、このトレイトはクライアントからのリクエストを受け取り、ある型の値を返す（非同期な）関数を表します。

説明のために簡略化した `Endpoint` の定義を以下に示します。

```rust
trait Endpoint<'a> {
    type Output: Tuple;
    type Future: TryFuture<Ok = Self::Output, Error = Error> + 'a;

    fn apply(&'a self, cx: &mut Context<'_>) -> EndpointResult<Self::Future>;
}
```

`Context` はクライアントからのリクエストの値と finchers 内部で使用されるコンテキスト値が格納された構造体です。
`EndpointResult<T>` は `apply` の実行結果を表すための型であり、リクエストにマッチした際に返される Future のインスタンス、およびマッチしなかったときの理由の 2 ケースを表す `Result<T, E>` のエイリアスになっています。

Finchers におけるエンドポイントの定義は、基本的には Finch で用いられている `Endpoint[A]` を参考にしています。

通常、ユーザはこのトレイトの実装を直接記述する必要はありません。
その代わり、あらかじめ組み込まれているコンポーネントを組み合わせていくことで Web アプリケーションを構築していきます。
これは丁度、combine などのパーザコンビネータの働きと類似しています。
基本的には、次の 3 つの方法でコンポーネントを組み合わせます:

* 2 つの `Endpoint` を結合し、それらの結果の直積（タプル）を返す
* 2 つの `Endpoint` のうち、リクエストに「よりマッチする」側の出力を返す
* 結果を別の値に変換する

## Built-in Endpoints

WIP

## Composing Endpoints

上に紹介したエンドポイントは、一般的な Web アプリケーションを構築するために用いることを想定した基本的な機能のみを提供します。
実用的な複雑なアプリケーションを実装するためには、これらのコンポーネントを組み合わせていくことで実現します。

ライブラリでは、基本的なコンビネータを提供するためのトレイト `EndpointExt` が用意されており、これをインポートすることで以下のコンビネータが使用可能になります。

### Product
コンビネータ `and` を用いることで、2つのエンドポイントの結果を結合したエンドポイントを作ります。
2つのエンドポイントの結果は、HList という仕組みを用いて単一のタプルに平滑化されます。
これにより、複数のエンドポイントを組み合わせていくことで `Output` の型が入り組んだものになることを防ぐことが可能になります。

```rust
let endpoint1 = path("posts");
let endpoint2 = param::<u32>();

let endpoint = endpoint1.and(endpoint2);
```

上の例の場合、2 つのエンドポイント（それぞれ `()`, `(u32,)` を `Output` に持つ）を組み合わせた結果の型は `(u32,)` となります。
このような結合を常に可能にするため、`Endpoint` の関連型 `Output` が取ることの出来る型がタプル型のみになるような制約が設けられています。

### Mapping

```rust
let endpoint = path("posts").and(param::<u32>()).and(body::parse::<String>())
    .map(|id: u32, body: String| {
        format!("id = {}, body = {}", id, body)
    });
```

* `e.map(f)`
* `e.then(f)`
* `e.and_then(f)`

### Coproduct (`or`)

```rust
let add_post = ...;
let create_post = ...;

let post_api = add_post.or(create_post);
```
