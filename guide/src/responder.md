# Converting to HTTP Responses

コンビネータにより返された値は、クライアントに送信する前に HTTP レスポンスへと変換する必要があります。
Finchers では、この変換処理を `Responder` というトレイトを用いて抽象化しています。

`Responder` は次のように定義されています。

```rust
trait Responder {
    type Body;
    type Error;

    fn respond(self: input: PinMut<Input>)
        -> Result<Response<Self::Body>, Self::Error>;
}
```