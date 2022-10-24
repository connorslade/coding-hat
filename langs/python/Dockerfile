FROM python:latest

RUN useradd -m runner -d /runner
RUN usermod -p '!!' root

USER runner
WORKDIR /runner

ADD --chown=runner run.sh .
ADD --chown=runner main.py .

ENTRYPOINT ["bash", "./run.sh"]