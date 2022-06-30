FROM ttbb/compile:rust AS build
COPY . /opt/sh/compile
WORKDIR /opt/sh/compile
RUN /root/.cargo/bin/cargo build


FROM ttbb/iotdb:nake

COPY docker-build /opt/sh/iotdb/mate

COPY --from=build /opt/sh/compile/target/debug/iotdb-mate-rust /opt/sh/iotdb/mate/iotdb-mate

WORKDIR /opt/sh/iotdb

CMD ["/usr/bin/dumb-init", "bash", "-vx", "/opt/sh/iotdb/mate/scripts/start.sh"]
