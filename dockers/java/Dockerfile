FROM openjdk:17-slim-bullseye as builder

RUN jlink --module-path /opt/java/openjdk/jmods \
    --compress=2 \
    --add-modules java.base,java.logging \
    --no-header-files  \
    --no-man-pages  \
    --output /opt/minimaljre

FROM debian:bullseye-slim

ARG VERSION="1.0.0-beta-1"

LABEL org.opencontainers.image.authors="Haruaki Tamada <tamada@users.noreply.github.com>" \
      org.opencontainers.image.url="https://github.com/tamada/btmeister" \
      org.opencontainers.image.documentation="Detecting software theft, the birthmarking toolkit for the Java platform." \
      org.opencontainers.image.source="https://raw.githubusercontent.com/tamada/btmeister/blob/main/dockers/minimaljre/Dockerfile" \
      org.opencontainers.image.version="${VERSION}"

RUN adduser --disabled-login --disabled-password --no-create-home nonroot \
    && mkdir -p /opt/btmeister/libs
COPY --from=builder /opt/minimaljre /opt/minimaljre
ADD build/libs /opt/btmeister/libs

ENV BTMEISTER_HOME=/opt/btmeister
ENV JAVA_HOME=/opt/minimaljre
ENV PATH=$JAVA_HOME/bin:$PATH
ENV HOME=/app
ENV BTMEISTER_VERSION=${VERSION}

WORKDIR /app
USER nonroot

ENTRYPOINT [ "java", "-p", "/opt/btmeister/libs/", "-m", "jp.cafebabe.btmeister/jp.cafebabe.btmeister.cli.Main" ]
