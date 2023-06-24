FROM ghcr.io/graalvm/native-image:ol8-java17 AS base

ARG JAR_FILE="build/libs/btmeister-1.0.0.jar"

COPY build/libs /opt/btmeister/libs
COPY src/native-image/reflect-config.json /opt/btmeister/config/reflect-config.json
COPY build/classes/java/main/META-INF/native-image/picocli-generated/btmeister/resource-config.json /opt/btmeister/config/resource-config.json

RUN native-image --module-path /opt/btmeister/libs \
      --module jp.cafebabe.btmeister/jp.cafebabe.btmeister.cli.Main \
      --add-modules java.logging \
      --enable-url-protocols=https \
      --no-fallback \
      --static \
      -H:+ReportExceptionStackTraces \
      -H:Log=registerResource:5 \
      -H:ReflectionConfigurationFiles=/opt/btmeister/config/reflect-config.json \
      -H:ResourceConfigurationFiles=/opt/btmeister/config/resource-config.json \
      /opt/btmeister/btmeister

FROM alpine:3.10.1

ARG VERSION="1.0.0"

LABEL org.opencontainers.image.authors="Haruaki Tamada <tamada@users.noreply.github.com>" \
      org.opencontainers.image.url="https://github.com/tamada/btmeister" \
      org.opencontainers.image.documentation="Identifies the build tools of the projects" \
      org.opencontainers.image.source="https://raw.githubusercontent.com/tamada/btmeister/Dockerfile" \
      org.opencontainers.image.version="1.0.0"

RUN    adduser -D nonroot

COPY --from=base /opt/btmeister/btmeister /opt/btmeister/btmeister

ENV HOME=/app
ENV BTMEISTER_HOME=/opt/btmeister
ENV BTMEISTER_VERSION=${VERSION}

WORKDIR "/app"
USER nonroot

ENTRYPOINT [ "/opt/btmeister/btmeister" ]