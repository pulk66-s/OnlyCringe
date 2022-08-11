FROM python:latest

COPY imgServer /imgServer
WORKDIR /imgServer
RUN pip3 install -r requirements.txt
ENTRYPOINT flask run --host=0.0.0.0