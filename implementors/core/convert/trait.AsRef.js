(function() {var implementors = {};
implementors["arrayvec"] = [{text:"impl&lt;A:&nbsp;<a class=\"trait\" href=\"arrayvec/trait.Array.html\" title=\"trait arrayvec::Array\">Array</a>&lt;Item = u8&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;str&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayString.html\" title=\"struct arrayvec::ArrayString\">ArrayString</a>&lt;A&gt;",synthetic:false,types:["arrayvec::array_string::ArrayString"]},{text:"impl&lt;A:&nbsp;<a class=\"trait\" href=\"arrayvec/trait.Array.html\" title=\"trait arrayvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;[A::<a class=\"type\" href=\"arrayvec/trait.Array.html#associatedtype.Item\" title=\"type arrayvec::Array::Item\">Item</a>]&gt; for <a class=\"struct\" href=\"arrayvec/struct.ArrayVec.html\" title=\"struct arrayvec::ArrayVec\">ArrayVec</a>&lt;A&gt;",synthetic:false,types:["arrayvec::ArrayVec"]},];
implementors["bytes"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"bytes/struct.Bytes.html\" title=\"struct bytes::Bytes\">Bytes</a>",synthetic:false,types:["bytes::bytes::Bytes"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"bytes/struct.BytesMut.html\" title=\"struct bytes::BytesMut\">BytesMut</a>",synthetic:false,types:["bytes::bytes::BytesMut"]},];
implementors["crossbeam_epoch"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;T&gt; for <a class=\"struct\" href=\"crossbeam_epoch/struct.Owned.html\" title=\"struct crossbeam_epoch::Owned\">Owned</a>&lt;T&gt;",synthetic:false,types:["crossbeam_epoch::atomic::Owned"]},];
implementors["either"] = [{text:"impl&lt;L, R, Target&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;Target&gt; for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;Target&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;Target&gt;,&nbsp;</span>",synthetic:false,types:["either::Either"]},];
implementors["failure"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"trait\" href=\"failure/trait.Fail.html\" title=\"trait failure::Fail\">Fail</a>&gt; for <a class=\"struct\" href=\"failure/struct.Error.html\" title=\"struct failure::Error\">Error</a>",synthetic:false,types:["failure::error::Error"]},];
implementors["finchers"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"trait\" href=\"finchers/error/trait.HttpError.html\" title=\"trait finchers::error::HttpError\">HttpError</a>&gt; for <a class=\"struct\" href=\"finchers/error/struct.Error.html\" title=\"struct finchers::error::Error\">Error</a>",synthetic:false,types:["finchers::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"finchers/input/struct.EncodedStr.html\" title=\"struct finchers::input::EncodedStr\">EncodedStr</a>",synthetic:false,types:["finchers::input::encoded::EncodedStr"]},{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"finchers/output/struct.Binary.html\" title=\"struct finchers::output::Binary\">Binary</a>&lt;T&gt;",synthetic:false,types:["finchers::output::binary::Binary"]},{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"finchers/output/struct.Text.html\" title=\"struct finchers::output::Text\">Text</a>&lt;T&gt;",synthetic:false,types:["finchers::output::text::Text"]},];
implementors["futures_util"] = [{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"futures_util/io/struct.Window.html\" title=\"struct futures_util::io::Window\">Window</a>&lt;T&gt;",synthetic:false,types:["futures_util::io::window::Window"]},];
implementors["http"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/header/struct.HeaderName.html\" title=\"struct http::header::HeaderName\">HeaderName</a>",synthetic:false,types:["http::header::name::HeaderName"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"http/header/struct.HeaderName.html\" title=\"struct http::header::HeaderName\">HeaderName</a>",synthetic:false,types:["http::header::name::HeaderName"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"http/header/struct.HeaderValue.html\" title=\"struct http::header::HeaderValue\">HeaderValue</a>",synthetic:false,types:["http::header::value::HeaderValue"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/method/struct.Method.html\" title=\"struct http::method::Method\">Method</a>",synthetic:false,types:["http::method::Method"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/uri/struct.Authority.html\" title=\"struct http::uri::Authority\">Authority</a>",synthetic:false,types:["http::uri::authority::Authority"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"http/uri/struct.Scheme.html\" title=\"struct http::uri::Scheme\">Scheme</a>",synthetic:false,types:["http::uri::scheme::Scheme"]},];
implementors["hyper"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"hyper/struct.Chunk.html\" title=\"struct hyper::Chunk\">Chunk</a>",synthetic:false,types:["hyper::body::chunk::Chunk"]},];
implementors["hyperx"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"hyperx/enum.Method.html\" title=\"enum hyperx::Method\">Method</a>",synthetic:false,types:["hyperx::method::Method"]},];
implementors["mime"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"mime/struct.Mime.html\" title=\"struct mime::Mime\">Mime</a>",synthetic:false,types:["mime::Mime"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"mime/struct.Name.html\" title=\"struct mime::Name\">Name</a>&lt;'a&gt;",synthetic:false,types:["mime::Name"]},];
implementors["owning_ref"] = [{text:"impl&lt;O, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;T&gt; for <a class=\"struct\" href=\"owning_ref/struct.OwningRef.html\" title=\"struct owning_ref::OwningRef\">OwningRef</a>&lt;O, T&gt;",synthetic:false,types:["owning_ref::OwningRef"]},{text:"impl&lt;O, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;T&gt; for <a class=\"struct\" href=\"owning_ref/struct.OwningRefMut.html\" title=\"struct owning_ref::OwningRefMut\">OwningRefMut</a>&lt;O, T&gt;",synthetic:false,types:["owning_ref::OwningRefMut"]},];
implementors["smallvec"] = [{text:"impl&lt;A:&nbsp;<a class=\"trait\" href=\"smallvec/trait.Array.html\" title=\"trait smallvec::Array\">Array</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a>A::<a class=\"type\" href=\"smallvec/trait.Array.html#associatedtype.Item\" title=\"type smallvec::Array::Item\">Item</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"smallvec/struct.SmallVec.html\" title=\"struct smallvec::SmallVec\">SmallVec</a>&lt;A&gt;",synthetic:false,types:["smallvec::SmallVec"]},];
implementors["tokio_io"] = [{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"tokio_io/io/struct.Window.html\" title=\"struct tokio_io::io::Window\">Window</a>&lt;T&gt;",synthetic:false,types:["tokio_io::window::Window"]},];
implementors["unicase"] = [{text:"impl&lt;S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"unicase/struct.Ascii.html\" title=\"struct unicase::Ascii\">Ascii</a>&lt;S&gt;",synthetic:false,types:["unicase::Ascii"]},{text:"impl&lt;S:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"unicase/struct.UniCase.html\" title=\"struct unicase::UniCase\">UniCase</a>&lt;S&gt;",synthetic:false,types:["unicase::UniCase"]},];
implementors["url"] = [{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"url/struct.Url.html\" title=\"struct url::Url\">Url</a>",synthetic:false,types:["url::Url"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
