(function() {var implementors = {};
implementors["futures"] = [];
implementors["futures_util"] = [{text:"impl&lt;T, E&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink01</a> for <a class=\"struct\" href=\"futures_util/compat/struct.Compat.html\" title=\"struct futures_util::compat::Compat\">Compat</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink03</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/task/spawn/trait.Spawn.html\" title=\"trait core::task::spawn::Spawn\">Spawn</a>,&nbsp;</span>",synthetic:false,types:["futures_util::compat::compat::Compat"]},];
implementors["tokio_udp"] = [{text:"impl&lt;C:&nbsp;<a class=\"trait\" href=\"tokio_io/codec/encoder/trait.Encoder.html\" title=\"trait tokio_io::codec::encoder::Encoder\">Encoder</a>&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_udp/struct.UdpFramed.html\" title=\"struct tokio_udp::UdpFramed\">UdpFramed</a>&lt;C&gt;",synthetic:false,types:["tokio_udp::frame::UdpFramed"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
