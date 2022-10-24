FROM openjdk:latest

RUN useradd -m runner -d /runner
RUN usermod -p '!!' root

USER runner
WORKDIR /runner

ADD --chown=runner run.sh .
ADD --chown=runner Main.java .

ENTRYPOINT ["bash", "./run.sh"]