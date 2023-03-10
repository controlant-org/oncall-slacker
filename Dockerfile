FROM ubuntu:20.04

RUN apt-get update && apt-get install -y libssl-dev ca-certificates

COPY target/release/oncall-slacker /oncall-slacker

ENTRYPOINT ["/oncall-slacker"]
