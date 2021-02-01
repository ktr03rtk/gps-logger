FROM rust:1.49

RUN apt update

ENV APP_ROOT /opt/gps-logger
WORKDIR $APP_ROOT
COPY . .

CMD /bin/sh
